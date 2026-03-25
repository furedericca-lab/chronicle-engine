---
description: Canonical technical architecture refresh for the remote Rust memory backend after turns-stage distill unification.
---

# remote-memory-backend 2026-03-25 Technical Documentation

## Canonical architecture

Target runtime architecture:

1. **Remote memory backend**
   - Rust service
   - LanceDB for memory storage and behavioral-facing storage-internal guidance rows
   - SQLite job table for distill jobs
   - owns ACL, scope derivation, retrieval/ranking, model config, transcript persistence, and all trajectory-derived knowledge generation

2. **Local integration shell**
   - OpenClaw plugin wiring in `index.ts`
   - tool registration and lifecycle hook registration
   - HTTP client adapter, retry/backoff, auth headers, fail-open/fail-closed boundaries

3. **Local context orchestration**
   - prompt rendering
   - behavioral-guidance recall planning
   - session exposure / error reminder state
   - no local authority for trajectory-derived writes

## Contract reset versus 2026-03-17

The 2026-03-17 snapshot documented a backend that still owned async reflection generation jobs while the shell could enqueue them on `/new` and `/reset`.

That is no longer the active architecture.

### Removed
- command-triggered reflection generation
- reflection source loading route
- reflection job enqueue route
- reflection job status route
- plugin/runtime command hooks whose purpose was to generate new knowledge on `/new` or `/reset`

### Retained
- backend-owned behavioral-guidance recall
- prompt-local behavioral-guidance injection planning in the plugin
- cadence-driven backend-native distill

## Distill ownership model

### `session-lessons`
This mode now owns:
- lesson extraction
- cause extraction
- fix extraction
- prevention extraction
- stable decision promotion
- durable practice promotion

Evidence gate:
- a stable decision or durable practice must be supported by at least two distinct evidence messages and either repeated target phrasing or corroborating cause/fix/prevention context;
- otherwise the summary remains an ordinary `Lesson`.

### `governance-candidates`
This mode owns:
- worth-promoting learnings
- skill extraction candidates
- AGENTS/SOUL/TOOLS promotion candidates

### Distill subtypes for downgraded reflection-style outputs
Instead of preserving a separate reflection persistence pipeline for derived/open-loop outputs, distill now emits:
- `follow-up-focus`
- `next-turn-guidance`

These remain distill artifacts under backend ownership.

## Runtime behavior

### On `agent_end`
- shell appends transcript rows to the backend
- shell counts completed user turns
- shell evaluates `distill.everyTurns`
- on boundary crossing, shell enqueues one backend distill job

### On `/new` and `/reset`
- no trajectory-derived knowledge generation occurs
- runtime behavior is limited to read-side prompt/session cleanup concerns; no public reflection-named surface remains

## Tooling / schema implications

### Removed tool surface
- `memory_reflection_status`

### Retained tool surface
- `memory_distill_enqueue`
- `memory_distill_status`
- `memory_recall_debug`
- ordinary memory mutation/list/stats tools

### Config implications
`memoryReflection` is no longer a supported config surface. Behavioral recall/injection control lives under `autoRecallBehavioral`, and legacy reflection-named config aliases are rejected.

## Operational guidance

When reviewing or extending the active backend/shell boundary after this snapshot:
- treat distill as the only generation/write authority for new trajectory-derived knowledge;
- treat behavioral-guidance recall as the only supported public read-side lane;
- treat behavioral-facing LanceDB field names as canonical for active storage;
- treat legacy reflection-era LanceDB tables as unsupported and require manual migration or reset before startup;
- do not reintroduce command-bound generation or reflection-named public routes/config unless a new architecture scope explicitly replaces this snapshot.
