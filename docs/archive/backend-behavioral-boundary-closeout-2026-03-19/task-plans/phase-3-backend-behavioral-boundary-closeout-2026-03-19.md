---
description: Task list for backend-behavioral-boundary-closeout-2026-03-19 phase 3.
---

# Tasks: backend-behavioral-boundary-closeout-2026-03-19 Phase 3

## Phase 3: Storage/schema compatibility and backend verification

Goal: implement or codify the persistence-layer decision and verify backend contract behavior.

Definition of Done: storage-facing behavior is explicit, backend tests cover the contract, and stats/debug surfaces remain correct.

Tasks:
- [x] T201 [Backend] Implement or freeze storage/category/field handling.
  - DoD: `reflection_kind`, `reflection_count`, `category=reflection`, and related schema behavior follow the chosen policy.
- [x] T202 [P] [QA] Update backend contract tests.
  - DoD: `backend/tests/phase2_contract_semantics.rs` and any nearby focused tests prove the chosen route/schema behavior.
- [x] T203 [Security] Re-check manual-write rejection and debug visibility semantics after the rename work.
  - DoD: behavioral row protections and inspectable debug behavior are still correct.

Checkpoint: completed; storage internals remain reflection-based by explicit policy while backend/API stats, trace, and manual-write semantics are tested against the behavioral-facing contract.
