# Forge Runtime Bridge Future Roadmap

## Purpose

This document defines the future work for the Forge runtime bridge.

It is a future-only roadmap. It does not assume the bridge is already
productized, and it does not treat the bridge as incidental glue. It exists to
sequence the remaining work required to turn the bridge into a real causal
protocol layer between `forge-relational` and `forge-signal`.

The governing rule remains:

- truth stays authoritative
- computation stays derived
- the bridge coordinates without collapsing either runtime into the other

## Roadmap Rules

Rules for every remaining bridge item:

- each milestone must describe a real bridge capability, not just “integration work”
- each milestone must preserve separate truth and compute ownership
- each milestone must preserve deterministic routing semantics and snapshot-backed reads
- each milestone must define concrete acceptance evidence through bridge harness scenarios, diagnostics artifacts, replay checks, or parity checks
- no milestone is complete until both implementation and bridge-specific trust evidence exist

## Geometry Kernel Critical Path

This section is the first build priority.

These are the bridge milestones that most directly make geometry kernels easier
to build, easier to debug, and easier to trust. They are the bridge features
that keep topology-aware truth and derived computation aligned without forcing
kernel code to devolve into manual invalidation and opaque rebuild logic.

If this section is weak, the kernel will inherit the classic dual-runtime
failure mode:

- topology changes route into recomputation inconsistently
- recomputation uses the wrong truth view
- lineage and identity evolution lose subscription continuity
- debugging stops at the runtime boundary instead of tracing through it

## Milestone 1: Patch-to-Invalidation and Snapshot Evaluation

### Goal

Make committed truth changes drive deterministic derived invalidation over
stable truth snapshots.

### Must Ship

- patch-to-invalidation routing as a first-class bridge surface
- snapshot-backed signal evaluation over committed truth
- deterministic invalidation routing for identical patch input
- bridge-side routing diagnostics for what patch items mapped to what invalidations
- bridge acceptance scenarios proving stable snapshot-backed evaluation under active truth mutation

### Must Preserve

- truth remains the only authority
- signal runtime remains the owner of scheduling and execution
- no live mutable truth reads inside bridge evaluation flows
- no scheduler-shaped bridge observability

### Acceptance Evidence

This milestone is complete only when the bridge harness can prove:

- identical committed patchsets route to identical invalidation artifacts
- signal evaluation reads from stable truth snapshots rather than drifting live state
- bridge diagnostics can explain the mapping from patch surfaces to invalidated nodes
- replayed patch routing matches original routing semantics

## Milestone 2: Aspect Mapping and Fine-Grained Subscriptions

### Goal

Preserve truth-side precision across the bridge so derived execution can depend
on exactly the changed truth surfaces instead of coarse whole-object routing.

### Must Ship

- aspect mapping layer between relational aspects and signal aspects
- field, lens, region, or facet subscription shapes where the bridge needs more than whole-entity routing
- deterministic mapping semantics for aspect-aware and field-aware subscriptions
- diagnostics for unmapped, ambiguous, or suppressed aspect routing
- bridge artifact surfaces that show why a change mapped to a given signal dependency slice

### Must Preserve

- explicit aspect semantics on both sides
- no hidden truth-runtime leakage into signal dependency ownership
- deterministic mapping order and reduction
- stable snapshot-backed reads for fine-grained subscriptions

### Acceptance Evidence

This milestone is complete only when the bridge harness can prove:

- fine-grained truth changes invalidate only the intended derived surfaces
- coarse and fine subscription routes remain parity-safe with bridge diagnostics
- aspect mapping behavior is replayable and explainable

## Milestone 3: Lineage-Aware Subscription Continuity

### Goal

Keep signal subscriptions intelligible when truth identity evolves through
replace, split, merge-like, or branch-local topology changes.

### Must Ship

- lineage-aware continuity rules for bridge subscriptions
- remapping behavior for replace and split-style truth evolution
- explicit handling for ambiguous continuity cases
- bridge diagnostics explaining how a subscription continued, split, or failed continuity
- historical lookup support where the bridge needs historical ID resolution to preserve continuity

### Must Preserve

- truth runtime remains the owner of lineage semantics
- signal runtime remains the owner of derived node identity
- no silent subscription drift across identity evolution
- explicit failure rather than accidental continuity when mapping is ambiguous

### Acceptance Evidence

This milestone is complete only when the bridge harness can prove:

- truth identity evolution preserves or rejects subscription continuity deterministically
- topology-style replace/split flows remain traceable through bridge diagnostics
- replayed lineage-aware routing matches original continuity behavior

## Milestone 4: Historical and Branch-Aware Evaluation

### Goal

Allow derived computation to evaluate intentionally against retained historical
truth and branch-local truth, not only the latest committed state.

### Must Ship

- historical snapshot evaluation contracts
- branch-aware bridge semantics for branch-local truth
- diagnostics showing what truth branch and truth version a derived run used
- parity checks between original historical evaluation and replayed historical evaluation
- bridge harness scenarios for divergent topology history and branch-local analysis

### Must Preserve

- branch identity remains explicit
- historical evaluation does not mutate truth
- no collapse of truth branches into derived-branch ownership
- canonical routing and observability over historical reads

### Acceptance Evidence

This milestone is complete only when the bridge harness can prove:

- historical and branch-local evaluation uses the intended truth surface
- branch-local truth does not leak into unrelated derived runs
- historical bridge evaluation remains replayable and diagnosable

## Milestone 5: Bulk Routing and Bridge Scale Path

### Goal

Make the bridge scale to large topology and geometry change sets without
degrading into per-event overhead or opaque routing behavior.

### Must Ship

- planned routing and reduction artifacts for large patchsets
- bulk change propagation as a first-class bridge path
- canonical ordering and reduction for bridge routing outputs
- bridge counters for routed item count, reduction width, subscription fanout, and fallback behavior
- performance-aware routing paths that preserve deterministic observability

### Must Preserve

- no hidden loss of bridge precision under load
- no non-deterministic reduction behavior
- no scheduler-shaped routing artifacts
- bridge diagnostics remain bounded and structured

### Acceptance Evidence

This milestone is complete only when the bridge harness can prove:

- large patchsets route through planned bulk paths rather than per-item ad hoc handlers
- routing artifacts remain deterministic and replayable
- bridge counters explain the scale behavior honestly

---

## Beyond Geometry Kernel Critical Path

Everything below this break still matters for the full product vision, but it
is less directly responsible for making a geometry kernel easy to build. This
is where the bridge roadmap expands from “make truth and recomputation sane for
kernel work” to “finish the full dual-runtime platform.”

## Milestone 6: Change Stream Protocol and Multi-Consumer Contracts

### Goal

Turn bridge-side change consumption into a stable protocol surface rather than
one host-specific feed path.

### Must Ship

- explicit bridge-facing change stream protocol
- stream correctness semantics for ordering, resume, checkpoint, replay, and idempotence
- bridge contracts that support more than one downstream consumer shape
- diagnostics for cursor, checkpoint, replay, and protocol mismatch failures

### Must Preserve

- canonical truth patch order
- deterministic interpretation of stream material
- no weakening of truth-runtime CDC semantics
- no host-specific glue becoming the public contract

### Acceptance Evidence

This milestone is complete only when bridge harness scenarios can prove:

- resumed and replayed consumption preserve bridge routing semantics
- multi-consumer protocol behavior stays deterministic
- protocol errors are explicit and diagnosable

## Milestone 7: Structural-Identity-Aware Remapping

### Goal

Use structural identity and structural fingerprint surfaces to improve bridge
mapping, reuse, and branch comparison without fusing runtime identities.

### Must Ship

- structural-identity-aware remapping rules where the bridge can use them safely
- explicit diagnostics for structural match, structural ambiguity, and structural mismatch
- remapping artifacts that remain subordinate to truth/runtime identity ownership
- harness scenarios covering structural reuse and branch comparison behavior

### Must Preserve

- structural identity never replaces authoritative truth identity
- no accidental ID fusion across runtimes
- explicit failure or ambiguity reporting when structural signals are insufficient

### Acceptance Evidence

This milestone is complete only when bridge harness scenarios can prove:

- structural identity can assist remapping without overriding truth identity
- ambiguous structural matches are explicit and replayable

## Milestone 8: Speculative Truth-Branch to Signal-Branch Coordination

### Goal

Coordinate speculative truth branches and speculative derived execution
branches without collapsing them into one runtime model.

### Must Ship

- explicit coordination rules between speculative truth branches and speculative signal branches
- preview or non-authoritative bridge flows for branch-local evaluation
- discard and commit semantics for speculative bridge outcomes
- diagnostics for speculative branch mismatch, invalid reuse, and branch leakage

### Must Preserve

- truth authority remains separate from speculative computation
- speculative derived state never becomes authoritative accidentally
- branch identity remains explicit end-to-end

### Acceptance Evidence

This milestone is complete only when bridge harness scenarios can prove:

- speculative truth and speculative compute stay coordinated deterministically
- discard paths leave no authoritative bridge residue
- committed speculative flows become explainable and replayable

## Milestone 9: End-to-End Causality and Bridge Certification

### Goal

Make the bridge fully certifiable as a causal protocol boundary, not just an
integration convenience layer.

### Must Ship

- end-to-end causality propagation from truth commit through invalidation to derived explanation
- bridge-native failure taxonomy
- bridge certification artifacts for routing, mapping, historical evaluation, and replay parity
- one public bridge diagnostics entrypoint
- reusable bridge harness suites for patch-driven, historical, branch-aware, and failure-path scenarios

### Must Preserve

- separate runtime ownership on both sides
- canonical observability and replay behavior
- bridge artifacts remain machine-checkable and bounded

### Acceptance Evidence

This milestone is complete only when the bridge harness can prove:

- end-to-end causality survives original execution and replay
- bridge failure classes are explicit and structured
- certification artifacts are sufficient to diagnose routing and historical-evaluation failures mechanically

## Completion Standard

The bridge roadmap is complete only when:

- all geometry-kernel-critical bridge milestones are shipped
- all broader dual-runtime bridge milestones are shipped
- bridge harness scenarios cover patch routing, aspect mapping, lineage continuity, historical evaluation, branch coordination, bulk routing, and causality transfer
- bridge diagnostics and artifacts are canonical, machine-checkable, and replay-safe

## Companion Documents

- [_docs/engineering/forge_runtime_bridge_vision.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/engineering/forge_runtime_bridge_vision.md)
- [_docs/forge-relational/forge_relational_roadmap.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/forge_relational_roadmap.md)
- [_docs/forge_signal/forge_signals2.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge_signal/forge_signals2.md)
