// isolates the Mutex vs RwLock difference - every connection hammers GET
// on the SAME key, no writes at all. Mutex serializes all of it regardless
// of concurrency; RwLock lets readers run in parallel
const net = require("net");

function encodeCommand(args) {
  let out = `*${args.length}\r\n`;
  for (const a of args) out += `$${a.length}\r\n${a}\r\n`;
  return out;
}

function seed(port) {
  return new Promise((resolve, reject) => {
    const sock = net.connect(port, "127.0.0.1");
    sock.on("connect", () => sock.write(encodeCommand(["SET", "hotkey", "hello-world-value"])));
    sock.on("data", () => { sock.end(); resolve(); });
    sock.on("error", reject);
  });
}

function runConnection(port, durationMs) {
  return new Promise((resolve, reject) => {
    const sock = net.connect(port, "127.0.0.1");
    let ops = 0;
    let stopped = false;
    const cmd = encodeCommand(["GET", "hotkey"]);

    const timer = setTimeout(() => { stopped = true; sock.end(); }, durationMs);

    sock.on("connect", () => sock.write(cmd));
    sock.on("data", () => {
      ops++;
      if (!stopped) sock.write(cmd);
    });
    sock.on("error", (e) => reject(e));
    sock.on("close", () => { clearTimeout(timer); resolve(ops); });
  });
}

async function main() {
  const port = parseInt(process.argv[2] || "6379", 10);
  const concurrency = parseInt(process.argv[3] || "20", 10);
  const durationMs = parseInt(process.argv[4] || "3000", 10);

  await seed(port);

  const start = Date.now();
  const results = await Promise.all(
    Array.from({ length: concurrency }, () => runConnection(port, durationMs))
  );
  const elapsed = (Date.now() - start) / 1000;
  const total = results.reduce((a, b) => a + b, 0);

  console.log(JSON.stringify({
    mode: "GET-only, same key",
    port, concurrency, durationMs,
    totalOps: total,
    opsPerSec: Math.round(total / elapsed),
  }, null, 2));
}

main().catch((e) => { console.error("BENCH FAILED:", e.message); process.exit(1); });
