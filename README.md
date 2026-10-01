# KiloDB 🚀

[![Rust](https://github.com/madhavkhoslaa/KiloDB/actions/workflows/rust.yml/badge.svg)](https://github.com/madhavkhoslaa/KiloDB/actions/workflows/rust.yml)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

**KiloDB** is a high-performance, Redis-compatible key-value database written in Rust. It implements the Redis protocol (RESP) and provides a subset of Redis commands, making it a lightweight alternative for applications that need Redis-like functionality.

## ✨ Features

### 🔑 Core Data Types & Commands
- **Strings**: SET, GET, DEL, EXISTS, INCR, DECR, APPEND, STRLEN, MGET, MSET
- **Hashes**: HSET, HGET, HGETALL, HDEL, HEXISTS, HLEN, HKEYS, HVALS
- **Lists**: LPUSH, RPUSH, LPOP, RPOP, LLEN, LRANGE, LINDEX
- **Sets**: SADD, SREM, SMEMBERS, SISMEMBER, SCARD, SUNION, SINTER
- **Key Management**: KEYS, TYPE, TTL, EXPIRE, PERSIST, RENAME

### 🚀 Performance Features
- **Multithreaded** - one thread per connection, no more waiting behind other clients
- **Read/write lock on the store** - concurrent reads don't block each other
- **In-memory storage** with fast access patterns
- **RESP protocol implementation** for Redis client compatibility
- **Efficient data structures** optimized for Rust

### 🔌 Protocol Support
- **Redis Protocol (RESP)** - Compatible with existing Redis clients
- **TCP server** listening on standard Redis port (6379)
- **Connection handling** with proper client lifecycle management

## 📈 Changelog

### Multithreaded TCP server + ioredis compatibility
**Changing:** the accept loop ran each client to completion before accepting
the next, and the RESP parser only looked at a single 512-byte read, so
pipelined or oversized commands broke. **To:** one thread per connection
sharing state through a lock, and a buffered parser that drains as many
complete commands as are already in the buffer. Also added the `HELLO`/
`CLIENT`/`INFO`/`QUIT` handshake real clients (ioredis, etc.) expect, and
fixed a bug where keys sharing a TTL could silently wipe each other out.

**Performance gain:** at 20 concurrent connections, the old server served
1 client and left the other 19 stuck at 1 op each in 3 seconds. After this
change all 20 are served evenly, 165k ops/sec total (up from 77k ops/sec
on just the one connection that got through). Full numbers in
`bench/RESULTS.md`.

### Mutex → RwLock on the shared store
**Changing:** the store was behind a single `Mutex`, so every command -
including `GET`, which never mutates anything - took an exclusive lock.
Two connections both reading serialized behind each other exactly like
there was only one connection. **To:** `RwLock`, with read-only commands
(`GET`, `HGET`, `EXISTS`, `DBSIZE`, `PING`, `ECHO`) dispatched under a
read guard and everything else under a write guard.

**Performance gain:** concurrent `GET` on one hot key, no writes:

| concurrency | Mutex | RwLock | delta |
|---|---|---|---|
| 1 | 119k ops/sec | 111k ops/sec | noise |
| 50 | 167k ops/sec | 179k ops/sec | +7% |
| 100 | 186k ops/sec | 216k ops/sec | +16% |

Gap widens with concurrency, as expected for lock contention. Full
numbers in `bench/RESULTS.md`.

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))

### Installation & Running

1. **Clone the repository**
   ```bash
   git clone https://github.com/madhavkhoslaa/KiloDB.git
   cd KiloDB
   ```

2. **Build the project**
   ```bash
   cargo build --release
   ```

3. **Run KiloDB**
   ```bash
   cargo run --release
   ```

4. **Connect with Redis client**
   ```bash
   redis-cli -p 6379
   ```

### Example Usage

```bash
# Start KiloDB server
cargo run

# In another terminal, connect with redis-cli
redis-cli -p 6379

## 🛠️ Development

### Building
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Check code quality
cargo clippy
```

### Adding New Commands
1. Add the command to `src/command/command_enum.rs`
2. Implement execution logic in `src/command/executor/`
3. Add tests in `tests/` directory

## 🗺️ Roadmap

### 🎯 Stage 1 [Current]
- ✅ Implement core Redis commands and data structures
- ✅ Basic RESP protocol support
- ✅ TCP server implementation

### 🚀 Stage 2
- [ ] TTL (Time To Live) support
- [ ] Cache eviction mechanisms (LRU, LFU)
- [ ] Memory usage optimization

### ⚡ Stage 3
- [ ] Async/await implementation
- [x] Multi-threading support
- [ ] Improved connection handling

### 🛡️ Stage 4
- [ ] Race condition elimination
- [ ] Mutex poisoning prevention
- [ ] Production-ready stability
