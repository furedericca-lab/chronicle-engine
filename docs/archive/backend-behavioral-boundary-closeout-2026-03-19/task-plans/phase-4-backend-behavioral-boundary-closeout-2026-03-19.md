---
description: Task list for backend-behavioral-boundary-closeout-2026-03-19 phase 4.
---

# Tasks: backend-behavioral-boundary-closeout-2026-03-19 Phase 4

## Phase 4: Active-doc closeout and release verification

Goal: make active docs truthful for the final chosen contract and record exact verification evidence.

Definition of Done: active docs, tests, and checklist all reflect the final behavior; residual risk is explicitly recorded.

Tasks:
- [x] T301 [Docs] Update active docs to the final contract state.
  - DoD: `README.md`, `README_CN.md`, `docs/runtime-architecture.md`, `docs/README.md`, and `docs/archive-index.md` do not misdescribe the chosen boundary.
- [x] T302 [QA] Run final verification bundle.
  - DoD: targeted Node tests, backend tests, doc scans, and `git diff --check` pass.
- [x] T303 [Docs] Record final status in `4phases-checklist.md` and archive/close the scope if implemented.
  - DoD: changed files, commands, outcomes, and any retained intentional boundary are documented.

Checkpoint: completed; active docs, verification evidence, and retained storage-only reflection internals are all recorded explicitly.
