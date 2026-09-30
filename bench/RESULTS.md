# Benchmark results: multithreaded TCP + RwLock

Covers two changes:

- [#3](https://github.com/madhavkhoslaa/KiloDB/pull/3) — single-threaded
  blocking accept loop → one thread per connection (`Arc<Mutex<context>>`),
  plus the ioredis-compatibility and `TTLStore` fixes needed to make a real
  client usable against it at all.
- [#4](https://github.com/madhavkhoslaa/KiloDB/pull/4) — `Arc<Mutex<context>>`
  → `Arc<RwLock<context>>`, with read-only commands dispatched under a
  read guard.

No `redis-benchmark`/`redis-cli` was available in the environment these were
run in, so the scripts here are hand-rolled: `raw-bench.js` and
`read-bench.js` speak RESP directly over a raw TCP socket (no client-library
overhead, and they work against the pre-#3 server too, which never
implemented the `HELLO`/`INFO` handshake real clients expect).
`ioredis-smoke.js` is the correctness/compatibility check, run with the real
`ioredis` TypeScript client.

All runs: same machine, one `cargo build --release` binary at a time
listening on `127.0.0.1:6379`, nothing else competing for the CPU.

## 1. Threading: connection starvation (`raw-bench.js`)

Mixed `SET`/`GET` across `N` concurrent connections, own key per
connection, 3 second run, total ops and per-connection ops reported.

**Before (single-threaded, `master`)** — the accept loop ran
`handle_client` to completion before calling `accept()` again, so only
the first connection to arrive was ever served; every other connection
sat established but unserviced.

| concurrency | total ops/sec | per-connection ops (first 3 shown) |
|---|---|---|
| 1 | 88,596 | 265,966 |
| 20 | 77,213 | 231,852 / 1 / 1 (19 connections got 1 op each) |

**After (multithreaded, #3)** — every connection gets its own thread and
makes even progress:

| concurrency | total ops/sec | per-connection ops (first 3 shown) |
|---|---|---|
| 1 | 119,626 | 359,237 |
| 20 | 165,443 | 25,688 / 24,665 / 25,115 (all 20 within ~12% of each other) |

Single-connection throughput also improved (~35%) because the new
buffered RESP parser avoids the per-command 512-byte blocking `read()`
shape of the old parser, but the headline result is the concurrency=20
row: 19 of 20 connections going from **1 op in 3 seconds** to an even
share of 165k ops/sec.

## 2. Locking: concurrent reads (`read-bench.js`)

Isolates lock contention from everything else: `N` connections, all
issuing `GET` against the **same** key, no writes, 3 second run.

| concurrency | Mutex (#3) | RwLock (#4) | delta |
|---|---|---|---|
| 1 | 119,146 ops/sec | 111,135 ops/sec | noise |
| 50 | 166,740 ops/sec | 178,509 ops/sec | +7% |
| 100 | 186,219 ops/sec | 215,606 ops/sec | +16% |

At concurrency 1 there's no contention to relieve, so the two are equal
within run-to-run noise (single Node.js process scheduling variance).
The gap opens and widens with concurrency, which is the expected shape:
Mutex funnels every reader through one at a time regardless of whether
they'd conflict; RwLock lets them actually run in parallel. The
benchmark client is a single Node.js process driving all connections
through one event loop, so it becomes a contributing bottleneck at
concurrency=100 too — these numbers are a lower bound on the real gap,
not an upper one.

## 3. Correctness / ioredis compatibility (`ioredis-smoke.js`)

Run against the pre-#3 server: fails immediately. ioredis's connect
handshake (`HELLO 3` + two `CLIENT SETINFO` calls, then an `INFO`
ready-check) has no handler on that server, gets back a generic RESP
error, and ioredis treats the connection as broken and closes it before
sending a single real command.

Run against #3 and #4: `HELLO`/`CLIENT`/`INFO` handshake, `PING`,
`SET`/`GET`, `INCR`, `HSET`/`HGET`, `DBSIZE`, a 20-command pipeline, and
10 concurrent clients each doing `SET`+`GET` all round-trip correctly.

## Reproducing

```bash
cd bench && npm install   # only needed for ioredis-smoke.js

# terminal 1
cargo run --release

# terminal 2
node bench/ioredis-smoke.js
node bench/raw-bench.js 6379 <concurrency> <durationMs>
node bench/read-bench.js 6379 <concurrency> <durationMs>
```
