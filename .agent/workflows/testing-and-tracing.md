---
description: Comprehensive guide for writing, running, and inspecting Forge kernel tests with tracing
---

---
description: How to run tests, read failures, and debug boolean pipeline issues
---
# Testing & Debugging Workflow
// turbo-all
## Running Tests
**Always release mode.** One command:
```bash
FORGE_LOG=compact cargo test --release -p <whatever you want to test> --nocapture 2>&1 | tail -40
```
## What Shows Up in the Output
### On Failure: Per-Entity Decision Ancestry
Every topology error (e.g. `MissingTwin`) embeds the full decision chain for each failing entity directly in the error string. For each entity you'll see:
- Its **ID and parent** (halfedge index + face index)
- Every **TracedDecision** scoped to that entity, each showing:
  - **Tier** — [deterministic](cci:1://file:///Users/spenstar/Documents/programming/Forge/crates/forge-topo/src/topology/integrity/hashing.rs:350:4-355:5), `resolved`, `near_boundary`, `policy_applied`, `escalated` (higher = more suspicious)
  - **Kind** — how the decision was resolved (`Exact`, `PolicyApplied`, `Forced`)
  - **Margin** — numeric distance to the tolerance boundary (lower = more fragile)
  - **Context** — what the decision was about (classification, dedup, selection, stitching)
Up to 5 entities are reported. The first decision per entity is typically closest to the root cause.
### On Success: Compact Decision Summary
`FORGE_LOG=compact` prints a filtered summary to stderr showing only `NearBoundary` and higher-tier decisions — the ones worth investigating. Deterministic decisions are suppressed. This is typically 5–20 lines, not hundreds.
### Data Structures on BooleanResult
The `BooleanResult` returned by every boolean operation carries:
- **DecisionLog** — flat list of every `TracedDecision` made during the operation. Queryable by tier (`tier_at_least`), by entity scope, or by span (pipeline phase).
- **ReplayLog** — one [ReplayEntry](cci:2://file:///Users/spenstar/Documents/programming/Forge/crates/forge-topo/src/topology/history/replay.rs:23:0-36:1) per pipeline phase (split, classify, select, assemble, postprocess). Each entry has pre/post topology hashes and a [DecisionDelta](cci:2://file:///Users/spenstar/Documents/programming/Forge/crates/forge-core/src/tracing/checkpoint_diff.rs:98:0-109:1) showing what decisions that phase introduced. Answers: "which phase broke it?"
- **LineageEvents** — one `EntityCreated` per result face, carrying an `EntityRef` (kind + index) and [Lineage](cci:2://file:///Users/spenstar/Documents/programming/Forge/crates/forge-topo/src/topology/history/lineage.rs:76:0-83:1) (ancestry hash + creation op). Answers: "where did this face come from?"
- **Platform metadata** on ReplayLog — target triple, FMA flag, opt level. For cross-platform divergence detection.
## Pipeline Phases
**split → classify → select → assemble → postprocess**
| Phase | What It Does | Failure Signature |
|-------|-------------|-------------------|
| Split | Cuts faces by intersection planes | "N resolved vertices after dedup (need >=2)" |
| Classify | Labels faces Inside/Outside/Boundary | Wrong classification in decision context |
| Select | Keeps/discards faces for the operation | Missing faces in result |
| Assemble | Copies + stitches into result solid | `MissingTwin` — unpaired halfedges |
| Postprocess | Merges coplanar, removes redundant verts | Rarely fails if assemble succeeds |
## Debugging Discipline
1. Read the error output first — it contains the decision chain
2. Form one hypothesis from the decision context
3. Fix one thing, re-run
4. Do not open source files until you have a hypothesis from the output