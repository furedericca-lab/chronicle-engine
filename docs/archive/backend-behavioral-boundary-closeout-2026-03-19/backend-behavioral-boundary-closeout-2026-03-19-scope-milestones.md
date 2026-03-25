---
description: Scope boundaries and milestones for backend-behavioral-boundary-closeout-2026-03-19.
---

# backend-behavioral-boundary-closeout-2026-03-19 Scope and Milestones

## In Scope

- Freeze the rename/compatibility decision for the backend behavioral-guidance lane.
- Audit active `reflection` naming that still exists outside archive material.
- Decide how to handle backend routes, DTO fields, trace kinds, storage fields, and temporary runtime keys.
- Update code/tests/docs according to the chosen policy.
- Record data-compatibility, rollback, and verification rules.

## Out of Scope

- Reopening already archived governance/self-improvement closeout work.
- Broad backend architecture redesign unrelated to the naming boundary.
- New distill semantics or new memory categories.
- Archive-wide wording cleanup outside what is needed to keep active docs truthful.

## Milestones

### Milestone 1 — freeze the compatibility decision

Definition of Done:
- docs state whether this scope is breaking, dual-acceptance, or storage-internal-only;
- touched active surfaces and test gates are enumerated;
- risks/rollback questions are concrete.

Status: **completed on 2026-03-19**.

Decision:
- canonical active route/client/tool/doc naming becomes behavioral-facing;
- reflection-named HTTP routes may remain temporarily as compatibility aliases;
- persisted storage names remain internal in this scope unless a minimal safe alias is required.

### Milestone 2 — implement backend/client/runtime naming changes

Definition of Done:
- code changes align with the frozen policy;
- route/client/tool/runtime naming is internally consistent;
- internal legacy names such as `temp:memory-reflection` are removed or explicitly justified.

Status: **completed on 2026-03-19**.

### Milestone 3 — migrate or codify storage/data semantics

Definition of Done:
- persisted field/category handling is implemented or explicitly frozen by policy;
- stats/debug behavior remains correct;
- compatibility or migration logic is covered by targeted tests.

Status: **completed on 2026-03-19**.

### Milestone 4 — active docs and verification closeout

Definition of Done:
- active docs no longer describe the old backend reflection boundary as current unless the contract intentionally retains part of it;
- placeholder scan, residual scan, targeted tests, and `git diff --check` pass;
- remaining risks are recorded explicitly.

Status: **completed on 2026-03-19**.

## Dependencies

- Depends on the post-merge baseline at `main` commit `575cf86`.
- Should reuse evidence and prior rationale from `docs/archive/governance-behavioral-closeout-2026-03-19/` rather than duplicating it.

## Exit Criteria

- The scope leaves the repo in one unambiguous semantic state.
- Either backend reflection naming is truly gone from active code/docs, or the residual pieces are few, intentional, and re-documented by explicit contract.
- Tests and docs prove the chosen state.
