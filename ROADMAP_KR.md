# PipeSQL Roadmap

> [English](./ROADMAP.md)

> PipeSQL 프로젝트의 개발 로드맵입니다.

## 현재 상태

- **브랜치**: `feat/repl`
- **진행률**: ~15%
- 기본 TUI 프레임워크 구축 완료
- Storage 모듈 구조 정의됨 (비활성화 상태)

---

## Phase 1: 기반 구축 (Foundation)

> 프로젝트의 핵심 인프라 완성

| 작업 | 설명 | 상태 |
|------|------|------|
| REPL 기본 UI | ratatui 기반 터미널 UI | ✅  |
| 키보드 이벤트 처리 | crossterm 이벤트 루프 | ✅  |
| Storage 모듈 활성화 | `lib.rs`에서 모듈 활성화 |   |
| 테스트 코드 활성화 | 주석 처리된 테스트 복원 |   |
| 프로젝트 설정 정리 | Cargo.toml edition 수정 |   |

---

## Phase 2: 스토리지 엔진 (Storage Engine)

> 인메모리 데이터 저장소 핵심 기능

| 작업 | 설명 | 상태 |
|------|------|------|
| 데이터 타입 확장 | `Text`, `Boolean`, `Float`, `Timestamp` 추가 |   |
| CRUD 연산 | Insert, Select, Update, Delete 구현 |   |
| DashMap 통합 | Lock-free 동시성 테이블 스토리지 |   |
| Primary Key | 기본 키 제약 조건 |   |
| 인덱싱 | B-Tree 기반 인덱스 |   |

---

## Phase 3: SQL 파서 및 실행기 (SQL Parser & Executor)

> SQL 쿼리 처리 파이프라인

| 작업 | 설명 | 상태 |
|------|------|------|
| 토크나이저 | SQL 렉싱 단계 |   |
| AST 파서 | `CREATE`, `INSERT`, `SELECT`, `UPDATE`, `DELETE` |   |
| 쿼리 플래너 | 실행 계획 수립 |   |
| 쿼리 실행기 | 스토리지 연동 실행 |   |
| WHERE 절 | 조건부 필터링 |   |
| ORDER BY / LIMIT | 정렬 및 페이징 |   |

---

## Phase 4: REPL 완성 (Interactive Shell)

> 사용자 친화적 터미널 인터페이스

| 작업 | 설명 | 상태 |
|------|------|------|
| SQL 입력창 | 다중 라인 입력 지원 |   |
| 결과 테이블 표시 | 포맷팅된 쿼리 결과 출력 |   |
| 명령어 히스토리 | rustyline 연동 |   |
| 자동완성 | 테이블명, 컬럼명, 키워드 |   |
| 에러 표시 | 친화적인 에러 메시지 UI |   |
| 메타 명령어 | `.tables`, `.schema`, `.help` 등 |   |

---

## v0.1.0 Release

> Phase 1-4 완료 시 v0.1.0 릴리스
> 
> **핵심 기능**: SQL 파서 + 기본 쿼리 실행 + REPL 인터페이스

---

## Phase 5: 네트워크 레이어 (QUIC Protocol)

> 고성능 네트워크 통신

| 작업 | 설명 | 상태 |
|------|------|------|
| QUIC 서버 | quinn/s2n-quic 기반 서버 |   |
| 클라이언트 연결 관리 | 멀티플렉싱 스트림 |   |
| 와이어 프로토콜 | 쿼리/응답 바이너리 포맷 |   |
| 인증 | 기본 인증 레이어 |   |
| TLS 설정 | 보안 연결 |   |
| 연결 풀링 | 커넥션 재사용 최적화 |   |

---

## Phase 5.5: 클라이언트 SDK - 1차 (Rust, Python)

> 핵심 언어 SDK 우선 개발

| 작업 | 설명 | 상태 |
|------|------|------|
| Rust 클라이언트 | 네이티브 Rust SDK |   |
| Python 클라이언트 | pipesql-python (PyO3 사용) |   |

---

## v0.2.0 Release

> Phase 5-5.5 완료 시 v0.2.0 릴리스
> 
> **핵심 기능**: QUIC 프로토콜 기반 네트워크 서버 + Rust/Python SDK

---

## Phase 6: 클라이언트 SDK - 2차 (Java, JavaScript)

> 추가 언어 SDK 개발

| 작업 | 설명 | 상태 |
|------|------|------|
| Java 클라이언트 | pipesql-java (JNI 또는 순수 Java) |   |
| JavaScript/Node.js | pipesql-js (NAPI-RS 사용) |   |

---

## v0.3.0 Release

> Phase 6 완료 시 v0.3.0 릴리스
> 
> **핵심 기능**: Java/JavaScript SDK 추가

---

## Phase 7: 고급 SQL 기능 (Advanced SQL)

> SQL 기능 확장

| 작업 | 설명 | 상태 |
|------|------|------|
| JOIN | INNER, LEFT, RIGHT JOIN |   |
| 집계 함수 | COUNT, SUM, AVG, MIN, MAX |   |
| GROUP BY / HAVING | 그룹화 및 필터링 |   |
| 서브쿼리 | 중첩 쿼리 지원 |   |
| 트랜잭션 | BEGIN, COMMIT, ROLLBACK |   |
| MVCC | Multi-Version Concurrency Control |   |

---

## Phase 8: 영속성 및 안정성 (Persistence & Reliability)

> 데이터 안정성 보장

| 작업 | 설명 | 상태 |
|------|------|------|
| 데이터 영속화 | 스냅샷 저장/복원 |   |
| WAL | Write-Ahead Logging |   |
| 복제 | Leader-Follower 복제 |   |
| 크래시 복구 | 장애 복구 메커니즘 |   |

---

## Phase 9: 운영 및 배포 (Operations & Deployment)

> 배포 및 운영 도구

| 작업 | 설명 | 상태 |
|------|------|------|
| CLI 완성 | clap 기반 명령어 인터페이스 |   |
| 설정 파일 | TOML 설정 지원 |   |
| 메트릭 | Prometheus 호환 메트릭 |   |
| 헬스체크 | 상태 조회 엔드포인트 |   |
| Docker | 컨테이너 이미지 |   |
| CI/CD | GitHub Actions 파이프라인 |   |
| 문서화 | API 문서, 사용자 가이드 |   |

---

## 즉시 작업 목록 (Next Steps)

현재 Phase 1 완료를 위한 우선 작업:

1. `src/lib.rs` - storage 모듈 주석 해제
2. `src/tests/table.rs` - 테스트 코드 활성화
3. `Cargo.toml` - edition을 `"2021"`로 수정
4. REPL - SQL 입력 기능 추가
5. Storage - 기본 CRUD 연산 구현

---

## 마일스톤

| 버전 | 목표 | Phase |
|------|------|-------|
| **v0.1.0** | **첫 릴리스 - 기본 SQL DB 기능 완성** | **Phase 1-4** |
| v0.2.0 | QUIC 네트워크 서버 + Rust/Python SDK | Phase 5-5.5 |
| v0.3.0 | Java/JavaScript SDK | Phase 6 |
| v0.4.0 | JOIN, 집계 함수, 트랜잭션 | Phase 7 |
| v0.5.0 | 데이터 영속화, WAL | Phase 8 |
| v0.6.0+ | 운영 및 배포 도구 | Phase 9 |

---

## 기여하기

[CONTRIBUTING.md](CONTRIBUTING.md)를 참고해주세요.
