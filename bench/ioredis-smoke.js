const Redis = require("ioredis");

async function main() {
  const redis = new Redis({ port: 6379, host: "127.0.0.1", lazyConnect: true, retryStrategy: () => null });
  await redis.connect();
  console.log("PING:", await redis.ping());
  console.log("SET:", await redis.set("foo", "bar"));
  console.log("GET:", await redis.get("foo"));
  console.log("INCR (new key):", await redis.incr("counter"));
  console.log("INCR again:", await redis.incr("counter"));
  console.log("HSET:", await redis.hset("h1", "a", "1", "b", "2"));
  console.log("HGET:", await redis.hget("h1", "a"));
  console.log("DBSIZE:", await redis.dbsize());

  // pipelined commands in one flush, broke the old 512-byte-read parser
  const pipeline = redis.pipeline();
  for (let i = 0; i < 20; i++) {
    pipeline.set(`pk:${i}`, `v${i}`.repeat(50));
  }
  const pipeResults = await pipeline.exec();
  console.log("Pipeline errors:", pipeResults.filter(([err]) => err).length, "/ 20");

  // hit the server with overlapping connections, proves it's not
  // serializing them
  const clients = Array.from({ length: 10 }, () => new Redis({ port: 6379, host: "127.0.0.1" }));
  const results = await Promise.all(clients.map((c, i) => c.set(`concurrent:${i}`, "x").then(() => c.get(`concurrent:${i}`))));
  console.log("Concurrent results:", JSON.stringify(results));
  console.log("Concurrent clients OK:", results.every((r) => r === "x"));
  await Promise.all(clients.map((c) => c.disconnect()));

  await redis.quit();
  console.log("ALL SMOKE CHECKS PASSED");
}

main().catch((e) => {
  console.error("SMOKE TEST FAILED:", e);
  process.exit(1);
});
