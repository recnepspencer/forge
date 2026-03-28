# Milestone 7D Deletion And Topology Merge Execution Spec

## Purpose

This document is the build specification for Milestone 7D.

Milestone 7C established authoritative merge execution for the admitted
non-deletion merge subset. Milestone 7D extends the merge ontology without
weakening the proof chain, serialized authority model, replay parity, or
diagnostics rigor established in 7C.

The governing rule remains:

`parallelize disposable work, serialize authority`

Milestone 7D does not treat deletion or topology semantics as “special merge
exceptions.” It treats them as truth-bearing merge classes that must either:

- compile into an authoritative executable class, or
- remain explicitly non-executable with typed denial and proof-bearing
  diagnostics

There is no silent middle ground.

## Problem Statement

After 7C, the runtime can already:

- classify deletion conflicts explicitly
- preserve branch-local visibility evidence
- produce typed blocked/rejected lowered plans
- execute admitted merge classes through the authoritative commit pipeline

What it still cannot do is express the deletion/topology universe honestly
enough to evolve execution incrementally.

The current failure mode is not correctness of 7C execution. The failure mode
is ontology collapse:

- all deletion cases look like one generic “blocked deletion”
- relation rewiring remains visible as conflict but not as execution-class
  ontology
- operators cannot distinguish “safe but not yet admitted” from “semantically
  impossible under current truth rules”
- future executable promotion risks leaking semantics into lowering or commit
  apply instead of passing through a clear proof boundary

Milestone 7D exists to fix that before we admit any new executable merge
classes.

## Scope

Milestone 7D covers:

- explicit deletion execution ontology
- explicit topology execution ontology for relation rewiring surfaces
- typed mapping from conflict class to execution class or denial class
- promotion of the safest deletion executable subset
- preservation of fail-closed behavior for all not-yet-admitted classes
- certification of branch-local deletion/topology merge behavior

Milestone 7D does not yet attempt:

- arbitrary manual-resolution execution
- broad topology-region reconciliation
- generic rewrite-based merge intent repair
- heuristic relation endpoint rehoming

## Governing Constraints

1. Execution must continue to consume proof-carrying lowered or compiled merge
   artifacts only.
2. Commit application must not rediscover deletion semantics, topology
   semantics, or relation continuity.
3. Cost must remain proportional to admitted merge surface, not full branch
   breadth or full topology neighborhood breadth.
4. Diagnostics must distinguish:
   - executable deletion classes
   - explicitly non-executable deletion classes
   - topology-local blocked classes
   - topology-region escalation classes
5. Any merge class not admitted in 7D must remain fail-closed and machine-
   localizable.

## Current Baseline To Preserve

7D builds on these already-certified 7C guarantees:

- sealed merge preparation
- freshness verification and authority binding
- bound executable plan compilation
- merge-to-mutation derivation for admitted non-deletion classes
- shared authoritative commit pipeline
- durable merge diagnostics and replay parity
- explicit visibility evidence and branch-local historical visibility repair
- explicit lowered deletion blocked-reason taxonomy

7D must not regress any of them.

## Architectural Decision

Deletion and topology semantics will be introduced in two layers:

1. `MergeResolutionClass`
   This is the full ontology: what kind of merge situation exists in truth.
2. `MergeExecutableClass`
   This is the admitted executable subset for the current milestone.

The architecture must never equate “represented” with “executable.”

The execution boundary may consume only `MergeExecutableClass`.
All other classes remain typed denial surfaces.

## Execution Ontology

### Deletion Resolution Classes

The runtime must explicitly preserve at least these deletion classes:

- `SourceDeletedTargetLive`
- `SourceLiveTargetDeleted`
- `DeletedOnBothSides`
- `DeletedVsModified`
- `DeletedVsRewired`

These classes already exist at conflict time. 7D elevates them into the
execution/denial ontology so that downstream phases do not collapse them back
into generic blocked deletion.

### Topology Resolution Classes

The runtime must explicitly preserve at least these topology classes:

- `RelationEndpointStable`
- `RelationEndpointRewiredLocal`
- `RelationEndpointRewiredEscalated`
- `TopologyRegionConflict`

7D may refine the exact names, but the ontology must preserve the distinction
between relation-local rewiring and topology-region escalation.

## 7D Admitted Executable Subset

The default 7D promotion target is:

- `DeletedOnBothSides`

Why this class first:

- it has the smallest semantic surface
- it does not require one branch to “win” a visible record over the other
- it does not imply topology rewiring
- it is the easiest class to replay/certify without hidden identity mutation

All other deletion classes remain non-executable in the initial 7D slice unless
their authoritative semantics are separately specified and certified.

## Type Surfaces

### New Or Extended Planning Types

Primary module targets:

```text
crates/forge-relational/src/merge/data/conflicts.rs
crates/forge-relational/src/merge/data/policy.rs
crates/forge-relational/src/merge/data/execution.rs
crates/forge-relational/src/merge/data/artifacts.rs
```

Types to add or extend:

- `DeletionExecutionClass`
- `TopologyExecutionClass`
- `MergeResolutionClass`
- `MergeExecutableClass`
- explicit denial taxonomy for non-executable deletion classes
- explicit denial taxonomy for topology-local vs topology-region classes

Required invariants:

- every deletion conflict must map to exactly one deletion execution or denial
  class
- every topology conflict must map to exactly one topology execution or denial
  class
- it must be impossible for execution compilation to receive a generic
  “blocked deletion” bucket

### Lowering Types

Primary module target:

```text
crates/forge-relational/src/merge/logic/lowering.rs
```

Lowering must:

- preserve the specific deletion/topology class
- derive typed denial bundles for non-executable classes
- admit only the current executable subset
- keep the current `Blocked` / `Rejected` / `Admitted` contracts intact

### Execution Types

Primary module targets:

```text
crates/forge-relational/src/merge/data/execution.rs
crates/forge-relational/src/merge/logic/execution.rs
crates/forge-relational/src/merge/logic/execution_mutation_plan.rs
```

If `DeletedOnBothSides` is promoted, execution must add a concrete executable
record-plan variant for authoritative no-op convergence or authoritative tombstone
convergence, whichever semantics the runtime formally adopts.

The execution type must make the choice explicit. It must not rely on “no
mutation happened so it must have been both deleted.”

## Semantic Requirements For The First Executable Deletion Class

If 7D promotes `DeletedOnBothSides`, the runtime must specify:

- whether authoritative execution publishes a merge commit with zero record
  mutations but positive merge truth
- whether diagnostics describe it as preserved mutual deletion truth or
  converged deletion truth
- whether lineage surfaces change at all
- whether merge execution artifact rows for this class are execution rows, not
  denial rows

This class must not be admitted until these semantics are explicit.

## Non-Executable Classes That Must Remain Fail-Closed

Unless explicitly promoted later in 7D, these remain non-executable:

- `SourceDeletedTargetLive`
- `SourceLiveTargetDeleted`
- `DeletedVsModified`
- `DeletedVsRewired`
- topology-region escalation classes

The runtime must localize each with:

- class-specific denial kind
- branch-local visibility evidence
- base evidence
- if relation-local, continuity/propagation evidence

## Implementation Phases

Implementation must proceed in this order.

### Phase A: Ontology Lift

Goal:
Lift deletion and topology classes into explicit execution/denial ontology
without changing executable behavior.

Code ownership:

```text
crates/forge-relational/src/merge/data/conflicts.rs
crates/forge-relational/src/merge/data/policy.rs
crates/forge-relational/src/merge/logic/lowering.rs
crates/forge-relational/src/merge/data/artifacts.rs
```

Required outcomes:

- generic blocked deletion is eliminated from authoritative planning surfaces
- every lowered blocked deletion carries a specific class
- artifact digest basis includes the class-bearing denial surface

Exit criteria:

- no authoritative artifact uses a generic deletion blocked reason

### Phase B: Executable-Class Boundary

Goal:
Define which deletion/topology classes are executable vs non-executable.

Required outcomes:

- explicit mapping from `MergeResolutionClass` to `MergeExecutableClass` or
  denial
- compilation rejects any non-admitted class structurally
- execution types cannot be constructed from non-admitted classes

Exit criteria:

- illegal state “non-admitted deletion/topology class reaches executable plan”
  is unrepresentable

### Phase C: First Deletion Promotion

Goal:
Promote the first deletion executable class, expected to be
`DeletedOnBothSides`.

Required outcomes:

- compiled executable record-plan variant exists
- mutation derivation handles the class without rediscovering semantics
- commit pipeline remains shared
- success/failure diagnostics distinguish executable deletion truth from denied
  deletion truth

Exit criteria:

- one deletion class is executable end to end with replay parity

### Phase D: Certification And Hostile-Path Hardening

Goal:
Certify the expanded ontology and first executable deletion class.

Required tests:

- branch-local deletion classification parity
- replay parity for deletion-bearing merge commits
- recovery parity for deletion-bearing merge commits
- denial artifact parity for non-executable deletion classes
- topology-local denial parity
- hostile stale-head and schema-drift rejection on deletion-bearing prepared
  merges

Exit criteria:

- machine-checkable artifacts distinguish executable deletion truth,
  non-executable deletion denial, and topology denial without ambiguity

## Performance Contracts

7D must obey these explicit cost rules:

- deletion/topology classification may scale with candidate merge record breadth
  and admitted relation-local evidence breadth
- execution compilation may scale only with admitted executable record breadth
- commit apply must not scan full visible views or topology regions
- topology-region escalation detection must be counted explicitly when it occurs
- no phase may hide broad view indexing or neighborhood rediscovery in helper
  code

## Diagnostics Requirements

The artifact family must teach the architecture:

- planning artifact: full deletion/topology classification and denial evidence
- execution artifact: executed rows only
- failure artifact: typed denial or freshness/authority failure only

Success-path execution artifacts must never contain blocked rows.

## Open Decisions To Make Explicit Before Promotion

Before promoting any deletion class, the implementation must answer:

1. Is mutual deletion execution a no-op truth convergence or a distinct
   authoritative deletion-preservation act?
2. Does deletion execution affect lineage surfaces at all?
3. Can any one-sided deletion class ever be safely executable without explicit
   user intent?
4. Is relation endpoint rewiring ever relation-local, or does it always escalate
   to topology-region denial in this milestone?

These decisions must be encoded in types or named policy surfaces, not left as
code comments or implicit behavior.

## Recommended Immediate Build Order

The next coding work should follow this exact sequence:

1. eliminate generic deletion blocked-denial surfaces everywhere
2. add `MergeResolutionClass` / `MergeExecutableClass` split
3. keep all deletion classes fail-closed under the new ontology
4. specify `DeletedOnBothSides` execution semantics explicitly
5. promote `DeletedOnBothSides` only if the semantics stay replay-stable and
   mutation-derivation clean
6. certify before touching one-sided deletion or topology-region execution

## Definition Of Done

7D is complete only when:

- deletion/topology ontology is explicit and proof-bearing
- at least one deletion class is either explicitly certified executable or
  explicitly certified non-executable with typed denial
- replay/recovery parity holds for the newly admitted executable set
- operators can distinguish every blocked deletion/topology class without
  reverse-engineering generic denial buckets
- the commit pipeline remains shared and merge semantics remain absent from
  generic commit apply
