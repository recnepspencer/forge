# Worth Test Requirements, Part 2

This document continues the Worth milestone-closeout proof bar for roadmap
milestones `11` through `20`.

Global proof rules, acceptable outcomes, and milestone-mapping discipline are
defined in:

- [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)

This part extends the canonical primitive corpus into freeform, specialized
feature, merge, interaction, and full-certification territory.

## Extended primitive corpus rule

Beyond the topology and analytic-carrier families defined in part `1`, Worth
must eventually prove these additional parameterized families:

- `FreeformPatch(p)`: arbitrary admitted freeform patch counts
- `FreeformTrimmedPatch(p, h...)`: freeform patches with one or more admitted
  trim loops
- `FreeformPatchNetwork(p, e)`: multi-patch neighborhoods with admitted shared
  boundaries
- `ChamferSet(e)`: arbitrary admitted chamfer edge-set sizes
- `ConstantFilletSet(e)`: arbitrary admitted constant-radius fillet edge-set
  sizes
- `VariableFilletRail(r)`: arbitrary admitted variable-radius rail counts
- `BlendJunction(j)`: arbitrary admitted junction valence within the milestone
  class
- `BranchHistory(b, d)`: arbitrary admitted branch counts and history depths
- `InteractionCommandSeq(c)`: arbitrary admitted UI / DSL command-sequence
  lengths

These families must also be proven as families, not as single examples.

For each admitted family in this part, the proof set must include:

- the smallest non-degenerate admitted member
- at least one larger generic member that is not a showcase model
- at least one hostile admitted member near the milestone boundary
- at least one explicit out-of-class member that must fail cleanly

## Milestone 11: NURBS And General Freeform Surface Foundation

### Purpose

Prove that freeform support is an honest admitted workflow surface rather than
an overclaim built from a few sample patches.

### Required workload surface

Run admitted freeform workloads containing:

- primitive-corpus coverage for:
  - `FreeformPatch(p)`
  - `FreeformTrimmedPatch(p, h...)`
  - admitted `FreeformPatchNetwork(p, e)`
- arbitrary admitted freeform patch counts
- arbitrary admitted trim counts
- admitted chained evaluation and rebinding workflows within the milestone's
  freeform class

### Must verify

- freeform binding and trim integrity hold for admitted classes
- unsupported freeform classes are classified explicitly
- exact, bounded, and policy-gated outcomes remain distinct
- admitted freeform histories replay identically

### Required verification output

- freeform_truth_digest
- freeform_trim_integrity_report
- unsupported_freeform_report
- freeform_replay_parity_report

### Closeout condition

Milestone `11` closes only when admitted freeform workflows operate generically
across the admitted patch and trim surface.

## Milestone 12: NURBS / Freeform Hostile Proof Program

### Purpose

Prove that hostile freeform workloads either stay within declared bounds or
fail with exact, replay-safe localization.

### Required workload surface

Run hostile freeform workloads containing:

- hostile primitive-corpus coverage for admitted `FreeformTrimmedPatch` and
  `FreeformPatchNetwork` families
- chained freeform histories of arbitrary admitted length
- trim-heavy workloads over arbitrary admitted trim counts
- freeform escalation and degradation workloads within the admitted class

### Must verify

- degradation is localized exactly
- accepted and rejected hostile freeform cases replay identically
- no hostile freeform workload crashes, hangs, or drifts silently

### Required verification output

- freeform_truth_digest_series
- freeform_degradation_report
- freeform_failure_localization_report
- freeform_hostile_replay_report

### Closeout condition

Milestone `12` closes only when hostile freeform proof covers the admitted
freeform workflow class instead of isolated hard examples.

## Milestone 13: Chamfers And Edge-Modification Features

### Purpose

Prove that chamfers are honest feature workflows across the admitted
edge-modification class.

### Required workload surface

Run chamfer workloads containing:

- primitive-corpus coverage for `ChamferSet(e)`
- arbitrary admitted edge counts
- arbitrary admitted local shell neighborhoods
- branch-local and replayed chamfer histories

### Must verify

- chamfer legality remains subordinate to topology and feature truth
- chamfer naming continuity holds or fails explicitly
- admitted chamfer histories replay identically

### Required verification output

- chamfer_truth_digest
- chamfer_failure_localization_report
- chamfer_naming_report
- chamfer_replay_parity_report

### Closeout condition

Milestone `13` closes only when chamfer workflows operate generically across
the admitted edge-modification surface rather than on one-off solids.

## Milestone 14: Constant-Radius Fillet Foundation

### Purpose

Prove that constant-radius fillets are honest feature and regeneration
workflows over the admitted edge-set class.

### Required workload surface

Run constant-radius workloads containing:

- primitive-corpus coverage for `ConstantFilletSet(e)`
- arbitrary admitted edge-set selections
- arbitrary admitted local neighborhood sizes
- branch-local and replayed constant-radius histories

### Must verify

- constant-radius legality remains subordinate to topology, geometry, and
  feature truth
- no-silent-sliver and no-hidden-non-manifold behavior holds for admitted
  cases
- admitted histories replay identically

### Required verification output

- constant_fillet_truth_digest
- constant_fillet_collapse_report
- constant_fillet_topology_legality_report
- constant_fillet_replay_parity_report

### Closeout condition

Milestone `14` closes only when constant-radius workflows operate generically
across the admitted edge-set surface, not only on a few showcase edges.

## Milestone 15: Variable-Radius Fillets, Junctions, And Blend-Cascade Honesty

### Purpose

Prove that admitted variable-radius, junction, and cascade workflows are honest
and explicit about failure.

### Required workload surface

Run blend workloads containing:

- primitive-corpus coverage for:
  - `VariableFilletRail(r)`
  - `BlendJunction(j)`
- arbitrary admitted variable-radius rail counts
- arbitrary admitted junction valence within the milestone's class
- arbitrary admitted cascade depths
- branch-local and replayed blend histories

### Must verify

- failure taxonomy is explicit for radius overflow, collapse, thin-feature
  swallow, tangent ambiguity, and continuity loss
- no silent collapse or silent continuity drift occurs
- admitted histories replay identically

### Required verification output

- variable_blend_truth_digest
- junction_resolution_report
- continuity_loss_report
- blend_failure_taxonomy_report

### Closeout condition

Milestone `15` closes only when the admitted variable-radius and junction
workflow classes are covered generically, not by a handful of hard-coded blend
examples.

## Milestone 16: Hostile Blend Proof Program

### Purpose

Prove that hostile blend workloads either succeed honestly or fail with exact,
replay-safe structural causes.

### Required workload surface

Run hostile blend workloads containing:

- hostile primitive-corpus coverage for `BlendJunction(j)` and
  `VariableFilletRail(r)` families under arbitrary admitted hostile chain depth
- high-valence junction workloads across arbitrary admitted junction counts
- variable-radius collapse pressure across arbitrary admitted history length
- chained blend histories across the admitted class

### Must verify

- hostile blend failures localize exactly
- continuity-loss outcomes are explicit
- accepted and rejected hostile blend cases replay identically
- no hostile blend workload crashes, hangs, or creates silent slivers

### Required verification output

- hostile_blend_truth_digest_series
- hostile_blend_localization_report
- hostile_blend_continuity_report
- hostile_blend_replay_report

### Closeout condition

Milestone `16` closes only when hostile blend proof covers the admitted blend
workflow class rather than a few dramatic stress parts.

## Milestone 17: Branching, History, And Identity Evolution For Worth Models

### Purpose

Prove that branch-local history and identity evolution remain honest across the
admitted historical workflow class.

### Required workload surface

Run history workloads containing:

- primitive-corpus coverage for `BranchHistory(b, d)`
- arbitrary admitted branch counts
- arbitrary admitted history depths
- admitted identity-evolution chains across topology, naming, feature, and
  geometry-binding truth

### Must verify

- branch isolation holds
- historical lookup is deterministic
- identity-evolution conclusions replay identically

### Required verification output

- branch_history_digest
- identity_evolution_report
- historical_lookup_report
- branch_history_replay_report

### Closeout condition

Milestone `17` closes only when branch-local history workflows operate
generically across the admitted branch and history surface.

## Milestone 18: Merge, Conflict Taxonomy, And Multi-Branch Intent Semantics

### Purpose

Prove that admitted merge workflows either merge with explicit semantics or
fail with typed conflict and continuity diagnostics.

### Required workload surface

Run merge workloads containing:

- primitive-corpus merge coverage over `BranchHistory(b, d)` families and
  admitted multi-domain conflict combinations
- arbitrary admitted branch-pair histories
- admitted continuity splits and name conflicts
- admitted multi-domain conflicts across topology, naming, feature,
  geometry-binding, specialized-feature, and intent truth

### Must verify

- merge comparison and conflict classification are deterministic
- admitted merge failures are explicit and typed
- continuity and intent do not drift silently during merge
- admitted merge outcomes replay identically

### Required verification output

- merge_outcome_digest
- conflict_taxonomy_report
- merge_continuity_report
- merge_replay_report

### Closeout condition

Milestone `18` closes only when admitted merge workflows are covered
generically across the admitted multi-branch conflict surface.

## Milestone 19: Interaction Language, AI Workflows, And Intent-Explicit UX

### Purpose

Prove that admitted UI and DSL workflows preserve explicit intent instead of
hiding ambiguity behind heuristics.

### Required workload surface

Run interaction workloads containing:

- primitive-corpus coverage for `InteractionCommandSeq(c)` over both UI and DSL
  pathways
- arbitrary admitted DSL-authored command sequences
- arbitrary admitted cursor-IDE interaction histories
- arbitrary admitted ambiguous-intent operations within the milestone's command
  class
- branch-local and replayed interaction histories

### Must verify

- UI and DSL converge to the same downstream result for the same admitted
  intent
- ambiguous operations prompt or fail explicitly rather than guessing
- admitted interaction histories replay identically
- requested intent, alternatives, and chosen interpretation are recorded

### Required verification output

- interaction_intent_digest
- ui_dsl_parity_report
- ambiguity_resolution_report
- interaction_replay_report

### Closeout condition

Milestone `19` closes only when admitted interaction workflows preserve intent
generically across the admitted command and history surface.

## Milestone 20: Worth Certification And Aircraft-Grade Auditability

### Purpose

Prove that the full admitted Worth workflow surface is certified as exact,
explicit, or clean-failing with machine-checkable evidence.

### Required workload surface

Run certification workloads spanning:

- the full canonical primitive corpus from part `1` and this part, parameterized
  across the admitted Worth classes
- topology and naming histories
- geometry binding and approximation histories
- feature and regeneration histories
- specialized-feature and blend histories
- branch, merge, and interaction histories
- MetaBoss and final-boss compound workloads within the admitted class

### Must verify

- every lower milestone closeout remains satisfied
- all admitted MetaBoss and final-boss workloads either pass or clean-fail
  within contract
- certification outcome classes and artifact bundles are deterministic
- no uncertified workflow class is being implicitly claimed complete

### Required verification output

- certification_bundle_digest
- metaboss_outcome_matrix
- final_boss_truth_digest
- final_boss_causal_trigger_report
- remaining_debt_register

### Closeout condition

Milestone `20` closes only when the full admitted Worth workflow surface is
certified with machine-checkable evidence and any remaining incompleteness is
explicit named debt.
