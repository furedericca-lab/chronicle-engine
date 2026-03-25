---
description: Canonical technical architecture for backend-behavioral-boundary-closeout-2026-03-19.
---

# backend-behavioral-boundary-closeout-2026-03-19 Technical Documentation

## Canonical Architecture

The target architecture for this scope is a single semantic lane:

- **behavioral guidance** is the public/runtime/backend-facing concept
- any retained `reflection` naming must be either:
  - archive-only history, or
  - explicitly documented compatibility/storage internals

This scope exists to eliminate the current mixed state where active backend code still uses reflection names while active runtime/docs describe behavioral guidance.

## Key Constraints and Non-Goals

### Constraints

- preserve behavioral recall semantics and manual-write protections;
- do not silently break stored data or old tests without an explicit policy;
- keep active docs, code, and tests synchronized;
- keep archive docs intact as historical evidence.

### Non-goals

- redesign the memory architecture;
- change distill semantics;
- reopen removed self-improvement/governance compatibility shims.

## Module Boundaries and Data Flow

Expected review path for this scope:

1. **Backend route layer**
   - `backend/src/lib.rs`
2. **Backend request/response models**
   - `backend/src/models.rs`
3. **Backend persistence / retrieval implementation**
   - `backend/src/state.rs`
4. **Node backend-client adapter**
   - `src/backend-client/client.ts`
   - `src/backend-client/types.ts`
5. **Tool/runtime surfaces**
   - `src/backend-tools.ts`
   - `index.ts`
   - `src/context/auto-recall-orchestrator.ts`
6. **Docs/tests**
   - active docs + targeted backend/Node tests

## Interfaces and Contracts

This scope must answer four contract questions:

1. What is the canonical backend route name?
2. What is the canonical stats/debug/trace naming?
3. What is the canonical persisted field/category naming?
4. What exact legacy names, if any, remain accepted?

A safe implementation should make these answers uniform across:

- code
- tests
- active docs
- error messages

## Security and Reliability

- Manual writes to backend-managed behavioral rows must remain blocked.
- If dual-route or dual-field compatibility exists, tests must ensure the preferred path and the legacy path do not diverge semantically.
- If a breaking rename is chosen, failure mode must be explicit and reviewer-visible.
- Any persistence migration must define rollback boundaries before implementation.

## Test Strategy

Minimum expected verification for implementation phase(s):

- Node tests covering backend-client/tool/runtime behavior
- backend tests covering route/model/storage semantics
- scope doc scans
- `git diff --check`

Recommended focused commands once code work begins:

- `node --test test/remote-backend-shell-integration.test.mjs test/governance-tools.test.mjs test/auto-recall-behavioral.test.mjs test/config-session-strategy-cutover.test.mjs`
- `cargo test --manifest-path backend/Cargo.toml --test phase2_contract_semantics -- --nocapture`
- `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/doc_placeholder_scan.sh docs/archive/backend-behavioral-boundary-closeout-2026-03-19`
- `bash /root/.openclaw/workspace/skills/repo-task-driven/scripts/post_refactor_text_scan.sh docs/archive/backend-behavioral-boundary-closeout-2026-03-19 README.md README_CN.md docs/runtime-architecture.md docs/README.md docs/archive-index.md`
