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
- **Single-threaded architecture** for predictable performance
- **In-memory storage** with fast access patterns
- **RESP protocol implementation** for Redis client compatibility
- **Efficient data structures** optimized for Rust

### 🔌 Protocol Support
- **Redis Protocol (RESP)** - Compatible with existing Redis clients
- **TCP server** listening on standard Redis port (6379)
- **Connection handling** with proper client lifecycle management

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
- [ ] Multi-threading support
- [ ] Improved connection handling

### 🛡️ Stage 4
- [ ] Race condition elimination
- [ ] Mutex poisoning prevention
- [ ] Production-ready stability
