---
description: Canonical technical architecture for backend-release-line-closeout-2026-03-25.
---

# backend-release-line-closeout-2026-03-25 Technical Documentation

## Canonical Architecture

- Router and auth boundary remain in `backend/src/lib.rs`.
- DTO validation and frozen request/response types remain in `backend/src/models.rs`.
- Retrieval, storage, distill execution, and indexing remain in `backend/src/state.rs`.
- Contract tests remain under one integration-test target: `backend/tests/contract_semantics.rs`.

## Key Constraints and Non-Goals

- No public behavioral/runtime contract changes.
- No legacy reflection compatibility reintroduction.
- Keep one backend contract integration-test target after the split.

## Module Boundaries and Data Flow

- `backend/tests/contract_semantics.rs` is the thin integration-test entrypoint.
- `backend/tests/contract_semantics/` contains four feature-scoped modules:
  - provider and retrieval behavior
  - memory route / contract behavior
  - distill job behavior
  - diagnostics, auth, idempotency, and persistence behavior
- Shared fixture and mock helpers remain centralized in the root `contract_semantics.rs` entrypoint instead of being repeated per module.

## Interfaces and Contracts

- Trace-return wrappers now fail with structured internal errors instead of panic-based `expect(...)`.
- Rerank application arguments are grouped into a single struct to keep the call boundary explicit and Clippy-clean.
- Minor code-style cleanups do not change route inputs, outputs, or auth rules.

## Security and Reliability

- Principal enforcement, admin-token isolation, and idempotency behavior remain test-backed.
- Legacy unsupported LanceDB schemas still fail closed.
- Cleaning `backend/target/` is part of release hygiene so the worktree reflects only source changes.

## Test Strategy

- Backend contract suite remains the primary behavior lock.
- Repo-level Node tests remain the integration check for the TypeScript adapter surface after backend changes.
