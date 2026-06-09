# Worth Geometry Query-Native Hard-Break Spec

> **ACTIVE EXECUTION (MUST BE REFERENCED):**
> [worth-geometry-query-native-ACTIVE.md](worth-geometry-query-native-ACTIVE.md)
>
> Every implementation agent session **must** read and follow the ACTIVE file
> first. It is the rolling window for the current pass, slice, allowed files,
> exit criteria, and deferred work. This full spec is architecture reference
> when blocked — not the per-turn execution queue.
>
> **Status:** In Progress (see ACTIVE file for current pass)
>
> **Roadmap parent:** [worth_roadmap.md](worth_roadmap.md)
>
> **Primary audit input:** [worth-geometry-query-native-hard-break-audit.md](worth-geometry-query-native-hard-break-audit.md)
>
> **Primary Query runtime reference:** `crates/forge-query/docs/AI_README.md`

## Goal

Refound the Worth geometry stack so its ordinary operating shape is fully
Query-native:

- geometry intent is authored as real Query declaration families
- geometry workflow is carried only by canonical Query runtime artifacts
- history, branch-local inspection, and replay consume retained Query truth
- `worth-kernel` composes and certifies geometry families rather than repairing
  or re-deciding them locally
- `worth-spatial` owns semantics as Query-native domain families rather than as
  pre-Query intent handoff and local authority services
- `worth-topo` serves as the lower-runtime read/write/materialization substrate
  for geometry rather than as an isolated example of Query-native design

This is a hard-break rewrite, not a compatibility migration.

The point is not to force every geometry file or every geometry family to touch
every Query surface.

The point is to make every admitted geometry capability tell one coherent Query
runtime story:

1. a geometry responsibility enters through a real Query domain and family
2. its meaning is declared canonically once
3. its workflow moves through readiness, progression, route, receipt, envelope,
   and ordinary outcome where those surfaces are actually relevant
4. retained truth is reused for history, branch-local inspection, replay, and
   downstream fact consumption where those responsibilities are relevant
5. unsupported or inapplicable surfaces are classified explicitly instead of
   being implied, faked, or silently bypassed

## Why This Spec Exists

The audit in
[worth-geometry-query-native-hard-break-audit.md](C:\Users\Esther\Documents\Programming\forge_workspace\worktree_2\_docs\worth\worth-geometry-query-native-hard-break-audit.md)
shows that the current geometry stack has three different architectural states:

- `worth-topo` is meaningfully Query-native
- `worth-spatial` is mostly still a semantic authority plus intent-handoff
  layer
- `worth-kernel` has Query-native seams, but still reconstructs geometry truth
  locally after Query has already done work

That split is tolerable for tactical milestone closure and wrong for the long
term.

If left in place, it will eventually produce:

- duplicate geometry authority
- parallel history and replay stories
- branch-local behavior that drifts from ordinary runtime truth
- projection and diagnostics lanes that cannot scale cleanly
- more local wrappers, summaries, and compatibility helpers every time a new
  geometry workflow arrives

This spec exists to stop that drift now and to define the destructive rewrite
needed to make geometry scale on one honest runtime skeleton.

## Canonical Geometry Runtime Story

This rewrite is not trying to "use more Query."

It is trying to make geometry tell one canonical runtime story from first entry
through retained reuse:

1. a geometry responsibility enters through a real Query domain and declaration
   family
2. the family declares geometry meaning once in its canonical declaration entry
3. the family moves through readiness, progression, route, receipt, envelope,
   and ordinary outcome where those workflow surfaces are actually part of that
   responsibility
4. the family emits a typed fact receipt whenever rich geometry semantics must
   survive beyond immediate ordinary outcome posture
5. retained artifacts, branch-scoped basis artifacts, and replay artifacts reuse
   those same family-owned facts instead of reconstructing geometry meaning
   locally
6. downstream projection consumers consume receipt-backed geometry truth rather
   than bypassing the runtime or rediscovering semantics from lower layers
7. kernel DX and certification sit above that runtime story without becoming a
   second geometry authority

The canonical geometry runtime model is therefore:

- one canonical Query declaration-entry truth regime
- one family-owned typed fact receipt regime
- one retained artifact regime for historical, branch-local, and replay work
- one projection-consumption regime for downstream consumers
- one lower-runtime topology substrate for read, write, routing, and
  materialization
- one kernel composition and certification layer over those artifacts, and
  nothing else

Any geometry path that still needs local semantic replay, post-Query outcome
repair, or pseudo-Query compatibility transport is outside the target runtime
story and must be deleted.

## Authority Model

Authoritative in this rewrite:

- canonical Query declaration identity for geometry families
- family-owned typed geometry fact receipts
- retained geometry artifacts derived from canonical family workflow
- basis admission and basis scoping artifacts where history or branch-local
  responsibilities apply
- lower-runtime topology truth only where geometry families explicitly route to
  it as substrate authority

Derived in this rewrite:

- ordinary DX views that add no new semantic meaning
- historical inspection views
- branch-local inspection views
- replay parity and certification bundles
- projection consumers
- diagnostics and explanatory formatting derived from canonical facts

Not admitted as authority in this rewrite:

- kernel-local summary bags
- local `.admit()` replay after Query progression
- pre-Query spatial intent handoff as ordinary geometry runtime truth
- raw lower-runtime digests without Query basis admission
- preview/session/widget-like local state analogs
- compatibility wrappers that preserve legacy semantics under new names

## Governing Summaries

- `MENTALITY.md`
  The most important thing it protects is solving the hard structural problem
  first. This spec must delete the dual runtime story before adding more
  geometry capability on top of it.
- `arch_laws.md`
  The most important thing it protects is authority separation and proof-bearing
  boundaries. Geometry semantics, Query workflow, lower-runtime routing, and
  certification must remain distinct owners with distinct artifacts.
- `composition_laws.md`
  The most important thing it protects is semantic naming and responsibility
  clarity. This spec must not permit helper bags, pseudo-Query wrappers, or
  session objects that hide real runtime roles.
- `domain_structure_laws.md`
  The most important thing it protects is physically encoded authority and
  lifecycle boundaries. This spec must rehome geometry by runtime truth shape,
  not by convenience or legacy crate survival.
- `perf_laws.md`
  The most important thing it protects is that geometry meaning is lowered once,
  routed once, and replayed from retained truth rather than rediscovered
  repeatedly across hot or critical paths.
- `worth_roadmap.md`
  The most important thing it protects is that Worth runtime/query work must
  enter through Forge Query and must preserve one canonical truth across live,
  historical, branch-local, replayed, and rebuilt views. This rewrite belongs
  now because the current split would otherwise poison future geometry scaling.
- `worth-geometry-query-native-hard-break-audit.md`
  The most important thing it protects is the hard-break diagnosis itself:
  `worth-topo` is the substrate, `worth-spatial` is still pre-Query in
  ordinary shape, and `worth-kernel` is still semantically dual-track in
  production.

## Adversarial Constraint

Worth geometry must survive this hostile condition:

> A long-lived design with persistent identity, local topology replacement,
> curved and planar rebinding, branch-local edits, replay, retained
> inspection, projection consumers, and AI-authored geometric operations must
> preserve the same geometry meaning, the same workflow posture, the same
> continuity and correspondence conclusions, the same recovery surface, and the
> same typed derived facts regardless of whether the result is observed live,
> historically, branch-locally, replayed from retained artifacts, or rebuilt
> from lower-runtime authority.

The rewrite fails if:

- geometry meaning is still re-decided locally after Query workflow
- spatial intent handoff remains the ordinary geometry runtime shape
- retained inspection still requires local semantic re-admission to become
  meaningful
- branch-local and historical truth are aliases instead of distinct Query-native
  responsibilities
- recovery is still a denial-summary story instead of a declaration-family
  story
- projection consumption remains topology-local instead of becoming ordinary
  geometry runtime behavior
- legacy shims preserve old entrypoints or old naming under new wrappers
- the code can still do the convenient wrong edit more easily than the correct
  Query-native edit

## Product Decision Lock

- This is a destructive rewrite, not a compatibility migration.
- No pseudo-API compatibility layer may survive as an ordinary path.
- No legacy entrypoint may be kept alive merely to ease call-site migration.
- No local summary object may carry geometry meaning parallel to canonical Query
  artifacts.
- `worth-spatial` must become a Query-native domain host, not remain a
  pre-Query lowering service.
- `worth-kernel` must become geometry composition, DX, and certification over
  real Query families, not a second semantic runtime.
- `worth-topo` remains the lower-runtime read/write/materialization substrate,
  not the owner of geometry semantics.
- If a geometry capability cannot be expressed honestly through Query domains,
  handles, declaration families, readiness, receipts, envelopes, retained
  artifacts, and typed outcomes, the capability is not admitted and the design
  must change before code ships.

## Must Ship

This rewrite is not complete unless it ships all of the following as one
coherent runtime:

- spatial-owned Query domains for geometry identity, continuity, and
  certification responsibilities
- the full declared geometry family inventory in this spec, with honest
  `Required Now`, `Not Applicable`, and `Denied For This Runtime`
  classifications
- family-owned typed fact receipts as the only rich geometry semantic transport
- one geometry-wide retained artifact contract layer with family-owned payloads
- retained-view families for historical, branch-local, and replay
  responsibilities
- grouped neighborhood workflow and contribution composition wherever local
  edit scope or policy-bearing families actually require them
- explicit lower-runtime routing to `worth-topo` where substrate authority is
  required
- receipt-backed projection consumption for downstream geometry consumers
- compile-fail and API-inventory enforcement proving the legacy non-Query
  entrypoints are gone

## Must Preserve

This rewrite must preserve these runtime truths while it deletes the legacy
architecture around them:

- one canonical Query declaration identity regime across live, historical,
  branch-local, and replayed geometry work
- `worth-topo` as lower-runtime substrate rather than geometry semantic owner
- `worth-spatial` as semantic authority expressed through real Query families
- `worth-kernel` as composition, DX, and certification rather than semantic
  replay
- the continuity-versus-correspondence distinction as family-owned semantic
  truth
- historical, branch-local, and replay as distinct retained-view
  responsibilities rather than one overloaded inspection mode
- explicit fail-closed posture for denied or unsupported surfaces
- no pseudo-API shims, no compatibility wrappers, and no legacy names that lie
  about runtime role

## Public Geometry Runtime Facade And Support Posture

Query-native geometry does not begin at route planning.

It begins at the public runtime facade, the domain entry marker, the operating
context, the configured-handle lifecycle, and the explicit support row that says
whether a family is honestly admitted.

This rewrite therefore requires:

- every ordinary product-facing geometry entrypoint to flow through a public
  geometry facade over real Query domains and admitted configured handles
- every public geometry family to have explicit support posture rather than
  relying on visibility or autocomplete as proof of admission
- every denied family or denied surface to fail closed at the Query-facing
  boundary rather than after local geometry work has already begun
- no kernel or spatial helper surface to act as a second undeclared public
  facade beside the admitted Query-native one

The important distinction is:

- the family inventory in this spec names the runtime vocabulary
- support posture says which of those families or surfaces are admitted now
- public vocabulary is not permission to skip explicit support and admission

For this rewrite, "100% Query-native" means the public geometry runtime story
starts at Query entry and stays there. It does not start in local authoring
helpers and become Query-shaped only after semantic work has already been done.

## Applicability Classification Rule

Not every geometry family should use every Query surface.

That would be cargo-cult architecture, not coherent runtime design.

For every geometry family introduced by this rewrite, the spec and
implementation must classify each major Query surface as exactly one of:

- `Required Now`
- `Not Applicable`
- `Denied For This Runtime`

These classifications must be explicit and reviewable.

### Examples

- `GeometryTargetIdentity`
  - grouped neighborhood workflow: `Not Applicable`
  - authoritative mutation evidence: `Not Applicable`
  - historical inspection: `Required Now`
- `PrimitiveRebinding`
  - grouped neighborhood workflow: `Required Now`
  - replay parity: `Required Now`
  - continuation/signal compatibility: `Required Now` where rebinding must
    preserve downstream reactive continuity after identity-preserving change
- `GeometryRecoveryAction`
  - ordinary mutation evidence: `Required Now` if the action mutates
  - replay parity: `Denied For This Runtime` if the action does not carry a
    retained-history replay responsibility in this rewrite

The important rule is:

if a surface is not relevant, say so cleanly.
If a surface is relevant but the family must fail closed for it, say so
cleanly.
Do not force fake usage just to satisfy a checklist.
Do not hide missing applicability behind vague family wording.

## Mandatory Deletions

The following categories are mandatory deletions, not suggested refactors:

1. direct kernel-to-spatial semantic replay in ordinary Query-native paths
2. spatial "intent declaration" as the ordinary runtime shape for geometry
3. local remapping of ordinary outcomes after Query has already produced them
4. history or branch-local paths that become meaningful only after local
   `admit()` re-execution
5. legacy names that lie about runtime role, such as "analysis", "helper", or
   "session" when the real construct is a declaration family or runtime
   boundary
6. local summary bags that preserve route, receipt, envelope, or replay meaning
   outside canonical Query artifacts
7. any dual-path API that keeps old non-Query geometry semantics alive under a
   compatibility wrapper

## New Query-Native Geometry Family Inventory

The rewrite should target this inventory as the ordinary geometry runtime
surface:

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
11. `GeometryProjectionConsumption`

These names are deliberately direct. They should not be hidden under legacy or
compatibility wording.

These families are not a legalistic list of structures to "use."

They are the runtime narrative geometry should tell:

- `GeometryTargetIdentity` names what geometry truth we are talking about
- `SpatialAnchorSelection` names where that truth is anchored
- `PrimitiveBinding` and `PrimitiveRebinding` carry continuity over change
- `TopologyNeighborhoodReplacement` gives local edit scope an explicit runtime
  shape
- `ToleranceAndPrecisionCertification` makes bounded geometric approximation a
  first-class retained decision
- `HistoricalGeometryInspection`, `BranchLocalGeometryInspection`, and
  `GeometryReplayParity` give the same geometry truth distinct retained-view
  responsibilities rather than one overloaded "inspection mode"
- `GeometryRecoveryAction` turns denied posture into real next-step runtime
  behavior
- `GeometryProjectionConsumption` lets downstream consumers use geometry truth
  without bypassing the runtime or re-deriving it

`GeometryTypedFactReceipt` is not a declaration family.

It is the shared artifact class emitted by geometry families that need rich,
retained, typed semantic fact delivery.

## Family Relationship Lock

The family inventory is not flat in the sense of "all families are the same kind
of thing."

The rewrite locks this relationship model:

- **Base declaration families**
  - `GeometryTargetIdentity`
  - `SpatialAnchorSelection`
  - `PrimitiveBinding`
  - `PrimitiveRebinding`
  - `TopologyNeighborhoodReplacement`
  - `ToleranceAndPrecisionCertification`
  - `GeometryRecoveryAction`
  - `GeometryProjectionConsumption`
- **Retained-view declaration families**
  - `HistoricalGeometryInspection`
  - `BranchLocalGeometryInspection`
  - `GeometryReplayParity`
- **Artifact classes, not declaration families**
  - `GeometryTypedFactReceipt`
  - family-layer mutation evidence artifacts

This means:

- historical, branch-local, and replay are separate declaration families over
  retained artifacts of a source family
- they are not just helper methods hanging off `PrimitiveRebinding`
- they are not one generic inspection mode with flags
- they are not globally family-agnostic unless a source family explicitly opts
  into them

Retained-view family clarification:

- retained-view families use the full declaration workflow spine on their own
  retained-view declaration entry
- they do **not** replay the source family's whole execution pipeline
- they consume retained fact receipts and basis artifacts from the source family
  as inputs to their own Query-native retained-view workflow

## Construction Track Lock

Construction should not get its own new kernel-owned semantic Query domains.

The rewrite locks **Option A**:

- authoritative primitive and topology birth remains topo-declaration-native
- kernel construction becomes workspace-entry and DX over topo construction
  families plus spatial identity, anchor, and certification families
- no new kernel-owned construction semantic domain should be introduced

This means:

- `worth-topo` remains the authoritative construction substrate
- `worth-spatial` contributes geometry semantics used by construction
- `worth-kernel` contributes front-door composition and certification only

If later work reveals a truly missing geometry family needed for construction, it
should be introduced in `worth-spatial`, not as a new kernel-owned runtime
domain.

## Architectural Notes

This rewrite should produce a physically coherent crate and module story, not
just a conceptual one.

### `worth-spatial`

`worth-spatial` should become the host for geometry semantic Query domains and
family contracts.

Target shape:

- domain homes grouped by authority boundary, not by legacy handoff flow
- family modules grouped under those domains
- typed fact receipt modules beside their owning families
- retained-view family modules that clearly depend on retained family facts
- no surviving "intent handoff" layer as the ordinary production path

### `worth-kernel`

`worth-kernel` should become the front-door composition, DX, and certification
layer over spatial and topo-owned runtime truth.

Target shape:

- grouped facades by runtime responsibility, not one god facade
- binding, inspection, replay, recovery, projection, and identity DX separated
  by honest runtime role
- certification housed explicitly as certification, not as pseudo-runtime proof
- no local semantic transport layers that callers must understand to use
  geometry honestly

### `worth-topo`

`worth-topo` remains the reference pattern for lower-runtime Query-native
discipline.

Target shape:

- authoritative construction and topology substrate remains here
- retained-envelope, receipt, routing, and materialization patterns should be
  copied from here where geometry needs them
- geometry must consume topo as substrate authority, not re-explain topo
  patterns locally

### Do Not Create

This rewrite should not introduce new modules or files with soft names that
hide real runtime role.

Do not create:

- `helpers`
- `utils`
- `common`
- `misc`
- `analysis` when the real thing is a declaration family or fact receipt
- pseudo-Query wrapper layers that keep old semantics alive under new names

## Query-Native Entry And Admission Lock

Every admitted geometry family must have a complete front-door Query story
before it can claim to be part of the ordinary runtime.

That front-door story includes:

- a real domain entry marker
- a real operating context
- configured-handle admission
- declaration-family entry through the admitted handle
- explicit support posture at the public geometry facade

This means:

- domain entry and handle admission are not optional prelude details
- no geometry family may begin life as a local authoring or service helper and
  "join Query later"
- retained-view families still need their own admitted declaration entry lane,
  even though they consume retained source-family artifacts

If a family does not yet have this full front-door story, it is not yet part of
the Query-native runtime.

## Front-Door Query Contract Matrix

Legend:

- `R` = `Required Now`
- `N` = `Not Applicable`
- `D` = `Denied For This Runtime`

| Family | DomainEntry | OpContext | HandleAdmission | PublicFacade | SupportRow |
|---|---:|---:|---:|---:|---:|
| `GeometryTargetIdentity` | R | R | R | R | R |
| `SpatialAnchorSelection` | R | R | R | R | R |
| `PrimitiveBinding` | R | R | R | R | R |
| `PrimitiveRebinding` | R | R | R | R | R |
| `TopologyNeighborhoodReplacement` | R | R | R | R | R |
| `ToleranceAndPrecisionCertification` | R | R | R | R | R |
| `HistoricalGeometryInspection` | R | R | R | R | R |
| `BranchLocalGeometryInspection` | R | R | R | R | R |
| `GeometryReplayParity` | R | R | R | R | R |
| `GeometryRecoveryAction` | R | R | R | R | R |
| `GeometryProjectionConsumption` | R | R | R | R | R |

This matrix is as mandatory as the workflow matrices below.

Implementers do not get to treat domain entry, operating context, or support
posture as informal setup outside the runtime story.

## Family x Query-Surface Applicability Matrix

Legend:

- `R` = `Required Now`
- `N` = `Not Applicable`
- `D` = `Denied For This Runtime`

### Workflow Artifact Matrix

| Family | Ready | Prog | Route | Rcpt | Env | OOut | Fact |
|---|---:|---:|---:|---:|---:|---:|---:|
| `GeometryTargetIdentity` | R | R | R | R | R | R | R |
| `SpatialAnchorSelection` | R | R | R | R | R | R | R |
| `PrimitiveBinding` | R | R | R | R | R | R | R |
| `PrimitiveRebinding` | R | R | R | R | R | R | R |
| `TopologyNeighborhoodReplacement` | R | R | R | R | R | R | R |
| `ToleranceAndPrecisionCertification` | R | R | R | R | R | R | R |
| `HistoricalGeometryInspection` | R | R | R | R | R | R | R |
| `BranchLocalGeometryInspection` | R | R | R | R | R | R | R |
| `GeometryReplayParity` | R | R | R | R | R | R | R |
| `GeometryRecoveryAction` | R | R | R | R | R | R | R |
| `GeometryProjectionConsumption` | R | R | R | R | R | R | N |

### Retained, Scope, And Grouping Matrix

| Family | Basis | Hist | Branch | Replay | Grouped | Contrib |
|---|---:|---:|---:|---:|---:|---:|
| `GeometryTargetIdentity` | R | R | R | D | N | N |
| `SpatialAnchorSelection` | R | R | R | D | N | N |
| `PrimitiveBinding` | R | R | R | R | N | R |
| `PrimitiveRebinding` | R | R | R | R | R | R |
| `TopologyNeighborhoodReplacement` | R | R | D | D | R | R |
| `ToleranceAndPrecisionCertification` | R | R | R | R | N | R |
| `HistoricalGeometryInspection` | R | R | N | N | N | N |
| `BranchLocalGeometryInspection` | R | N | R | N | N | N |
| `GeometryReplayParity` | R | N | N | R | N | N |
| `GeometryRecoveryAction` | D | D | D | D | R | R |
| `GeometryProjectionConsumption` | R | R | R | N | N | N |

### Runtime Integration Matrix

| Family | LRRoute | Recovery | MutEv | Projection | Signal |
|---|---:|---:|---:|---:|---:|
| `GeometryTargetIdentity` | R | D | N | N | D |
| `SpatialAnchorSelection` | R | D | N | N | D |
| `PrimitiveBinding` | R | R | R | N | D |
| `PrimitiveRebinding` | R | R | R | N | R |
| `TopologyNeighborhoodReplacement` | R | R | R | N | D |
| `ToleranceAndPrecisionCertification` | R | R | N | N | D |
| `HistoricalGeometryInspection` | R | D | N | N | D |
| `BranchLocalGeometryInspection` | R | D | N | N | D |
| `GeometryReplayParity` | N | D | N | N | D |
| `GeometryRecoveryAction` | R | N | R | N | D |
| `GeometryProjectionConsumption` | R | D | N | R | R |

This matrix is mandatory.

Implementers do not get to guess applicability from examples or from existing
partial code shape.

Retained-view family note:

- `HistoricalGeometryInspection`, `BranchLocalGeometryInspection`, and
  `GeometryReplayParity` use the full workflow spine on their own retained-view
  declaration entry
- they do not replay the source family's entire execution pipeline merely
  because the source family emitted the retained artifacts they consume

Grouped workflow note:

- grouped neighborhood authoring belongs to declaration structure where the
  family's semantic input is truly neighborhood-shaped
- grouped products or grouped result packaging are not substitutes for grouped
  declaration workflow
- contribution composition is the policy-bearing companion to grouped
  declaration authoring, not a generic payload bag

## Mandatory Typed Fact Receipt Schemas

These receipt schemas are mandatory design artifacts for the rewrite.

## Typed Fact Receipt Attachment Contract

Typed fact receipts attach through one consistent geometry receipt mechanism:

- each fact-emitting family owns a family-specific typed fact receipt type
- the family-specific typed fact receipt is attached to the canonical Query
  envelope through a family-owned geometry fact sidecar
- the same fact receipt type is reused in retained next-step and retained
  inspection artifacts where that family supports retention
- projection consumers read from the receipt-backed fact sidecar, not from local
  semantic summaries

This means:

- no family invents its own ad hoc fact transport packet
- no kernel-local wrapper becomes required to discover rich geometry semantics
- the attachment point is the family envelope sidecar and its retained
  descendants, not an unrelated auxiliary report object

The implementation pattern should copy the strongest retained-envelope and
receipt discipline already present in `worth-topo`, not invent a parallel
geometry-only transport convention.

### `GeometryTargetIdentityFactReceipt`

- Required fields:
  - `target_identity`
  - `target_kind`
  - `source_authority`
  - `declaration_digest`
  - `fact_digest`
- Optional fields:
  - `alias_identities`
- Replaces:
  - ad hoc target identity digests
  - local target identity summary objects
- Attaches to:
  - envelope sidecar
  - retained next-step artifact when retained

### `SpatialAnchorSelectionFactReceipt`

- Required fields:
  - `anchor_identity`
  - `anchor_kind`
  - `anchor_site_identity`
  - `target_identity`
  - `fact_digest`
- Optional fields:
  - `frame_basis`
  - `witness_class`
- Replaces:
  - local anchor identity summaries
  - frame-specific ad hoc anchor packaging
- Attaches to:
  - envelope sidecar
  - retained inspection subject when retained

### `PrimitiveBindingFactReceipt`

- Required fields:
  - `binding_identity`
  - `binding_kind`
  - `target_identity`
  - `anchor_identity`
  - `binding_posture`
  - `fact_digest`
- Optional fields:
  - `witness_class`
  - `tolerance_certificate_ref`
- Replaces:
  - local binding digest carriers
  - non-receipt binding explanation summaries
- Attaches to:
  - envelope sidecar
  - mutation evidence rows
  - retained next-step artifact when retained

### `PrimitiveRebindingFactReceipt`

- Required fields:
  - `prior_binding_identity`
  - `prior_site_identity`
  - `selected_candidate_identity`
  - `selected_candidate_site_identity`
  - `continuity_class`
  - `correspondence_class`
  - `decision_class`
  - `neighborhood_identity`
  - `fact_digest`
- Optional fields:
  - `candidate_frontier`
  - `unsupported_reason`
  - `tolerance_certificate_ref`
  - `motion_posture`
- Replaces:
  - `AdmittedRebindingDecision` as ordinary runtime transport
  - kernel-local rebinding outcome remapping
  - branch-local and replay digest derivation from local semantic replay
- Attaches to:
  - envelope sidecar
  - retained next-step artifact
  - retained inspection subject

### `TopologyNeighborhoodReplacementFactReceipt`

- Required fields:
  - `replacement_neighborhood_identity`
  - `replacement_scope`
  - `affected_target_identities`
  - `existing_target_identity_basis`
  - `fact_digest`
- Optional fields:
  - `structural_correspondence_frontier`
- Replaces:
  - local neighborhood replacement payload summaries
- Attaches to:
  - envelope sidecar
  - mutation evidence rows

### `ToleranceAndPrecisionCertificationFactReceipt`

- Required fields:
  - `certificate_kind`
  - `precision_policy_identity`
  - `tolerance_basis`
  - `certified_bound`
  - `certification_posture`
  - `fact_digest`
- Optional fields:
  - `escalation_trace`
  - `unsupported_reason`
- Replaces:
  - local tolerance explanation payloads
  - non-retained precision certification summaries
- Attaches to:
  - envelope sidecar
  - retained certification artifact

### `HistoricalGeometryInspectionFactReceipt`

- Required fields:
  - `source_family`
  - `retained_subject_identity`
  - `retained_basis_digest`
  - `inspected_fact_digest`
  - `historical_view_digest`
- Optional fields:
  - `truncation_reason`
- Replaces:
  - local historical interpretation summaries
- Attaches to:
  - retained inspection result

### `BranchLocalGeometryInspectionFactReceipt`

- Required fields:
  - `source_family`
  - `branch_basis_digest`
  - `branch_binding_digest`
  - `inspected_fact_digest`
  - `branch_local_view_digest`
- Optional fields:
  - `branch_divergence_marker`
- Replaces:
  - local branch-local digest derivation packets
- Attaches to:
  - retained inspection result

### `GeometryReplayParityFactReceipt`

- Required fields:
  - `left_source_family`
  - `right_source_family`
  - `left_fact_digest`
  - `right_fact_digest`
  - `parity_class`
  - `fact_digest`
- Optional fields:
  - `mismatch_reason`
- Replaces:
  - digest-only replay summaries
  - kernel-local replay parity bags
- Attaches to:
  - replay parity artifact

### `GeometryRecoveryActionFactReceipt`

- Required fields:
  - `recovery_action_kind`
  - `source_posture`
  - `source_family`
  - `recovery_target_scope`
  - `fact_digest`
- Optional fields:
  - `resulting_binding_identity`
  - `resulting_target_identity`
- Replaces:
  - denial-summary-only recovery glue
- Attaches to:
  - recovery envelope sidecar
  - mutation evidence when mutating

### `GeometryProjectionConsumptionReceipt`

- Required fields:
  - `projected_fact_kind`
  - `source_family`
  - `source_receipt_digest`
  - `projection_contract_identity`
  - `projection_digest`
- Optional fields:
  - `materialization_basis_digest`
- Replaces:
  - local projection parity reports as ordinary product surfaces
- Attaches to:
  - projection consumption envelope

## Current To Target Migration Map

| Current surface | Target surface | Owner after rewrite | Fate |
|---|---|---|---|
| `worth-kernel::binding::authoring::query_domain::PrimitiveBindingQueryDomain` | spatial identity and binding domain | `worth-spatial` | move semantic ownership out of kernel |
| `worth-kernel::binding::anchoring::query_domain::PrimitiveAnchorBindingQueryDomain` | spatial identity and anchor domain | `worth-spatial` | move semantic ownership out of kernel |
| `worth-spatial::spatial_intent::arbitration::declaration` | spatial Query domains + declaration families | `worth-spatial` | keep as declaration-owned semantic surface; old `declared_analysis` path must stay deleted |
| `worth-spatial::spatial_intent::lowering::lowered_intents::operation_plan` | family-native declaration inputs | `worth-spatial` | keep as lowering-owned operation plan; old `runtime_declaration` path must stay deleted |
| `worth-kernel::binding::rebinding::PrimitiveRebindingQueryDomain` | spatial continuity and rebinding domain | `worth-spatial` | move semantic ownership out of kernel |
| `PrimitiveRebindingDeclarationEntry::admit(...)` | `PrimitiveRebindingFactReceipt` carried by Query workflow | `worth-spatial` + `forge-query` artifact path | delete from ordinary production path; retain only in test parity harnesses if still needed |
| `binding/rebinding/workflow_transport.rs` | canonical workflow artifact + fact receipt transport | `forge-query` artifact path + spatial family payload | delete |
| `binding/rebinding/historical_inspection.rs` local semantic replay | retained historical fact receipts | retained-view family ownership | rewrite |
| `binding/rebinding/branch_local_inspection.rs` local semantic replay | retained branch-local fact receipts | retained-view family ownership | rewrite |
| `binding/rebinding/replay_parity.rs` local semantic replay | retained replay parity fact receipts | retained-view family ownership | rewrite |
| `binding/workflow_boundary/canonical_artifacts.rs` | keep as thin canonical artifact access layer only if it carries zero independent semantic meaning | `worth-kernel` DX only | trim aggressively or move semantics out |
| `binding/workflow_boundary/*summary*` | thin read-only views over canonical artifacts | `worth-kernel` DX only | trim or delete |
| `construction/authoring.rs` session shape | kernel DX facade over topo construction + spatial families | `worth-kernel` | thin wrapper only |
| `construction/runtime_proof/query/boundary_gap_register.rs` | certification/doc-only artifact | `worth-kernel` certification | remove from ordinary production path |
| `worth-topo::construction::query_native_boundary` | reference construction pattern | `worth-topo` | keep and extend as authoritative pattern |

## Phase Dependency Edges

Although the phases below are kept in conceptual order, their implementation
dependencies are strict:

1. `Construction Track Lock`, `Family Relationship Lock`, the applicability
   matrix, and the typed fact receipt schemas are prerequisites for execution.
2. Phase 2 must land before Phase 3 can be closed, because domain homes must
   exist before family inventory migration is honest.
3. Phase 3 must land before Phase 4 can be closed, because artifact transport
   needs actual family contracts.
4. Phase 4 must land before Phase 1 can be considered complete in production,
   because local remapping cannot die until receipt schemas and artifact
   transport exist.
5. Phase 1 and Phase 4 together must land before Phase 5 can be honest,
   because retained history cannot consume local semantic replay.
6. Phase 6 and Phase 7 depend on the family and artifact structure from Phases
   3 and 4.
7. Phase 8 depends on Phases 1 through 7, because kernel cannot collapse until
   the real runtime story exists below it.
8. Phase 9 closes only after all prior phases land and legacy path deletion is
   mechanically enforced.

## Execution Order

> **Live slice and gates:** see
> [worth-geometry-query-native-ACTIVE.md](worth-geometry-query-native-ACTIVE.md).

The phase numbers below describe conceptual ownership and closure.

They are **not** the order an implementation agent should start editing files.

The required implementation order is:

### Pass A: Domain Homes And Family Ownership

Includes:

- Phase 2 domain-home work
- Phase 3 family inventory and ownership work
- `Family Relationship Lock`
- `Construction Track Lock`

Goal:

- move geometry family ownership out of kernel-owned semantic domains
- create the target spatial domain homes
- lock family roles before transport work begins

### Pass B: Workflow Transport And Typed Fact Receipts

Includes:

- Phase 4 artifact transport work
- mandatory typed fact receipt schemas
- receipt attachment mechanics

Goal:

- make canonical Query artifacts capable of carrying the geometry meaning now
  being recovered locally

### Pass C: Delete The Dual Runtime Binding Story

Includes:

- Phase 1 production deletion work for binding and rebinding

Goal:

- delete kernel-local semantic replay only after Pass B has created the
  replacement transport

### Pass D: Retained-View Migration

Includes:

- Phase 5 historical, branch-local, and replay migration

Goal:

- make retained Query artifacts the only retained geometry truth source

### Pass E: Neighborhood, Routing, Recovery, And Projection Closure

Includes:

- Phase 6 grouped neighborhood and contribution routing work
- Phase 7 recovery, projection consumption, and mutation evidence work

Goal:

- complete the geometry runtime narrative around retained and mutating families

### Pass F: Kernel Collapse And Certification Closeout

Includes:

- Phase 8 kernel collapse
- Phase 9 certification closeout

Goal:

- remove kernel pseudo-runtime leftovers
- prove only one Query-native geometry runtime story survives

No implementation agent should start with Pass C just because it is called
"Phase 1" conceptually.

## Sequencing Notes

This rewrite is the foundation for later geometry scaling. It is not the place
to preserve ambiguous intermediate architecture.

Later work should be able to assume:

- geometry identity, continuity, history, branch-local inspection, replay, and
  projection all already live on one canonical Query artifact story
- downstream projection consumers can read receipt-backed geometry truth without
  bespoke kernel glue
- future recovery growth can extend real declaration families instead of
  inventing special-case denial handlers
- future construction DX can compose topo-native construction with spatial
  families without reviving kernel-owned semantic construction domains

This rewrite must still not admit:

- local semantic replay as a convenience fallback
- ad hoc non-Query geometry authoring paths
- fake continuation or signal support where the family does not honestly own it
- branch labels or replay claims that bypass basis lifecycle admission
- certification artifacts that become ordinary runtime transport

## Mechanical Gates Per Pass

These gates are intentionally grep-able and binary where possible.

### Pass A gates

- spatial hosts the new domain markers for the migrated geometry families
- kernel is no longer the semantic owner of migrated rebinding domains
- family-role classification is present:
  - base declaration family
  - retained-view declaration family
  - artifact class
- construction remains locked to topo-native substrate and no new kernel-owned
  semantic construction domain exists

### Pass B gates

- typed fact receipt Rust homes are defined for the migrated families
- receipt attachment mechanics are defined and used consistently
- canonical workflow artifacts can carry migrated geometry semantic payloads
- no new local summary bag is introduced as a substitute transport

### Pass C gates

- `workflow_transport.rs` is deleted
- production binding paths contain zero local post-progression semantic replay
  calls
- grep for production `.admit()` calls after Query progression in migrated
  binding paths returns zero intentional matches
- ordinary outcomes are no longer remapped from local semantic replay

### Pass D gates

- historical inspection derives from retained Query geometry receipts only
- branch-local inspection derives from retained Query geometry receipts plus
  admitted basis evidence only
- replay parity derives from retained Query geometry receipts only
- no retained-view production path requires local semantic replay to become
  meaningful

### Pass E gates

- grouped neighborhood posture exists for all relevant families
- contribution-composed policy exists for all policy-bearing families
- lower-runtime routing posture is explicit per family
- admitted recovery families are real declaration families
- projection consumption is receipt-backed for admitted downstream geometry fact
  consumers

### Pass F gates

- kernel production facades are DX-only over real Query-native geometry
  families
- legacy non-Query geometry entrypoints are absent from the allowed public API
- compile-fail absence tests for forbidden legacy imports and entrypoints pass
- certification bundles prove one surviving Query-native geometry runtime story

## Test Contract Lock

Every phase in this spec must name test shapes, file placement, and proof
ownership explicitly.

The default categories are:

- **Compile-fail**
  - public legacy entrypoint absence
  - forbidden deep imports
  - forbidden local semantic replay APIs in production paths
- **Integration**
  - family-level readiness, workflow, history, branch-local, replay, and
    recovery behavior
- **Certification bundle**
  - cross-family parity, determinism, and deletion proof

Baseline seeds from the current codebase already exist in:

- `crates/worth-kernel/src/binding/tests/inspection/workflow_boundary.rs`
- `crates/worth-kernel/src/binding/tests/inspection/historical_inspection.rs`
- `crates/worth-kernel/src/binding/tests/inspection/branch_local_inspection.rs`
- `crates/worth-kernel/src/binding/tests/inspection/replay_parity.rs`
- `crates/worth-kernel/src/binding/tests/inspection/binding_layer_closeout_details.rs`
- `crates/worth-kernel/src/binding/tests/inspection/binding_layer_closeout_mismatches.rs`

Phase 9 should treat those as migration seeds, not as the final file topology.

Missing target suites that should be introduced explicitly include:

- compile-fail legacy geometry entrypoint absence
- compile-fail forbidden kernel-local semantic replay imports
- family applicability certification for every admitted geometry family
- typed fact receipt schema certification for every fact-emitting family
- construction facade conformance to the topo-native pattern

## Certification Placement Lock

Certification code must not remain ambiguously half-production and half-test.

The rewrite locks this placement rule:

- compile-fail absence and surface-enforcement tests live in certification/test
  harnesses
- hostile parity bundles and closeout bundles live under explicit
  certification-owned module trees
- proof reports, gap registers, and siege artifacts do not remain ordinary
  production DX surfaces
- `runtime_proof/` content should either:
  - move under explicit certification ownership, or
  - be deleted if it only explained a transitional rewrite state

The default target is:

- production runtime shape in ordinary family/domain modules
- certification shape in explicit certification modules
- absence enforcement in compile-fail or API inventory suites


## Phase 1: Delete The Dual Runtime Story

Freeze the rule that geometry truth may not exist in both Query workflow
artifacts and post-Query local semantic replay.

### Relevant subsystems

- `worth-kernel`
- `worth-spatial`
- `forge-query`

### Query API Contract

- `PrimitiveRebindingDeclarationEntry::admit(...)`
- `primitive_rebinding_workflow_transport(...)`
- `primitive_rebinding_branch_local_inspection(...)`
- `ForgeQueryOrdinaryOutcome`
- `ForgeQueryDeclarationEnvelope`
- `ForgeQueryDeclarationEntryInspection`

### Warnings

- Do not wrap old local semantic calls in "Query-aware" adapters.
- Do not keep both old and new ordinary-outcome stories alive while migrating.
- Do not preserve legacy names for dual-path code once one path is declared
  illegal.

### Engineering decisions

- Any geometry meaning currently recovered by local `admit()` after Query
  progression must be re-expressed as Query-native semantic artifact content.
- The canonical meaning source for ordinary geometry workflow must become the
  Query family artifact set, not a local semantic helper that happens to agree.
- This phase is allowed to break call sites aggressively to make the wrong path
  uncompilable.

### Test requirements

1. Add an adversarial compile-fail or structural certification test proving that
   the old local semantic replay path cannot be used from ordinary production
   geometry workflow.
2. Add an adversarial parity test proving that the surviving Query-native
   ordinary outcome still preserves the same admitted, denied, ambiguous, and
   unsupported geometry meanings without any local remapping step.

### Resolved decisions

- Ordinary outcomes stay slim and workflow-oriented. They carry immediate
  posture and next-step truth only.
- Rich geometry meaning moves into adjacent typed fact receipts attached to the
  same canonical workflow story.
- Kernel-local summary types may remain only as thin read-only formatting views
  derived mechanically from canonical Query artifacts.
- Any summary type that carries independent semantic digest, reclassifies
  posture, or becomes required to interpret workflow truth must be deleted.

## Phase 2: Promote Spatial Authority Into Real Query Domains

Freeze `worth-spatial` as a Query-native domain owner instead of a semantic
preprocessor that lowers into intent admission.

### Relevant subsystems

- `worth-spatial`
- `forge-query`

### Query API Contract

- public geometry facade support-posture surfaces
- `ForgeQueryDomainEntryMarker`
- `ForgeQueryDomainOperatingContext`
- `ForgeQueryAdmittedConfiguredDomainHandle`
- `ForgeQueryDeclarationFamilyMarker`
- `ForgeQueryDeclarationInput`
- current spatial intent lowering and arbitration surfaces

### Warnings

- Do not keep intent-handoff objects as ordinary runtime entrypoints "for now."
- Do not put new declaration-family code beside old intent-handoff code under
  the same names.
- Do not let spatial remain both a semantic authority and a pre-Query lowering
  service in parallel.

### Engineering decisions

- Introduce one or more real spatial Query domains with explicit operating
  contexts rather than relying on kernel-owned domains for ordinary geometry
  work.
- Re-express spatial semantic responsibilities as declaration families rather
  than as `ForgeQueryIntentDeclaration` payload production.
- Keep semantic authority in spatial, but make the authority's ordinary runtime
  shape Query-native from the point of domain entry forward.
- Every admitted geometry family must acquire a public support row and a public
  facade entry shape at the same time it acquires a domain-entry and
  configured-handle story. Visibility without support posture is not admission.

### Test requirements

1. Add an adversarial domain-entry proof showing that the admitted spatial Query
   handle is now the only ordinary way to enter a geometry family.
2. Add an adversarial denial test proving that old intent-handoff production
   entrypoints are either removed or fail closed with no surviving
   compatibility path.
3. Add an adversarial support-posture test proving that the public geometry
   facade does not imply family admission unless the family's support row says
   it is admitted.

### Resolved decisions

- `worth-spatial` should use a small number of responsibility-separated Query
  domains rather than one giant geometry bucket.
- The default split is:
  - identity domain
  - continuity and rebinding domain
  - certification and retained semantic fact domain
- Pure semantic authority surfaces may remain internal or facade-visible where
  they do not define runtime shape.
- Intent-handoff, runtime declaration packaging, and query-handoff-specific
  facades are runtime-shape leaks and should disappear.
- Public geometry facade vocabulary and support posture must land together.
  No family should be "public now, admitted later" through guesswork.

## Phase 3: Replace Intent Handoff With Declaration Families

Freeze the new geometry family inventory as the ordinary runtime grammar.

### Relevant subsystems

- `worth-spatial`
- `worth-kernel`
- `forge-query`

### Query API Contract

- declaration family markers
- canonical declaration entries
- aspect contracts
- legality contracts
- route contracts
- grouped posture
- signal posture

### Warnings

- Do not create "bridge" family names that preserve legacy intent vocabulary at
  the cost of runtime honesty.
- Do not use one generic geometry family with mode flags where distinct
  responsibilities need distinct ownership and proof boundaries.
- Do not let family names hide whether a surface is identity, execution,
  inspection, replay, or recovery.

### Engineering decisions

- Adopt the family inventory in this spec as the public geometry runtime shape.
- Encode canonical identity and semantic aspects at the family level instead of
  reconstructing them downstream.
- Make grouped posture, signal posture, and route posture explicit per family,
  even when the answer is a typed "not admitted."
- `PrimitiveRebinding` is not permanently signal-denied. It must own explicit
  signal compatibility and continuation posture wherever rebinding results need
  downstream invalidation, recomputation, preview continuity, or
  identity-preserving subscription continuity.
- Signal posture must not stop at a boolean compatibility row. Where a family is
  signal-capable, the spec requires an honest continuation story from envelope
  truth into prepared continuation or signal-facing execution artifacts.

### Test requirements

1. Add an adversarial family-certification test proving each admitted geometry
   family has canonical entries, explicit route and legality contracts, and a
   non-legacy public name.
2. Add an adversarial rejection test proving no single catch-all "geometry
   intent" family survives as a loophole for future semantic collapse.

### Resolved decisions

- Public immediately:
  - `GeometryTargetIdentity`
  - `SpatialAnchorSelection`
  - `PrimitiveBinding`
  - `PrimitiveRebinding`
  - `HistoricalGeometryInspection`
  - `BranchLocalGeometryInspection`
  - `GeometryReplayParity`
  - `GeometryRecoveryAction`
- Internal first, then promote when stable:
  - `TopologyNeighborhoodReplacement`
  - `ToleranceAndPrecisionCertification`
  - `GeometryProjectionConsumption`
- Decompose by modules first, not crates.
- `GeometryTypedFactReceipt` is an artifact class and should be introduced with
  the earliest fact-emitting families rather than treated as a separate public
  declaration family rollout.
- New crates are justified only when authority, dependency direction, or public
  API stability truly diverges.

## Phase 4: Make Workflow Artifacts The Only Geometry Transport

Freeze readiness, progression, route plan, receipt, envelope, ordinary outcome,
and typed fact delivery as the only allowed transport for ordinary geometry
meaning.

### Relevant subsystems

- `worth-spatial`
- `worth-kernel`
- `forge-query`

### Query API Contract

- `ForgeQueryDeclarationEntryReadinessReport`
- `ForgeQueryAdmittedDeclarationProgression`
- route plan surfaces
- receipt surfaces
- envelope surfaces
- ordinary outcomes
- typed fact or projection-consumption receipts

### Warnings

- Do not keep local semantic remapping after envelopes are built.
- Do not allow "helpful" local DTOs to become required for understanding
  receipts or envelopes.
- Do not leave route/receipt/envelope truth as internal-only while external code
  still depends on parallel local objects.

### Engineering decisions

- Promote whichever geometry semantics are currently missing from Query workflow
  artifacts into those artifacts rather than repairing them downstream.
- Make readiness, route, receipt, and envelope public runtime facts for geometry
  families, not optional debug details.
- Distinguish ordinary outcomes from typed fact receipts cleanly, but let both
  be canonical Query-owned transport rather than local domain summaries.

### Test requirements

1. Add an adversarial artifact-consumption test proving a consumer can derive
   all required geometry meaning from Query workflow artifacts alone, with no
   access to local semantic helper calls.
2. Add an adversarial denial-path test proving denied, stale, rebind-required,
   ambiguous, and unsupported geometry workflows remain fully interpretable from
   canonical Query artifacts alone.

### Resolved decisions

- Ordinary outcomes carry:
  - admitted vs denied vs deferred vs stale vs rebind-required vs ambiguous vs
    unsupported posture
  - immediate next-step posture
- Typed fact receipts carry:
  - prior identity
  - prior site identity
  - selected candidate identity
  - selected candidate site identity
  - continuity and correspondence truth
  - unsupported reason
  - ambiguity frontier and candidate set
  - tolerance and precision certification
  - witness and motion posture
- Workflow retention should use one geometry-wide retained artifact contract
  layer with family-owned semantic payloads.

## Phase 5: Move History, Branch, And Replay Onto Retained Query Truth

Freeze retained Query artifacts as the only authority for historical inspection,
branch-local inspection, and replay parity.

### Relevant subsystems

- `worth-spatial`
- `worth-kernel`
- `worth-topo`
- `forge-query`

### Query API Contract

- retained inspection subject inputs
- `ScopedInspectionBasis`
- `LowerRuntimeBasisEvidence`
- retained next-step artifacts
- replay parity surfaces
- historical materialization surfaces

### Warnings

- Do not keep local semantic replay as a "temporary retained interpretation"
  tool.
- Do not let historical, branch-local, and replay become mode flags on one
  generic helper surface.
- Do not accept raw branch or snapshot identifiers where admitted basis
  artifacts should exist.

### Engineering decisions

- Historical, branch-local, and replay responsibilities remain distinct, but all
  must consume retained Query geometry truth that is already semantically
  complete.
- Basis lifecycle becomes mandatory for every geometry family that supports
  historical, preview, or branch-local work.
- Replay parity compares canonical retained next-step truth, not local summary
  digests and not live-state fallback.

### Test requirements

1. Add an adversarial retained-truth test proving historical and branch-local
   geometry inspection stay correct after live/current geometry diverges.
2. Add an adversarial replay-honesty test proving equivalent retained
   geometries replay identically and semantically different retained artifacts
   fail loudly without local fallback.

### Resolved decisions

- `forge-query` owns retained artifact lifecycle, basis admission and scoping,
  historical materialization primitives, and generic retained next-step
  transport skeletons.
- Geometry families own retained semantic payloads, replay equivalence meaning,
  and geometry-specific replay mismatch semantics.
- Ordinary runtime keeps:
  - replay family entrypoints
  - replay artifacts
  - typed replay mismatch results
- Proof bundles, hostile comparison matrices, and parity siege reports remain
  certification-only surfaces.

## Phase 6: Promote Neighborhoods, Contributions, And Runtime Routing

Freeze grouped-neighborhood semantics, contribution-composed policy, and
lower-runtime routing as ordinary geometry workflow structure.

### Relevant subsystems

- `worth-spatial`
- `worth-topo`
- `worth-kernel`
- `forge-query`

### Query API Contract

- `ForgeQueryGroupedDeclarationInput`
- `ForgeQueryDeclarationSupportsNeighborhoodGrouping`
- `ForgeQueryContributionComposedOrchestrationInput`
- lower-runtime capability routing surfaces
- authoritative mutation evidence support

### Warnings

- Do not keep neighborhood as a payload field where grouping is the real runtime
  structure.
- Do not keep policy ambient if contribution composition is the honest shape.
- Do not hide lower-runtime routing in certification-only or proof-only layers.

### Engineering decisions

- Make local topology replacement, neighborhood rebinding, and related geometry
  locality shapes real grouped Query workflow.
- Promote tolerance, fallback, naming, preview, and continuity strictness into
  contribution-composed orchestration rather than local booleans or ambient
  configuration.
- Declare lower-runtime routes per family so geometry execution honestly states
  whether it uses topology read, topology write, historical materialization,
  branch preview, or projection consumption.
- When grouped geometry work is admitted, the grouped shape must be the
  declaration-authoring shape itself. It must not be simulated by scalar
  declarations plus grouped result packaging after the fact.

### Test requirements

1. Add an adversarial grouped-neighborhood test proving two geometry operations
   with the same semantic neighborhood lower to the same grouped Query shape
   regardless of caller packaging.
2. Add an adversarial routing test proving every admitted geometry family
   exposes its lower-runtime route posture explicitly and fails closed when the
   needed route is not admitted.

### Resolved decisions

- Reuse directly from topology:
  - local-neighborhood grouping mechanics
  - grouped declaration scaffolding
  - contribution-composed orchestration mechanics
  - lower-runtime locality and route plumbing
- Keep geometry-owned:
  - neighborhood identity semantics
  - rebinding neighborhood law
  - carrier/support region semantics
  - tolerance neighborhood semantics
  - curved correspondence frontier semantics
- Authoritative mutation evidence should be emitted at the geometry family
  layer over a shared evidence skeleton, not flattened into one generic
  geometry-wide mutation report.

## Phase 7: Promote Recovery, Projection Consumption, And Mutation Evidence

Freeze geometry denial handling and fact delivery as first-class Query-native
families.

### Relevant subsystems

- `worth-spatial`
- `worth-topo`
- `worth-kernel`
- `forge-query`

### Query API Contract

- recovery boundary surfaces
- authoritative mutation evidence
- projection consumption receipts
- typed fact delivery surfaces

### Warnings

- Do not let recovery remain explanation-heavy and action-light.
- Do not keep projection consumption as topology-only infrastructure while
  geometry still emits local diagnostics and ad hoc facts.
- Do not add report layers that summarize mutation evidence without carrying the
  underlying canonical artifacts.

### Engineering decisions

- Promote geometry recovery actions into real declaration families such as basis
  correction, neighborhood widening, tolerance escalation, and ambiguity
  narrowing.
- Make projection consumption the ordinary way to publish geometry facts to
  downstream consumers such as previews, diagnostics, and later analysis lanes.
- Require geometry mutation evidence to preserve target identity, prior truth
  identity, route evidence, and resulting typed geometry fact posture.
- Where `PrimitiveRebinding` is signal-capable, this phase must also close the
  envelope-to-signal-or-continuation story explicitly:
  - signal compatibility review from admitted rebinding truth
  - prepared continuation or denial artifacts from rebinding envelopes
  - identity-preserving continuation semantics for downstream preview or
    subscription consumers

### Test requirements

1. Add an adversarial recovery test proving denied geometry workflows expose
   typed next-step actions rather than only denial explanations.
2. Add an adversarial fact-delivery test proving downstream geometry consumers
   can subscribe to receipt-backed typed facts without querying local kernel or
   spatial summary surfaces.

### Resolved decisions

- Standardize at the Query layer only generic fact categories such as:
  - fact identity
  - provenance
  - authority posture
  - retained basis linkage
  - route and evidence linkage
  - read vs mutating fact posture
- Keep geometry semantics above Query, including:
  - continuity and correspondence classes
  - selected candidate identity
  - unsupported geometry reason
  - tolerance certificate
  - anchor and witness classification
  - neighborhood and motion posture
- Admit early:
  - ambiguity narrowing
  - branch basis correction
  - rebind-context widening
  - support-check recovery
  - canonical local neighborhood correction
- Keep typed but fail-closed:
  - aggressive tolerance escalation actions
  - cross-family recovery synthesis
  - heuristic recovery paths that are not replay-safe

## Phase 8: Collapse Kernel Back To Composition And DX

Freeze `worth-kernel` as the composition, DX, and certification layer over real
geometry Query families and delete its remaining pseudo-runtime behavior.

### Relevant subsystems

- `worth-kernel`
- `worth-spatial`
- `forge-query`

### Query API Contract

- current kernel binding workflow boundaries
- current kernel construction authoring and runtime proof surfaces
- geometry family facades introduced by earlier phases

### Warnings

- Do not keep kernel-local semantic replay behind helper or workflow-boundary
  names.
- Do not preserve old construction session shapes if the real runtime role is a
  geometry Query domain/family facade.
- Do not keep certification-only abstractions in production just because they
  are already convenient.

### Engineering decisions

- Kernel should expose product-facing authoring and inspection DX over real
  geometry Query families, not over local semantic service objects.
- Kernel proof surfaces should remain, but only as certification over the real
  runtime, not as a substitute transport for runtime meaning.
- Any kernel-local artifact that cannot be justified as a thin read-only view or
  certification bundle over canonical Query artifacts must be deleted.

### Test requirements

1. Add an adversarial kernel-minimization test proving that the surviving kernel
   production path can no longer produce geometry meaning without admitted Query
   family workflow.
2. Add an adversarial API-topology test proving the next correct edit for a new
   geometry capability is to add or extend a Query family, not to add another
   kernel-local workflow helper.

### Resolved decisions

- Move to certification-only or documentation-only:
  - gap registers
  - authority chain reports
  - preview and basis parity reports
  - closeout evidence aggregation reports
  - hostile replay and parity siege artifacts
- Production kernel surfaces should remain only where they provide DX over real
  Query-native geometry families.
- Kernel should expose multiple grouped facades, not one god facade.
- The default grouping is:
  - identity
  - binding
  - inspection
  - replay
  - recovery
  - projection

## Phase 9: Certification Closeout For A Single Query-Native Geometry Runtime

Freeze one hostile certification bundle that proves the geometry stack now has
one runtime story rather than three partially overlapping ones.

### Relevant subsystems

- `worth-kernel`
- `worth-spatial`
- `worth-topo`
- `forge-query`

### Query API Contract

- certification bundles
- retained inspection and replay surfaces
- authoritative mutation evidence
- projection consumption facts
- grouped neighborhood and recovery families

### Warnings

- Do not certify behavior while tolerating production dual paths.
- Do not let final closeout aggregate summaries instead of certifying named
  runtime properties.
- Do not leave deleted-path compatibility code in place and then certify only
  the new path.

### Engineering decisions

- Certification must prove that live, historical, branch-local, replayed, and
  projection-consumed geometry meanings cohere through one Query-native runtime
  artifact story.
- Certification must explicitly include admitted and denied geometry paths.
- Certification must prove the absence of legacy semantic transport, not just
  the success of the new path.

### Test requirements

1. Add an adversarial end-to-end parity bundle proving that live, historical,
   branch-local, and replayed geometry meanings agree across equivalent retained
   histories and diverge loudly when they should.
2. Add an adversarial deletion proof showing that removed legacy entrypoints,
   wrappers, and summary carriers cannot be reintroduced without compile or
   certification failure.

### Resolved decisions

- `worth-kernel` owns the cross-family geometry coherence bundle that proves the
  geometry stack now tells one runtime story.
- `worth-spatial` owns family semantic correctness bundles.
- `worth-topo` owns lower-runtime routing, materialization, and projection
  substrate correctness bundles.
- `forge-query` owns generic lifecycle, basis, history, and retained transport
  correctness bundles.
- There should be both:
  - compile-fail tests for forbidden legacy geometry entrypoints and import
    paths
  - API inventory certification that names the allowed geometry runtime
    entrypoints and fails if legacy ones reappear

## Acceptance Matrix

The rewrite is only accepted when every row below is closed.

| Surface | Required closeout |
|---|---|
| Public entry | Ordinary geometry work enters through Query-native geometry families only |
| Spatial runtime shape | `worth-spatial` no longer uses intent handoff as ordinary runtime shape |
| Workflow transport | geometry meaning is carried only by Query workflow artifacts |
| Ordinary outcomes | no kernel-local ordinary outcome remapping survives |
| Retained history | historical, branch-local, and replay paths consume retained Query truth only |
| Basis lifecycle | no raw branch/snapshot identifiers remain on ordinary geometry APIs |
| Grouped locality | every neighborhood-bearing geometry family classifies grouped workflow honestly as `Required Now`, `Not Applicable`, or `Denied For This Runtime`; all families where it is relevant use grouped Query workflow |
| Contributions | every policy-bearing geometry family classifies contribution composition honestly; all families where policy is a runtime input use contribution-composed workflow instead of ambient settings |
| Routing | every geometry family declares lower-runtime route posture honestly, including `Not Applicable` where no lower-runtime route exists |
| Recovery | every denial-bearing geometry family classifies recovery honestly; denied geometry workflows that require a next-step lane expose typed recovery families rather than summary-only denials |
| Mutation evidence | every mutating geometry family classifies mutation evidence honestly; all admitted geometry writes preserve authoritative mutation evidence as canonical artifacts |
| Projection consumption | every fact-publishing geometry family classifies projection consumption honestly; downstream geometry facts that are admitted are receipt-backed and Query-native |
| Signal and continuation | every geometry family classifies signal or continuation posture honestly instead of inheriting silent defaults |
| Kernel role | kernel is composition, DX, and certification only |
| Legacy deletion | old non-Query geometry entrypoints and shims are removed, not deprecated in place |

## Milestone Done When

This rewrite is done only when all of the following are true:

1. No ordinary production geometry path calls local spatial authority after
   Query progression to recover meaning.
2. `worth-spatial` hosts real geometry Query domains and declaration families.
3. The geometry family inventory in this spec exists with honest names and
   explicit contracts.
4. Historical, branch-local, and replay surfaces are retained-artifact-native
   and require no local semantic replay.
5. Grouped neighborhood work, contribution composition, lower-runtime routing,
   recovery, and projection consumption are all part of the ordinary geometry
   runtime story where relevant, and explicitly `Not Applicable` or denied where
   they are not relevant.
6. `worth-kernel` no longer acts as a second semantic runtime.
7. The certification suite proves there is one surviving Query-native geometry
   runtime story and that the old one is gone.
8. There are no pseudo-API shims, legacy dual paths, or shadow semantic
   carriers left in production code.
