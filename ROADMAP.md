# PipeSQL Roadmap

> [한국어 (Korean)](./ROADMAP_KR.md)

> Development roadmap for the PipeSQL project.

## Current Status

- **Branch**: `feat/repl`
- **Progress**: ~15%
- Basic TUI framework established
- Storage module structure defined (currently inactive)

---

## Phase 1: Foundation & Interactive Shell

> **Goal**: Successful project build + typeable REPL shell

| Task | Description | Status |
|------|-------------|--------|
| REPL Basic UI | ratatui-based terminal UI | ✅  |
| Keyboard Event Handling | crossterm event loop | ✅  |
| Cleanup Project Settings |  fix code typos (`reuslt`→`result`, `ecs`→`esc`) |   |
| Activate Storage Module | Uncomment in `lib.rs`, restore tests, fix compile errors |   |
| REPL Layout | 3-panel layout: input area + result area + status bar |   |
| Text Input Handling | Character input, backspace, cursor movement, Enter to execute |   |
| Basic Meta Commands | `.quit`, `.help` (commands that work without SQL) |   |
| Error Handling Structure | Unified error type definition, REPL error message display area |   |

**Phase 1 complete**: Launch app → type text → Enter echoes input (no SQL execution yet)

---

## Phase 2: Storage Engine + Basic SQL

> **Goal**: `CREATE TABLE` → `INSERT` → `SELECT *` end-to-end working

| Task | Description | Status |
|------|-------------|--------|
| Extend Data Types | Add `Text`, `Boolean`, `Float` (currently Integer only) |   |
| Basic SQL Tokenizer | SQL string → token separation |   |
| AST Parser (Basic) | Parse `CREATE TABLE`, `INSERT INTO`, `SELECT * FROM` |   |
| Storage CRUD (Basic) | Create/drop table, insert row, select all rows |   |
| Execution Engine Integration | SQL input → parser → storage → return results |   |
| Result Table Display | Formatted table output in REPL result area |   |

**Phase 2 complete**: Real SQL input → create table → insert data → query results

---

## Phase 3: SQL Feature Completion

> **Goal**: Practical level of SQL support

| Task | Description | Status |
|------|-------------|--------|
| WHERE Clause | Conditional filtering (`=`, `<`, `>`, `!=`, `AND`, `OR`) |   |
| UPDATE | Conditional data modification |   |
| DELETE | Conditional data deletion |   |
| SELECT Extension | Column selection, selective queries beyond `*` |   |
| ORDER BY / LIMIT | Sorting and paging |   |
| Primary Key | Primary key constraints |   |
| Error Message Improvement | Parse error position display, detailed execution errors |   |

**Phase 3 complete**: Basic CRUD + filtering + sorting SQL DB

---

## Phase 4: Polish & Quality

> **Goal**: UX completion + concurrency foundation

| Task | Description | Status |
|------|-------------|--------|
| Command History | Up/down arrow key navigation through previous commands |   |
| Autocomplete | Table names, column names, SQL keyword Tab completion |   |
| Multi-line Input | Continue input on next line when `;` is missing |   |
| Extended Meta Commands | `.tables`, `.schema`, `.describe <table>` |   |
| DashMap Integration | Lock-free concurrent table storage |   |
| B-Tree Indexing | Index-based query optimization |   |

**Phase 4 complete**: Ready for v0.1.0 release

---

## v0.1.0 Release

> Release v0.1.0 upon completion of Phase 1-4
>
> **Core Features**: REPL Interface + SQL Parser + Storage Engine + CRUD/Filtering/Sorting

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

1. `Cargo.toml` - Update edition to `"2021"`
2. Fix code typos - `reuslt`→`result`, `ecs`→`esc`
3. `src/lib.rs` - Uncomment storage module
4. `src/tests/table.rs` - Enable tests and fix compile errors
5. REPL 3-panel layout - Input area + result area + status bar
6. Text input handling - Character input, backspace, cursor movement, Enter to execute
7. Basic meta commands - `.quit`, `.help`
8. Unified error type definition

---

## Milestones

| Version | Goal | Phase |
|---------|------|-------|
| **v0.1.0** | **First Release - REPL + SQL CRUD + Filtering/Sorting** | **Phase 1-4** |
| v0.2.0 | QUIC Network Server + Rust/Python SDK | Phase 5-5.5 |
| v0.3.0 | Java/JavaScript SDK | Phase 6 |
| v0.4.0 | JOIN, Aggregate Functions, Transactions | Phase 7 |
| v0.5.0 | Data Persistence, WAL | Phase 8 |
| v0.6.0+ | Operations and Deployment Tools | Phase 9 |

---

## Contributing

Please refer to [CONTRIBUTING.md](CONTRIBUTING.md).
