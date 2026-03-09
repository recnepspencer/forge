# Forge Signal State, Replay, Provenance, and Lineage

## Thesis

`forge-signal` is not just a runtime that decides what to recompute next. It must also be able to explain why derived state exists, preserve enough execution state to support replay and recovery, and eventually track how computed artifacts evolve across refresh, replacement, restore, and branching.

This is not MVP work. It is defensive-against-the-future architecture.

If Forge is meant to support CAD, game engines, financial platforms, chip simulation, AI systems, and other hard software, then the runtime must assume:

- bugs are guaranteed
- host logic will become complex
- derived artifacts will be expensive
- debugging will often happen after the fact
- branch and replay workflows will matter
- users will need to trust not only outputs, but the path that produced them

For that reason, snapshots, replay, provenance, and signal lineage are first-class runtime concepts. They are not optional diagnostics, and they are not domain-specific add-ons.

## Why This Matters Across Domains

The same architectural needs show up in very different software classes:

- CAD and geometry systems need auditable derived artifacts, replay of failure cases, and stable reasoning about refresh versus replacement.
- Financial systems need deterministic replay, causal traceability, and confidence that small upstream changes did not silently contaminate downstream outputs.
- Game and simulation engines need cheap state capture, branch exploration, and tooling that explains update storms and stale derived data.
- Chip and hardware tooling needs replayable incremental execution, explicit invalidation causes, and trustworthy explanations for why expensive analyses reran.
- AI-driven systems need inspectable causal chains so agents can debug themselves, minimize regressions, and explain output evolution over time.

These are not five separate requirements. They point to one runtime principle:

> derived computation must be inspectable through time, not just correct in the moment.

## Core Distinctions

These concepts must stay separate.

### Provenance

Provenance answers:

> Why is this node in this state right now?

It is about current causality:

- what invalidated the node
- which dependencies were considered
- which versions changed
- which conditions deferred or allowed work
- which comparators suppressed work
- whether the node actually recomputed
- what host-provided causality metadata was attached

### Replay

Replay answers:

> Can the runtime reconstruct or re-run the evaluation story deterministically?

It is about reproducing execution-relevant state and decisions, not merely storing current values.

### Snapshot

Snapshot answers:

> What evaluation state can be captured, restored, inspected, or branched?

It is about stable capture of signal-runtime state at a point in time.

### Signal Lineage

Signal lineage answers:

> What did this computed artifact become across time, replacement, restore, branch change, or cache refresh?

It is about evolution of derived artifacts, not host truth identity.

Signal lineage is distinct from relational lineage. Relational lineage tracks the evolution of truth-side entities. Signal lineage tracks the evolution of computed artifacts and their continuity semantics across runtime events.

## Design Goals

The state/replay/lineage architecture must satisfy these goals:

1. Deterministic reconstruction of evaluation-relevant state.
2. Readable provenance for humans and machine consumers.
3. Cheap enough capture to be practical in hard-software workflows.
4. Clear separation between truth lineage and signal lineage.
5. Compatibility with future diff-aware propagation, memoization, and branchable evaluation.
6. No forced coupling to any one host domain or bridge implementation.
7. Stable semantics under failure, rollback, and restore.

## Provenance Model

`forge-signal` should treat provenance as a layered model rather than a single log blob.

### Layer 1: Invalidation provenance

Tracks why a node became `Dirty` or `MaybeStale`.

Examples:

- upstream aspect changed
- transactional invalidation touched this node
- dependency was removed
- upstream version is uncertain relative to cached snapshot

### Layer 2: Dependency provenance

Tracks what the runtime compared when deciding whether work mattered.

Examples:

- cached dependency version versus current version
- missing snapshot for an upstream dependency
- dependency no longer present in the current graph

### Layer 3: Condition provenance

Tracks why work was allowed, deferred, or forced.

Examples:

- `OnDemand` deferred default evaluation
- `Debounce` was not ready
- custom host condition rejected evaluation
- forced evaluation overrode a condition gate

### Layer 4: Comparator provenance

Tracks why changed inputs did or did not count as meaningful.

Examples:

- exact comparator treated any delta as meaningful
- tolerance comparator suppressed a small version delta
- future output-identity diffing suppressed downstream propagation

### Layer 5: Recompute provenance

Tracks whether work ran and what it produced in summary form.

Examples:

- recomputed or skipped
- dependency count considered
- number of meaningful input changes
- output hash or future output identity token
- generic labels useful for host inspection

### Layer 6: Host causality metadata

Tracks optional upstream provenance attached by the host or future bridge.

Examples:

- truth commit identifier
- patch stream cursor/checkpoint
- host transaction identifier
- origin subsystem marker

### Provenance rule

These layers must compose into a coherent explanation surface. The runtime should not have one story for `explain(node)`, another for metrics, and another for future replay. Diagnostics must share the same underlying causal model.

## Snapshot Model

Snapshots should be defined as stable captures of signal-runtime evaluation state, not copies of host truth.

### A signal snapshot should include

- live node entries and their state
- current dependency graph structure
- aspect versions
- dependency snapshots used for validation
- condition/comparator configuration
- trace summaries
- causality metadata
- enough runtime metadata to support deterministic restore and later replay

### A signal snapshot should not include

- ownership of host truth storage
- domain-specific payload semantics
- permanent merge of truth and compute state
- ad hoc debug logs that are not part of the runtime contract

### Snapshot design rule

Snapshots must capture evaluation state, not just serialization-friendly storage. A “snapshot” that cannot support meaningful restore, inspection, or branch semantics is not good enough.

## Replay Model

Replay should be defined narrowly and explicitly.

`forge-signal` replay does not mean “re-run arbitrary host code and hope for the same result.” It means the runtime can reconstruct and inspect the evaluation-relevant state transitions that led to a result, subject to the determinism guarantees of the host and runtime policies.

### Replay should support

- deterministic inspection of prior graph/evaluation state
- deterministic re-walk of invalidation and evaluation decisions
- restoration of prior signal snapshots for debugging or analysis
- future bridge integration with truth-side snapshot/cursor checkpoints

### Replay should not promise

- hidden capture of arbitrary external side effects
- deterministic behavior from non-deterministic host compute closures
- domain-specific debugging semantics inside the generic runtime

### Replay design rule

The runtime should preserve enough structured state that a host can answer:

- what was dirty
- what was compared
- what was deferred
- what recomputed
- what changed meaningfully
- what this artifact became afterward

without relying on ephemeral logs.

## Signal Lineage Model

Signal lineage must be modeled as evolution of computed artifacts, not as node identity alone.

Node identity is not enough because the same node can:

- recompute and refresh the same artifact
- replace the artifact it previously represented
- restore to an earlier snapshot state
- switch branches
- eventually reuse memoized results that originated elsewhere

### Signal lineage should answer

- Is this current computed artifact a refresh of the prior artifact or a replacement?
- Did this artifact survive a snapshot restore unchanged?
- Did this artifact continue across a branch switch?
- Did this output originate from memoized reuse rather than direct recomputation?
- What prior artifact did this one descend from?

### Candidate lineage events

- `Refreshed`
- `Replaced`
- `Restored`
- `BranchedFrom`
- `MergedFrom`
- `MemoizedFrom`
- `InvalidatedWithoutReplacement`

These are conceptual events, not yet implementation commitments.

### Signal lineage design rule

Lineage should describe artifact continuity semantics. It should not collapse into provenance, and it should not pretend to be truth-side identity lineage.

## Relationship Between Provenance and Lineage

The simplest rule is:

- provenance explains the current state
- lineage explains the evolution of the artifact

Examples:

- “Node 42 recomputed because upstream aspect 3 changed and debounce was ready” is provenance.
- “The output of node 42 was replaced after branch switch and now descends from artifact A17” is lineage.

Both matter. A system that has provenance but no lineage can explain the present but not continuity. A system that has lineage but no provenance can explain ancestry but not causality.

## Relationship to Smarter Propagation

Phase 3 work will directly affect snapshots and lineage semantics.

In particular:

- output identity / result diffing changes what counts as meaningful replacement
- partial recomputation changes what continuity means for partitioned artifacts
- structural memoization changes where an artifact may have originated

This is why the concepts should be locked now, even if full implementation waits until later phases.

## Relationship to the Bridge

The future bridge layer should connect truth provenance to signal provenance, but it should not own signal lineage.

The ownership split should remain:

- truth runtime owns truth identity, truth lineage, commit history, and patch provenance
- bridge carries causal references and snapshot/cursor context across the runtime boundary
- signal runtime owns evaluation provenance and signal lineage

This keeps the system decoupled while still supporting end-to-end causal traceability.

## Diagnostics as Product Architecture

Diagnostics must be treated as a first-class product surface because the runtime will be embedded in difficult software.

The diagnostic stack should eventually include:

- structured `explain(node)` causality
- dependency inspection by node and aspect
- graph export and traversal inspection
- surfaced metrics and hot-path visibility
- snapshot inspection
- replay-oriented inspection
- signal lineage inspection
- end-to-end provenance once bridge integration matures

If these are weak, the runtime becomes much harder to trust in exactly the environments it is supposed to serve.

## Constraints on Near-Term Implementation

These concepts should shape current implementation decisions now.

### Required constraints

- do not assume in-place-only evaluation history forever
- do not define output change semantics in a way that blocks lineage later
- do not treat trace summaries as throwaway debug fields
- do not tie provenance to domain-specific data
- do not fuse signal lineage with relational lineage
- do not rely on ephemeral execution logs as the only explanation mechanism

### Immediate implication for current roadmap

- Phase 3 should proceed, but with explicit awareness of future snapshot and lineage semantics
- Phase 5 should become the implementation phase for snapshot/restore, replay-oriented inspection, and signal lineage foundations
- bridge work should preserve causal references cleanly so provenance can go end to end later

## Non-Goals

- Making `forge-signal` the owner of truth-side history or identity
- Promising deterministic replay for non-deterministic host compute logic
- Baking domain-specific artifact semantics into the generic runtime
- Treating provenance and lineage as the same concept
- Reducing snapshots to plain serialization without restore/replay meaning

## Recommended Next Step

Use this document as the concept lock for future phases.

That means:

- Phase 3 can move forward on diff-aware propagation and smarter incremental semantics
- Phase 5 should be planned explicitly around this snapshot/replay/lineage model
- bridge provenance work should treat this document as the signal-side contract

The point is not to delay the roadmap. The point is to stop future work from quietly making these concepts harder to support well.
