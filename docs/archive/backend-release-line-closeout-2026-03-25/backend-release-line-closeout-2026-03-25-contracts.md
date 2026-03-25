---
description: API and schema contracts for backend-release-line-closeout-2026-03-25.
---

# backend-release-line-closeout-2026-03-25 Contracts

## API Contracts

- No public route additions or removals in this scope.
- Existing behavioral-facing routes remain unchanged:
  - `POST /v1/recall/generic`
  - `POST /v1/recall/behavioral`
  - `POST /v1/debug/recall/generic`
  - `POST /v1/debug/recall/behavioral`
  - memory store/update/delete/list/stats routes
  - distill enqueue/status routes

## Shared Types / Schemas

- `backend/src/models.rs` remains the source of truth for request/response DTOs.
- Storage-internal behavioral schema introduced by the previous cleanup remains unchanged in this scope.
- No config schema expansion is allowed in this scope.

## Event and Streaming Contracts

- No new streaming/event surfaces are introduced.
- Distill remains async-job based and transcript append remains request/response only.

## Error Model

- Release-line cleanup may improve error wording and invariant handling, but must not widen accepted inputs.
- Internal invariant failures should return `AppError` rather than panic where practical.

## Validation and Compatibility Rules

- `cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings` must pass.
- `cargo test --manifest-path backend/Cargo.toml -- --nocapture` must pass.
- `npm test` must pass.
- `cargo test --manifest-path backend/Cargo.toml --test contract_semantics -- --nocapture` must remain the executable backend contract target while its coverage is reorganized by feature.
