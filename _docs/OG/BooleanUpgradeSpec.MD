1. Full weakly-simple polygon recognition
What’s missing:

A geometric boundary representation independent of halfedge IDs
projected 2D segments for each boundary cycle candidate
dominant-axis or local-frame projection with deterministic tie-breaking
Robust segment intersection / touch classification
proper intersection vs endpoint touch vs overlap
exact or certified predicates in projected space
A weakly-simple recognition algorithm implementation (or equivalent sufficient certifier)
Boundary event graph / sweep-line infrastructure (deterministic ordering)
Explicit policy for coincident/overlapping boundary segments
Why boolean tests hit this:

coplanar face groups can produce boundary self-touches after prior splits / slivers even if local topology “looks” connected.
2. Non-manifold region merge support (radial valence > 2)
What’s missing:

A formal non-manifold region model for merge eligibility
which incident sheets count as the merge region
which must remain untouched
Inclusion/topology semantics beyond current manifold shell assumptions
likely Q-Complex / radial-edge style representation or compatibility layer
Non-manifold-safe Euler operator equivalents
JoinFaces semantics for edges with more than 2 incident uses is not the manifold operator anymore
Validation extensions
current manifold validators reject the very states we’d need to reason about
Failure taxonomy for partial sheet merges / ambiguous radial selections
Why boolean tests hit this:

complex coplanar contacts can create local high-valence edges during stitching/classification before cleanup.
3. Curved-surface coplanar-equivalent region merge
What’s missing:

Geometric support identity/equivalence query
“same plane” today becomes “same analytic surface” / same parameterized support patch family
Surface compatibility checks
same cylinder axis/radius, same cone apex/angle, same NURBS patch or certified equivalence
Boundary extraction in parameter space (UV coedges)
topological loops alone are insufficient for robust curved-face merge eligibility
Normal and tangent continuity evaluation integrated with surface refs
EvaluateNormal is only a trait stub currently
Rebuild/merge postconditions for trims
resulting face may need trim-loop recomputation/normalization in UV
Why boolean tests hit this:

fillet and curved boolean outputs will quickly require “same support merge” rather than planar-only merge.
4. Tolerant / fuzzy boundary certification

What’s missing:

A lower-layer ambiguity representation (PolicyResult-style) for boundary certification outcomes
Kernel policy plumbing into merge preflight
explicit tolerance config passed in
decision logging for every ambiguity (D2)
Certified geometric proximity helpers
point-on-segment within tolerance
near-coincident vertex/edge clustering with deterministic order
Canonical snapping / clustering policy
deterministic, logged, reversible or transactional
Test corpus for near-degenerate merge boundaries

Why boolean tests hit this:

exact topological checks fail on imported/healed or numerically perturbed data that is geometrically intended to merge.

5. Per-join deep validation by default

What’s missing:

Fast incremental validators (local neighborhood validation)
full validation each join is too expensive for large regions
Validation checkpoint configuration surfaced in API/options
Performance baselines and thresholds
Debug vs production profiles with deterministic behavior unchanged
Instrumentation for “which join invalidated topology”

Why boolean tests hit this:

deep validation is useful for debugging failing complex cases, but too slow to run always unless incrementalized.

6. Complete formal join applicability for all topology cases

What’s missing:

A comprehensive case taxonomy for JoinFaces applicability
hole-producing, hole-consuming, shared-vertex edge cases, nested loop interactions
Pre/post invariants per case
Join simulation/preflight model
“what would loop topology become if we join here?”
Operator capability introspection
what current JoinFaces actually guarantees vs rejects
Exhaustive topology tests (small graph fixtures), not just geometry-driven boolean cases

Why boolean tests hit this:

ad hoc eligibility fails on rare loop/topology configurations that only appear in complex unions/differences.

7. Automatic geometric reclassification if preconditions drift during merge

What’s missing:

A callback/adapter contract from topo back to kernel-owned geometry classification
while preserving layer boundaries (Adapter Rule)
Incremental geometry support cache updates as faces are joined
Policy/logging integration for reclassification decisions
Deterministic restart/reschedule strategy if region membership changes mid-merge
Clear transaction semantics (restart merge loop vs fail vs split region)

Why boolean tests hit this:

regions chosen as coplanar by caller can become invalid after local edits if geometry bindings or classification assumptions are incomplete.

8. Rich lineage composition inside topo algorithm

What’s missing:

A stable lineage-composition contract shared between topo and kernel
merge provenance rules
face remap semantics
multi-parent lineage policy
Structured result payload in topo (MergeRegionResult) carrying lineage hooks/IDs
Kernel-side lineage reducer utilities to consume topo outcomes deterministically
Tests for lineage stability under deterministic edge-order changes

Why boolean tests hit this:

regression/debugging and replay become hard if face merges lose parentage in complex boolean cascades.

9. Inclusion-topology handling for auxiliary/internal construction entities

What’s missing:

Data model support for inclusion topology / non-boundary entities
Boundary extraction that distinguishes authoritative boundary vs internal/support entities
Query APIs exposing inclusion relationships
Validation rules for inclusion-consistent merges
Boolean pipeline tagging for construction/generated auxiliary topology

Why boolean tests hit this:

as operations get richer (fillet, loft, healing), internal helper entities can leak into downstream traversal unless modeled explicitly.

10. Global rectification / repair fallback inside merge

What’s missing:

A dedicated repair pipeline contract (separate from local merge)
candidate generation
scoring/objective
deterministic selection policy
Structured “repair requested” result instead of hard failure
Geometry+topology rectification algorithms (non-local)
Performance/timeout policy and observability
Clear doctrine boundary so local ops don’t silently become repair routines

Why boolean tests hit this:

some pathological outputs are not locally mergeable but still salvageable with repair; mixing this into merge would hide failure causes.