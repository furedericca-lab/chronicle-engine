---
description: Implementation research notes for backend-behavioral-boundary-closeout-2026-03-19.
---

# backend-behavioral-boundary-closeout-2026-03-19 Implementation Research Notes

## Baseline (Current State)

Active-code matches identified during scope opening:

- `backend/src/lib.rs`
  - routes still mounted at `/v1/recall/reflection` and `/v1/debug/recall/reflection`
- `backend/src/models.rs`
  - stats field still serialized as `reflection_count`
- `backend/src/state.rs`
  - persisted schema still uses `reflection_kind`
  - generic exclusion still uses `exclude_reflection`
- `src/backend-client/client.ts`
  - behavioral recall still calls reflection routes
  - stats still map `reflectionCount -> behavioralCount`
- `src/backend-client/types.ts`
  - type system still includes `reflection` category and `reflection` trace kind
- `src/backend-tools.ts`
  - manual write guard still refers to `category=reflection`
- `index.ts`
  - temporary session key helper still recognizes `temp:memory-reflection`
- tests
  - Node integration tests and backend contract tests still assert the reflection route/path/schema names

## Gap Analysis

### Gap 1: active API naming split

The backend contract and client adapter still speak reflection while the outer runtime/docs speak behavioral guidance.

### Gap 2: storage/schema naming split

Persistence currently still encodes behavioral rows as `reflection` category + `reflection_kind`, which may be acceptable only if explicitly frozen.

### Gap 3: internal temporary legacy semantics

Small active runtime leftovers such as `temp:memory-reflection` still encode the old semantic name even after the closeout.

### Gap 4: doc truthfulness depends on the chosen policy

`docs/runtime-architecture.md` currently documents the backend reflection boundary as intentional. If this scope removes that boundary, active docs must be rewritten accordingly.

## Candidate Designs and Trade-offs

### Design 1 — full breaking rename

Rename routes, DTOs, trace kinds, and storage-facing names in one batch.

Trade-off:
- cleanest end state
- highest compatibility/migration risk

### Design 2 — additive route/DTO rename, storage kept internal

Expose behavioral route/DTO names externally while keeping persisted `reflection` category/field names as storage internals.

Trade-off:
- cleaner external contract
- lower data migration risk
- still leaves some internal naming debt, but in a narrower place

### Design 3 — internal cleanup only

Remove temporary/local legacy naming and keep backend route/storage names.

Trade-off:
- cheapest implementation
- weakest payoff
- likely not enough for the stated user goal

## Selected Design

**Phase 1 decision: choose Design 2 with a bounded compatibility policy.**

Frozen policy:

- rename the active HTTP/tool/client/debug/stats surface to behavioral-facing names;
- keep persistence/storage internals (`category=reflection`, `reflection_kind`, existing stored rows) unchanged in this scope unless implementation proves a very small safe alias layer is needed;
- remove small active internal legacy semantic names such as `temp:memory-reflection`;
- old reflection-named HTTP routes may remain as temporary compatibility aliases only if they are implemented as thin aliases to the new behavioral routes and covered by tests;
- active docs must stop describing the reflection boundary as the intended steady state.

Why this decision:

- it closes the highest-value semantic split on the active backend/client/runtime surface;
- it avoids unnecessary storage migration risk in the same batch;
- it still satisfies the user goal of closing the backend compatibility boundary and cleaning internal legacy semantics.

## Validation Plan

Likely verification surfaces:

- targeted Node tests covering backend-client/tool/runtime path changes
- targeted backend contract tests for route/schema behavior
- `npm test` or focused `node --test` slices if touched areas require it
- `cargo test --manifest-path backend/Cargo.toml --test phase2_contract_semantics -- --nocapture`
- placeholder/refactor scans for scope docs
- `git diff --check`

## Risks and Assumptions

- Assumption: changing active route/DTO naming is acceptable if tests and docs are updated together.
- Risk: persisted data migration may be more expensive than route/client rename.
- Risk: archive docs may mention the old boundary; that is acceptable as long as active docs stop presenting it as current.
