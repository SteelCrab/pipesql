# PipeSQL Roadmap

> [English](./ROADMAP.md)

> PipeSQL 프로젝트의 개발 로드맵입니다.

## 현재 상태

- **브랜치**: `feat/repl`
- **진행률**: ~15%
- 기본 TUI 프레임워크 구축 완료
- Storage 모듈 구조 정의됨 (비활성화 상태)

---

## Phase 1: 기반 구축 & REPL 쉘 (Foundation & Interactive Shell)

> **목표**: 프로젝트 정상 빌드 + 타이핑 가능한 REPL 쉘

| 작업 | 설명 | 상태 |
|------|------|------|
| REPL 기본 UI | ratatui 기반 터미널 UI | ✅  |
| 키보드 이벤트 처리 | crossterm 이벤트 루프 | ✅  |
| 프로젝트 설정 정리 |  코드 오타 수정 (`reuslt`→`result`, `ecs`→`esc`) | ✅ |
| Storage 모듈 활성화 | `lib.rs` 주석 해제, 테스트 코드 복원, 컴파일 에러 수정 | ✅ |
| REPL 레이아웃 구현 | 화면 3분할: 입력 영역 + 결과 영역 + 상태바 |   |
| 텍스트 입력 처리 | 문자 입력, 백스페이스, 커서 이동, Enter 실행 |   |
| 기본 메타 명령어 | `.quit`, `.help` (SQL 없이 동작하는 것들) |   |
| 에러 처리 구조 | 통합 에러 타입 정의, REPL 에러 메시지 표시 영역 |   |

**Phase 1 완료 시**: 앱 실행 → 텍스트 입력 가능 → Enter 시 에코 출력 (아직 SQL 실행 없음)

---

## Phase 2: 스토리지 엔진 + 기본 SQL (Storage Engine + Basic SQL)

> **목표**: `CREATE TABLE` → `INSERT` → `SELECT *` end-to-end 동작

| 작업 | 설명 | 상태 |
|------|------|------|
| 데이터 타입 확장 | `Text`, `Boolean`, `Float` 추가 (현재 Integer만) |   |
| 기본 SQL 토크나이저 | SQL 문자열 → 토큰 분리 |   |
| AST 파서 (기본) | `CREATE TABLE`, `INSERT INTO`, `SELECT * FROM` 파싱 |   |
| Storage CRUD (기본) | 테이블 생성/삭제, Row 삽입, 전체 Row 조회 |   |
| 실행 엔진 연결 | SQL 입력 → 파서 → 스토리지 → 결과 반환 |   |
| 결과 테이블 표시 | REPL 결과 영역에 포맷팅된 테이블 출력 |   |

**Phase 2 완료 시**: 실제 SQL 입력 → 테이블 생성 → 데이터 삽입 → 조회 가능

---

## Phase 3: SQL 기능 확장 (SQL Feature Completion)

> **목표**: 실용적 수준의 SQL 지원

| 작업 | 설명 | 상태 |
|------|------|------|
| WHERE 절 | 조건부 필터링 (`=`, `<`, `>`, `!=`, `AND`, `OR`) |   |
| UPDATE | 조건부 데이터 수정 |   |
| DELETE | 조건부 데이터 삭제 |   |
| SELECT 확장 | 컬럼 지정, `*` 외 선택적 조회 |   |
| ORDER BY / LIMIT | 정렬 및 페이징 |   |
| Primary Key | 기본 키 제약 조건 |   |
| 에러 메시지 개선 | 파싱 에러 위치 표시, 실행 에러 상세화 |   |

**Phase 3 완료 시**: 기본적인 CRUD + 필터링 + 정렬이 가능한 SQL DB

---

## Phase 4: REPL 완성 & 품질 (Polish & Quality)

> **목표**: 사용성 완성 + 동시성 기반

| 작업 | 설명 | 상태 |
|------|------|------|
| 명령어 히스토리 | 위/아래 키로 이전 명령 탐색 |   |
| 자동완성 | 테이블명, 컬럼명, SQL 키워드 Tab 완성 |   |
| 다중 라인 입력 | `;` 없으면 다음 줄 이어서 입력 |   |
| 메타 명령어 확장 | `.tables`, `.schema`, `.describe <table>` |   |
| DashMap 통합 | Lock-free 동시성 테이블 스토리지 |   |
| B-Tree 인덱싱 | 인덱스 기반 조회 최적화 |   |

**Phase 4 완료 시**: v0.1.0 릴리스 가능 수준

---

## v0.1.0 Release

> Phase 1-4 완료 시 v0.1.0 릴리스
>
> **핵심 기능**: REPL 인터페이스 + SQL 파서 + 스토리지 엔진 + CRUD/필터링/정렬

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

1. `Cargo.toml` - edition을 `"2021"`로 수정
2. 코드 오타 수정 - `reuslt`→`result`, `ecs`→`esc`
3. `src/lib.rs` - storage 모듈 주석 해제
4. `src/tests/table.rs` - 테스트 코드 활성화 및 컴파일 에러 수정
5. REPL 레이아웃 3분할 - 입력 영역 + 결과 영역 + 상태바
6. 텍스트 입력 처리 - 문자 입력, 백스페이스, 커서 이동, Enter 실행
7. 기본 메타 명령어 - `.quit`, `.help`
8. 통합 에러 타입 정의

---

## 마일스톤

| 버전 | 목표 | Phase |
|------|------|-------|
| **v0.1.0** | **첫 릴리스 - REPL + SQL CRUD + 필터링/정렬** | **Phase 1-4** |
| v0.2.0 | QUIC 네트워크 서버 + Rust/Python SDK | Phase 5-5.5 |
| v0.3.0 | Java/JavaScript SDK | Phase 6 |
| v0.4.0 | JOIN, 집계 함수, 트랜잭션 | Phase 7 |
| v0.5.0 | 데이터 영속화, WAL | Phase 8 |
| v0.6.0+ | 운영 및 배포 도구 | Phase 9 |

---

## 기여하기

[CONTRIBUTING.md](CONTRIBUTING.md)를 참고해주세요.
