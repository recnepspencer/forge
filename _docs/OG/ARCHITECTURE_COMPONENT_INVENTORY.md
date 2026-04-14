# Forge Architecture Component Inventory (Source-Derived)

Generated: 2026-02-25 (local workspace scan)

## Purpose

This document is a unified reference for architectural components already present in the codebase.
It is intentionally source-derived to reduce omissions.

It is meant to answer:

- "What subsystems exist already?"
- "What are the actual code names/types for them?"
- "Where do they live?"

## Scope / Coverage

Primary coverage source: `crates/*/src/**/*.rs` in this workspace.

Coverage counts (Rust source files):

- `forge-core`: total 19 (`prod=16`, `test=3`)
- `worth-geom`: total 52 (`prod=47`, `test=5`)
- `forge-io`: total 6 (`prod=5`, `test=1`)
- `forge-kernel`: total 148 (`prod=117`, `test=31`)
- `worth-math`: total 26 (`prod=26`, `test=0`)
- `forge-repr`: total 4 (`prod=3`, `test=1`)
- `forge-schema`: total 1 (`prod=1`, `test=0`)
- `forge-signal`: total 10 (`prod=9`, `test=1`)
- `forge-test`: total 8 (`prod=7`, `test=1`)
- `forge-topo`: total 89 (`prod=67`, `test=22`)
- `forge-view`: total 8 (`prod=8`, `test=0`)

Notes:

- This includes implementation modules and test/proof suites (listed in the appendix).
- The "Quick Lookup" section below is curated; the appendix is the completeness backstop.

## Unified Architectural Component List (Flat)

Single list of implemented architectural systems/components (no "known vs missed" split).

- `worth-math` exactness foundation (`crates/worth-math/src/lib.rs`)
- `TriSign`, `CertifiedTriSign` (`crates/worth-math/src/numeric/sign.rs`)
- `Rational` exact arithmetic (`crates/worth-math/src/arithmetic/rational.rs`)
- `Double` compensated arithmetic (`crates/worth-math/src/arithmetic/double.rs`)
- `Interval` arithmetic (`crates/worth-math/src/arithmetic/interval.rs`)
- Precision escalation framework: `PrecisionMode`, `PrecisionEscalation`, `PrecisionBudget` (`crates/worth-math/src/arithmetic/precision.rs`)
- Expansion arithmetic + Shewchuk-style error bounds / tiny-float safety (`crates/worth-math/src/arithmetic/expansion.rs`)
- Robust predicates: `orient2d`, `orient3d`, `incircle`, `in_sphere` (`crates/worth-math/src/predicates/*.rs`)
- Grid predicates (`crates/worth-math/src/predicates/grid_predicates.rs`)
- Symbolic perturbation / SoS (`crates/worth-math/src/coincidence/sos.rs`)
- Coincidence graph + merge actions (`crates/worth-math/src/coincidence/mod.rs`)
- Math geometry-access contract: `GeometrySource`, `PlaneCoefficients` (`crates/worth-math/src/data_access/schema.rs`)
- Deterministic RNG utilities (`crates/worth-math/src/numeric/deterministic_rng.rs`)
- Math error taxonomy: `MathError` (`crates/worth-math/src/error.rs`)
- Linear algebra helper layer (`crates/worth-math/src/linalg/mod.rs`)

- `forge-core` shared kernel language (`crates/forge-core/src/lib.rs`)
- Kernel error taxonomy: `KernelError`, `TopologyError`, `ErrorContext`, `ErrorScope`, `SuggestedFix` (`crates/forge-core/src/errors/schema.rs`)
- Serializable typed error summaries: `ErrorSummary`, `KernelErrorSummary`, `MergeErrorSummary`, `TopologyErrorSummary` (`crates/forge-core/src/errors/summary.rs`)
- Policy system: `PolicyKind`, `PolicyQuery`, `PolicyResult<T>` (`crates/forge-core/src/policy/schema.rs`)
- Tracing schema: `TracedDecision`, `DecisionKind`, `DecisionTier`, `DecisionContext`, `DecisionId`, `TraceEvent` (`crates/forge-core/src/tracing/schema.rs`)
- Span-based decision log: `DecisionLog`, `DecisionSummary`, `TraceSummary`, `TraceDiff` (`crates/forge-core/src/tracing/decision_log.rs`)
- Typed trace adjunct transport: `TraceAdjunctRecord`, `TraceAdjunctSet` (`crates/forge-core/src/tracing/adjunct.rs`)
- Typed policy decision adjuncts: `PolicyDecisionTracePayload`, `PolicyResolutionSource`, `PolicyResolutionOutcome` (`crates/forge-core/src/tracing/policy_trace.rs`)
- Typed persistent-resolution adjuncts: `ResolutionTracePayload`, `ResolutionOutcome`, `ResolutionRoute` (`crates/forge-core/src/tracing/resolution_trace.rs`)
- Typed re-identification adjuncts: `ReidentificationTracePayload`, `ReidentificationOutcome`, `ReidentificationCompatibilitySummary` (`crates/forge-core/src/tracing/reidentification_trace.rs`)
- Deterministic trace fingerprinting: `TraceFingerprint`, `compute_trace_fingerprint` (`crates/forge-core/src/tracing/fingerprint.rs`)
- Trace checkpoint diffing: `DecisionDelta`, `DecisionChange`, `CheckpointLog` (`crates/forge-core/src/tracing/checkpoint_diff.rs`)
- Divergence scan tooling: `DivergenceReport`, `DivergenceDetail`, `scan_for_divergences` (`crates/forge-core/src/tracing/divergence.rs`)
- Delta-debug utility (`crates/forge-core/src/tracing/delta_debug.rs`)
- Trace persistence + logging utilities (`crates/forge-core/src/tracing/persistence.rs`, `crates/forge-core/src/tracing/logging.rs`)
- Typed trace persistence path: `try_write_trace_file_with_adjuncts`, `TracePersistenceError` (`crates/forge-core/src/tracing/persistence.rs`)
- Operation envelope: `OperationResult<T>` (`crates/forge-core/src/envelope/schema.rs`)
- Envelope composition/drain utilities: `absorb_metadata`, `take_warnings`, `take_metrics`, `take_lineage_delta`, `take_accumulated_budget` (`crates/forge-core/src/envelope/schema.rs`)
- Operation metrics: `OperationMetrics` (`crates/forge-core/src/envelope/schema.rs`)
- Warning system: `KernelWarning` (includes sliver/short-edge/auto-decision/error-budget/regime-mismatch warnings) (`crates/forge-core/src/envelope/schema.rs`)
- Lineage summary envelope: `LineageDelta` (`crates/forge-core/src/envelope/schema.rs`)
- Structured provenance payloads: `SnapshotHandleRef`, `BoundarySegmentProvenance`, `MergeStepProvenance` (`crates/forge-core/src/provenance/schema.rs`)
- Tolerance provider abstraction: `ToleranceProvider`, `FlatToleranceProvider` (`crates/forge-core/src/tolerance.rs`)
- Core-level `GeometrySource` firewall trait (`crates/forge-core/src/lib.rs`)

- `worth-geom` geometry layer (`crates/worth-geom/src/lib.rs`)
- Primitive geometry: `Plane`, `Aabb`, rays, polygons, points (`crates/worth-geom/src/primitives/*`)
- Plane classification/intersection exact+approx helpers (`crates/worth-geom/src/primitives/plane/eval.rs`)
- Implicit vertex resolution / symbolic triple selection (`crates/worth-geom/src/primitives/implicit_vertex.rs`)
- Vertex geometry + provenance (`crates/worth-geom/src/primitives/vertex_geom.rs`)
- BSP pipeline: `BspSolid`, `BspNode`, `BspOp`, `BspConfig`, `PlaneSet` (`crates/worth-geom/src/spatial/bsp/*`)
- BVH spatial acceleration: `BvhNode`, overlap queries (`crates/worth-geom/src/spatial/bvh/*`)
- Epsilon welding (`crates/worth-geom/src/spatial/epsilon_weld.rs`)
- Edge matching / fuzzy weld candidates (`crates/worth-geom/src/spatial/edge_match.rs`)
- Local coordinate space + scale analysis (`crates/worth-geom/src/spatial/local_space.rs`)
- Union-find utility (`crates/worth-geom/src/spatial/union_find.rs`)
- Geometry algorithms: chord extraction/clipping (`crates/worth-geom/src/algorithms/chord.rs`)
- Geometry algorithms: CDT triangulation (`crates/worth-geom/src/algorithms/cdt.rs`)
- Geometry algorithms: clipping/intersection/polygon overlap (`crates/worth-geom/src/algorithms/*.rs`)
- Boundary certification subsystem (`crates/worth-geom/src/algorithms/boundary_cert/*`)
- Surface subsystem scaffolding: `SurfaceKind`, `SurfaceData`, `ParameterDomain`, `SurfaceRelation` (`crates/worth-geom/src/surface/*`)
- Curve subsystem scaffolding: `CurveKind`, `CurveGeom`, `CurveProvenance`, `SpCurveApproximation` (`crates/worth-geom/src/curve/*`)
- Coedge / UV trim representation: `Coedge`, `ParametricCurve2D` (`crates/worth-geom/src/coedge/mod.rs`)

- `forge-topo` topology foundation (`crates/forge-topo/src/lib.rs`)
- Topology arena storage: `TopologyArena` (`crates/forge-topo/src/arena/eval.rs`)
- Arena entity schemas: `FaceData`, `HalfEdgeData`, `VertexData`, `LoopData`, `EdgeData`, `ShellData`, `BodyData`, `LumpData`, `RegionData` (`crates/forge-topo/src/arena/schema.rs`)
- Typed generational handles: `FaceId`, `HalfEdgeId`, `VertexId`, `LoopId`, `EdgeId`, `ShellId`, `BodyId`, `LumpId`, `RegionId` (`crates/forge-topo/src/topology/handles.rs`)
- Curved-reference handles: `CurveRef`, `SurfaceRef`, `CoedgeRef` (`crates/forge-topo/src/topology/handles.rs`)
- Immutable topology snapshot: `TopologyState` (`crates/forge-topo/src/topology/state.rs`)
- Transactional topology mutations: `MutableDraft`, `DraftConfig` (`crates/forge-topo/src/topology/state.rs`)
- Committed re-identification index on topology snapshots: `TopologyState::reidentification_link_index` (`crates/forge-topo/src/topology/state.rs`)
- Single lineage channel via `LineageStore` — all provenance events flow through `LineageStore::apply()` invariant-enforcing choke point (`crates/forge-topo/src/provenance/data/lineage/tracking_store.rs`)
- Replay logging: `ReplayLog`, `ReplayEntry` (`crates/forge-topo/src/topology/history/replay.rs`)
- Provenance/lineage: `OpSignature`, `Lineage`, `LineageEvent` (`crates/forge-topo/src/topology/history/lineage.rs`)
- Live lineage registry: `LineageStore` (`crates/forge-topo/src/topology/history/lineage_store.rs`)
- Re-identification linkage substrate (P2-4A): `ReidentificationLinkRecord`, `ReidentificationLinkIndex`, `ReidentificationQuery`, `ReidentificationQueryResult`, `resolve_reidentification_query_v1` (`crates/forge-topo/src/topology/history/lineage_link.rs`)
- Topology-local generational lineage refs: `LineageEntityRef`, `TopoSnapshotHandleRef` (`crates/forge-topo/src/topology/history/lineage.rs`, `crates/forge-topo/src/topology/history/lineage_link.rs`)
- Entity bitmaps/bitsets: `EntityBitset`, `BitsetIterator` (`crates/forge-topo/src/topology/bitset.rs`)
- Attribute system: `EntityKey`, `TagValue`, `AttributeStore` (`crates/forge-topo/src/topology/attributes.rs`)
- Persistent naming subsystem: `PersistentName`, `Selector`, `resolve_name`, `resolve_selector`, `assign_name` (`crates/forge-topo/src/topology/naming/*`)
- Integrity validation stack: structural/geometric/healing/hashing/diff (`crates/forge-topo/src/topology/integrity/*`)
- Topology query suite: traverse/classify/classification/bounds/continuity/hierarchy/ordering/polygon/radial (`crates/forge-topo/src/topology/queries/*`)
- Deterministic ordering keys (`crates/forge-topo/src/topology/queries/ordering.rs`)
- Point-in-solid / point-on-face classification (`crates/forge-topo/src/topology/queries/classify.rs`)
- Euler operator framework: `EulerOperator`, `apply_op`, `ExecutionResult`, `EulerDelta` (`crates/forge-topo/src/topology/operations/operator.rs`)
- Euler operators: MVF/MEF/MEV/split/join/sew/unsew/KEML/MEKL/shell ops/face-from-vertices ops (`crates/forge-topo/src/topology/operations/euler/*`)
- Topology algorithms: BFS/components/bridge-edge/flip-edge/triangulate/extract-shell/region-extraction/simplify (`crates/forge-topo/src/topology/operations/algorithms/*`)

- `forge-signal` reactive dependency graph (`crates/forge-signal/src/lib.rs`)
- Signal graph core: `SignalGraph`, slot arena (`crates/forge-signal/src/graph.rs`)
- Node handles: `NodeId` (`crates/forge-signal/src/handles.rs`)
- Signal schema: `NodeState`, `Aspect`, `AspectVersion`, `DependencyEdge`, `NodeEntry` (`crates/forge-signal/src/schema.rs`)
- Dirty propagation (`mark_dirty`) (`crates/forge-signal/src/evaluation/push.rs`)
- Lazy recomputation (`evaluate`) (`crates/forge-signal/src/evaluation/pull.rs`)
- Parallel-safe evaluation context (`crates/forge-signal/src/evaluation/context.rs`)

- `forge-kernel` application/orchestration layer (`crates/forge-kernel/src/lib.rs`)
- Modeling policy engine/context: `ModelingContext` (`crates/forge-kernel/src/core/context.rs`)
- Topology delta snapshots for context/tracing: `ArenaSnapshot`, `compute_topology_delta` (`crates/forge-kernel/src/core/context.rs`)
- Sub-operation metadata sink/drain contract: `SubOperationMetadata`, `take_sub_metadata`, `absorb_sub_result` (`crates/forge-kernel/src/core/context.rs`)
- Trace adjunct sink lifecycle in context: `push_trace_adjunct`, `get_trace_adjuncts`, `take_trace_adjuncts` (`crates/forge-kernel/src/core/context.rs`)
- Policy registry + resolver (P2-3): `PolicyRegistrySnapshot`, `ResolvedPolicySource`, `ResolvedPolicyDecision`, `resolve_policy_query` (`crates/forge-kernel/src/core/context.rs`)
- Persistent naming resolution contracts (P2-4): `ResolutionQuery`, `ResolutionResult<T>`, `ResolutionCandidate`, `ResolutionEvidence`, `ResolutionIncompatibility` (`crates/forge-kernel/src/core/naming_resolution.rs`)
- Operation finalization contract (P2-2): `OperationFinalizer`, `CollectedFinalization`, `FinalizationSummary`, `FinalizationEmitError` (`crates/forge-kernel/src/core/finalization.rs`)
- Kernel tolerance policies: `TolerancePolicy`, `TangencyPolicy`, `SliverPolicy`, `GapClosurePolicy`, `PrecisionEscalationPolicy` (`crates/forge-kernel/src/core/tolerance.rs`)
- Kernel tolerance config bundle: `ToleranceConfig` (`crates/forge-kernel/src/core/tolerance.rs`)
- Absolute tolerance floor for tiny models: `ABSOLUTE_MINIMUM_TOLERANCE` (`crates/forge-kernel/src/core/tolerance.rs`)
- Kernel state bundle: `KernelState` (`crates/forge-kernel/src/core/kernel_state.rs`)
- Kernel transactional draft: `KernelDraft` (`crates/forge-kernel/src/core/kernel_draft.rs`)
- Operation-space abstraction: `OperationSpace` (`crates/forge-kernel/src/core/operation_space.rs`)
- B-rep workspace wrapper: `BRepWorkspace` (`crates/forge-kernel/src/core/brep_workspace.rs`)
- Tolerance decision macro (`check_tolerance!`) (`crates/forge-kernel/src/core/macros.rs`)

- Geometry side-car state: `GeometryState` (`crates/forge-kernel/src/geometry_state/schema.rs`)
- Exact vertex positions with cached f64 and symbolic planes: `ExactPosition` (`crates/forge-kernel/src/geometry_state/schema.rs`)
- Transactional geometry diff layer: `GeometryPatch` (`crates/forge-kernel/src/geometry_state/patch.rs`)
- Unified read-only geometry abstraction across base+patch: `GeometryView` (`crates/forge-kernel/src/geometry_state/mod.rs`)
- Vertex snap/coalescence subsystem (`crates/forge-kernel/src/geometry_state/coalescence.rs`)
- Split propagation for curve attachments (`crates/forge-kernel/src/geometry_state/split_propagation.rs`)
- Geometry position lookup builder (`crates/forge-kernel/src/geometry_state/eval.rs`)
- Geometry state curved entity arenas (surfaces/curves/coedges + mappings) (`crates/forge-kernel/src/geometry_state/schema.rs`)

- Feature system trait + outputs: `Feature`, `FeatureOutput` (`crates/forge-kernel/src/features/traits.rs`)
- Parametric feature graph manager: `FeatureTree` (`crates/forge-kernel/src/features/tree.rs`)
- Serializable feature enum: `NativeFeature` (`crates/forge-kernel/src/features/tree.rs`)
- Feature wrappers currently implemented: `MakeCubeFeature`, `BooleanFeature` (`crates/forge-kernel/src/features/wrappers.rs`)
- Feature intent layer (`crates/forge-kernel/src/features/intent.rs`)

- Mesh construction service: `MeshBuildResult`, `build_halfedge_mesh`, `make_cube`, `make_tetrahedron`, `make_dodecahedron`, `make_convex_solid` (`crates/forge-kernel/src/mesh_builder/eval.rs`)

- Boolean operation public API: `BooleanInput`, `BooleanOp`, `BooleanResult`, `execute_boolean` (`crates/forge-kernel/src/operations/boolean/mod.rs`, `crates/forge-kernel/src/operations/boolean/schema.rs`)
- Boolean introspection data (`crates/forge-kernel/src/operations/boolean/schema.rs`)
- Boolean pipeline phases: split/classify/assemble/postprocess (`crates/forge-kernel/src/operations/boolean/*`)
- Boolean engine abstraction traits: splitter/classifier/coplanar-resolver/assembler/postprocessor + `BooleanEngine` (`crates/forge-kernel/src/operations/boolean/traits.rs`)
- Planar engine implementations (`crates/forge-kernel/src/operations/boolean/engines/planar.rs`)
- EMBER boolean subsystem (adaptive/quantized path) (`crates/forge-kernel/src/operations/ember_boolean/*`)
- Boolean split exact-sign/reconcile/cut subsystems (`crates/forge-kernel/src/operations/boolean/split/*`)
- Boolean assembly stitch/copy/select/cleanup/rebuild-face subsystems (`crates/forge-kernel/src/operations/boolean/assemble/*`)
- Boolean postprocess subsystems: coplanar merge, hole splice, polygon extract, merge eligibility, redundant vertex removal (`crates/forge-kernel/src/operations/boolean/postprocess/*`)
- Region merge persistent/NMT execution stack (Epic B + P2-4): `execute_sheet_region_merge`, `execute_sheet_region_merge_persistent`, `MergeRegionSelectionPersistent`, persistent resolution + lineage fallback integration (`crates/forge-kernel/src/operations/boolean/postprocess/merge_eligibility/*`)
- Region-merge snapshot/runtime output contract split: `MergePlan`, `MergeResult`, `MergeResultSummary`, `SheetRegionMergeOutput` (`crates/forge-kernel/src/operations/boolean/postprocess/merge_eligibility/schema.rs`)
- Boolean debug and internal diagnostics modules (`crates/forge-kernel/src/operations/boolean/debug/*`)
- Boolean brutality/regression stress suites (test-focused but architecturally important coverage harness) (`crates/forge-kernel/src/operations/boolean/brutality/*`)

- Kernel analysis: sliver analysis (`crates/forge-kernel/src/analysis/sliver.rs`)
- Kernel analysis: gap measurement (`crates/forge-kernel/src/analysis/gap.rs`)
- Kernel analysis: region extractor (`crates/forge-kernel/src/analysis/region_extractor/*`)
- Kernel analysis: causal chain reconstruction (`crates/forge-kernel/src/analysis/causal_chain/*`)
- Kernel analysis: counterfactual replay (`crates/forge-kernel/src/analysis/counterfactual/*`)
- Kernel analysis: proof validation framework + checkpoints + invariants + milestone suites (`crates/forge-kernel/src/analysis/proof_validation/*`)

- `forge-io` JSON persistence layer (`crates/forge-io/src/lib.rs`)
- Versioned JSON model schema: `VersionedModel`, `SCHEMA_VERSION` (`crates/forge-io/src/json/schema.rs`)
- Model save/load (`crates/forge-io/src/json/eval.rs`)
- Model diffing: `ModelChange`, `diff_models` (`crates/forge-io/src/json/diff.rs`)
- Versioned audit storage substrate (H1/H5): `VersionedAuditRecord`, `AuditBundleManifest`, `write_audit_bundle`, `append_audit_record_jsonl` (`crates/forge-io/src/audit/*`)
- Audit schema conventions/validators: `AuditIdentityScope`, `AuditFieldLabel`, `AuditConventionError`, `VersionedAuditRecord::validate_conventions` (`crates/forge-io/src/audit/schema.rs`)
- IO error taxonomy: `IoError` (`crates/forge-io/src/lib.rs`)

- `forge-schema` declarative command language (`crates/forge-schema/src/lib.rs`)
- Declarative command enum: `Command` (`crates/forge-schema/src/lib.rs`)
- Stable entity references for API schema: `EntityRef` (`crates/forge-schema/src/lib.rs`)
- Edge selection schema: `EdgeSelector` (`crates/forge-schema/src/lib.rs`)
- Schema-level tag values (`crates/forge-schema/src/lib.rs`)

- `forge-repr` representation contracts (`crates/forge-repr/src/lib.rs`)
- `TriangleMesh` render/export representation (`crates/forge-repr/src/schema.rs`)
- `Viewable`, `Tessellatable` traits (`crates/forge-repr/src/traits.rs`)

- `forge-view` trace inspection stack (`crates/forge-view/src/lib.rs`)
- Trace store/query models (default/minimal feature path) (`crates/forge-view/src/trace/store.rs`)
- Trace HTTP server/router (feature-gated `server`) (`crates/forge-view/src/trace/server.rs`)
- Native trace viewer app (feature-gated `gui`) (`crates/forge-view/src/trace/viewer.rs`)
- Viewer/CLI binaries (`crates/forge-view/src/viewer_main.rs`, `crates/forge-view/src/cli_main.rs`, `crates/forge-view/src/main.rs`)

- `forge-test` shared test infrastructure crate (`crates/forge-test/src/lib.rs`)
- Test fixtures (`crates/forge-test/src/fixtures.rs`)
- Region-merge certifier/gate fixture builders (H7): `simple_square_boundary_2d`, `weakly_simple_endpoint_touch_boundary_2d`, `rejected_crossing_boundary_2d`, `face_group_bitset`, `hash_face_group_indices` (`crates/forge-test/src/region_merge_fixtures.rs`)
- Random generators / planar corpus generation (`crates/forge-test/src/generators/*`)
- Boolean fuzz harness + corpus runner (`crates/forge-test/src/harness/*`)
- Test logging adapters (`crates/forge-test/src/logging.rs`)

## Crate-Level Architecture Index

### `worth-math` (foundation math / exactness)

Top-level modules (`crates/worth-math/src/lib.rs`):

- `error`
- `env`
- `data_access`
- `numeric`
- `arithmetic`
- `predicates`
- `linalg`
- `coincidence`
- `prelude`, `traits`

Key architectural types/functions:

- Sign/certification: `TriSign`, `CertifiedTriSign`
- Arithmetic layers: `Double`, `Interval`, `Rational`
- Precision tracking: `PrecisionMode`, `PrecisionEscalation`, `PrecisionBudget`
- Predicates: `orient2d`, `orient3d`, `incircle`, `in_sphere`
- Grid predicates: `orient3d_grid`, `orient2d_grid`, `classify_point_grid`
- Symbolic perturbation / SoS: `orient3d_sos`, `orient2d_sos`, `SosPoint`
- Geometry access contracts: `GeometrySource`, `PlaneCoefficients`

<!-- PURPOSE: The kernel must never silently lose precision when dealing with
     very small or adversarial floating-point inputs. These five modules form a
     layered defence: expansion arithmetic tracks error bounds so the kernel knows
     *exactly* how many bits of precision it has consumed; compensated arithmetic
     (Double) uses Dekker/Knuth two-sum to extract the round-off error from each
     f64 op; interval arithmetic brackets every result with a guaranteed lower and
     upper bound, letting the kernel detect when a result straddles zero;
     exact rational arithmetic is the last-resort fallback that uses arbitrary-
     precision rationals so no decision is ever made on approximate data; and
     precision escalation bookkeeping tracks which level the current computation
     is at (fast → compensated → interval → rational) so the tracing system can
     report how often the kernel had to escalate. Together they guarantee that
     every geometric predicate is *certified* — the sign returned is provably
     correct regardless of the magnitude of the input coordinates. -->

Small-floating-point safety components (explicit):

- Error-bound constants + expansion ops (`expansion.rs`)
- Compensated arithmetic (`double.rs`)
- Intervals (`interval.rs`)
- Exact rational fallback (`rational.rs`)
- Precision escalation bookkeeping (`precision.rs`)

### `forge-core` (shared kernel language)

<!-- PURPOSE: forge-core is the *lingua franca* of the kernel — it defines the
     vocabulary that every other crate uses to communicate errors, policy
     questions, traced decisions, and operation results. By keeping these
     definitions in a single, low-dependency crate, the rest of the stack can
     import them without creating cyclic dependencies. -->

Top-level domains (`crates/forge-core/src/lib.rs`):

- `errors`
- `policy`
- `tracing`
- `envelope`
- `tolerance`

<!-- PURPOSE — Errors: A rich, structured error taxonomy so that every failure
     carries machine-readable context (where it happened, what entity was
     involved, what the caller might try differently). `KernelError` is the
     top-level enum; `TopologyError` covers structural violations;
     `ErrorContext` attaches the operation name, entity handle, and optional
     stack of scopes; `ErrorScope` lets the kernel push/pop nested contexts
     like "inside boolean split → inside face #42"; `SuggestedFix` gives
     downstream consumers (UI, AI agents) actionable remediation hints. -->

<!-- PURPOSE — Policy: The three-state decision protocol. Instead of silently
     rounding an ambiguous geometric result, lower layers return
     `PolicyResult::Ambiguous { kind, query }`. The `PolicyKind` enum
     classifies *what* is ambiguous (coincident geometry, near-tangency,
     sliver face, etc.) and `PolicyQuery` carries the raw data the kernel
     needs to make the call. This forces every tolerance-sensitive branch to
     be explicitly logged and auditable. -->

<!-- PURPOSE — Tracing: The observability backbone. `TracedDecision` records
     a single kernel decision with its kind (Exact / PolicyApplied / Forced),
     tier (how critical), and the raw numeric context. `DecisionLog` collects
     all decisions from one operation into a span tree. `DecisionSummary` and
     `TraceSummary` provide rollup statistics for quick triage.
     `CheckpointLog` / `DecisionDelta` / `DecisionChange` power
     before-vs-after diffing so you can see what changed between two runs.
     `DivergenceReport` / `DivergenceDetail` flag any non-deterministic
     decisions across repeated executions — critical for proving
     repeatability. -->

<!-- PURPOSE — Envelope: The universal return wrapper. `OperationResult<T>`
     bundles the actual value with the `DecisionLog`, timing metrics
     (`OperationMetrics`), lineage deltas (`LineageDelta`), and any
     `KernelWarning` instances (sliver faces, short edges, error-budget
     exhaustion, etc.). This ensures that no operation can return a result
     without also surfacing its audit trail. `into_value()` auto-persists
     the trace to disk when `FORGE_TRACE_DIR` is set. -->

<!-- PURPOSE — Tolerance provider: A trait abstraction that decouples
     geometry-state implementations from the tolerance values they need.
     `ToleranceProvider` is implemented by `GeometryState` and
     `GeometryPatch` so that tolerance values flow anonymously through the
     stack without the lower crates importing `ToleranceConfig`.
     `FlatToleranceProvider` is a simple struct for tests that just wraps a
     single `f64` value. -->

Key architectural types:

- Errors: `KernelError`, `TopologyError`, `ErrorContext`, `ErrorScope`, `SuggestedFix`
- Policy: `PolicyKind`, `PolicyQuery`, `PolicyResult<T>`
- Tracing:
  - `TracedDecision`, `DecisionKind`, `DecisionTier`
  - `DecisionLog`, `DecisionSummary`, `TraceSummary`, `TraceDiff`
  - `CheckpointLog`, `DecisionDelta`, `DecisionChange`
  - `DivergenceReport`, `DivergenceDetail`
- Envelope:
  - `OperationResult<T>`
  - `OperationMetrics`
  - `LineageDelta`
  - `KernelWarning`
- Tolerance provider abstraction:
  - `ToleranceProvider`
  - `FlatToleranceProvider`

Important tracing submodules (`crates/forge-core/src/tracing/`):

- `decision_log.rs`
- `checkpoint_diff.rs`
- `delta_debug.rs`
- `divergence.rs`
- `persistence.rs`
- `logging.rs`
- `schema.rs`

### `worth-geom` (geometry primitives, spatial structures, algorithms)

Top-level modules (`crates/worth-geom/src/lib.rs`):

- `primitives`
- `spatial`
- `curve`
- `surface`
- `coedge`
- `algorithms`
- `prelude`, `traits`

Key architectural exports:

- Primitives: `Plane`, `Aabb`, ray intersection helpers, implicit `Vertex`, `VertexGeom`
- Spatial:
  - BSP: `BspSolid`, `BspOp`, `BspConfig`, `PlaneSet`
  - BVH: `BvhNode`
  - Edge matching: `fuzzy_match_edges`, `FuzzyMatchMode`
  - Local scale analysis: `LocalCoordinateSpace`, `ScaleAnalysis`
  - `epsilon_weld`, `union_find`, `coincidence`
- Algorithms:
  - Chord/intersection line clipping
  - CDT triangulation
  - Polygon clipping/overlap
  - Boundary certification (`algorithms/boundary_cert/*`)
- Curved geometry scaffolding:
  - `SurfaceKind`, `SurfaceData`, `ParameterDomain`, `SurfaceRelation`
  - `CurveKind`, `CurveGeom`, `CurveProvenance`, `SpCurveApproximation`
  - `Coedge`, `ParametricCurve2D`

### `forge-topo` (topology arena, handles, Euler ops, integrity, provenance)

<!-- PURPOSE: forge-topo owns the *structure* of B-rep solids — the graph of
     vertices, edges, loops, faces, shells, regions, lumps, and bodies that
     describes "what is connected to what" without any floating-point
     geometry. All mutations go through a transactional `MutableDraft` so
     that partial failures never leave the arena in a corrupt state. -->

Top-level modules (`crates/forge-topo/src/topology/mod.rs`):

- `state`
- `handles`
- `attributes`
- `bitset`
- `integrity`
- `operations`
- `queries`
- `history`
- `naming`

Core architecture pieces:

<!-- PURPOSE — Transactional topology state: `TopologyState` is an immutable
     snapshot of the entire topology arena at a point in time. To mutate it,
     you open a `MutableDraft` (configured by `DraftConfig`), make changes,
     and then commit — producing a new `TopologyState`. If any step fails
     the draft is simply dropped and the original state is untouched. This
     enforces Doctrine D6 (Atomic Transactionality). -->

- Transactional topology state:
  - `TopologyState`
  - `MutableDraft`
  - `DraftConfig`

<!-- PURPOSE — Typed generational handles: Every topology entity is referenced
     by a strongly-typed handle (e.g. `FaceId`, `VertexId`) that embeds a
     generation counter. If an entity is removed and its slot reused, the old
     handle's generation will not match, preventing ABA bugs. The curved-ref
     handles (`CurveRef`, `SurfaceRef`, `CoedgeRef`) extend the same safety
     to parametric geometry attachments. -->

- Typed generational handles:
  - `FaceId`, `HalfEdgeId`, `VertexId`, `LoopId`, `BodyId`, `LumpId`, `RegionId`, `ShellId`, `EdgeId`
  - Curved refs: `CurveRef`, `SurfaceRef`, `CoedgeRef`

<!-- PURPOSE — Arena data store: `TopologyArena` is the raw slot-map storage
     that backs all entity handles. Each entity kind has its own data schema
     struct (e.g. `FaceData` stores the outer loop, shell parent, and
     optional attributes) defined in `arena/schema.rs`. The arena never
     exposes raw indices — all access goes through the typed handles. -->

- Arena data store:
  - `TopologyArena`
  - entity data schemas in `arena/schema.rs` (`FaceData`, `HalfEdgeData`, `VertexData`, `LoopData`, `ShellData`, `BodyData`, `LumpData`, `RegionData`, `EdgeData`)

<!-- PURPOSE — Bitset utility: A high-performance dense bitset for tracking
     visited entities in BFS traversals, validation passes, and component
     extraction. `EntityBitset` provides O(1) test/set/clear and
     `BitsetIterator` yields set indices without allocation — significantly
     faster than `HashSet` for the common case of iterating all entities in
     a bounded arena. -->

- Bitset/bitmap utility:
  - `EntityBitset`, `BitsetIterator`

<!-- PURPOSE — Provenance/history: Every Euler operator stamps each created
     entity with an `OpSignature` (a unique fingerprint of the operation and
     its inputs). `Lineage` and `LineageEvent` track the full birth/split/
     merge history of an entity, enabling causal-chain queries ("why does
     this face exist?"). `LineageStore` is a live, indexed registry.
     `ReplayLog` / `ReplayEntry` record every operator invocation so that
     an entire modeling session can be deterministically replayed. -->

- Provenance/history:
  - `OpSignature`, `Lineage`, `LineageEvent`
  - `LineageStore`
  - `ReplayLog`, `ReplayEntry`

<!-- PURPOSE — Attributes: A side-car key-value store that lets the kernel
     attach semantic metadata (material IDs, feature names, user tags) to
     any entity without polluting the core arena schema. `EntityKey`
     identifies the entity, `TagValue` is the typed payload, and
     `AttributeStore` holds the mapping. -->

- Attributes:
  - `EntityKey`, `TagValue`, `AttributeStore`

<!-- PURPOSE — Persistent naming: Allows entities to survive topology edits
     with a stable identity. `PersistentName` is a user-facing label;
     `Selector` is a query DSL ("the face adjacent to edge X on body Y");
     `resolve_name` / `resolve_selector` find the current handle for a
     previously-named entity; `assign_name` binds a name to a handle.
     This is essential for parametric re-evaluation — when a feature is
     edited, downstream features must relocate their reference entities. -->

- Persistent naming:
  - `PersistentName`, `Selector`, `resolve_name`, `resolve_selector`, `assign_name`

Euler operator framework (`crates/forge-topo/src/topology/operations/`):

- Runner/trait: `operator.rs` (`EulerOperator`, `apply_op`, `ExecutionResult`, `EulerDelta`)
- Atomic ops (module set in `euler/mod.rs`):
  - `make_vertex_face`
  - `make_edge_face`
  - `make_edge_vertex`
  - `split_edge`
  - `join_faces`
  - `join_faces_nmt`
  - `kill_edge_vertex`
  - `kill_edge_make_loop`
  - `make_edge_kill_loop`
  - `sew_edge`
  - `unsew_edge`
  - `make_shell_face`
  - `kill_shell_face`
  - `kill_vertex_face`
  - `make_face_from_vertices`
  - `make_face_in_shell_from_vertices`
  - `make_loop_in_face_from_vertices`

Topology algorithms and queries:

- Algorithms: BFS/components, `bridge_edge`, `flip_edge`, `triangulate`, `extract_shell`, region extraction, simplify cleanup/collinear consolidation
- Queries: traversal iterators, bounds, point classification, continuity, hierarchy, deterministic ordering, polygon extraction, radial use indexing
- Integrity: structural/geometric validation, hashing, healing, diffing

### `forge-kernel` (application layer orchestration)

Top-level modules (`crates/forge-kernel/src/lib.rs`):

- `core`
- `features`
- `geometry_state`
- `mesh_builder`
- `operations`
- `analysis`
- `brep`
- `prelude`

#### `core` (kernel orchestration, tolerance, state wrappers)

<!-- PURPOSE: The kernel core is the central orchestration point. It bundles
     topology + geometry into a single transactional unit (`KernelState` /
     `KernelDraft`), owns all tolerance thresholds (`ToleranceConfig`), and
     provides the `ModelingContext` which every operation uses to log
     decisions, query policies, and snapshot state for tracing.
     `BRepWorkspace` wraps the full B-rep context (state + context +
     geometry) for high-level operations. `OperationSpace` provides a
     scoped context for individual operation execution. `ArenaSnapshot` /
     `compute_topology_delta` let the tracing system capture before/after
     diffs of the topology for every operation. -->

Main exports (`crates/forge-kernel/src/core/mod.rs`):

- `ModelingContext`
- `ArenaSnapshot`, `compute_topology_delta`
- Tolerance policy/config structs
- `BRepWorkspace`
- `KernelState`
- `KernelDraft`
- `OperationSpace`

Important files:

- `context.rs` (decision logging + modeling context)
- `tolerance.rs` (all kernel tolerance policies/config)
- `kernel_state.rs` (owned topo+geom bundle)
- `kernel_draft.rs` (transactional topo+geom draft)
- `operation_space.rs`
- `brep_workspace.rs`
- `macros.rs` (`check_tolerance!`)

#### `geometry_state` (side-car geometry for topology entities)

<!-- PURPOSE: Topology handles are intentionally geometry-free — they store
     only connectivity. `GeometryState` is the side-car that maps topology
     handles to their actual geometric data (face → plane, vertex → exact
     position, edge → curve). `ExactPosition` stores both a cached `f64`
     position and the symbolic triple of defining planes, so the kernel can
     recompute the vertex to arbitrary precision on demand.
     `GeometryPatch` mirrors the transactional pattern: during an
     operation, geometry changes accumulate in a patch overlay; on commit
     they merge into the base `GeometryState`. `GeometryView` is a
     read-only trait that abstracts over both base and patched state so
     queries don't care whether a commit has happened yet.
     `snap_or_coalesce_vertex` handles the tolerance-aware merging of
     nearly-coincident vertices. `propagate_curve_on_split` ensures that
     when an edge is split, the curve attachment is correctly inherited by
     both children. `build_position_lookup` produces an indexed map for
     fast coordinate retrieval during heavy passes like boolean split. -->

Exports (`crates/forge-kernel/src/geometry_state/mod.rs`):

- `GeometryState`
- `ExactPosition`
- `GeometryPatch`
- `build_position_lookup`
- `snap_or_coalesce_vertex`, `CoalescenceResult`
- `propagate_curve_on_split`
- `GeometryView` trait (read-only abstraction across `GeometryState` and `GeometryPatch`)

Important implementation modules:

- `schema.rs` (face planes, exact vertex positions, tolerances, curved-entity arenas)
- `patch.rs` (transactional geometry diff layer)
- `coalescence.rs`
- `split_propagation.rs`
- `eval.rs`

#### `features` (parametric feature graph)

Current implemented feature infrastructure:

- `Feature` trait and `FeatureOutput`
- `FeatureTree`
- `NativeFeature` enum (currently includes `MakeCube`, `Boolean`)
- Feature wrappers in `wrappers.rs`
- `intent.rs` (feature intent layer)

#### `mesh_builder`

Exports:

- `MeshBuildResult`
- `build_halfedge_mesh`
- Primitive builders: `make_cube`, `make_tetrahedron`, `make_dodecahedron`, `make_convex_solid`

#### `operations`

Current operation domains (`crates/forge-kernel/src/operations/mod.rs`):

- `boolean`
- `ember_boolean`

Boolean architecture (`crates/forge-kernel/src/operations/boolean/`):

- Public entry/types:
  - `BooleanInput`
  - `BooleanOp`
  - `BooleanResult`
  - `FaceClassification`
  - `ClassifiedFace`
  - `execute_boolean(...)` (adaptive EMBER -> standard fallback)
- Pipeline modules:
  - `split`
  - `classify`
  - `assemble`
  - `postprocess`
  - `engines`
  - `traits` (engine interfaces + `BooleanEngine`)
- Additional support:
  - `eval.rs`, `schema.rs`, `debug/`
  - test helpers + extensive brutality suites (test-only)

EMBER boolean (`crates/forge-kernel/src/operations/ember_boolean/`):

- `execute_ember_boolean`
- `execute_boolean_adaptive`
- `QuantizedSpace`
- `checkpoint`, `classify`, `mesh`, `quantize`, `schema`

#### `analysis` (diagnostics, replay, proof checks)

<!-- PURPOSE: The analysis subsystem is the kernel's self-inspection toolkit.
     It answers questions that go beyond "did the operation succeed?" to
     "WHY did it succeed, and can we prove it will always succeed?"

     Gap analysis (`GapReport`, `measure_gap`) samples adjacent faces to
     detect geometric gaps or overlaps that violate manifold closure — this
     is the primary quality metric for boolean output.

     Sliver analysis (`SliverReport`, `analyze_slivers`) detects degenerate
     near-zero-area faces that could cause downstream numerical failures.

     Region extraction (`ExtractedRegion`, `extract_n_ring`) pulls out a
     local topological neighbourhood (n-ring) around a target entity for
     focused debugging and visualization.

     Causal chain (`CausalChain`, `CausalStep`, `query_causal_chain`)
     reconstructs the lineage path: "this face exists because vertex X was
     split by boolean op Y, which classified face Z as inside" — essential
     for explaining unexpected results to users and AI agents.

     Counterfactual replay lets you re-run an entire operation with a
     single decision flipped (e.g. "what if this near-boundary vertex had
     been classified OUTSIDE instead of INSIDE?"), enabling automated
     sensitivity analysis and tolerance-boundary testing.

     Proof validation (`ValidationCheckpoint`, `run_checkpoint`) is the
     formal verification layer. Each checkpoint asserts a set of
     mathematical invariants (Euler formula, manifold closure, consistent
     face orientations, etc.) at a specific pipeline stage. The `pv_p*`
     milestone suites progressively prove correctness properties from
     basic arithmetic (P0) through full boolean output (P3). -->

Modules (`crates/forge-kernel/src/analysis/mod.rs`):

- `sliver`
- `proof_validation`
- `region_extractor`
- `causal_chain`
- `counterfactual`
- `gap`

Notable exports/types:

- Gap analysis: `GapReport`, `GapSampleDensity`, `measure_gap`
- Sliver analysis: `SliverReport`, `analyze_slivers`
- Region extraction: `ExtractedRegion`, `extract_n_ring`
- Causal chain: `CausalChain`, `CausalStep`, `ChainSummary`, `query_causal_chain`
- Counterfactual replay: decision replay/replay-all-near-boundary APIs
- Proof validation:
  - `ValidationCheckpoint`, `ValidationConfig`, `ValidationResult`, `run_checkpoint`
  - pipeline diagnostics and proof invariant validators
  - many milestone proof test suites (`pv_p*`)

### `forge-signal` (reactive dependency graph)

Top-level modules:

- `handles`
- `schema`
- `graph`
- `evaluation` (`push`, `pull`, `context`)

Key types/functions:

- `NodeId`
- `SignalGraph`
- `EvaluationContext`
- `mark_dirty(...)`
- `evaluate(...)`
- schema types: `NodeState`, `Aspect`, `AspectVersion`, `DependencyEdge`, `NodeEntry`

### `forge-io` (JSON persistence / diff)

Modules:

- `json::schema`
- `json::eval`
- `json::diff`

Key types/functions:

- `IoError`
- `VersionedModel`
- `SCHEMA_VERSION`
- `save_model(...)`
- `load_model(...)`
- `ModelChange`
- `diff_models(...)`

### `forge-schema` (declarative command schema)

Single-file crate: `crates/forge-schema/src/lib.rs`

Key types:

- `Command`
- `EntityRef`
- `EdgeSelector`
- `TagValue`
- `SCHEMA_VERSION`

### `forge-repr` (render/export representations)

Key types:

- `TriangleMesh`
- `Viewable`
- `Tessellatable`

### `forge-view` (trace inspection UI/CLI/server)

Trace subsystem (`crates/forge-view/src/trace/`):

- `store.rs` (`TraceStore`, trace view models)
- `server.rs` (router/app state)
- `viewer.rs` (`TraceViewerApp`)

Binaries present in crate:

- `main.rs`
- `viewer_main.rs`
- `cli_main.rs`

### `forge-test` (test harness support crate)

Modules:

- `fixtures`
- `generators`
- `harness`
- `logging`

Notable pieces:

- planar random generators and boolean pair generators
- corpus fuzz harness (`run_fuzz_corpus`)
- shared fixture builders and logging adapters

## Cross-Crate Architectural Connections (What Exists Today)

- Exactness path:
  - `forge-kernel` -> `forge-topo` / `worth-geom`
  - `worth-geom` / `forge-topo` decisions -> `worth-math` predicates/arithmetic
- Policy/tolerance path:
  - `forge-kernel::core::ModelingContext` + `ToleranceConfig`
  - `forge-core::PolicyResult` and tracing decisions
  - `forge-core::ToleranceProvider` implemented by `GeometryState` / `GeometryPatch`
- Transactionality path:
  - `TopologyState -> MutableDraft -> TopologyState`
  - `KernelState -> KernelDraft -> KernelState`
  - `GeometryPatch` mirrors transactional behavior for geometry side-car data
- Provenance/replay path:
  - Euler op runner (`apply_op`) -> `OpSignature` / `Lineage` / `LineageEvent`
  - Replay logs in topo + tracing in core + proof/counterfactual tooling in kernel analysis

## Appendix A: Complete Rust Source File Inventory (Grouped by Crate)

The following appendix is generated from `crates/*/src/**/*.rs` and is included as a completeness checklist.

```text
## crates/forge-core
crates/forge-core/src/envelope/mod.rs
crates/forge-core/src/envelope/schema.rs
crates/forge-core/src/envelope/tests.rs
crates/forge-core/src/errors/mod.rs
crates/forge-core/src/errors/schema.rs
crates/forge-core/src/errors/tests.rs
crates/forge-core/src/lib.rs
crates/forge-core/src/policy/mod.rs
crates/forge-core/src/policy/schema.rs
crates/forge-core/src/tolerance.rs
crates/forge-core/src/tracing/checkpoint_diff.rs
crates/forge-core/src/tracing/decision_log.rs
crates/forge-core/src/tracing/delta_debug.rs
crates/forge-core/src/tracing/divergence.rs
crates/forge-core/src/tracing/logging.rs
crates/forge-core/src/tracing/mod.rs
crates/forge-core/src/tracing/persistence.rs
crates/forge-core/src/tracing/schema.rs
crates/forge-core/src/tracing/tests.rs

## crates/worth-geom
crates/worth-geom/src/algorithms/angular_sort.rs
crates/worth-geom/src/algorithms/boundary_cert/adversarial_tests.rs
crates/worth-geom/src/algorithms/boundary_cert/eval.rs
crates/worth-geom/src/algorithms/boundary_cert/exact_intersect.rs
crates/worth-geom/src/algorithms/boundary_cert/mod.rs
crates/worth-geom/src/algorithms/boundary_cert/schema.rs
crates/worth-geom/src/algorithms/boundary_cert/split.rs
crates/worth-geom/src/algorithms/boundary_cert/tests.rs
crates/worth-geom/src/algorithms/cdt.rs
crates/worth-geom/src/algorithms/chord.rs
crates/worth-geom/src/algorithms/clipping.rs
crates/worth-geom/src/algorithms/intersection.rs
crates/worth-geom/src/algorithms/mod.rs
crates/worth-geom/src/algorithms/polygon.rs
crates/worth-geom/src/algorithms/polygon_overlap.rs
crates/worth-geom/src/coedge/mod.rs
crates/worth-geom/src/curve/eval.rs
crates/worth-geom/src/curve/mod.rs
crates/worth-geom/src/curve/schema.rs
crates/worth-geom/src/lib.rs
crates/worth-geom/src/prelude.rs
crates/worth-geom/src/primitives/aabb.rs
crates/worth-geom/src/primitives/implicit_vertex.rs
crates/worth-geom/src/primitives/mod.rs
crates/worth-geom/src/primitives/plane/eval.rs
crates/worth-geom/src/primitives/plane/mod.rs
crates/worth-geom/src/primitives/plane/tests.rs
crates/worth-geom/src/primitives/point.rs
crates/worth-geom/src/primitives/polygon.rs
crates/worth-geom/src/primitives/ray.rs
crates/worth-geom/src/primitives/shapes.rs
crates/worth-geom/src/primitives/vertex_geom.rs
crates/worth-geom/src/spatial/bsp/convert.rs
crates/worth-geom/src/spatial/bsp/eval.rs
crates/worth-geom/src/spatial/bsp/merge.rs
crates/worth-geom/src/spatial/bsp/mod.rs
crates/worth-geom/src/spatial/bsp/schema.rs
crates/worth-geom/src/spatial/bsp/tests.rs
crates/worth-geom/src/spatial/bvh/eval.rs
crates/worth-geom/src/spatial/bvh/mod.rs
crates/worth-geom/src/spatial/bvh/schema.rs
crates/worth-geom/src/spatial/bvh/tests.rs
crates/worth-geom/src/spatial/coincidence.rs
crates/worth-geom/src/spatial/edge_match.rs
crates/worth-geom/src/spatial/epsilon_weld.rs
crates/worth-geom/src/spatial/local_space.rs
crates/worth-geom/src/spatial/mod.rs
crates/worth-geom/src/spatial/union_find.rs
crates/worth-geom/src/surface/eval.rs
crates/worth-geom/src/surface/mod.rs
crates/worth-geom/src/surface/schema.rs
crates/worth-geom/src/traits.rs

## crates/forge-io
crates/forge-io/src/json/diff.rs
crates/forge-io/src/json/eval.rs
crates/forge-io/src/json/mod.rs
crates/forge-io/src/json/schema.rs
crates/forge-io/src/json/tests.rs
crates/forge-io/src/lib.rs

## crates/forge-kernel
crates/forge-kernel/src/analysis/causal_chain/eval.rs
crates/forge-kernel/src/analysis/causal_chain/mod.rs
crates/forge-kernel/src/analysis/causal_chain/schema.rs
crates/forge-kernel/src/analysis/counterfactual/eval.rs
crates/forge-kernel/src/analysis/counterfactual/mod.rs
crates/forge-kernel/src/analysis/counterfactual/schema.rs
crates/forge-kernel/src/analysis/gap.rs
crates/forge-kernel/src/analysis/mod.rs
crates/forge-kernel/src/analysis/proof_validation/checkpoint.rs
crates/forge-kernel/src/analysis/proof_validation/diagnose_pipeline.rs
crates/forge-kernel/src/analysis/proof_validation/mod.rs
crates/forge-kernel/src/analysis/proof_validation/proof_invariants.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p0_1_tests.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p0_2_tests.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p0_3_tests.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p0_4_tests.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p0_5_tests.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p0_5b_tests.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p2_1_tests.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p2_2_tests.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p2_3_tests.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p2_4_tests.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p2_5_mb_n_tests.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p3_1_tests.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p3_2_tests.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p3_3_tests.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p3_4_tests.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p3_5_tests.rs
crates/forge-kernel/src/analysis/proof_validation/pv_p3_6_tests.rs
crates/forge-kernel/src/analysis/proof_validation/test_support.rs
crates/forge-kernel/src/analysis/region_extractor/eval.rs
crates/forge-kernel/src/analysis/region_extractor/mod.rs
crates/forge-kernel/src/analysis/region_extractor/schema.rs
crates/forge-kernel/src/analysis/sliver.rs
crates/forge-kernel/src/brep/mod.rs
crates/forge-kernel/src/core/brep_workspace.rs
crates/forge-kernel/src/core/context.rs
crates/forge-kernel/src/core/kernel_draft.rs
crates/forge-kernel/src/core/kernel_state.rs
crates/forge-kernel/src/core/macros.rs
crates/forge-kernel/src/core/mod.rs
crates/forge-kernel/src/core/operation_space.rs
crates/forge-kernel/src/core/tolerance.rs
crates/forge-kernel/src/features/intent.rs
crates/forge-kernel/src/features/mod.rs
crates/forge-kernel/src/features/traits.rs
crates/forge-kernel/src/features/tree.rs
crates/forge-kernel/src/features/wrappers.rs
crates/forge-kernel/src/geometry_state/adversarial_tests.rs
crates/forge-kernel/src/geometry_state/coalescence.rs
crates/forge-kernel/src/geometry_state/eval.rs
crates/forge-kernel/src/geometry_state/mod.rs
crates/forge-kernel/src/geometry_state/patch.rs
crates/forge-kernel/src/geometry_state/patch_tests.rs
crates/forge-kernel/src/geometry_state/schema.rs
crates/forge-kernel/src/geometry_state/split_propagation.rs
crates/forge-kernel/src/geometry_state/tests.rs
crates/forge-kernel/src/lib.rs
crates/forge-kernel/src/mesh_builder/eval.rs
crates/forge-kernel/src/mesh_builder/mod.rs
crates/forge-kernel/src/mesh_builder/tests.rs
crates/forge-kernel/src/operations/boolean/assemble/cleanup.rs
crates/forge-kernel/src/operations/boolean/assemble/copy.rs
crates/forge-kernel/src/operations/boolean/assemble/copy_stitch_tests.rs
crates/forge-kernel/src/operations/boolean/assemble/disjoint/assemble.rs
crates/forge-kernel/src/operations/boolean/assemble/disjoint/eval.rs
crates/forge-kernel/src/operations/boolean/assemble/disjoint/mod.rs
crates/forge-kernel/src/operations/boolean/assemble/merge/assemble.rs
crates/forge-kernel/src/operations/boolean/assemble/merge/eval.rs
crates/forge-kernel/src/operations/boolean/assemble/merge/mod.rs
crates/forge-kernel/src/operations/boolean/assemble/mod.rs
crates/forge-kernel/src/operations/boolean/assemble/rebuild_face.rs
crates/forge-kernel/src/operations/boolean/assemble/select.rs
crates/forge-kernel/src/operations/boolean/assemble/stitch/eval.rs
crates/forge-kernel/src/operations/boolean/assemble/stitch/fallback.rs
crates/forge-kernel/src/operations/boolean/assemble/stitch/mod.rs
crates/forge-kernel/src/operations/boolean/brutality/coincidence.rs
crates/forge-kernel/src/operations/boolean/brutality/deep_chains.rs
crates/forge-kernel/src/operations/boolean/brutality/determinism.rs
crates/forge-kernel/src/operations/boolean/brutality/features.rs
crates/forge-kernel/src/operations/boolean/brutality/fuzzing.rs
crates/forge-kernel/src/operations/boolean/brutality/integrity.rs
crates/forge-kernel/src/operations/boolean/brutality/mb1_coplanar_apocalypse.rs
crates/forge-kernel/src/operations/boolean/brutality/mb2_menger_graze.rs
crates/forge-kernel/src/operations/boolean/brutality/mb3_singularity_star.rs
crates/forge-kernel/src/operations/boolean/brutality/mb4_thin_labyrinth.rs
crates/forge-kernel/src/operations/boolean/brutality/mb5_cancellation_chain.rs
crates/forge-kernel/src/operations/boolean/brutality/mb6_halfspace_storm.rs
crates/forge-kernel/src/operations/boolean/brutality/mb7_micro_feature_avalanche.rs
crates/forge-kernel/src/operations/boolean/brutality/mb8_ultimate_degeneracy.rs
crates/forge-kernel/src/operations/boolean/brutality/mod.rs
crates/forge-kernel/src/operations/boolean/brutality/performance.rs
crates/forge-kernel/src/operations/boolean/brutality/predicates.rs
crates/forge-kernel/src/operations/boolean/brutality/serialization.rs
crates/forge-kernel/src/operations/boolean/brutality/sliver.rs
crates/forge-kernel/src/operations/boolean/brutality/splitting.rs
crates/forge-kernel/src/operations/boolean/brutality/tier1_manifold.rs
crates/forge-kernel/src/operations/boolean/brutality/tier2_numerical.rs
crates/forge-kernel/src/operations/boolean/brutality/tier3_adversarial.rs
crates/forge-kernel/src/operations/boolean/brutality/tolerance_integration.rs
crates/forge-kernel/src/operations/boolean/brutality/trace_dump.rs
crates/forge-kernel/src/operations/boolean/brutality/tracing.rs
crates/forge-kernel/src/operations/boolean/classify/coplanar.rs
crates/forge-kernel/src/operations/boolean/classify/eval.rs
crates/forge-kernel/src/operations/boolean/classify/mod.rs
crates/forge-kernel/src/operations/boolean/debug/mod.rs
crates/forge-kernel/src/operations/boolean/engines/mod.rs
crates/forge-kernel/src/operations/boolean/engines/planar.rs
crates/forge-kernel/src/operations/boolean/eval.rs
crates/forge-kernel/src/operations/boolean/mod.rs
crates/forge-kernel/src/operations/boolean/postprocess/coplanar.rs
crates/forge-kernel/src/operations/boolean/postprocess/hole_splice.rs
crates/forge-kernel/src/operations/boolean/postprocess/merge_eligibility/boundary_adapter.rs
crates/forge-kernel/src/operations/boolean/postprocess/merge_eligibility/eval.rs
crates/forge-kernel/src/operations/boolean/postprocess/merge_eligibility/mod.rs
crates/forge-kernel/src/operations/boolean/postprocess/merge_eligibility/nmt_eval.rs
crates/forge-kernel/src/operations/boolean/postprocess/merge_eligibility/schema.rs
crates/forge-kernel/src/operations/boolean/postprocess/merge_eligibility/tests.rs
crates/forge-kernel/src/operations/boolean/postprocess/mod.rs
crates/forge-kernel/src/operations/boolean/postprocess/polygon_extract.rs
crates/forge-kernel/src/operations/boolean/postprocess/vertex.rs
crates/forge-kernel/src/operations/boolean/schema.rs
crates/forge-kernel/src/operations/boolean/split/cut.rs
crates/forge-kernel/src/operations/boolean/split/eval.rs
crates/forge-kernel/src/operations/boolean/split/gate.rs
crates/forge-kernel/src/operations/boolean/split/mod.rs
crates/forge-kernel/src/operations/boolean/split/reconcile.rs
crates/forge-kernel/src/operations/boolean/split/schema.rs
crates/forge-kernel/src/operations/boolean/split/signs.rs
crates/forge-kernel/src/operations/boolean/test_helpers.rs
crates/forge-kernel/src/operations/boolean/tests/debug_tests.rs
crates/forge-kernel/src/operations/boolean/tests/diag_tests.rs
crates/forge-kernel/src/operations/boolean/tests/edge_case_tests.rs
crates/forge-kernel/src/operations/boolean/tests/introspection_tests.rs
crates/forge-kernel/src/operations/boolean/tests/mod.rs
crates/forge-kernel/src/operations/boolean/tests/sector_classification_tests.rs
crates/forge-kernel/src/operations/boolean/tests/tests.rs
crates/forge-kernel/src/operations/boolean/traits.rs
crates/forge-kernel/src/operations/ember_boolean/checkpoint.rs
crates/forge-kernel/src/operations/ember_boolean/classify.rs
crates/forge-kernel/src/operations/ember_boolean/eval.rs
crates/forge-kernel/src/operations/ember_boolean/mesh.rs
crates/forge-kernel/src/operations/ember_boolean/mod.rs
crates/forge-kernel/src/operations/ember_boolean/quantize.rs
crates/forge-kernel/src/operations/ember_boolean/schema.rs
crates/forge-kernel/src/operations/ember_boolean/tests.rs
crates/forge-kernel/src/operations/mod.rs
crates/forge-kernel/src/prelude.rs

## crates/worth-math
crates/worth-math/src/arithmetic/double.rs
crates/worth-math/src/arithmetic/expansion.rs
crates/worth-math/src/arithmetic/interval.rs
crates/worth-math/src/arithmetic/mod.rs
crates/worth-math/src/arithmetic/precision.rs
crates/worth-math/src/arithmetic/rational.rs
crates/worth-math/src/coincidence/mod.rs
crates/worth-math/src/coincidence/sos.rs
crates/worth-math/src/data_access/mod.rs
crates/worth-math/src/data_access/schema.rs
crates/worth-math/src/env.rs
crates/worth-math/src/error.rs
crates/worth-math/src/lib.rs
crates/worth-math/src/linalg/mod.rs
crates/worth-math/src/numeric/deterministic_rng.rs
crates/worth-math/src/numeric/mod.rs
crates/worth-math/src/numeric/sign.rs
crates/worth-math/src/predicates/grid_predicates.rs
crates/worth-math/src/predicates/in_sphere.rs
crates/worth-math/src/predicates/incircle.rs
crates/worth-math/src/predicates/mod.rs
crates/worth-math/src/predicates/orient2d.rs
crates/worth-math/src/predicates/orient3d.rs
crates/worth-math/src/predicates/vendored.rs
crates/worth-math/src/prelude.rs
crates/worth-math/src/traits.rs

## crates/forge-repr
crates/forge-repr/src/lib.rs
crates/forge-repr/src/schema.rs
crates/forge-repr/src/tests.rs
crates/forge-repr/src/traits.rs

## crates/forge-schema
crates/forge-schema/src/lib.rs

## crates/forge-signal
crates/forge-signal/src/evaluation/context.rs
crates/forge-signal/src/evaluation/mod.rs
crates/forge-signal/src/evaluation/pull.rs
crates/forge-signal/src/evaluation/push.rs
crates/forge-signal/src/graph.rs
crates/forge-signal/src/handles.rs
crates/forge-signal/src/lib.rs
crates/forge-signal/src/prelude.rs
crates/forge-signal/src/schema.rs
crates/forge-signal/src/tests.rs

## crates/forge-test
crates/forge-test/src/feature_tests.rs
crates/forge-test/src/fixtures.rs
crates/forge-test/src/generators/mod.rs
crates/forge-test/src/generators/planar.rs
crates/forge-test/src/harness/boolean.rs
crates/forge-test/src/harness/mod.rs
crates/forge-test/src/lib.rs
crates/forge-test/src/logging.rs

## crates/forge-topo
crates/forge-topo/src/arena/eval.rs
crates/forge-topo/src/arena/mod.rs
crates/forge-topo/src/arena/schema.rs
crates/forge-topo/src/arena/tests.rs
crates/forge-topo/src/lib.rs
crates/forge-topo/src/prelude.rs
crates/forge-topo/src/testing.rs
crates/forge-topo/src/topology/attributes.rs
crates/forge-topo/src/topology/bitset.rs
crates/forge-topo/src/topology/handles.rs
crates/forge-topo/src/topology/history/lineage.rs
crates/forge-topo/src/topology/history/lineage_store.rs
crates/forge-topo/src/topology/history/mod.rs
crates/forge-topo/src/topology/history/replay.rs
crates/forge-topo/src/topology/integrity/diff.rs
crates/forge-topo/src/topology/integrity/geometric.rs
crates/forge-topo/src/topology/integrity/hashing.rs
crates/forge-topo/src/topology/integrity/healing.rs
crates/forge-topo/src/topology/integrity/mod.rs
crates/forge-topo/src/topology/integrity/shell.rs
crates/forge-topo/src/topology/integrity/structural.rs
crates/forge-topo/src/topology/integrity/validate.rs
crates/forge-topo/src/topology/mod.rs
crates/forge-topo/src/topology/naming/eval.rs
crates/forge-topo/src/topology/naming/mod.rs
crates/forge-topo/src/topology/naming/schema.rs
crates/forge-topo/src/topology/naming/tests.rs
crates/forge-topo/src/topology/operations/algorithms/bfs.rs
crates/forge-topo/src/topology/operations/algorithms/bridge_edge.rs
crates/forge-topo/src/topology/operations/algorithms/components.rs
crates/forge-topo/src/topology/operations/algorithms/extract_shell.rs
crates/forge-topo/src/topology/operations/algorithms/flip_edge.rs
crates/forge-topo/src/topology/operations/algorithms/mod.rs
crates/forge-topo/src/topology/operations/algorithms/region_extraction.rs
crates/forge-topo/src/topology/operations/algorithms/simplify/cleanup.rs
crates/forge-topo/src/topology/operations/algorithms/simplify/consolidate_collinear_vertices.rs
crates/forge-topo/src/topology/operations/algorithms/simplify/mod.rs
crates/forge-topo/src/topology/operations/algorithms/triangulate.rs
crates/forge-topo/src/topology/operations/euler/join_faces.rs
crates/forge-topo/src/topology/operations/euler/join_faces_nmt.rs
crates/forge-topo/src/topology/operations/euler/kill_edge_make_loop.rs
crates/forge-topo/src/topology/operations/euler/kill_edge_vertex.rs
crates/forge-topo/src/topology/operations/euler/kill_shell_face.rs
crates/forge-topo/src/topology/operations/euler/kill_vertex_face.rs
crates/forge-topo/src/topology/operations/euler/make_edge_face.rs
crates/forge-topo/src/topology/operations/euler/make_edge_kill_loop.rs
crates/forge-topo/src/topology/operations/euler/make_edge_vertex.rs
crates/forge-topo/src/topology/operations/euler/make_face_from_vertices.rs
crates/forge-topo/src/topology/operations/euler/make_face_in_shell_from_vertices.rs
crates/forge-topo/src/topology/operations/euler/make_loop_in_face_from_vertices.rs
crates/forge-topo/src/topology/operations/euler/make_shell_face.rs
crates/forge-topo/src/topology/operations/euler/make_vertex_face.rs
crates/forge-topo/src/topology/operations/euler/mod.rs
crates/forge-topo/src/topology/operations/euler/sew_edge.rs
crates/forge-topo/src/topology/operations/euler/split_edge.rs
crates/forge-topo/src/topology/operations/euler/tests/brutality_tests.rs
crates/forge-topo/src/topology/operations/euler/tests/helpers.rs
crates/forge-topo/src/topology/operations/euler/tests/integration_tests.rs
crates/forge-topo/src/topology/operations/euler/tests/invariant_checker.rs
crates/forge-topo/src/topology/operations/euler/tests/join_faces_nmt_tests.rs
crates/forge-topo/src/topology/operations/euler/tests/join_faces_tests.rs
crates/forge-topo/src/topology/operations/euler/tests/kill_edge_vertex_tests.rs
crates/forge-topo/src/topology/operations/euler/tests/lineage_tests.rs
crates/forge-topo/src/topology/operations/euler/tests/mef_tests.rs
crates/forge-topo/src/topology/operations/euler/tests/mekl_keml_tests.rs
crates/forge-topo/src/topology/operations/euler/tests/mev_tests.rs
crates/forge-topo/src/topology/operations/euler/tests/mod.rs
crates/forge-topo/src/topology/operations/euler/tests/mvf_tests.rs
crates/forge-topo/src/topology/operations/euler/tests/sew_edge_tests.rs
crates/forge-topo/src/topology/operations/euler/tests/shell_edge_tests.rs
crates/forge-topo/src/topology/operations/euler/tests/split_edge_tests.rs
crates/forge-topo/src/topology/operations/euler/unsew_edge.rs
crates/forge-topo/src/topology/operations/mod.rs
crates/forge-topo/src/topology/operations/operator.rs
crates/forge-topo/src/topology/queries/bounds.rs
crates/forge-topo/src/topology/queries/classification.rs
crates/forge-topo/src/topology/queries/classify.rs
crates/forge-topo/src/topology/queries/continuity.rs
crates/forge-topo/src/topology/queries/hierarchy.rs
crates/forge-topo/src/topology/queries/mod.rs
crates/forge-topo/src/topology/queries/ordering.rs
crates/forge-topo/src/topology/queries/polygon.rs
crates/forge-topo/src/topology/queries/radial.rs
crates/forge-topo/src/topology/queries/traverse.rs
crates/forge-topo/src/topology/state.rs
crates/forge-topo/src/topology/tests/brutality.rs
crates/forge-topo/src/topology/tests/diagnostic.rs
crates/forge-topo/src/topology/tests/mod.rs
crates/forge-topo/src/topology/tests/topology_stress.rs

## crates/forge-view
crates/forge-view/src/cli_main.rs
crates/forge-view/src/lib.rs
crates/forge-view/src/main.rs
crates/forge-view/src/trace/mod.rs
crates/forge-view/src/trace/server.rs
crates/forge-view/src/trace/store.rs
crates/forge-view/src/trace/viewer.rs
crates/forge-view/src/viewer_main.rs

```
