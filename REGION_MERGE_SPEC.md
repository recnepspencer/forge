Topo Region Merge Upgrade — Engineering Specification
Version: 1.1
Status: Design only (no implementation in this milestone)
Scope: Planar merge correctness, NMT-intermediate cleanup support, curved same-support extensibility
Parent: BooleanUpgradeSpec.MD

1. Problem Statement
   1.1 Current Failure Modes
   Coplanar boolean cleanup can produce boundaries that are topologically connected but geometrically invalid for merge (self-touches, overlaps, or crossings), and intermediate boolean states can contain non-manifold radial valence.

Current concrete issues:

# Defect Code Location

1 JoinFaces requires radial valence == 2; intermediate boolean states can produce valence > 2 join_faces.rs
2 merge_face_group_by_join_faces merges by internal-edge iteration without geometric boundary certification region_extraction.rs
3 Coplanar postprocess merge paths use local heuristics (single-edge / iterative candidate selection) and no weakly-simple boundary certifier coplanar.rs, polygon_extract.rs
4 Killed faces after merge leave stale GeometryStore face bindings (benign for planes today, unsafe for future surface/coedge-backed merges) schema.rs consumers in postprocess merge paths
1.2 Architectural Drivers
Curved booleans need “same-support surface merge”, not planar-only plane equality.
D8 already allows NMT-capable storage (radial_next rings of arbitrary length), but validation and Euler semantics are manifold-default.
Boundary certification must be reusable for:
planar projected boundaries
future UV trim loop boundaries on curved faces
1.3 Existing Assets We Keep
Asset Location Decision
Halfedge + radial_next ring storage forge-topo::arena::schema Keep
JoinFaces manifold-only semantics forge-topo::euler::join_faces Keep unchanged
ValidationLevel forge-topo::integrity::validate Extend (not replace)
SurfaceRelation + classify_surface_pair forge-geom::surface::{schema,eval} Evolve
GeometryStore generational refs (SurfaceRef, CurveRef, CoedgeRef) forge-kernel::geometry_store::schema Reuse/extend
EntityBitset forge-topo::topology::bitset Reuse
merge_face_group_by_join_faces forge-topo::algorithms::region_extraction Keep pure-topology; call from certified kernel wrapper
Dominant-axis projection utilities / segment crossing helpers forge-geom::algorithms::polygon_overlap Reuse/extend 2. Scope Framing
2.1 Architectural Requirements (Future-Proofing, In Scope Now)
Design must be extensible to persistent/user-visible non-manifold modeling later without changing the core topology representation.
NMT semantics must be explicit in architecture:
radial-use selection
validation modes
manifold vs NMT-safe operator families
Boundary certification must be backend-agnostic (planar projection now, UV trim later).
Geometry merge qualification must generalize from “coplanar” to “same support surface”.
New diagnostics/certificates/error taxonomy must remain valid when persistent NMT is added later.
2.2 Implementation Milestone Scope (What We Build Now)
Planar boundary certification and weakly-simple recognition for coplanar merges.
NMT-intermediate validation + selective merge support for boolean cleanup.
Final outputs remain manifold by default.
Curved same-support merge: interfaces/contracts only (no full implementation this milestone).
2.3 Explicit Deferrals (Implementation, Not Architecture)
User-facing persistent NMT workflows
Persistent NMT import/export semantics
UI/repair tooling for NMT bodies
Full NURBS support equivalence and UV trim certification implementation 3. Architecture Alignment and Crate Ownership
3.1 Layer Ownership (Corrected)
forge-geom: stateless geometry certifiers and classifiers
forge-topo: connectivity, radial queries, mode-aware topology validation
forge-kernel: orchestration, policy, OperationResult<T>, decision logging, merge execution flow
3.2 New Type Placement
forge-geom

ProjectionFrame2D
ProjectedBoundary2D
BoundaryEvent
BoundaryArrangement
WeakSimpleCertificate
weakly-simple certifier functions
evolved SurfaceRelation semantics/classifiers
forge-topo

TopologyMode
RadialUseIndex (ephemeral snapshot-scoped local radial position)
radial query helpers (radial_uses, radial_uses_by_face)
local sheet-likeness validation helpers (topological)
forge-kernel

BoundaryCycleCandidate (topology + provenance adapter)
MergeRegionSelection (policy-level selection built on EntityBitset + radial plan)
MergeSheetRegion (compound algorithm)
merge eligibility orchestration
OperationResult wrappers, TracedDecision, policy fallback/escalation
3.3 Communication Rules
forge-geom must not accept TopologyState, TopologyArena, ModelingContext, or ToleranceConfig.
forge-kernel converts topo/geometry data into raw geometry inputs for forge-geom.
forge-topo remains geometry-agnostic.
3.4 Error Flow
Introduce MergeError and wrap in KernelError.

pub enum MergeError {
AmbiguousRadialSelection { edge_index: u32, valence: u32 },
SelectedUsesNotSheetLike { edge_index: u32 },
ProtectedUseConflict { edge_index: u32 },
WouldDisconnectSheet { face_index: u32 },
BoundaryCertificationFailed {
reason: String,
witness: Option<[f64; 2]>,
},
/// step_index: None = rejected during plan building (before execution).
/// step_index: Some(n) = rejected during execution of step n.
PartialMergePlanRejected { step_index: Option<u32>, reason: String },
UnsupportedPersistentNmtOutput,
}
KernelError wraps this as a structured variant (or equivalent sub-enum wrapper).

3.5 Tracing and Result Envelopes
forge-geom returns plain certifiers/results.
forge-kernel wraps merge/certification orchestration in OperationResult<T>.
Tests for new merge/certification flows must support FORGE_TRACE_DIR. 4. Epic A — Boundary Certification (Planar Now, UV-Reusable)
4.1 Goal
Before merging a face group, certify the group boundary is geometrically valid for merge:

Simple or WeaklySimple => eligible
Rejected => do not merge
4.2 Boundary Certification Architecture
Boundary certification is split into:

forge-kernel adapter: extract boundary candidate from topo + geometry provenance
forge-geom certifier: project/build arrangement/classify weak simplicity
4.3 Kernel Adapter Type (forge-kernel)
pub struct BoundaryCycleCandidate {
/// Boundary segments in 3D with stable provenance (not halfedge-ID semantics).
segments_3d: Vec<BoundarySegment3D>,
/// Optional source metadata for diagnostics/tracing.
provenance: Vec<BoundaryProvenance>,
}
This is topology-derived and therefore kernel-owned.

4.4 Geometry Types (forge-geom::algorithms::boundary_cert)
pub struct ProjectionFrame2D {
drop_axis: usize,
u_axis: usize,
v_axis: usize,
orientation_sign: f64,
}

pub struct ProjectedBoundary2D {
segments: Vec<Segment2D>,
frame: ProjectionFrame2D,
}

pub struct Segment2D {
start: [f64; 2],
end: [f64; 2],
provenance: u64,
}

pub enum BoundaryEventKind {
ProperCrossing,
EndpointTouch,
OverlapStart,
OverlapEnd,
DegenerateSegment,
}

pub struct BoundaryEvent {
kind: BoundaryEventKind,
location: [f64; 2],
segments: [usize; 2],
}

pub struct BoundaryArrangement {
atomic_segments: Vec<Segment2D>,
events: Vec<BoundaryEvent>,
}

pub enum BoundaryRejectReason {
SelfCrossing,
OverlappingSegments, // if policy disallows overlap in current phase
DegenerateBoundary,
}

pub enum WeakSimpleCertificate {
Simple,
WeaklySimple { touch_count: usize },
Rejected {
reason: BoundaryRejectReason,
witness: [f64; 2],
},
}
4.5 Numeric Robustness Requirement
ProjectedBoundary2D may store f64 for transport/diagnostics, but certification must use robust predicates:

exact/certified predicate evaluation in projected space for planar mode, or
exact re-evaluation callbacks from source geometry data
The spec requirement is on predicate correctness, not storage type alone.

4.6 Deterministic Projection
Reuse/extend existing dominant-axis projection helper in forge-geom.
Require fixed tie-break (X > Y > Z) when magnitudes tie.
Projection frame metadata must be test-stable and explicit.
4.7 Algorithm (Two-Phase: Fast Path + Fallback)
Fast Path (default)

Extract boundary candidate in kernel.
Project to 2D.
Run simple-crossing check using existing crossing helpers (extended as needed).
If no evidence of degeneracy, return Simple.
Fallback Trigger Conditions (required)
Fast path must escalate to arrangement/certifier when any of the following are detected:

repeated projected vertices (beyond adjacency expectations)
endpoint touch candidate (endpoint lies on non-incident segment)
collinearity / overlap candidate
duplicate segment image (same geometric segment, opposite or same orientation)
degenerate segment candidate
ambiguous fast-path classification
Fallback (degenerate path)

Build BoundaryArrangement (deterministic event ordering).
Classify all interactions: crossing / touch / overlap / degeneracy.
Run weakly-simple recognizer (Akitaya-inspired certifying approach).
Return WeakSimpleCertificate with witness on rejection.
4.8 Integration Points (Corrected for Crate Boundaries)
forge-kernel adds a wrapper that certifies boundary first, then calls topology merge.
forge-topo::merge_face_group_by_join_faces remains pure-topology and is not geometry-aware.
Kernel-side changes

Gate polygon_extract.rs
Gate coplanar.rs
Topo-side changes

None for geometry certification call flow.
Optional topo helpers for boundary extraction remain pure-topology.
4.9 Module Layout
forge-geom/src/algorithms/boundary_cert/
mod.rs
schema.rs
eval.rs
tests.rs

forge-kernel/src/operations/boolean/postprocess/merge_eligibility/
mod.rs
boundary_adapter.rs
eval.rs
tests.rs 5. Epic B — NMT-Compatible Region Merge (Intermediate States)
5.1 Goal
Permit controlled merge reasoning and cleanup when local radial valence exceeds 2, while preserving manifold-default final output semantics.

5.2 Validation Model Extension (forge-topo)
Extend validation with a topology mode that is separate from ValidationLevel.

ValidationLevel = how much to check (breadth / depth of checks).
TopologyMode = what semantic world is allowed (manifold vs NMT).

Conflating them is an implementation shortcut that must be rejected.

pub enum TopologyMode {
ManifoldStrict, // Default. Rejects valence > 2 at commit.
NmtIntermediate, // Allows valence > 2 for internal pipeline checkpoints.
// NmtPersistent reserved, not implemented.
}

Add validate_topology_with_mode(arena, level, mode). Existing validate_topology keeps its
signature (callers receive ManifoldStrict by default) to avoid breaking Epic A callers.

5.3 Validation Semantics
ValidationLevel controls breadth/depth of checks.
TopologyMode controls manifoldness policy.
In NmtIntermediate:

Named skip-list (exhaustive — any extension requires spec amendment + dedicated tests):
SKIP: validate_manifold_edges (valence > 2 permitted)

Still executed in NmtIntermediate:
RUNS: validate_radial_rings — ring closure still enforced
RUNS: validate_prev_consistency — prev/next pointer consistency
RUNS: validate_vertex_continuity — endpoint consistency
RUNS: validate_vertex_outgoing — vertex outgoing pointers
RUNS: validate_loops — loop integrity (including same-face slit traversal)
RUNS: validate_hierarchy — Lump/Region/Shell chain integrity
RUNS: validate_orientation_consistency — twin orientation

IMPORTANT: validate_orientation_consistency and validate_loops both walk same-face twin
pairs. Slit/same-face exemption behavior for JoinFacesNmt output must be verified in
structural.rs before treating NmtIntermediate as safe.

NmtIntermediate must NEVER become a generic bypass mode. It is not
"skip whatever fails" — it is "skip exactly the named checks above."

5.4 Mode Propagation (Must Be Explicit — No Hidden Globals)
MutableDraft::commit_with_mode(level: ValidationLevel, mode: TopologyMode)
The default commit() always uses ManifoldStrict — this must not be silently changed.
NMT mode must be passed explicitly. Callers cannot rely on ambient state.

5.5 Commit Semantics vs D8 (Clarification)
ManifoldStrict remains the default for normal commits and final outputs.
NmtIntermediate is opt-in and intended for internal pipeline checkpoints / specialized ops.
Persistent NMT outputs remain disabled in this milestone even if NmtIntermediate commits are allowed internally.
5.6 Radial Query Helpers (forge-topo)
pub struct RadialUseIndex {
edge_index: u32,
position: u32,
// Snapshot-scoped only: valid only for the arena snapshot used to compute it.
}

pub fn radial_uses(
arena: &TopologyArena,
he: HalfEdgeId,
) -> Result<Vec<HalfEdgeId>, KernelError>;

pub fn radial_uses_by_face(
arena: &TopologyArena,
he: HalfEdgeId,
) -> Result<BTreeMap<FaceId, Vec<HalfEdgeId>>, KernelError>;
RadialUseIndex is explicitly ephemeral and invalid after topology mutation.

5.7 Merge Selection (forge-kernel)
Face sets alone are insufficient in ambiguous high-valence neighborhoods.

pub struct MergeRegionSelection {
selected_faces: EntityBitset,
protected_faces: EntityBitset,
surviving_face: FaceId,

    /// Optional explicit radial-use plan for disambiguation.
    /// Required when face-only selection is ambiguous.
    selected_radial_uses: Vec<RadialUseSelector>,

}
RadialUseSelector is kernel-facing but references topo-defined radial concepts.

5.8 MergeSheetRegion (Compound Algorithm, Not Euler Primitive)
JoinFaces remains manifold-only (radial valence == 2 required).

JoinFacesNmt is a NEW Euler operator for NMT merge contexts:

- Takes two faces sharing an edge with global radial valence > 2.
- Merges the two faces, converting their shared boundary halfedges into a
  "slit" (a same-face twin pair) in the surviving face.
- Preserves the EdgeId and the radial ring for all non-selected faces.
- Invariants — 5 concrete postconditions that MUST all hold after every JoinFacesNmt call:
  (1) Loop traversal: surviving face outer loop is fully reachable via next() walking;
  no dangling or unreachable halfedges.
  (2) Slit pair: the two former boundary halfedges are now a same-face twin pair:
  he.face() == twin.face() == surviving_face, he.radial_next() == twin.
  (3) Protected ring order: all non-selected (protected) radial uses of the edge form
  a valid closed radial ring. Cyclic adjacency preserved (up to rotation).
  No protected halfedge's radial_next() links into the slit pair.
  (4) No dangling references: killed face's FaceId, LoopId, and halfedges all removed.
  No live halfedge points to killed FaceId.
  (5) Validator exemption: validate_orientation_consistency must not false-positive on
  the slit pair. Must be verified in structural.rs explicitly.

MergeSheetRegion is a compound algorithm (not a single Euler primitive) that orchestrates
both JoinFaces and JoinFacesNmt.

Key rules:
No implicit partial merges.

Ownership model: execute_sheet_region_merge accepts KernelState by value and creates its
own KernelDraft internally. On success: calls draft.commit() and returns Ok(KernelState).
On failure: drops draft, which atomically discards all topology + geometry mutations.
The caller never holds a reference to the draft. This eliminates rollback/ownership
tension. This mirrors the pattern used in coplanar.rs.

MergePlan is emitted as a structured artifact in OperationResult so it appears in
trace output. Per-step TracedDecisions include step index and EdgeId.

Intent (MergeRegionSelection) is separated from execution (MergePlan):
MergeRegionSelection = intent-level, stable, serializable, agent-facing
MergePlan = kernel-derived blueprint, deterministic but snapshot-scoped
RadialUseIndex/RadialUseSelector = ephemeral only, snapshot-scoped

Execution outcomes must be explicit:
full success
fail-fast with MergeError (no silent partial merges)

5.9 NMT Merge Strategy (Decomposition)
MergeSheetRegion validates the selection, builds a deterministic merge plan, then applies
reductions using JoinFaces (manifold) or JoinFacesNmt (NMT) per edge.

Algorithm:

1. GATE: call certify_merge_boundary (Epic A certifier) before touching topology.
   If Rejected, return early. This gate is mandatory — not optional for NMT paths.

2. Validate connectivity of selected faces (BFS/DFS on draft.arena()).

3. Derive MergePlan from MergeRegionSelection:
   - Plan stores an ordered sequence of MergeStepPlans (each with edge_index: u32 and
     selected_face_indices: [u32; 2]) derived from the planning snapshot.
   - EdgeIds provide stable identifiers for lookup, NOT a guarantee of persistence.
     Each step re-derives and validates that the edge still exists and is mergeable.
     A missing or changed edge fails fast with PartialMergePlanRejected.
   - Steps sorted by edge_index for determinism. plan_hash computed over step sequence.
   - MergeStepPlan uses raw indices (not opaque handles) for serializability and replay.

4. Execution loop (strictly one step at a time):
   a. Re-derive halfedge for the current MergeStepPlan.edge_index from draft.arena().
   Validate edge still exists and selected_face_indices match current arena state.
   If missing or changed: fail fast with PartialMergePlanRejected { step_index: Some(n), reason }.
   c. Apply JoinFaces or JoinFacesNmt based on current global radial valence.
   d. Call draft.geometry_mut().remove_face_plane(killed_face) to clean the stale binding.
   The surviving face's plane binding is inherited from the base GeometryState and
   must not be touched by merge steps.
   e. Log a TracedDecision to ctx.get_decision_log_mut() with step index and EdgeId.
   f. On any failure: drop draft (implicit atomic rollback of topo + geom).
   Return KernelError::MergeFailure(MergeError::PartialMergePlanRejected { step_index, reason }).
5. Call draft.commit() and return Ok(new_state).
   5.10 Failure Taxonomy
   All failures flow through KernelError wrapping MergeError, including:

AmbiguousRadialSelection
SelectedUsesNotSheetLike
ProtectedUseConflict
WouldDisconnectSheet
PartialMergePlanRejected
UnsupportedPersistentNmtOutput
5.11 Module Layout
forge-topo/src/topology/queries/radial.rs
forge-kernel/src/operations/boolean/postprocess/merge_eligibility/
schema.rs // MergeRegionSelection and plan/result types
nmt_eval.rs // selection validation + plan builder + execution
tests.rs 6. Epic C — Curved Same-Support Surface Merge (Design Contracts Only)
6.1 Goal
Generalize planar coplanar merge eligibility to curved faces that share the same geometric support surface and have valid merged trim boundaries in UV.

6.2 SurfaceRelation Evolution (forge-geom)
Do not create a parallel kernel enum. Evolve existing SurfaceRelation.

Current semantics:

Coincident
Disjoint
General
Proposed extension:

add Undetermined (for bounded/certified inability to decide)
optionally refine General in future (Intersecting vs unknown), but not required this milestone
For merge eligibility:

SurfaceRelation::Coincident is the “same support” signal for analytic surfaces.
6.3 Extend classify_surface_pair
Design target:

add cone/cone and torus/torus analytic support classification
preserve existing plane/sphere/cylinder behavior
return Undetermined when classification cannot be safely decided under future bounded precision policies
6.4 Surface Evaluation API Contracts (forge-geom)
Acknowledge existing SurfaceData::{point_at, normal_at} and define traits as abstractions/extensions, not replacements.

pub trait EvaluateSurface {
fn point_at_uv(&self, u: f64, v: f64) -> [f64; 3];
fn normal_at_uv(&self, u: f64, v: f64) -> [f64; 3];
fn tangent_u_at_uv(&self, u: f64, v: f64) -> [f64; 3];
fn tangent_v_at_uv(&self, u: f64, v: f64) -> [f64; 3];
fn domain(&self) -> &ParameterDomain;
}
6.5 Trim Curve API Contracts (forge-geom)
pub trait TrimCurveOps {
fn uv_endpoints(&self) -> ([f64; 2], [f64; 2]);
fn uv_direction(&self) -> [f64; 2];
fn uv_overlap(&self, other: &Self, tol: f64) -> TrimOverlapResult;
}
6.6 Curved Merge Eligibility Contract
A curved merge candidate is eligible only if:

Support relation is SurfaceRelation::Coincident
Shared trims match/resolve in UV
Resulting UV loops certify Simple or WeaklySimple (same certifier architecture as Epic A, UV backend)
Normal/tangent continuity checks pass per policy
6.7 GeometryStore Post-Merge Updates (Cross-Cutting Requirement)
This section applies to planar now and curved later.

Planar (Milestone 1)

remove stale killed-face plane entries from GeometryStore
preserve surviving face plane binding
clean any stale per-entity bindings created during merge (if applicable)
Curved (Later)

preserve/attach surviving SurfaceRef
rebuild boundary CoedgeRef set
remove stale face/coedge/curve bindings
validate geometry binding integrity after merge 7. Boundary Certifier Backend Abstraction (For Planar + UV Reuse)
7.1 Rationale
Boundary certification must support both:

planar projected segments
UV trim boundaries
without duplicating weakly-simple logic.

7.2 Contract Shape (Design Requirement)
forge-kernel builds a backend-neutral boundary candidate from topology + geometry references.
forge-geom provides backend-specific input builders and a common certifier API.
Example direction (illustrative, not final API):

pub trait BoundaryCertifierInputBuilder<TInput> {
fn build(&self) -> Result<TInput, GeomError>;
}

pub fn certify_weakly_simple(input: &impl BoundaryInput) -> WeakSimpleCertificate;
This keeps backend pluggability without violating crate dependencies.

8. Implementation Sequence
   Milestone 1 — Planar Boundary Certification (Build Now)
   forge-geom::algorithms::boundary_cert
   ProjectionFrame2D (reuse dominant-axis helper, add deterministic tie-break)
   ProjectedBoundary2D
   fast-path simplicity check
   fallback triggers (touch/overlap/duplicate/degenerate detection)
   BoundaryArrangement
   weakly-simple certifier
   forge-kernel merge-eligibility wrapper
   boundary adapter from face group to certifier input
   gate coplanar merge paths on cert
   GeometryStore cleanup for killed faces after planar merge
   trace-emitting tests (FORGE_TRACE_DIR)
   Milestone 2 — NMT-Intermediate Merge (Build Now)
   TopologyMode in forge-topo
   mode-aware manifold validation
   explicit mode propagation into validation/commit
   radial query helpers
   MergeRegionSelection + deterministic merge plan builder
   MergeSheetRegion fail-fast execution (no implicit partial merges)
   MergeError integration into KernelError
   trace-emitting tests (FORGE_TRACE_DIR)
   Milestone 3 — Curved Merge Scaffolding (Design/Scaffold Now, Implement Later)
   extend SurfaceRelation (Undetermined)
   extend classify_surface_pair (cone/torus)
   define EvaluateSurface / TrimCurveOps contracts
   document curved merge eligibility and trim rebuild postconditions
9. Testing Strategy
   9.1 General Requirements
   All new merge/certification regression suites support trace output via FORGE_TRACE_DIR.
   Determinism tests compare stable outputs (certificate kind, witness, plan structure, trace summary/hash), not timestamps.
   9.2 Planar Weakly-Simple (Milestone 1)
   Test Case Expected
   Clean coplanar merge (adjacent quads) Simple
   Endpoint self-touch from sliver cleanup WeaklySimple { ... }
   Overlapping collinear chains Rejected { OverlappingSegments } (or accepted later if policy changes)
   Proper crossing Rejected { SelfCrossing }
   Repeated vertex / duplicate segment image fallback path invoked; deterministic result
   Determinism (same input repeated) identical certificate + trace summary/hash
   9.3 NMT Intermediate (Milestone 2)
   Test Case Expected
   Valence-3 local merge with explicit selection deterministic merge success
   Valence-4 ambiguous radial neighborhood AmbiguousRadialSelection
   Protected-use preservation protected radial uses unchanged
   ManifoldStrict mode commit on valence>2 commit_with_mode(Full, ManifoldStrict) → manifold error
   NmtIntermediate mode validation/commit commit_with_mode(Full, NmtIntermediate) → passes (if other invariants valid)
   Non-executable plan fail-fast MergeError (no silent partial merge)
   Determinism of merge plans same input twice → identical MergePlan structure hash and trace decision IDs
   Fail-fast leaves no bleed fail midway → draft dropped → topology AND geometry bindings fully restored
   Trace artifact present MergePlan and per-step TracedDecisions visible via forge-trace-cli decisions <ID>
   9.4 Curved Readiness (Milestone 3)
   Test Case Expected
   Same-cylinder split patches SurfaceRelation::Coincident
   Same-cone near apex correct classification or Undetermined
   Same-sphere with parameter shift Coincident
   Cone vs torus General (or refined non-coincident result)
10. Acceptance Criteria (This Spec’s Buildable Outcomes)
    Milestone 1
    Coplanar merge paths in kernel are gated by geometric boundary certification.
    forge-topo merge helpers remain geometry-agnostic.
    Weakly-simple fallback exists and is deterministic.
    Planar merge path cleans stale face geometry bindings.
    Milestone 2
    Validation supports explicit TopologyMode.
    NMT-intermediate states can be validated/processed without weakening default manifold final-output policy.
    NMT merge path is explicit-selection-based and fail-fast on ambiguity/non-executable plans.
    Milestone 3 (Scaffold)
    Curved same-support merge contracts are documented in code-facing terms.
    Existing SurfaceRelation is extended (not duplicated).
    Trait/API contracts are aligned with existing SurfaceData and GeometryStore design.
11. References
    Akitaya et al. — weakly simple polygon recognition (SoCG 2016)
    Shewchuk — robust adaptive predicates
    Weiler — radial-edge data structure / non-manifold modeling lineage
    Mäntylä — Euler operator formalism and solid modeling foundations

12. Explicit Assumptions (Made Explicit — Not Left Implicit)
    The following assumptions were identified during review and must not be left unstated:

Assumption | Status | Rule
---------------------------------------------------+----------+----------------------------------------------
execute_sheet_region_merge takes raw arena+geom | FALSE | Must accept &mut KernelDraft (D6)
Rollback is topology-only | FALSE | KernelDraft::rollback() drops topo + geom patch
Killed face planes clean themselves up | FALSE | remove_face_plane(killed_face) must be explicit
Surviving face plane binding is always valid | TRUE | Inherited from base GeometryState; do not touch
RadialUseSelector handles survive across steps | FALSE | Re-derive handles from arena after each step
JoinFacesNmt slit won't trigger validators | UNTESTED | Verify slit exemption in structural.rs checks
MergePlan carries locked arena handles | FALSE | Plan carries edge order only, handles re-derived
Default commit() may silently use NmtIntermediate | FALSE | commit() must always call ManifoldStrict
Epic A certifier is optional for NMT paths | FALSE | certify_merge_boundary gate is mandatory

13. Notes for Repo Integration
    Use workspace-relative links in the actual REGION_MERGE_SPEC.md instead of file:///... URIs.
    Keep forge-topo geometry-free; all boundary certification calls should originate in forge-kernel.
    Prefer explicit "design target" wording where APIs are placeholders to avoid accidental implementation commitments.
