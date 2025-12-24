# VedDB Rust Client Overview

Welcome to the VedDB Rust client documentation. This document provides a high-level view of the project structure, design goals, and the relationship between the library, CLI, and the core VedDB server.

## ✨ Project Goals

- **High-performance client** with async/await support built on Tokio.
- **Unified experience** across library and CLI tooling.
- **Simple integration** for Rust developers building services on top of VedDB.
- **Platform focus** on Windows (current production target) with planned cross-platform support.

## 📦 Repository Structure

```
ved-db-rust-client/
├── src/             # Library source code
│   ├── lib.rs       # Public API exports and docs
│   ├── connection.rs
│   ├── error.rs
│   ├── types.rs
│   └── bin/
│       ├── veddb-cli.rs   # CLI entrypoint
│       └── bench.rs       # Internal benchmarking tool
├── examples/       # Library usage examples
├── benches/        # Criterion benchmarks (planning)
├── tests/          # Integration tests
├── docs/           # Developer documentation
├── Cargo.toml      # Package manifest
└── Justfile        # Productivity commands
```

## 🧱 Architectural Components

- **`Client`**: High-level asynchronous client that manages connections and provides the VedDB API (set, get, delete, list, ping).
- **`Connection`**: Low-level TCP protocol implementation handling framing, retries, and error translation.
- **`ConnectionPool`**: Configurable connection pooling for parallel workloads.
- **`Error`**: Rich error type capturing protocol, I/O, and server errors.
- **`veddb-cli`**: CLI tool built on the same library code, demonstrating idiomatic usage patterns.

## 🔄 Data Flow

1. Applications construct a `Client` using `Client::connect` or `Client::with_pool_size`.
2. The `Client` delegates to `Connection` instances that implement the VedDB binary protocol.
3. Responses are parsed into strongly typed `Response` objects with helpful status codes.
4. Errors are propagated via the unified `Error` enum to the caller.
5. The CLI reuses the library capabilities to provide a human friendly interface.

## 📚 Documentation Set

This overview is part of a documentation suite:

- [Installation](./installation.md)
- [Library Usage Guide](./library.md)
- [CLI Usage Guide](./cli.md)
- [Configuration & Tuning](./configuration.md)
- [Troubleshooting](./troubleshooting.md)

## 🗺️ Roadmap Snapshot

- ✅ Core KV operations and pooling
- ✅ CLI for interactive workflows
- ⏳ Batch operations
- ⏳ Pub/Sub support
- ⏳ Linux & macOS builds
- ⏳ TLS and authentication

## 🔗 Related Projects

- VedDB Server: https://github.com/Mihir-Rabari/ved-db-server
- VedDB JavaScript Client: https://github.com/Mihir-Rabari/veddb-js-client
- VedDB API Gateway: https://github.com/Mihir-Rabari/veddb-api

---

Continue with [Installation](./installation.md) to get the tooling set up locally.
