// raw RESP, no client lib - works against the old single-threaded server
// too since that one never implemented HELLO/INFO and can't do a real
// ioredis handshake. opens `concurrency` connections, each firing SET/GET
// back-to-back for `durationMs`, reports total ops/sec
const net = require("net");

function encodeCommand(args) {
  let out = `*${args.length}\r\n`;
  for (const a of args) out += `$${a.length}\r\n${a}\r\n`;
  return out;
}

function runConnection(port, durationMs, id) {
  return new Promise((resolve, reject) => {
    const sock = net.connect(port, "127.0.0.1");
    let ops = 0;
    let stopped = false;
    let buf = "";

    const timer = setTimeout(() => {
      stopped = true;
      sock.end();
    }, durationMs);

    function sendNext() {
      if (stopped) return;
      const key = `bench:${id}:${ops % 1000}`;
      const cmd = ops % 2 === 0 ? encodeCommand(["SET", key, "v"]) : encodeCommand(["GET", key]);
      sock.write(cmd);
    }

    sock.on("connect", sendNext);

    sock.on("data", (d) => {
      buf += d.toString();
      // one reply per write here
      while (buf.includes("\r\n")) {
        const nl = buf.indexOf("\r\n");
        const line = buf.slice(0, nl + 2);
        buf = buf.slice(nl + 2);
        // bulk strings have a data line after the length line
        if (line.startsWith("$") && !line.startsWith("$-1")) {
          const len = parseInt(line.slice(1), 10);
          if (len >= 0) {
            const dataEnd = buf.indexOf("\r\n");
            if (dataEnd === -1) { buf = line + buf; break; }
            buf = buf.slice(dataEnd + 2);
          }
        }
        ops++;
        sendNext();
        break;
      }
    });

    sock.on("error", (e) => reject(e));
    sock.on("close", () => {
      clearTimeout(timer);
      resolve(ops);
    });
  });
}

async function main() {
  const port = parseInt(process.argv[2] || "6379", 10);
  const concurrency = parseInt(process.argv[3] || "10", 10);
  const durationMs = parseInt(process.argv[4] || "3000", 10);

  const start = Date.now();
  const results = await Promise.all(
    Array.from({ length: concurrency }, (_, i) => runConnection(port, durationMs, i))
  );
  const elapsed = (Date.now() - start) / 1000;
  const total = results.reduce((a, b) => a + b, 0);

  console.log(JSON.stringify({
    port, concurrency, durationMs,
    totalOps: total,
    opsPerSec: Math.round(total / elapsed),
    perConnection: results,
  }, null, 2));
}

main().catch((e) => { console.error("BENCH FAILED:", e.message); process.exit(1); });
