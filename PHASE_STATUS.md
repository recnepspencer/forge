# Forge Kernel — Phase Status

> Single-table summary for AI agents and contributors. See `DEVELOPMENT_BLUEPRINT.MD` for full specs.

| Phase | Milestone | Status | Key Files |
|-------|-----------|--------|-----------|
| **0 — Foundation** | 0.1 Scaffold & CI | ✅ | workspace `Cargo.toml` |
| | 0.2 Filtered Arithmetic | ✅ | `forge-math/src/filtered/` |
| | 0.2.1 TriSign Predicates | ✅ | `forge-math/src/predicates/` |
| | 0.2.2 Coincidence Framework | ✅ | `forge-math/src/coincidence/` |
| | 0.2.3 Lazy Exact Eval | ✅ | `forge-math/src/double/` |
| | 0.3 Plane & Implicit Vertex | ✅ | `forge-geom/src/plane/`, `forge-geom/src/implicit_vertex/` |
| | 0.4 Determinism & Replay | ✅ | `forge-topo/src/replay.rs` |
| | 0.5.1 Epoch-Versioned State | ✅ | `forge-topo/src/state.rs` |
| | 0.5.2 Topology Hashing | ✅ | `forge-topo/src/hashing.rs` |
| **1 — Topology** | 1.1 Halfedge Mesh Core | ✅ | `forge-topo/src/arena.rs`, `forge-topo/src/euler/` |
| | 1.2 Euler Lineage Tracking | ✅ | `forge-topo/src/lineage.rs` |
| | 1.3 BSP Polyhedron | ✅ | `forge-geom/src/bsp/` |
| | 1.4 Point-in-Solid | ✅ | `forge-topo/src/classify.rs` |
| **1B — AI Affordances** | 1B.1 Structured Errors | ✅ | `forge-core/src/lib.rs` |
| | 1B.2 Operation Result Envelope | ✅ | `forge-core/src/result.rs` |
| | 1B.3 Decision Log Protocol | ✅ | `forge-core/src/result.rs`, `forge-kernel/src/core/context.rs` |
| | 1B.4 Topology Diff | ✅ | `forge-topo/src/diff.rs` |
| | 1B.5 Blueprint Accessibility | ✅ | `PHASE_STATUS.md`, `docs/ai-agent-guide.md` |
| **1C — Reactive Substrate** | 1C.1 Signal Graph Core | ✅ | `forge-signal/src/graph.rs`, `forge-signal/src/eval.rs` |
| | 1C.2 Multi-Aspect Topology Firewall | ✅ | `forge-signal/src/schema.rs` (Aspect, AspectVersion) |
| | 1C.3 Graph Lifecycle & Arena Safety | ✅ | `forge-signal/src/graph.rs` (unregister, GC) |
| | 1C.4 Cycle Detection & Parallelism | ✅ | `forge-signal/src/eval.rs`, `forge-signal/src/context.rs` |
| **2 — Planar Booleans** | 2.0 Spatial Acceleration | ✅ | `forge-geom/src/bsp/` |
| | 2.1 Face Splitting | ✅ | `forge-kernel/src/boolean/split.rs` |
| | 2.2 Face Classification & Assembly | ✅ | `forge-kernel/src/boolean/classify.rs`, `forge-topo/src/classify.rs`, `forge-kernel/src/boolean/assemble.rs` |
| | 2.3 Sketch→Extrude→Cut | ✅ | `forge-kernel/src/features/intent.rs` |
| | 2.4 Corpus Fuzzing | ✅ | `forge-test/src/generators.rs`, `forge-test/src/harness.rs` |
| | 2.5 Planar Edge-Case Battery | ✅ | `forge-kernel/src/boolean/edge_case_tests.rs` |
| | 2.6 Boolean Introspection | ✅ | `forge-kernel/src/boolean/schema.rs`, `forge-kernel/src/boolean/introspection_tests.rs` |
| | 2.7 Feature Tree | 🟢 | — |
| | 2.8 Native Serialization | 🔴 | — |
| **3 — Curved Geometry** | 3.0 Modeling Context | ✅ | `forge-kernel/src/core/context.rs` |
| | 3.0.1 Exactness Contract | 🔴 | — |
| | 3.1 Analytic Surface Library | ✅ | `forge-geom/src/surfaces/` |
| | 3.2–3.8 Curved SSI & Booleans | 🔴 | — |
| **4 — Fillets/Chamfers** | 4.1–4.6 | 🔴 | — |
| **5 — Sweep, Pattern & Selectors** | 5.1–5.3 | 🔴 | — |
| | 5.4 Selector System | 🔴 | — |
| | 5.5 Ambiguity Protocol | 🔴 | — |
| | 5.6 Undo/Redo | 🔴 | — |
| **6 — NURBS/Shell** | 6A.1–6B.3 | 🔴 | — |
| **7 — Manufacturing** | 7.1–7.3 | 🔴 | — |
| **8 — I/O, Validation & Agent API** | 8.1–8.4 File I/O | 🔴 | — |
| | 8.5 Parametric Test Cases | 🔴 | — |
| | 8.6 Agent API | 🔴 | — |
| | 8.7 Cost Estimation | 🔴 | — |

**Legend:** ✅ Complete · 🟡 In Progress · 🔴 Not Started
