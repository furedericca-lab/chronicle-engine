---
description: Implementation research notes for backend-release-line-closeout-2026-03-25.
---

# backend-release-line-closeout-2026-03-25 Implementation Research Notes

## Baseline (Current State)

- Backend functional tests were green before this scope.
- Release-line gap was engineering quality, not product contract drift:
  - dead `RateLimited` enum variant
  - derivable `Default` impl in config
  - trace wrappers using `expect(...)`
  - several small Clippy findings in `state.rs`
- `contract_semantics.rs` mixed provider mocks, storage fixtures, distill flows, auth checks, diagnostics checks, and persistence checks in one file.

## Gap Analysis

- The backend was close to ship quality but not clean enough for a strict release gate.
- Contract-test readability had become a maintenance risk even though coverage was strong.

## Candidate Designs and Trade-offs

- Keep all tests in one file but add section comments:
  - lowest code churn
  - weakest structural improvement
- Split into multiple files:
  - strongest separation
  - higher helper churn and test-target movement
- Split into internal feature modules:
  - clear grouping
  - preserves one integration-test target
  - lowest compatibility risk

## Selected Design

- Fix all current Clippy blockers directly in production code.
- Replace panic-style trace invariant unwraps with `AppError::internal(...)`.
- Group the contract tests into feature modules:
  - `provider_and_retrieval`
  - `memory_contracts`
  - `distill_contracts`
  - `diagnostics_auth_and_persistence`
- Keep shared helper fixtures centralized at the top of the contract test file.

## Validation Plan

- `cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo test --manifest-path backend/Cargo.toml --test contract_semantics -- --nocapture`
- `cargo test --manifest-path backend/Cargo.toml -- --nocapture`
- `npm test`
- `git diff --check`

## Risks and Assumptions

- Assumption: reorganizing the contract tests into internal modules is sufficient for the requested functional split.
- Residual risk after completion should be limited to normal reviewer judgment around code style and future helper extraction granularity.
