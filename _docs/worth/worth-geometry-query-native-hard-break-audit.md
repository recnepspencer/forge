# Worth Geometry Query-Native Hard-Break Audit

## Purpose

This document audits `worth-topo`, `worth-spatial`, and `worth-kernel` against the
runtime model in `crates/forge-query/docs/AI_README.md`, using
`_docs/worth/VISION.md` as the product bar.

This is not an incremental migration memo.

This is the hard-break version:

- no pseudo-API compatibility layers
- no "temporary" legacy shims
- no local summary objects that become shadow truth
- no preserving old naming if the old naming lies
- no dual runtime story where Query is present but not actually authoritative

If we want the geometry kernel to scale, the target is not "more Query surfaces."
The target is a geometry stack whose ordinary operating shape is Query-native from
the start.

## Governing Rule

The central `AI_README` rule is:

> declare intent once, lower it once, execute or inspect it through canonical
> runtime-owned artifacts

The geometry stack is not yet doing that consistently.

## Strict Conclusion

`worth-topo` is the only crate that is meaningfully shaped by the Query runtime.

`worth-spatial` is still mostly a semantic authority plus Query intent handoff
layer.

`worth-kernel` has real Query-native seams, but too much of its geometry meaning
is still reconstructed locally after Query has already done work.

So the next spec should not be "make current code use more Query surfaces."

It should be:

1. promote geometry semantics into real Query declaration families
2. delete kernel-local semantic replay
3. make retained Query artifacts the only ordinary transport for geometry truth
4. treat `worth-topo` as the lower-runtime and projection backend for geometry,
   not as a sidecar example of what Query-native architecture could look like

## Ratings

### Query-native status by crate

| Crate | Rating | Strict read |
|---|---:|---|
| `worth-topo` | 7/10 | Real Query domain, real handles, real grouped workflow, real basis lifecycle, real projection/materialization/runtime posture. Still not the ordinary geometry runtime spine for the whole stack. |
| `worth-kernel` | 4/10 | Real Query family work exists, especially in rebinding, but meaning still falls back to local semantic replay and locally-owned artifact bundles. |
| `worth-spatial` | 2/10 | Mostly a semantic authority and intent-encoding layer. Very little of the ordinary Query runtime grammar is present. |

### Architectural honesty by crate

| Crate | Rating | Strict read |
|---|---:|---|
| `worth-topo` | 8/10 | Honest about support posture, basis, read/write boundaries, and what is not admitted yet. |
| `worth-kernel` | 5/10 | Honest in tests and some workflow boundaries, but still semantically dual-track in production. |
| `worth-spatial` | 3/10 | Honest semantically, but architecturally still treats Query as a downstream handoff instead of the ordinary runtime skeleton. |

## Surface-by-Surface Audit

The sections below use the actual surface families from `AI_README.md`.

### 1. Public Runtime Facade

#### Ideal

Geometry should be entered through real Query-backed public runtime families, not
through direct kernel or spatial calls wrapped in convenience APIs.

#### Current state

**`worth-topo`: real**

- `crates/worth-topo/src/query_domain.rs`
- `crates/worth-topo/src/projection/runtime_boundary/query_runtime/contracts.rs`
- `crates/worth-topo/src/projection/runtime_boundary/query_runtime/runtime_posture.rs`

Topo has a real public runtime posture story, including capability support and
explicitly denied capabilities.

**`worth-spatial`: missing**

- `crates/worth-spatial/src/spatial_intent/arbitration/declared_analysis.rs`
- `crates/worth-spatial/src/spatial_intent/lowering/lowered_intents/runtime_declaration.rs`

Spatial exposes semantic declarations and intent admission helpers, not a real
public runtime facade family for ordinary geometry work.

**`worth-kernel`: partial**

- `crates/worth-kernel/src/construction/authoring.rs`

Kernel construction checks Query public API family support, but that is still a
session/checkpoint shape, not a real geometry domain facade.

#### Gap

The geometry stack does not yet have one honest public runtime facade for:

- target identity
- binding
- rebinding
- topology replacement neighborhood
- tolerance and precision certification
- historical inspection
- branch-local inspection
- replay parity
- recovery
- projection consumption

#### Hard-break requirement

Create real geometry-facing Query families and delete direct "semantic service"
entrypoint posture as the ordinary path.

No adapter layer that preserves old non-Query call shapes should survive.

### 2. Domain Entry And Configured Handles

#### Ideal

Ordinary geometry work should happen under admitted configured handles in explicit
operating contexts.

#### Current state

**`worth-topo`: real**

- `TopologyQueryDomain`
- `TopologyCurrentHeadAuthoritativeContext`
- `TopologySnapshotReadOnlyContext`

in `crates/worth-topo/src/query_domain.rs`

**`worth-spatial`: missing**

Search shows no meaningful use of:

- `ForgeQueryDomainEntryMarker`
- `ForgeQueryDomainOperatingContext`
- `ForgeQueryAdmittedConfiguredDomainHandle`

inside spatial source.

**`worth-kernel`: partial**

- `PrimitiveRebindingQueryDomain`
- `PrimitiveRebindingQueryWorld`

in `crates/worth-kernel/src/binding/rebinding/query_domain.rs`

This exists for rebinding, but not as a coherent geometry-wide domain strategy.

#### Gap

Spatial authority does not live inside admitted Query domain contexts. It is still
called as a local semantic authority from kernel code.

#### Hard-break requirement

Every ordinary geometry family must live under explicit Query domains and
configured contexts.

Delete direct semantic authority entrypoints as ordinary call surfaces.

If a geometry operation cannot be expressed as work under an admitted handle, it is
not Query-native and should be redesigned, not wrapped.

### 3. Declarations And Family Contracts

#### Ideal

Geometry intent should be expressed as real declaration families with:

- canonical identity entries
- aspect contracts
- legality contracts
- route contracts
- grouped posture
- signal posture

#### Current state

**`worth-topo`: real**

Topology operators are real Query declaration families with grouped-neighborhood
support.

**`worth-spatial`: missing**

Spatial mostly lowers to:

- `ForgeQueryIntentDeclaration`
- `ForgeQueryRawIntentAdmissionRequest`

from:

- `crates/worth-spatial/src/spatial_intent/arbitration/declared_analysis.rs`
- `crates/worth-spatial/src/spatial_intent/lowering/lowered_intents/runtime_declaration.rs`

That is not the same thing as real declaration-entry families.

**`worth-kernel`: partial**

Rebinding does have a real declaration family:

- `crates/worth-kernel/src/binding/rebinding/query_domain.rs`

But that family is too shallow because its semantic meaning is still pulled from a
local `admit()` lane after Query progression.

#### Gap

The geometry stack lacks first-class Query families for most of its real semantic
responsibilities.

#### Hard-break requirement

Promote these into real Query declaration families:

1. `GeometryTargetIdentity`
2. `SpatialAnchorSelection`
3. `PrimitiveBinding`
4. `PrimitiveRebinding`
5. `TopologyNeighborhoodReplacement`
6. `ToleranceAndPrecisionCertification`
7. `HistoricalGeometryInspection`
8. `BranchLocalGeometryInspection`
9. `GeometryReplayParity`
10. `GeometryRecoveryAction`
11. `ProjectionConsumption`

Do not preserve legacy names that imply "service," "analysis," or "helper" when
the real thing is a declaration family.

### 4. Readiness, Orchestration, Route, Receipt, And Envelope

#### Ideal

These are not optional internals. They are the canonical workflow spine.

#### Current state

**`worth-topo`: real**

Topo has strong use of:

- declaration progression
- route plan
- receipt
- envelope
- recovery
- query-native construction receipt/envelope handoff

Examples:

- `crates/worth-topo/src/topology_operators/application/declaration_entry/orchestration_boundary.rs`
- `crates/worth-topo/src/construction/query_native_boundary.rs`

**`worth-spatial`: missing**

Spatial does not use the declaration-entry workflow grammar as its ordinary shape.

**`worth-kernel`: partial**

Kernel rebinding captures these artifacts in:

- `crates/worth-kernel/src/binding/workflow_boundary/canonical_artifacts.rs`

But then local semantic replay still happens afterward:

- `crates/worth-kernel/src/binding/rebinding/workflow_transport.rs`

#### Gap

Geometry workflow artifacts exist in parts of the stack, but they are not yet the
only carriers of geometry truth.

#### Hard-break requirement

No geometry family should be allowed to:

- progress through Query
- then re-decide meaning locally afterward

If the canonical envelope does not carry enough semantic truth, fix the family
contract and its artifacts. Do not patch it with local post-processing.

### 5. Ordinary Outcomes

#### Ideal

Ordinary outcomes must preserve actual runtime posture:

- bound
- deferred
- denied
- stale
- rebind required
- ambiguous
- unsupported
- failed

and must preserve geometry meaning without flattening it.

#### Current state

**`worth-topo`: real**

Strong ordinary workflow posture and denial preservation.

**`worth-spatial`: missing**

Spatial does not use ordinary Query outcomes as its ordinary semantic delivery
vehicle.

**`worth-kernel`: partial**

`PrimitiveRebindingDeclarationEntry::ordinary_outcome_with_query(...)` exists in:

- `crates/worth-kernel/src/binding/rebinding/workflow.rs`

But `workflow_transport.rs` still:

1. orchestrates Query ordinary outcome
2. calls local `entry.clone().admit()`
3. remaps ordinary outcome from the local semantic result

#### Gap

Kernel still treats Query ordinary outcomes as insufficient without local semantic
repair.

#### Hard-break requirement

Delete the production need for local ordinary-outcome remapping from kernel-local
authority calls.

Geometry family outcomes must arrive already semantically complete as Query-owned
ordinary outcomes.

### 6. Typed Binding And Retained Artifact Reuse

#### Ideal

History, branch-local, and replay should consume retained authoritative artifacts,
not live-state fallback or local semantic recomputation.

#### Current state

**`worth-topo`: real**

Topo has meaningful retained basis and historical materialization infrastructure.

**`worth-spatial`: missing**

Spatial is not using retained Query artifacts as the ordinary geometry runtime
shape.

**`worth-kernel`: partial**

Kernel does use:

- historical inspection inputs
- branch-local basis evidence
- replay parity

But it still derives semantic truth from local `admit()` paths in production.

Examples:

- `crates/worth-kernel/src/binding/rebinding/branch_local_inspection.rs`
- `crates/worth-kernel/src/binding/rebinding/workflow.rs`
- `crates/worth-kernel/src/binding/rebinding/replay_parity.rs`

#### Gap

Retained artifacts are being used as proof scaffolding, not yet as the only real
semantic transport.

#### Hard-break requirement

Historical, branch-local, and replay paths must derive from retained Query geometry
artifacts that already contain the spatial semantics.

Delete all production code paths where retained inspection is followed by local
semantic re-admission of the same declaration.

### 7. Basis Capability Lifecycle

#### Ideal

Basis is not a string or ad hoc branch label. It is a typed lifecycle.

#### Current state

**`worth-topo`: real**

Topo has real basis admission and historical materialization paths.

**`worth-spatial`: missing**

Spatial has local concepts like `SpatialFrameBasis`, but that is not Query basis
lifecycle.

**`worth-kernel`: partial**

Kernel branch-local inspection uses:

- `ScopedInspectionBasis`
- `LowerRuntimeBasisEvidence`
- `readmit_lower_runtime_evidence(...)`

in:

- `crates/worth-kernel/src/binding/rebinding/branch_local_inspection.rs`

This is good, but still too isolated.

#### Gap

Geometry-wide basis lifecycle does not exist as an ordinary design rule. It exists
only in slices.

#### Hard-break requirement

Every geometry family that can be historical, branch-local, preview-scoped, or
read-only must explicitly declare basis posture.

Delete any geometry API that accepts raw branch, snapshot, or preview identifiers
as ordinary inputs.

### 8. State Readiness Vs Inspection

#### Ideal

These are separate responsibilities:

- state readiness
- declaration entry readiness
- inspection

#### Current state

**`worth-topo`: partial but real**

Strong posture/read support, though not yet geometry-wide.

**`worth-spatial`: missing**

Spatial mostly has semantic analysis and support matrices, not the full runtime
split.

**`worth-kernel`: partial**

Kernel has declaration-entry readiness and inspection in rebinding, but not a
geometry-wide state-readiness model.

#### Gap

Geometry readiness is not yet a first-class public model across the stack.

#### Hard-break requirement

Add explicit geometry readiness families and stop using semantic analysis or
"support summary" objects as substitutes.

### 9. Recovery

#### Ideal

Recovery is a typed next-step lane, not local error handling.

#### Current state

**`worth-topo`: real**

Topo uses recovery in orchestration outcomes.

**`worth-spatial`: missing**

Spatial mostly exposes semantic hints and support diagnostics, not recovery
families.

**`worth-kernel`: weak**

Kernel ordinary outcomes preserve some posture, but recovery is not yet a strong
first-class geometry family.

#### Gap

Geometry denial handling is still too explanation-shaped and not enough
declaration-shaped.

#### Hard-break requirement

Promote recovery into real geometry Query families:

- ambiguous successor narrowing
- branch basis correction
- neighborhood widening
- tolerance escalation
- correspondence-only promotion denial handling
- unsupported family fallback routing

Delete "helper" APIs that just summarize denial reasons without exposing actual
typed recovery paths.

### 10. Grouped And Neighborhood Work

#### Ideal

Neighborhood work is semantic grouping, not batching.

#### Current state

**`worth-topo`: real**

- `ForgeQueryGroupedDeclarationInput::local_neighborhood(...)`
- grouped topology operator workflows

**`worth-spatial`: missing**

Spatial absolutely has neighborhood semantics, but they do not live as grouped
Query workflow.

**`worth-kernel`: weak**

Kernel rebinding references neighborhood semantics in declaration content, but not
as true grouped-neighborhood family workflow.

#### Gap

Neighborhood is still more of a domain field than a runtime grammar.

#### Hard-break requirement

Promote neighborhood-bearing geometry work into true grouped Query families.

Delete ad hoc neighborhood payloads where the runtime grouping contract should be
authoritative.

### 11. Domain Capability Contributions

#### Ideal

Domain semantics travel as contributions, not ambient settings.

#### Current state

**`worth-topo`: real**

- `ForgeQueryContributionComposedOrchestrationInput::new(...).with_contributions(...)`

in:

- `crates/worth-topo/src/topology_operators/query_workflow/grouped_and_contribution_builders.rs`

**`worth-spatial`: missing**

Spatial has policy profiles and semantic capabilities, but not Query contribution
composition as the ordinary lane.

**`worth-kernel`: missing**

Kernel binding and construction mostly do not use contribution-composed geometry
workflow.

#### Gap

Geometry policy is still too ambient and too locally encoded.

#### Hard-break requirement

Move geometry policy into contribution-composed orchestration:

- tolerance policy
- fallback policy
- naming preservation policy
- branch preview policy
- witness strictness
- continuity/correspondence strictness

Delete ambient configuration or declaration-local booleans where contribution
contracts should exist instead.

### 12. Lower-Runtime Capability Routing

#### Ideal

Query should own routing to lower runtimes, not leave domains to stitch together
backend semantics informally.

#### Current state

**`worth-topo`: real**

Topo clearly owns more of the lower-runtime routing story than the other crates.

**`worth-spatial`: missing**

Spatial talks about support and graph composition, but not in the shape of ordinary
capability-routed runtime artifacts.

**`worth-kernel`: weak**

Kernel construction knows about family support and topology handoff, but this is
still largely proof and boundary reporting, not ordinary geometry family routing.

#### Gap

Geometry routing is too implicit and too split between kernel proof artifacts and
topology runtime internals.

#### Hard-break requirement

Every geometry family must declare its lower-runtime routes explicitly:

- topology read
- topology write
- historical materialization
- projection consumption
- branch preview
- branch-local inspection

Delete implicit backend stitching.

### 13. Authoritative Mutation Evidence

#### Ideal

Mutation evidence should preserve causality and target identity, not just success.

#### Current state

**`worth-topo`: real**

Examples:

- `crates/worth-topo/src/topology_operators/application/declared_mutation_artifact/mutation_evidence.rs`
- `crates/worth-topo/src/topology_operators/application/declared_mutation_artifact/query_anchor.rs`

**`worth-spatial`: weak**

Spatial reports support for authoritative mutation evidence, but does not host its
ordinary semantic families through it.

**`worth-kernel`: weak**

Kernel construction consumes topology evidence but geometry families do not yet
speak through authoritative mutation evidence as their ordinary runtime truth.

#### Gap

Geometry-wide mutation evidence is not yet a first-class runtime shape.

#### Hard-break requirement

Binding, rebinding, topology replacement, and projection-affecting geometry writes
must all produce authoritative mutation evidence as retained Query artifacts.

Delete "success report" style APIs that do not carry causality and identity.

### 14. Signal Compatibility And Continuation

#### Ideal

Where appropriate, geometry families should be signal-compatible or continuation-
capable.

#### Current state

**`worth-topo`: partial**

The runtime clearly has more live/subscription capability than many declaration
families currently admit.

**`worth-spatial`: missing**

No meaningful ordinary use of signal/continuation surfaces.

**`worth-kernel`: missing**

Binding query families explicitly use `ForgeQuerySignalNotCompatiblePosture`.

#### Gap

The geometry stack is underusing Query's continuation and reactive grammar.

#### Hard-break requirement

Identify which geometry families are:

- fundamentally non-signal
- continuation-capable
- preview/live-materialization-capable

Then encode that honestly in family contracts.

Do not leave `SignalNotCompatible` as the default forever because it is easier.

### 15. Structural Correspondence And Historical Materialization

#### Ideal

Historical and structural comparison should be explicit, retained, and ambiguity-
honest.

#### Current state

**`worth-topo`: real**

Topo has actual basis and historical materialization machinery.

**`worth-spatial`: missing**

Spatial has semantic continuity/correspondence reasoning, but not as Query
historical materialization families.

**`worth-kernel`: partial**

Kernel has historical and branch-local inspection, plus replay, but still treats
retained artifacts as inputs to local semantic interpretation rather than as the
complete runtime truth.

#### Gap

Historical materialization is not yet the ordinary geometry truth path.

#### Hard-break requirement

Promote continuity, correspondence, and historical materialization into explicit
retained Query artifacts owned by geometry families.

Delete local semantic reconstruction from retained data.

### 16. Projection Consumption And Typed Facts

#### Ideal

Projection consumption is a declared lane for typed facts, not a local facade
pattern.

#### Current state

**`worth-topo`: real**

Strong query-native construction receipt/envelope/projection-consumption shape.

**`worth-spatial`: missing**

No ordinary projection-consumption lane for spatial facts.

**`worth-kernel`: partial**

Kernel construction consumes topology projection-consumption reports, but geometry
families do not yet emit typed facts through that spine as their ordinary runtime
shape.

#### Gap

Typed fact emission is too topology-local and too certification-oriented.

#### Hard-break requirement

Geometry families must expose typed fact projection consumption for:

- continuity class
- correspondence class
- witness validity
- tolerance certificate
- selected candidate identity
- denial provenance

Delete local diagnostic structs that are not receipt-backed typed fact lanes.

### 17. Cross-Runtime Causal Inspection

#### Ideal

The ordinary runtime should answer:

- what caused this geometry result
- what retained basis was used
- what route was taken
- what mutation evidence supports it
- what continuation or denial followed

#### Current state

**`worth-topo`: partial**

Closest to this through runtime proof and retained handoff artifacts.

**`worth-spatial`: missing**

Mostly semantic explanation, not cross-runtime causal inspection.

**`worth-kernel`: weak**

Kernel certifies a lot in tests, but ordinary geometry runtime APIs do not yet
present causal inspection as first-class behavior.

#### Gap

Causal inspection is still more of a certification/proof concern than an ordinary
geometry runtime concern.

#### Hard-break requirement

Every retained geometry artifact family should support causal inspection.

Delete local "report" types that summarize causal truth without carrying canonical
artifact lineage.

## Hard Prohibitions We Are Still Violating

These are the most important `AI_README`-style violations still present.

### 1. Local pseudo-Query semantic replay in kernel

Violating files:

- `crates/worth-kernel/src/binding/rebinding/workflow.rs`
- `crates/worth-kernel/src/binding/rebinding/workflow_transport.rs`
- `crates/worth-kernel/src/binding/rebinding/branch_local_inspection.rs`

Problem:

Query progression happens, then kernel still calls local spatial authority through
`entry.clone().admit()` to recover semantic truth.

This is the single clearest "not yet Query-native" smell in the stack.

### 2. Intent handoff as ordinary runtime shape in spatial

Violating files:

- `crates/worth-spatial/src/spatial_intent/arbitration/declared_analysis.rs`
- `crates/worth-spatial/src/spatial_intent/lowering/lowered_intents/runtime_declaration.rs`

Problem:

Spatial still behaves like a semantic authoring/lowering layer that hands work to
Query intent admission, not like a domain whose ordinary workflow lives in Query
declaration families.

### 3. Query runtime family support checks as a substitute for real geometry family design

Violating files:

- `crates/worth-kernel/src/construction/authoring.rs`
- `crates/worth-kernel/src/construction/runtime_proof/query/boundary_gap_register.rs`

Problem:

These are honest and useful, but they also expose that kernel construction is
still too aware of missing neighbor families because the geometry runtime shape is
not yet natively made of them.

### 4. Topology is Query-native in isolation, not yet as the full geometry substrate

Representative files:

- `crates/worth-topo/src/query_domain.rs`
- `crates/worth-topo/src/projection/runtime_boundary/...`
- `crates/worth-topo/src/topology_operators/query_workflow/...`

Problem:

Topo has the strongest Query-native design, but the rest of the geometry stack is
not fully founded on it.

It is a strong substrate, not yet the ordinary geometry runtime spine.

## What Must Be Deleted

This section is intentionally blunt.

If we want the most honest migration, these categories should be annihilated rather
than preserved behind compatibility facades.

### Delete as ordinary entrypoints

- direct semantic service-style geometry calls from kernel into spatial for
  production Query-native paths
- "authoring sessions" that are really capability checkers without real family
  ownership
- intent-admission-only geometry runtime stories

### Delete as semantic transport

- local kernel summary bags that carry meaning parallel to Query artifacts
- locally remapped ordinary outcomes
- replay/history/branch semantics derived by re-running local authority calls

### Delete as naming

- names that imply "analysis" when the real thing is a declaration family
- names that imply "helper" when the real thing is a runtime boundary
- names that imply "intent handoff" when the real thing should be ordinary domain
  execution

### Delete as migration strategy

- dual-path APIs where old non-Query and new Query-native flows coexist long-term
- "temporary" wrappers around legacy entrypoints
- soft deprecations that keep semantic authority split across layers

## The Target Shape

The target is not "kernel uses Query more."

The target is:

### `worth-topo`

Owns:

- lower-runtime read/write/materialization
- basis admission and scoping
- projection consumption
- authoritative mutation evidence
- signal and continuation substrate where admitted

Does not own:

- kernel-local semantic replay
- spatial meaning
- ad hoc geometry authoring policy

### `worth-spatial`

Owns:

- geometry semantics
- binding and anchor identity laws
- continuity and correspondence laws
- tolerance and precision certification laws
- denial classes

But owns them as:

- Query declaration families
- Query inspections
- Query recovery families
- Query retained artifacts

Not as:

- intent handoff objects
- local semantic service calls

### `worth-kernel`

Owns:

- product-facing geometry composition
- DX over real Query families
- certification and parity proof

Does not own:

- semantic re-admission of geometry truth
- branch/history/replay meaning reconstruction
- shadow artifact stories

## Recommended Spec Direction

The next spec should be a hard-break spec with language like:

1. "Promote spatial authority into first-class Query declaration families."
2. "Delete kernel-local semantic replay after Query progression."
3. "Require retained Query artifacts to be the only geometry history, branch, and
   replay truth."
4. "Refound geometry write, read, inspection, recovery, and projection
   consumption on topo-backed Query runtime substrate."
5. "Forbid pseudo-Query wrapper layers, legacy shims, and local summary carriers
   as migration tools."

## Acceptance Bar For The Rewrite

The geometry stack is only "100% Query native" when all of the following are true:

1. No ordinary production geometry path calls local spatial authority after Query
   progression to recover meaning.
2. `worth-spatial` no longer treats intent admission as its ordinary runtime shape.
3. History, branch-local inspection, and replay consume retained Query geometry
   artifacts that are already semantically complete.
4. Geometry recovery is a real declaration-family lane, not a denial-summary lane.
5. Grouped neighborhood work and contributions are first-class across geometry,
   not just topology.
6. Projection consumption and typed fact delivery are ordinary geometry runtime
   behavior.
7. Support posture is explicit and honest for every geometry family.
8. No legacy shim or dual-path semantic transport remains in production code.

That is the honest bar.
