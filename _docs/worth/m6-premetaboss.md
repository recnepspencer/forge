# Milestone 6 Pre-MetaBoss Test Plan

> **Status:** Draft test-design note.
>
> **Purpose:** define the MetaBoss-equivalent proof families for Milestone 6
> without turning Milestone 6 into boolean implementation.

## Scope

Milestone 6 proves boolean-readiness, not boolean execution.

These tests stop before split/classify/assemble. They must prove that exact
planar predicate authority, planar structural identity, retained planar facts,
projection-consumed planar facts, recovery posture, movement/rotation posture,
and clean-fail diagnostics survive the same hostile planar conditions that later
booleans will consume.

The certified predicate machinery already exists in `worth-math`: Shewchuk
adaptive predicates, `CertifiedTriSign`, `PrecisionEscalation`,
`PrecisionMode`, and exact-rational budget tracking. These tests must prove
that M6 routes planar authority through that machinery and retains its proof
basis; they must not require or permit a second local predicate engine.

M6 also owns the planar contract support layer around that math substrate:
`PlanarLocalFrameCertificate`, `ProjectPointToCertifiedPlane2D`,
`CertifiedSegmentSegment2D`, `CertifiedPolygonWinding2D`,
`CertifiedSignedArea2D`, `CoplanarOverlapContractExtractor`,
`PlanarContractBundleValidator`, and
`PredicateCertificateConsumptionValidator`.

Every test below must either:

- produce a stable, machine-checkable planar contract bundle suitable for later
  boolean input, or
- fail cleanly with a typed diagnostic naming the exact unsupported class,
  policy denial, topology contract failure, predicate uncertainty, or
  binding/rebinding failure.

Crashing, hanging, silently repairing, silently coercing, producing
non-deterministic classifications, or emitting unlocalized failure is always a
test failure.

## Common Assertion Contract

Every `MB-M6-*` test must assert all applicable rows below.

- The live planar classification outcome is typed as admitted, denied,
  unsupported, policy-required, or predicate-uncertain.
- The structural identity digest is separate from topology identity, naming
  identity, lineage identity, and binding identity.
- Equivalent authoring order, host order, movement order, and rotation order do
  not perturb planar classification or structural identity.
- Semantically different movement, rotation, topology replacement, binding, or
  planar predicate inputs do perturb the relevant identity or outcome class.
- Retained planar facts replay to the same outcome class and structural identity
  as the live admitted workflow.
- Projection-consumed planar facts match retained facts and live facts for the
  same semantic basis.
- Recovery posture is typed and does not invent missing planar truth.
- Movement and rotation posture is consumed as explicit semantic input, not
  inferred from later candidate availability or coordinate coincidence.
- Diagnostics distinguish predicate uncertainty, topology contract failure,
  binding/rebinding failure, unsupported planar class, policy denial, and
  movement/rotation invalidation.
- Counters expose precision escalation breadth, identity lookup breadth,
  retained basis breadth, projection-consumption breadth, movement/rotation
  posture breadth, and clean-fail localization breadth.
- Predicate assertions expose the consumed `worth-math` certified sign and
  precision metadata, plus the spatial local-basis and Query declaration context
  that made that predicate meaningful for the Worth planar workload.
- Every applicable test asserts the relevant local-frame, certified projection,
  segment-contact, winding/containment, signed-area/degeneracy,
  coplanar-overlap, bundle-validation, and predicate-consumption certificates
  rather than accepting boolean-side recomputation.

## MB-M6-1: Coplanar Overlap Contract Storm

Equivalent pressure: `MB-T4-1`.

### Scenario

Construct a planar contract workload with hundreds of exactly coplanar faces
across multiple overlap regions:

- partial overlaps
- nested holes
- figure-8 boundary touch points
- long collinear runs with near-zero spacing
- simultaneous exact flush and near-graze planes

Stack movement and rotation pressure by evaluating the same workload under:

- identity transform
- translation that preserves exact coplanarity
- 180-degree rotation that preserves exact planar meaning
- tiny rotation that intentionally exits the admitted exact-coplanar class
- move-then-rotate and rotate-then-move order variation where semantic meaning
  is equivalent

### Must Assert

- Equivalent coplanar overlap workloads converge to the same planar
  classification set under admitted movement and rotation order variation.
- Structural identity digests stay stable for exact semantic equivalents and
  change when the tiny rotation genuinely exits the admitted exact-planar class.
- Coplanar winner/tie decisions are deterministic across host order, authoring
  order, retained replay, movement order, and rotation order.
- Projection-consumed planar facts match live and retained planar facts for each
  admitted coplanar region.
- Near-graze and non-admitted tiny-rotation cases deny typed before any snap,
  closest-plane fallback, or topology-only reconstruction can produce success.
- Diagnostics identify the exact region, plane basis, transform basis, and
  predicate class responsible for each denial.
- Counters show bounded classification breadth per affected coplanar region,
  not a hidden whole-model scan.

## MB-M6-2: High-Valence Planar Singularity Contract

Equivalent pressure: `MB-T4-3`.

### Scenario

Construct a high-valence planar/topological singularity with more than one
hundred incident planar elements competing at one vertex or local neighborhood.
Include exact coplanar subsets, near-miss tool planes, and boundary elements
whose topology legality is intentionally on the edge of the admitted class.

Stack movement and rotation pressure by applying:

- rotations around the singular vertex
- translations that preserve incidence
- reorientation that reverses local basis orientation while preserving semantic
  planar meaning
- one movement that breaks the local replacement neighborhood contract

### Must Assert

- The singularity produces deterministic predicate posture across all admitted
  incidence-preserving movement and rotation variants.
- Precision escalation is explicit and localized to the high-valence
  neighborhood.
- The topology-to-spatial contract completeness validator runs before later
  planar identity or projection facts are emitted.
- Structural identity remains separate from topology identity even when all
  topology ids and names are stable.
- A movement that breaks the local replacement neighborhood is denied before
  rebinding or correspondence can masquerade as authoritative continuity.
- Retained replay preserves the same admitted or denied outcome class and the
  same diagnostic trigger.
- Diagnostics name the singular vertex/neighborhood, valence pressure, movement
  posture, and exact failed contract.

## MB-M6-3: Thin-Feature Scale-Separation Contract

Equivalent pressure: `MB-T4-7`.

### Scenario

Construct a workload with extreme scale separation: a very large planar body
containing thousands of micro-planar features separated by near-epsilon gaps.
Evaluate planar classifications and structural identity under tools that graze
the micro-features while remaining exactly flush with large-scale faces.

Stack movement and rotation pressure by applying:

- large-coordinate translations
- local-coordinate normalized translations
- exact 90-degree rotations preserving grid structure
- tiny rotations that must force typed uncertainty or unsupported posture
- repeated move/rotate cycles that should cancel exactly

### Must Assert

- Local coordinate normalization is mandatory and visible in counters.
- Relative precision escalation is based on local feature scale, not global
  coordinate magnitude.
- Exact movement/rotation cancellation cycles replay to identical structural
  identity and planar classification facts.
- Tiny rotations that invalidate the admitted exact-planar class fail cleanly
  instead of creating false thin-feature success.
- Projection-consumed facts preserve the same local-coordinate basis as retained
  facts.
- Identity lookup breadth scales with touched micro-feature scope, not total
  feature count.
- Diagnostics name the micro-feature, local coordinate basis, transform basis,
  and precision escalation tier responsible for each outcome.

## MB-M6-4: Retained Planar History Cancellation Chain

Equivalent pressure: `MB-T4-5`.

### Scenario

Build a long planar history chain where repeated movement, rotation,
reorientation, and local planar rebuild operations should periodically cancel
back to the original planar contract basis or to an explicitly empty/denied
basis. Inject one near-graze event at a known step.

### Must Assert

- Every exact cancellation checkpoint produces bit-identical planar
  classification facts and structural identity digests.
- The injected near-graze is localized to the exact step where it enters the
  history.
- Retained replay never repairs, hides, or reorders the near-graze trigger.
- Projection-consumed facts before and after the trigger match retained history
  facts for the same basis.
- Movement and rotation posture is part of the retained fact basis.
- Equivalent regrouping of move/rotate operations does not perturb identity
  when the canonical transform basis is identical.
- Non-equivalent transform histories do not collapse to the same retained
  structural identity merely because the final coordinates are close.
- Counters expose per-step retained basis breadth and do not defer localization
  until the end of the chain.

## MB-M6-5: Dirty Planar Input Clean-Fail Localization

Equivalent pressure: `MB-T4-4`.

### Scenario

Feed deliberately dirty planar input into the M6 contract layer:

- self-intersecting planar loops
- non-manifold wire edges
- thin planar walls
- inconsistent orientation
- topology identities and names that remain stable despite bad geometry

Stack movement and rotation pressure by evaluating dirty input after exact
translations, exact rotations, and one orientation-reversing transform.

### Must Assert

- Dirty input never produces an admitted planar contract bundle by heuristic
  repair.
- Failures are classified as topology contract failure, predicate uncertainty,
  unsupported planar class, policy denial, or movement/rotation invalidation.
- Stable topology ids and names cannot reconstruct passing structural identity
  after planar meaning fails.
- Orientation-reversing transforms are typed explicitly and cannot silently flip
  structural identity into success.
- Retained facts preserve the original failure class and trigger after movement
  or rotation.
- Projection-consumed facts cannot consume dirty retained basis as if it were
  admitted planar truth.
- Diagnostics identify the first blocking dirty input feature and the transform
  posture that exposed or preserved it.

## MB-M6-6: Unbounded Half-Space Planar Posture

Equivalent pressure: `MB-T4-6`.

### Scenario

Classify open or unbounded planar domains, including large sets of half-space
planes with exact coplanar groups, near-graze groups, and sliver-inducing
arrangements. M6 does not convert these to boolean results; it classifies their
planar contract posture.

Stack movement and rotation pressure by applying:

- translations that preserve half-space orientation
- rotations that preserve the half-space arrangement up to canonical basis
- rotations that invert or otherwise change half-space meaning
- repeated transform cycles that should canonicalize back to the same posture

### Must Assert

- Each unbounded/open planar domain is classified as admitted, unsupported,
  policy-required, or predicate-uncertain before later boolean work can consume
  it.
- Canonically equivalent half-space arrangements produce identical structural
  identity under admitted transform variation.
- Orientation-changing rotations perturb identity or outcome where semantics
  genuinely change.
- No hidden bounded conversion, clipping, or inferred manifold repair occurs in
  M6.
- Retained and projection-consumed facts preserve the unbounded/open posture.
- Recovery posture suggests only typed next steps; it does not synthesize
  bounded truth.
- Diagnostics name the half-space group, transform basis, and exact reason the
  arrangement is admitted or denied.

## MB-M6-7: Projection-Consumed Planar Fact Parity

M6-specific Query-native proof.

### Scenario

For a representative set of admitted and denied planar workloads, compare the
same semantic planar meaning across:

- live classification
- retained planar facts
- projection-consumed planar facts
- recovery posture
- replayed history
- movement/rotation variants
- local planar rebuild variants

### Must Assert

- Live, retained, projection-consumed, recovered, and replayed views converge on
  the same outcome class for equivalent semantic inputs.
- The structural identity basis used by projection consumption is the same
  basis used by retained replay.
- Movement and rotation posture is present in every relevant basis and cannot
  be reconstructed from final coordinates alone.
- Denied paths remain denied across all views and never become projection
  success through summary-only facts.
- Recovery posture consumes typed denial facts and never reclassifies planar
  truth.
- Local planar rebuild variants converge only when their semantic planar basis
  and transform basis are equivalent.
- Diagnostics expose the exact surface where any parity mismatch occurs: live,
  retained, projection, recovery, replay, movement, rotation, or rebuild.

## MB-M6-8: Boolean-Readiness Final Boss

Equivalent pressure: `MB-T4-8`, stopped before boolean execution.

### Scenario

Combine all M6 pre-MetaBoss pressure families into one workload:

- coplanar overlap storm
- high-valence planar singularity
- thin-feature scale separation
- retained history cancellation chain
- dirty planar input
- open/unbounded planar posture
- local planar rebuilds
- topology replacement
- binding/rebinding pressure
- projection consumption
- recovery posture
- repeated movement and rotation
- one intentional orientation flip at a known step

The test must stop at the M6 boolean-readiness boundary. It must not perform
M7 split/classify/assemble.

### Must Assert

- The final output is either a complete boolean-readiness contract bundle or a
  typed clean failure with exact trigger localization.
- A complete bundle includes exact planar classifications, structural identity
  digests, topology-to-spatial completeness facts, retained planar facts,
  projection-consumed facts, movement/rotation posture, recovery posture, and
  diagnostics.
- Every admitted sub-workload produces the same outcome across live, retained,
  projection-consumed, recovered, and replayed views.
- Every denied sub-workload preserves its denial class across live, retained,
  projection-consumed, recovered, and replayed views.
- Movement/rotation cancellation chains produce identical identity at exact
  cancellation checkpoints.
- The intentional orientation flip is localized at its exact step and cannot be
  hidden by later rebuild, rebinding, projection consumption, or recovery.
- No kernel-local workflow summary can substitute for spatial planar predicate
  authority or retained Query-native planar facts.
- The bundle is acceptable M7 input only if all admitted planar fact families
  are present and all unsupported families are explicitly typed.
- Counters prove the bundle avoided hidden whole-model scans, hidden retained
  basis rebuilds, and hidden projection-consumption broadening.

## NMT and Mixed-Surface Extensions

These four bosses extend M6 boolean-readiness proof before M7 booleans. They
prove that admitted open non-manifold topology and mixed surface families cannot
be manifold-laundered, plane-smuggled, open-class-normalized, or grazing-stack
collapsed through the workload platform, retained replay, projection
consumption, or user-response layers.

Tier 4 execution counterparts for these themes live in `METABOSS.md` as
`MB-T4-NMT-*`, `MB-CT4-NMT-*`, and `MB-FT4-NMT-*`.

Every `MB-M6-NMT-*` test must follow the same implementation bar as
`MB-M6-4`, `MB-M6-5`, and `MB-M6-7`:

- one admitted-path receipt contract with counter breadth
- one production-owned outcome matrix branching every stop
- dedicated trap tests with typed enum equality and human prose guards
- checkpoint- or lane-localized failure summaries
- denial digest equality against the consumed production receipt

## MB-M6-NMT-1: Open Radial Fan Cannot Be Manifold-Laundered

Equivalent pressure: `MB-T4-NMT-1` (planar execution), `MB-CT4-NMT-1` and
`MB-FT4-NMT-1` (curved/fillet open-hub variants), stopped before boolean or
fillet execution.

M6-specific NMT proof. Prerequisite: `TopologySeed::open_shell_nmt_edge_fan(k)`
and a matching `WorkloadCatalog` recipe wired from the worth-topo
`open_shell_nmt_fan_view` carrier.

### Scenario

Construct an admitted open-shell non-manifold edge fan with radial adjacency at
`k ∈ {3, 4}`. Run the full workload platform pipeline:

- topology receipt with open non-manifold posture
- planar geometry binding on admitted plane carriers only
- `SurfaceFamily::Plane` surface support
- projection, transform (`HostileCancellation` and `MovementRotationStack`),
  retained replay, diagnostics, and response

Stack movement and rotation pressure by evaluating the same NMT workload under:

- identity transform
- translation that preserves radial incidence
- rotation that preserves open non-manifold planar meaning
- one label-only motion row where transform step count advances without
  coordinate change
- one hostile fork that perturbs radial adjacency

Attach `ProjectedOverlapFaceSet` and coplanar overlap extraction only where
planar carriers exist. Do not clip the open fan to a closed shell or repair
radial edges into manifold loops.

### Must Assert

- Topology evidence counters include non-manifold edge count ≥ 1, open boundary
  count > 0, and shell interpretation class `OpenNonManifold`; the admitted path
  must not report closed-manifold posture.
- Structural identity digest is separate from topology identity even when entity
  ids and names remain stable across admitted motion.
- Retained replay preserves the same admitted or denied outcome class and the
  same open-NMT diagnostic trigger under admitted transform variation.
- Transform pressure preserves the dirty or open failure class; label-only
  motion denies before overlap extraction or boolean-readiness can succeed.
- Projection-consumed facts preserve open non-manifold posture; they must not
  consume closed-shell retained basis as admitted planar truth.
- Counters expose retained artifact breadth, replay checkpoint breadth,
  projection-consumption breadth, and topology posture breadth tied to receipt
  rows, not summary-only facts.

### Outcome Matrix

The production-owned matrix must branch exactly these stops:

- admitted open non-manifold planar posture on the honest catalog path
- unsupported non-plane surface family when surface support is not `Plane`
- dirty input when a non-manifold wire or self-intersecting loop receipt is
  smuggled into the NMT workload
- integrity mismatch when a closed `cube` retained checkpoint is replayed on the
  NMT workload at a named checkpoint (for example checkpoint 11)
- integrity mismatch when a foreign retained checkpoint stage from another
  workload is substituted at a named checkpoint (for example checkpoint 17)
- denied movement or rotation when label-only motion is injected at a named
  step (for example checkpoint 21)
- no-options missing-evidence when radial adjacency evidence is absent at a
  named checkpoint (for example checkpoint 9)

Each matrix row must assert outcome kind, cause kind, non-empty evidence
digest, and human-readable summary without machine tokens or slug fragments.

### Dedicated Trap Tests

- Manual authority substitution must deny for every workload evidence stage when
  a hand-filled row replaces a source receipt.
- Cube topology receipt substitution onto the NMT ledger must deny with typed
  integrity mismatch naming topology posture, not generic missing evidence.
- Storm overlap extraction bundle from `coplanar_overlap_storm` linked into the
  NMT ledger must deny as mismatched operator stage link or integrity mismatch.
- Foreign clean-fail boundary or user-response receipt from another workload must
  deny before certification.
- Stable topology identity equal to the NMT clean-fail identity must not hide
  manifold-laundered geometry.
- Transform pressure on the honest NMT path must keep `OpenNonManifold`
  posture and the same failure or admission class.

## MB-M6-NMT-2: Mixed Surface Kill Box

Equivalent pressure: `MB-T4-NMT-2`, `MB-CT4-NMT-2`, `MB-FT4-NMT-2`, stopped
before boolean, trim, or fillet execution.

M6-specific mixed-surface proof. One topology, five surface families, zero
smuggling.

### Scenario

Use one valid closed topology (`WorkloadCatalog::cube()` or equivalent tetrahedron
carrier) with identical declaration stem across five runs. Vary only
`SurfaceFamily`:

- `Plane` (control — must admit through the full evidence ledger or explicit
  admitted planar posture)
- `AnalyticNonPlanar`
- `Freeform`
- `GeneratedFeature`
- `Unknown`

Run each family through topology, binding, surface support, projection,
transform, retained replay, diagnostics, and response. Attempt boolean-readiness
certification only on the honest `Plane` path.

### Must Assert

- `Plane` produces a complete workload evidence ledger, projection-consumed
  facts, and either overlap extraction receipts or an explicit admitted planar
  posture suitable for later M7 input.
- Each non-plane family fails with a distinct typed unsupported reason, distinct
  workload or stage receipt digest, and distinct human-readable summary; a
  `BTreeSet` over family digests must have size four.
- Boolean-readiness gate: every non-plane family must report
  `is_acceptable_m7_input() == false` with localized policy denial naming the
  surface family in prose.
- Stable cube entity ids on unsupported surface runs cannot produce admitted
  structural identity or overlap success.
- Surface-support stage counters must be receipt-backed; manual family-row
  substitution must deny as manual authority at the surface-support stage.
- Denial evidence digests must equal the digests of the production receipts they
  consumed, matching the `MB-M6-8` final-boss matrix contract.

### Outcome Matrix

The production-owned matrix must branch exactly these stops:

- admitted `Plane` path through full platform evidence
- unsupported `AnalyticNonPlanar` with localized family reason
- unsupported `Freeform` with localized family reason
- unsupported `GeneratedFeature` with localized family reason
- unsupported `Unknown` with localized family reason
- integrity mismatch when a `Plane` surface-support or overlap receipt is
  attached to a `Freeform` workload ledger
- integrity mismatch when a kernel summary substitutes for readiness receipts

Each unsupported row must use a different `human_reason` string and a different
evidence digest. User-response receipts must explain the correct family; wrong
family response evidence must deny before certification.

### Dedicated Trap Tests

- Manual authority substitution must deny for all eight workload evidence stages
  on the cube carrier.
- Cross-family receipt smuggling (`Plane` ledger + `Freeform` surface-support
  receipt) must deny as integrity mismatch or mismatched operator stage link.
- Kernel summary substitution for boolean-readiness receipts must deny typed
  before M7 acceptance.
- Boolean-readiness workload with plane topology and smuggled
  `GeneratedFeature` support must fail policy gate with localized denial.
- Surface-support counters on unsupported families must prove the stage ran and
  denied; absent stage evidence must not masquerade as unsupported success.

## MB-M6-NMT-3: Open-Class Topology Triad Parity Kill Switch

Equivalent pressure: `MB-T4-NMT-3`, `MB-CT4-NMT-3`, `MB-FT4-NMT-3`, stopped
before boolean, trim, or fillet execution.

M6-specific open-topology proof. Composes `MB-M6-6` posture law with
`MB-M6-7` nine-lane parity and `MB-M6-4` retained-checkpoint forgery traps.

### Scenario

Construct three open-valid topology workloads through identical transform and
retained recipes:

- `open_wire`
- `open_sheet`
- `open_shell_nmt_edge_fan(128)` as the hostile open-fan scale

For each topology class, run the full platform stack and M6.7-style parity
comparison across nine lanes:

- live classification
- projected geometry
- projection-consumed facts
- retained facts
- replayed retained facts
- transformed geometry
- recovery posture
- local rebuild
- diagnostics

Stack movement and rotation pressure with `HostileCancellation` and
`MovementRotationStack` variants. Inject cross-class forgery by replaying
retained checkpoints from one open class onto another and from closed
`coplanar_overlap_storm` onto each open class.

### Must Assert

- Within each topology class, all nine parity lanes admit on the honest path
  and counters show `lanes_compared == 9` with `receipt_backed_lanes == 9`.
- Structural identity digests across wire, sheet, and fan form a three-element
  set; stable within each class under admitted motion, distinct across classes.
- Denied paths remain denied across all nine lanes; upgrading one lane (for
  example recovery) to admitted while others stay denied must fail as denied
  upgrade with the upgraded lane named in prose.
- Cross-class retained checkpoint replay must deny as integrity mismatch at a
  named checkpoint with topology posture named in the summary (for example
  checkpoint 9 for wire versus sheet, checkpoint 11 for fan versus storm).
- Bounded-conversion law: open wire must not gain `topology_face_count` or sheet
  closure counters after projection; open sheet must not close to a bounded
  solid; NMT fan must preserve radial edge count and open boundary count.
- Upgrade attack: injecting plane overlap extraction receipts from
  `coplanar_overlap_storm` into a denied open-sheet or open-wire ledger must
  deny as synthetic overlap extraction or integrity mismatch, not success.
- Diagnostics for wire, sheet, and fan failures must use distinct prose; shared
  boilerplate strings across classes are a test failure.
- Transform recipe divergence (`MovementRotationStack` versus
  `HostileCancellation`) must change replay breadth while preserving within-class
  outcome class and parity digest on the honest path.

### Outcome Matrix

For each topology class, the parity mismatch matrix must localize integrity
mismatch to every lane:

- projected geometry lane
- projection-consumed fact lane
- retained fact lane
- replayed retained fact lane
- recovery lane
- transformed geometry lane
- local rebuild lane
- diagnostic lane

That yields twenty-four localized mismatch surfaces across the triad (eight
lanes × three classes). Each mismatch outcome must name the lane in human prose,
assert integrity mismatch cause kind, and pass human-readability guards.

Additionally branch these cross-class traps:

- integrity mismatch when closed-storm retained checkpoint replays on open wire
- integrity mismatch when open-sheet retained artifact replays on NMT fan with
  radial and open-boundary evidence cited
- no-options or unsupported when storm operator extraction bundle is linked into
  an open-wire or open-sheet ledger

### Dedicated Trap Tests

- Denied-upgrade trap per class: all lanes denied except recovery set to admitted
  must fail before outcome classification.
- Foreign checkpoint stage substitution at checkpoint 11 must cite workload
  catalog transform evidence receipt identity mismatch.
- Projection-consumed identity forgery shaped like retained replay must deny at a
  named checkpoint (for example checkpoint 25).
- Trigger-local replay omission at a predicate-uncertain or policy-required
  checkpoint must deny as missing evidence, not integrity mismatch mislabel.
- Counters on each class must prove identity lookup breadth scales with touched
  open scope, not whole-model face count from closed storm workloads.

## MB-M6-NMT-4: Grazing Open-Shell Basket Stack Storm

Equivalent pressure: `MB-T4-NMT-4`, `MB-CT4-NMT-4`, and `MB-FT4-NMT-4`
if those later execution tiers name matching curved or feature variants. M6
stops before boolean, trim, fillet, or feature execution.

M6-specific stacked-open-topology proof. This boss exists because a single open
sheet or open non-manifold fan can be handled honestly while a stack of
near-grazing open shells still exposes projection collapse, radial adjacency
bleed, retained checkpoint confusion, false closure, and whole-stack diagnostic
broadening.

Prerequisite: reusable production NMT topology construction support, not a
special-purpose basket primitive. The construction boundary must be pleasant
enough to use outside this test: callers should specify open topology pattern,
layer count, strip count, boundary/radial evidence posture, and later hostile
pressures through named specs rather than hand-authoring topology rows or
coordinates. The basket stack workload must consume that generic NMT
construction boundary and add grazing/hostile pressure as workload semantics.
The builder must create real topology and receipts. It must not be a
test-support coordinate fixture.

### Scenario

Construct a stack of open-shell basket layers, where each layer is an open
sheet/open-shell weave made of alternating planar strips. The admitted bounded
profile is intentionally hostile but finite:

- `layer_count in 4..=7`
- `strip_count_per_layer in 8..=16`
- at least two orientation families per layer
- at least three grazing offset classes:
  - exactly separated by a certified local normal offset
  - near-grazing within the local feature scale
  - predicate-hostile near-grazing that must become typed uncertain or denied
- at least one layer with `HostileCancellation`
- at least one layer with `MovementRotationStack`
- one attack layer with label-only motion
- one attack layer with missing open-boundary or projection evidence
- one cross-layer retained checkpoint substitution
- one cross-layer projection-consumed identity substitution

Run the full workload platform pipeline:

- topology receipt with per-layer open topology posture
- open boundary ownership and, where present, radial adjacency evidence
- planar geometry binding on admitted plane carriers only
- `SurfaceFamily::Plane` support for honest layers and at least one non-plane
  unsupported layer
- local-frame and projection receipts per layer
- transform, retained replay, diagnostics, response, and projection-consumed
  facts
- optional certified overlap extraction only for layer-local planar carriers
  that genuinely meet the certified overlap preconditions

The stack must not close into a solid, merge near-grazing layers, infer topology
from coordinate coincidence, or allow an aggregate basket summary to replace
per-layer receipts.

### Must Assert

- Every honest layer preserves its own open topology posture, open-boundary
  ownership, projection identity, retained basis, movement/rotation posture,
  and structural identity under admitted transform variation.
- Structural identity is stable within a layer under semantic equivalents and
  distinct across grazing layers even when projected coordinates are nearly
  coincident.
- No layer gains closed-shell posture, bounded-solid posture, extra face closure,
  or unrelated radial adjacency from neighboring layers.
- Projection-consumed facts preserve layer identity; coordinate-near layers must
  not collapse into one projected region or one certified candidate-pair set.
- Retained replay preserves layer-local checkpoints; swapping checkpoints
  between grazing layers denies as integrity mismatch at the consuming boundary.
- Label-only motion denies before overlap extraction, readiness certification,
  or user-response admission.
- Near-graze predicate pressure localizes to the affected layer, strip, boundary,
  local frame, and precision tier. It must not become a whole-stack failure
  unless the production receipts prove a shared authority boundary.
- A non-plane surface family on one layer denies for that layer and does not
  poison honest plane layers.
- Missing evidence on one layer produces no-options for that exact layer,
  basket, strip, boundary, projection, or retained checkpoint. It must not be
  mislabeled as generic unsupported or generic integrity mismatch.
- Counters expose layer count, strip count, touched-layer breadth,
  open-boundary breadth, projection-consumption breadth, retained checkpoint
  breadth, precision escalation breadth, and clean-fail localization breadth.
  Attack rows must prove touched-layer breadth rather than hidden whole-stack
  relabeling, replay, or projection broadening.

### Outcome Matrix

The production-owned matrix must branch exactly these stops:

- admitted honest basket stack with per-layer receipts and no closed-shell
  posture
- admitted equivalent transform variants that preserve per-layer identity
- predicate-uncertain near-graze row localized to a named layer, strip, boundary,
  local frame, and precision tier
- unsupported non-plane surface family on one layer with localized family reason
- denied label-only movement on one layer before overlap extraction or readiness
  admission
- denied radial/open-boundary perturbation on one layer before projection
  success can hide it
- integrity mismatch when retained checkpoint from layer `i` is replayed on
  layer `j`
- integrity mismatch when projection-consumed identity from one layer is shaped
  like retained replay for another layer
- integrity mismatch when storm overlap extraction receipts are attached to the
  basket stack or to the wrong layer
- no-options when a layer's open-boundary evidence is absent
- no-options when a projection or retained checkpoint lane is absent at a named
  layer checkpoint

Every matrix row must assert:

- outcome kind
- cause kind
- layer identity
- basket/strip/boundary identity when applicable
- non-empty consumed evidence digest
- human-readable summary with no slug-only machine token
- counters proving whether the row touched one layer, multiple layers, or the
  whole stack

### Dedicated Trap Tests

- Manual authority substitution must deny for every workload evidence stage on
  the basket stack.
- Per-layer receipt substitution must deny for topology posture, surface
  support, projection, transform, retained replay, diagnostics, and response.
- Cross-layer retained checkpoint replay must deny with source layer, target
  layer, and checkpoint identity in prose.
- Cross-layer projection identity substitution must deny before any certified
  overlap or readiness bundle can consume it.
- Coordinate-near but topologically unrelated basket layers must remain
  unrelated; topology must not be inferred from geometric grazing.
- False-closure attack must prove the stack does not gain closed shell,
  bounded-solid, or aggregate manifold posture.
- Whole-stack broadening attack must prove one hostile layer does not force
  whole-stack relabeling, whole-stack replay, or whole-model projection scans.
- Human-readable diagnostics must be distinct for:
  - false closure
  - cross-layer retained replay
  - cross-layer projection identity
  - missing open-boundary evidence
  - predicate-uncertain near-graze
  - unsupported surface family

