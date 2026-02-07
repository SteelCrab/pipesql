# PipeSQL Roadmap

> [한국어 (Korean)](./ROADMAP_KR.md)

> Development roadmap for the PipeSQL project.

## Current Status

- **Branch**: `feat/repl`
- **Progress**: ~15%
- Basic TUI framework established
- Storage module structure defined (currently inactive)

---

## Phase 1: Foundation

> Completion of the project's core infrastructure

| Task | Description | Status |
|------|-------------|--------|
| REPL Basic UI | ratatui-based terminal UI | ✅  |
| Keyboard Event Handling | crossterm event loop | ✅  |
| Activate Storage Module | Enable module in `lib.rs` |   |
| Enable Test Code | Restore commented-out tests |   |
| Cleanup Project Settings | Update Cargo.toml edition |   |

---

## Phase 2: Storage Engine

> Core features for in-memory data storage

| Task | Description | Status |
|------|-------------|--------|
| Extend Data Types | Add `Text`, `Boolean`, `Float`, `Timestamp` |   |
| CRUD Operations | Implement Insert, Select, Update, Delete |   |
| DashMap Integration | Lock-free concurrent table storage |   |
| Primary Key | Primary key constraints |   |
| Indexing | B-Tree based indexing |   |

---

## Phase 3: SQL Parser & Executor

> SQL query processing pipeline

| Task | Description | Status |
|------|-------------|--------|
| Tokenizer | SQL lexing stage |   |
| AST Parser | `CREATE`, `INSERT`, `SELECT`, `UPDATE`, `DELETE` |   |
| Query Planner | Establish execution plans |   |
| Query Executor | Execute with storage integration |   |
| WHERE Clause | Conditional filtering |   |
| ORDER BY / LIMIT | Sorting and paging |   |

---

## Phase 4: REPL Completion (Interactive Shell)

> User-friendly terminal interface

| Task | Description | Status |
|------|-------------|--------|
| SQL Input Area | Multi-line input support |   |
| Display Result Table | Formatted query result output |   |
| Command History | Integration with rustyline |   |
| Autocomplete | Table names, column names, keywords |   |
| Error Display | Friendly error message UI |   |
| Meta Commands | `.tables`, `.schema`, `.help`, etc. |   |

---

## v0.1.0 Release

> Release v0.1.0 upon completion of Phase 1-4
>
> **Core Features**: SQL Parser + Basic Query Execution + REPL Interface

---

## Phase 5: Network Layer (QUIC Protocol)

> High-performance network communication

| Task | Description | Status |
|------|-------------|--------|
| QUIC Server | Server based on quinn/s2n-quic |   |
| Client Connection Management | Multiplexing streams |   |
| Wire Protocol | Binary format for queries/responses |   |
| Authentication | Basic authentication layer |   |
| TLS Configuration | Secure connection |   |
| Connection Pooling | Optimization for connection reuse |   |

---

## Phase 5.5: Client SDK - 1st Wave (Rust, Python)

> Prioritize development of core language SDKs

| Task | Description | Status |
|------|-------------|--------|
| Rust Client | Native Rust SDK |   |
| Python Client | pipesql-python (using PyO3) |   |

---

## v0.2.0 Release

> Release v0.2.0 upon completion of Phase 5-5.5
>
> **Core Features**: Network Server based on QUIC Protocol + Rust/Python SDKs

---

## Phase 6: Client SDK - 2nd Wave (Java, JavaScript)

> Development of additional language SDKs

| Task | Description | Status |
|------|-------------|--------|
| Java Client | pipesql-java (JNI or pure Java) |   |
| JavaScript/Node.js | pipesql-js (using NAPI-RS) |   |

---

## v0.3.0 Release

> Release v0.3.0 upon completion of Phase 6
>
> **Core Features**: Addition of Java/JavaScript SDKs

---

## Phase 7: Advanced SQL

> Expansion of SQL capabilities

| Task | Description | Status |
|------|-------------|--------|
| JOIN | INNER, LEFT, RIGHT JOIN |   |
| Aggregate Functions | COUNT, SUM, AVG, MIN, MAX |   |
| GROUP BY / HAVING | Grouping and filtering |   |
| Subqueries | Nested query support |   |
| Transactions | BEGIN, COMMIT, ROLLBACK |   |
| MVCC | Multi-Version Concurrency Control |   |

---

## Phase 8: Persistence & Reliability

> Ensuring data stability

| Task | Description | Status |
|------|-------------|--------|
| Data Persistence | Snapshot save/restore |   |
| WAL | Write-Ahead Logging |   |
| Replication | Leader-Follower replication |   |
| Crash Recovery | Failure recovery mechanisms |   |

---

## Phase 9: Operations & Deployment

> Tools for deployment and operations

| Task | Description | Status |
|------|-------------|--------|
| CLI Completion | Command-line interface based on clap |   |
| Configuration Files | Support for TOML configuration |   |
| Metrics | Prometheus-compatible metrics |   |
| Health Check | Status query endpoint |   |
| Docker | Container images |   |
| CI/CD | GitHub Actions pipelines |   |
| Documentation | API documentation, user guides |   |

---

## Next Steps

Priority tasks for completing Phase 1:

1. `src/lib.rs` - Uncomment storage module
2. `src/tests/table.rs` - Enable test code
3. `Cargo.toml` - Update edition to `"2021"`
4. REPL - Add SQL input functionality
5. Storage - Implement basic CRUD operations

---

## Milestones

| Version | Goal | Phase |
|---------|------|-------|
| **v0.1.0** | **First Release - Complete Basic SQL DB Functionality** | **Phase 1-4** |
| v0.2.0 | QUIC Network Server + Rust/Python SDK | Phase 5-5.5 |
| v0.3.0 | Java/JavaScript SDK | Phase 6 |
| v0.4.0 | JOIN, Aggregate Functions, Transactions | Phase 7 |
| v0.5.0 | Data Persistence, WAL | Phase 8 |
| v0.6.0+ | Operations and Deployment Tools | Phase 9 |

---

## Contributing

Please refer to [CONTRIBUTING.md](CONTRIBUTING.md).
