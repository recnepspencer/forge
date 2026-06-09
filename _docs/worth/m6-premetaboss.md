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

