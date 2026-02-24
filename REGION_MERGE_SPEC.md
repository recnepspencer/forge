Topo Region Merge Upgrade — Engineering Specification
Version: 1.1
Status: Design only (no implementation in this milestone)
Scope: Planar merge correctness, NMT-intermediate cleanup support, curved same-support extensibility
Parent: BooleanUpgradeSpec.MD

1. Problem Statement
1.1 Current Failure Modes
Coplanar boolean cleanup can produce boundaries that are topologically connected but geometrically invalid for merge (self-touches, overlaps, or crossings), and intermediate boolean states can contain non-manifold radial valence.

Current concrete issues:

#	Defect	Code Location
1	JoinFaces requires radial valence == 2; intermediate boolean states can produce valence > 2	join_faces.rs
2	merge_face_group_by_join_faces merges by internal-edge iteration without geometric boundary certification	region_extraction.rs
3	Coplanar postprocess merge paths use local heuristics (single-edge / iterative candidate selection) and no weakly-simple boundary certifier	coplanar.rs, polygon_extract.rs
4	Killed faces after merge leave stale GeometryStore face bindings (benign for planes today, unsafe for future surface/coedge-backed merges)	schema.rs consumers in postprocess merge paths
1.2 Architectural Drivers
Curved booleans need “same-support surface merge”, not planar-only plane equality.
D8 already allows NMT-capable storage (radial_next rings of arbitrary length), but validation and Euler semantics are manifold-default.
Boundary certification must be reusable for:
planar projected boundaries
future UV trim loop boundaries on curved faces
1.3 Existing Assets We Keep
Asset	Location	Decision
Halfedge + radial_next ring storage	forge-topo::arena::schema	Keep
JoinFaces manifold-only semantics	forge-topo::euler::join_faces	Keep unchanged
ValidationLevel	forge-topo::integrity::validate	Extend (not replace)
SurfaceRelation + classify_surface_pair	forge-geom::surface::{schema,eval}	Evolve
GeometryStore generational refs (SurfaceRef, CurveRef, CoedgeRef)	forge-kernel::geometry_store::schema	Reuse/extend
EntityBitset	forge-topo::topology::bitset	Reuse
merge_face_group_by_join_faces	forge-topo::algorithms::region_extraction	Keep pure-topology; call from certified kernel wrapper
Dominant-axis projection utilities / segment crossing helpers	forge-geom::algorithms::polygon_overlap	Reuse/extend
2. Scope Framing
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
Full NURBS support equivalence and UV trim certification implementation
3. Architecture Alignment and Crate Ownership
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
    PartialMergePlanRejected { reason: String },
    UnsupportedPersistentNmtOutput,
}
KernelError wraps this as a structured variant (or equivalent sub-enum wrapper).

3.5 Tracing and Result Envelopes
forge-geom returns plain certifiers/results.
forge-kernel wraps merge/certification orchestration in OperationResult<T>.
Tests for new merge/certification flows must support FORGE_TRACE_DIR.
4. Epic A — Boundary Certification (Planar Now, UV-Reusable)
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
  tests.rs
5. Epic B — NMT-Compatible Region Merge (Intermediate States)
5.1 Goal
Permit controlled merge reasoning and cleanup when local radial valence exceeds 2, while preserving manifold-default final output semantics.

5.2 Validation Model Extension (forge-topo)
Extend validation with a topology mode in addition to ValidationLevel.

pub enum TopologyMode {
    ManifoldStrict,
    NmtIntermediate,
    NmtPersistent, // reserved, not implemented
}
5.3 Validation Semantics
ValidationLevel controls breadth/depth of checks.
TopologyMode controls manifoldness policy.
In NmtIntermediate:

allow valence > 2
still enforce:
radial ring closure
endpoint consistency
loop integrity
pointer consistency
hierarchy integrity (as applicable)
5.4 Mode Propagation (Must Be Explicit)
The spec requires an explicit API path to pass mode into validation/commit. Example acceptable designs:

MutableDraft::commit_with_mode(level, mode)
validation policy embedded in operation context and passed to commit
internal boolean cleanup draft commit APIs with explicit mode
This must not rely on hidden globals.

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
JoinFaces remains manifold-only. MergeSheetRegion is a compound kernel/topo algorithm.

Key rule: no implicit partial merges.

Execution outcomes must be explicit:

full success
fail-fast with MergeError
or explicit PartialMergePlan (if later supported; not default in this milestone)
For this milestone, default behavior is fail-fast on non-executable merge plans.

5.9 NMT Merge Strategy (Decomposition)
MergeSheetRegion validates the selection, builds a deterministic merge plan, then applies manifold-safe reductions only where the selected local neighborhood forms a 2-manifold sub-sheet.

Example plan shape:

Validate selected faces are connected.
Validate/derive radial-use plan.
For each internal selected interface edge:
confirm selected local valence == 2 for the selected sub-sheet
confirm protected uses remain intact
Build deterministic ordered merge steps.
Execute steps (delegating to JoinFaces where valid).
On any invalid step, abort and return MergeError (no silent skipping).
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
  schema.rs        // MergeRegionSelection and plan/result types
  nmt_eval.rs      // selection validation + plan builder + execution
  tests.rs
6. Epic C — Curved Same-Support Surface Merge (Design Contracts Only)
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
validate geometry binding integrity after merge
7. Boundary Certifier Backend Abstraction (For Planar + UV Reuse)
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
Test Case	Expected
Clean coplanar merge (adjacent quads)	Simple
Endpoint self-touch from sliver cleanup	WeaklySimple { ... }
Overlapping collinear chains	Rejected { OverlappingSegments } (or accepted later if policy changes)
Proper crossing	Rejected { SelfCrossing }
Repeated vertex / duplicate segment image	fallback path invoked; deterministic result
Determinism (same input repeated)	identical certificate + trace summary/hash
9.3 NMT Intermediate (Milestone 2)
Test Case	Expected
Valence-3 local merge with explicit selection	deterministic merge success
Valence-4 ambiguous radial neighborhood	AmbiguousRadialSelection
Protected-use preservation	protected radial uses unchanged
ManifoldStrict mode commit on valence>2	manifold error
NmtIntermediate mode validation/commit	passes (if other invariants valid)
Non-executable plan	fail-fast MergeError (no silent partial merge)
9.4 Curved Readiness (Milestone 3)
Test Case	Expected
Same-cylinder split patches	SurfaceRelation::Coincident
Same-cone near apex	correct classification or Undetermined
Same-sphere with parameter shift	Coincident
Cone vs torus	General (or refined non-coincident result)
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
12. Notes for Repo Integration
Use workspace-relative links in the actual REGION_MERGE_SPEC.md instead of file:///... URIs.
Keep forge-topo geometry-free; all boundary certification calls should originate in forge-kernel.
Prefer explicit “design target” wording where APIs are placeholders to avoid accidental implementation commitments.