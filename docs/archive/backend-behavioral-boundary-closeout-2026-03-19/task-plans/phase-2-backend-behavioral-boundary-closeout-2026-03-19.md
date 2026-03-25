---
description: Task list for backend-behavioral-boundary-closeout-2026-03-19 phase 2.
---

# Tasks: backend-behavioral-boundary-closeout-2026-03-19 Phase 2

## Phase 2: Backend/client/runtime rename execution

Goal: implement the chosen route/client/tool/runtime naming changes and clean the obvious internal legacy semantics.

Definition of Done: active code uses one policy-consistent naming model and the internal leftovers such as `temp:memory-reflection` are removed or deliberately justified.

Tasks:
- [x] T101 [Backend] Update backend route/model naming according to the frozen policy.
  - DoD: `backend/src/lib.rs`, `backend/src/models.rs`, and any affected route/model helpers are aligned.
- [x] T102 [P] [Agentic] Update backend-client and tool/runtime adapters.
  - DoD: `src/backend-client/*`, `src/backend-tools.ts`, `index.ts`, and any touched context helpers use the chosen semantics.
- [x] T103 [QA] Update active Node tests for the chosen route/DTO/runtime behavior.
  - DoD: changed tests assert the new contract instead of translating old names silently.

Checkpoint: completed; active runtime path now uses canonical behavioral HTTP/client/tool/runtime naming, with legacy reflection HTTP routes retained only as tested compatibility aliases.
