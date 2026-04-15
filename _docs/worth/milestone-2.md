# Milestone 2 Engineering Spec: Derived Topology Materialization, Bridge-Causal Invalidation, And Rebuildable Interpretation

> **Status:** Complete
>
> **Roadmap parent:** [worth_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/worth_roadmap.md)
>
> **Predecessor:** [milestone-1.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-1.md)
>
> **Predecessor closeout:** [milestone-1-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-1-closeout.md)
>
> **Closeout:** [milestone-2-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-2-closeout.md)
>
> **Vision parent:** [VISION.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/VISION.md)
>
> **Test requirements:** [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)
>
> **Primary architectural driver:** make derived topology an explicit, rebuildable, bridge-causal layer over authoritative truth before topology editing, geometry binding, or regeneration widen the read side
>
> **Companion docs:**
> - [MENTALITY.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/MENTALITY.md)
> - [arch_laws.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/arch_laws.md)
> - [perf_laws.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/perf_laws.md)
> - [domain_laws.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/domain_laws.md)
> - [worth_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/worth_roadmap.md)
> - [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)
> - [forge_test_architecture.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_test_architecture.md)
> - [forge_runtime_bridge_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-runtime-bridge/forge_runtime_bridge_roadmap.md)
> - [forge_relational_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-relational/forge_relational_roadmap.md)

## Goal

Define one explicit derived-topology layer for Worth that is:

- rebuildable from authoritative truth alone
- causally driven by bridge-routed truth changes
- deterministic across mainline, branch-local, and replayed reads
- semantically pure enough that interpretation never redefines authority

## Why This Milestone Exists

Milestone 2 is not "cache a topology view."

It is the milestone that decides whether Worth becomes:

- a system where derived topology is an honest, typed, and replayable projection
  over authoritative truth, or
- a system where materializers, validators, caches, and bridge helpers quietly
  become a second topology runtime

Everything later depends on this line being frozen:

- topology editing needs a trustworthy derived read surface before edit
  semantics can be widened
- geometry binding must attach to authoritative truth while consuming a derived
  topology interpretation that is explicitly subordinate to that truth
- regeneration and later geometry analysis must invalidate and recompute from
  stable causal inputs rather than topo-owned hidden state
- diagnostics and agent workflows must be able to explain exactly which truth
  change invalidated which derived interpretation and why

If Milestone 2 is vague, later milestones will smuggle meaning into caches,
materializers, ad hoc bridge routing, or validator-local summaries. This spec
exists to stop that failure before geometry and editing work make it expensive.

## Governing Summaries

- `MENTALITY.md`
  The most important thing it protects here is solving the hard structural
  problem before widening features. Milestone 2 therefore freezes honest
  derivation, invalidation, and replay semantics now instead of bolting them on
  after topology editing and geometry work already depend on them.
- `arch_laws.md`
  The most important thing it protects here is the authority-versus-derivation
  boundary. Derived topology must be reproducible, proof-bearing, and clearly
  downstream of authoritative truth rather than a shadow authority object.
- `perf_laws.md`
  The most important thing it protects is semantic-delta-bounded recompute.
  Milestone 2 must make invalidation breadth, rebuild breadth, and fallback
  breadth explicit rather than allowing hidden whole-view work to masquerade as
  incremental derivation.
- `domain_laws.md`
  The most important thing it protects is responsibility-shaped decomposition.
  Materialization, interpretation, bridge mapping, invalidation policy,
  replay/parity proof, and certification must be separate subdomains rather
  than one generic `derived_topology` bucket.
- `VISION.md`
  The most important thing it protects is that the spec graph is the truth and
  derived projections are disposable. Milestone 2 must therefore make topology
  read models, diagnostics, and future geometry consumers rebuild honestly from
  authority.
- `worth_roadmap.md`
  The most important thing it protects is sequencing. Milestone 2 belongs here
  because topology editing, geometry binding, and regeneration should build on
  a trustworthy derived topology layer instead of inventing one ad hoc.
- `worth/test-requirements.md`
  The most important thing it protects is proof over workflow classes instead
  of one recompute demo. Milestone 2 is not closed until branch, replay, and
  bridge-causal derived workflows are machine-checkable across the admitted
  Milestone 1 primitive families.
- `milestone-1.md`
  The most important thing it protects is the frozen topology and naming
  authority boundary. Milestone 2 must consume that boundary rather than
  redefining truth through derivation.
- `milestone-1-closeout.md`
  The most important thing it protects is the concrete proof bar already
  earned. Milestone 2 must inherit that authoritative substrate and extend it
  only by honest derived behavior, not by reopening Milestone 1 assumptions.
- `forge_test_architecture.md`
  The most important thing it protects is scalable harness shape. Milestone 2
  should use phase-oriented fixtures, explicit certification grammar, and thin
  milestone declarations rather than growing milestone-local test ceremony.
- `forge_runtime_bridge_roadmap.md`
  The most important thing it protects is that truth-to-derived causality lives
  in the bridge. Milestone 2 must let the bridge own routing and historical
  truth evaluation instead of embedding manual invalidation inside topo code.
- `forge_relational_roadmap.md`
  The most important thing it protects is that relational truth remains the
  source of authority. Milestone 2 must read and replay from committed truth
  snapshots rather than from topo-owned mutable state.

## Adversarial Constraint

Milestone 2 must survive this hostile condition:

> Worth must be able to materialize, interpret, invalidate, replay, and inspect
> derived topology across arbitrary admitted Milestone-1 truth histories, where
> mainline, branch-local, and replayed reads all converge on the same shell,
> wire, boundary, radial, and naming-adjacent interpretation without any
> cache, materializer, validator, or bridge helper becoming a second authority
> surface.

Concretely, the design fails if any admitted path:

- stores derived topology meaning in a cache that cannot be destroyed and
  rebuilt from authoritative truth alone
- allows bridge routing to be manual, implicit, or partially encoded in topo
  callsites instead of being explicitly declared
- allows a validator or interpreter to silently widen, narrow, or reinterpret
  Milestone-1 topology truth
- allows replay or branch-local reads to produce different derived summaries
  for the same authoritative history
- hides whole-view rebuilds, invalidation fanout, or fallback breadth behind
  cheap-looking APIs
- requires a future geometry or editing milestone to renegotiate what counts as
  the "real" topology view

The hostile question for this milestone is:

`if every topology cache, materialized view, and bridge-owned record disappeared tomorrow, could Worth rebuild the same derived topology meaning from authoritative truth and the same declared bridge contracts alone?`

## Product Decision Lock

The following decisions are locked in this milestone:

- derived topology is not authoritative truth
- derived topology must be reproducible from authoritative truth alone
- bridge routing owns truth-to-derived causality for topology and naming-driven
  invalidation
- materialization, interpretation, and validation are separate derived phases
  with separate proof surfaces
- branch-local and replayed derived reads must preserve the same meaning for
  the same authoritative history
- whole-view fallbacks may exist only as explicit, counted, and certified
  policy surfaces
- no topology editing, geometry binding, or regeneration milestone may invent
  its own hidden derived topology substrate

Normative consequence:

- any implementation that persists the "real" topology read model outside
  rebuildable derived state is out of spec
- any implementation that encodes invalidation as hand-maintained topo-side
  bookkeeping instead of bridge-owned mapping contracts is out of spec
- any implementation that lets materialization and interpretation collapse into
  one opaque read helper is out of spec
- any implementation that depends on replaying cached derivation artifacts
  rather than replaying authoritative truth is out of spec

## Scope

### In Scope

- signal-backed topology materialization for the Milestone-1 admitted topology
  surface
- bridge mappings from topology and naming truth aspects into derived topology
  invalidation scopes
- rebuildable topology interpretation for:
  - shell classification
  - wire classification
  - boundary interpretation
  - radial and admitted non-manifold adjacency interpretation
  - branch-aware and replay-aware read parity
- derived topology validation over the materialized interpretation surface
- explicit diagnostics for:
  - truth-to-derived invalidation
  - rebuild breadth
  - fallback breadth
  - changed interpretation scope
- certification surfaces proving branch-local, replay, and bridge-causal parity
  over the admitted Milestone-1 families

### Admitted Surface

Milestone 2 admits only derived workflows over the Milestone-1 admitted truth
class:

- `WireOpen(n)`
- `WireClosed(n)`
- `WireBranch(k)`
- `SheetDisk(n)`
- `SheetPatch(f)`
- `SolidShell(f)`
- `NmtEdgeFan(k)`

Admitted derived workflow classes are:

- materialize derived topology from any admitted authoritative snapshot
- invalidate derived topology from admitted topology and naming truth deltas
- rebuild the same derived interpretation from:
  - current mainline truth
  - branch-local truth
  - replayed truth
  - historically evaluated branch heads
- certify changed interpretation and unchanged interpretation with explicit
  digests and counters

For each admitted family, Milestone 2 must preserve the same canonical proof
roles used to close Milestone 1:

- `Smallest`
- `Generic`
- `HostileAdmitted`
- `OutOfClass`

Milestone 2 is a full-robustness milestone for its admitted derived workflow
surface.

That means:

- every admitted truth change must either invalidate and rebuild correctly or
  fail cleanly with explicit diagnostics
- no admitted family may depend on one showcase derived view or one tiny edit
  history to count as supported
- anything not fully supported must remain outside the admitted derived surface
  and fail closed

### Excluded Surface

- topology editing semantics themselves
- geometry materialization or topology-to-geometry binding
- feature graphs and regeneration
- speculative writeback from derived topology into authority
- broad optimization surfaces beyond explicit and tested fallback policies
- unsupported non-manifold classes excluded by Milestone 1

Milestone 2 may reserve future surfaces for editing, geometry, and
regeneration. It may not pretend those surfaces are already served by the
derived topology layer.

### Explicitly Out Of Scope

- broad topology edit-operator catalogs
- geometry carrier semantics
- UV-space or trim interpretation
- feature-intent semantics
- boolean, blend, or healing workflows
- bridge writeback authority
- long-lived cross-domain cached projections that are not rebuildable from
  truth

## Derived Semantics Lock

Milestone 2 must make implicit derived-topology assumptions explicit.

The following meanings must be explicit and frozen in this milestone:

- what counts as a materialized topology view
- what counts as an interpreted topology summary
- which summaries are authoritative reads of truth versus derived convenience
- what exact truth aspects can invalidate which derived topology scopes
- what exact equivalence basis justifies reusing a prior derived result
- when a whole-view fallback is permitted
- how branch-local identity and replay basis are carried through derivation

The equivalence basis must be explicit enough to answer all of these questions
mechanically:

- what exact authoritative truth digest the derived read consumed
- what exact invalidation target set was declared
- what exact branch and replay basis the read was evaluated against
- what exact comparison justifies cache reuse, rebuild suppression, or parity
  claims

No future milestone may rely on:

- "the materializer already knows what changed"
- "this read helper implicitly rebuilds enough"
- "the cache key is obvious from the shape"
- "branch-local reads behave like mainline reads most of the time"

If a later geometry or editing milestone needs one of those assumptions, it
must already be encoded explicitly by Milestone 2 types, contracts, and proof
artifacts.

## Authority And Derivation Model

Milestone 2 must preserve one non-negotiable relation split:

- authoritative topology and naming truth remain in `forge-relational`
- bridge mapping declarations describe truth-to-derived invalidation ownership
- `worth-topo` materialization consumes authoritative truth snapshots
- `worth-topo` interpretation consumes materialized topology
- derived validators consume interpreted topology
- certification consumes the proof surfaces emitted by those derived phases

Representative proof-bearing flow:

```rust
AuthoritativeTopologySnapshot
    -> DerivedTopologyReadBasis
    -> TopologyInvalidationPlan
    -> MaterializedTopologyView
    -> InterpretedTopologyView
    -> DerivedTopologyValidationReport
    -> CertifiedDerivedTopologyRead
```

Rules:

- `TopologyInvalidationPlan` is derived from declared truth-to-derived mapping,
  not from ad hoc invalidation code
- `MaterializedTopologyView` may not redefine or patch authority
- `InterpretedTopologyView` may not silently widen Milestone-1 truth meaning
- `DerivedTopologyValidationReport` may validate interpretation, but may not
  become the only place where interpretation meaning exists
- `CertifiedDerivedTopologyRead` must carry the exact proof surfaces needed by
  diagnostics, replay, and later milestones without re-deriving hidden facts
- no derived read may claim reuse, suppression, or parity without an explicit
  equivalence contract visible in the proof artifacts

## Phases

These phases are the implementation sequencing of the Milestone-2 authority and
derivation model. They are not an alternate model.

### Phase 1: Freeze the derived read basis

Define the one explicit proof-bearing input to derived topology.

This phase must freeze:

- the authoritative snapshot basis for derivation
- branch identity and replay identity carried into derivation
- truth digest and mutation-basis identity carried into derivation
- the exact handoff boundary from authority to derivation

This phase exists to stop later code from deriving topology from "whatever read
state is on hand."

Implementation targets:

- `worth-schema`
  - define the proof-bearing basis types and identifiers needed for derived
    topology reads
- `worth-topo`
  - consume only the proof-bearing derived read basis for Milestone-2
    materialization entrypoints

### Phase 2: Freeze truth-to-derived invalidation ownership

Define the one explicit mapping layer from authoritative topology and naming
truth changes into derived topology invalidation scopes.

This phase must freeze:

- the topology and naming aspect surfaces that can invalidate derived topology
- the invalidation target vocabulary
- the mapping ownership boundary between Worth and `forge-runtime-bridge`
- the rule that invalidation declarations dominate runtime discovery

This phase exists to stop topo-local helpers from becoming an implicit
invalidation engine.

Implementation targets:

- `worth-schema`
  - freeze the topology and naming aspect surfaces visible to derivation
- `worth-topo`
  - define the mapping pack from truth aspects to derived topology invalidation
    scopes
- `forge-runtime-bridge`
  - consume the Worth mapping pack without redefining Worth semantics

### Phase 3: Freeze materialization ownership

Define what a materialized topology view is, and what it is not.

This phase must freeze:

- the exact materialized topology view vocabulary
- what structural facts are copied directly from truth
- what structural adjacency or indexing is derived only for read efficiency
- the rule that materialization may not reinterpret topology truth

This phase exists to stop materialization from collapsing into interpretation.

Implementation targets:

- `worth-topo`
  - define `MaterializedTopologyView` and its substructures
  - separate materialization helpers from interpretation and validation helpers
  - expose explicit counters for materialization breadth and whole-view fallback

### Phase 4: Freeze interpretation ownership

Define what interpretation means over a materialized topology view.

This phase must freeze:

- wire interpretation ownership
- shell interpretation ownership
- boundary interpretation ownership
- radial and admitted NMT interpretation ownership
- the exact outputs that interpretation is allowed to produce

This phase exists to stop validators from becoming de facto interpreters and to
stop later milestones from hiding meaning in helper-specific summaries.

Implementation targets:

- `worth-schema`
  - define shared interpretation record shapes where they must survive beyond a
    local helper boundary
- `worth-topo`
  - split interpretation by real semantic subdomain
  - define explicit interpreted-view or interpretation-record types

### Phase 5: Freeze derived-validator ownership

Define what derived validators are allowed to judge and what they are not
allowed to redefine.

This phase must freeze:

- validator ownership over interpreted topology
- validator family splits
- the exact derived diagnostics emitted by each validator family
- the rule that derived validators may reject derived inconsistency without
  redefining authoritative truth

This phase exists to preserve semantic purity between truth, interpretation,
and validation.

Implementation targets:

- `worth-topo`
  - separate derived validators by semantic family
  - make validator inputs proof-bearing interpreted types rather than raw
    materialized structures where appropriate

### Phase 6: Freeze rebuild, replay, and branch-local parity ownership

Define the one honest parity story for derived topology.

This phase must freeze:

- replay parity basis
- branch-local parity basis
- cross-branch parity basis
- equivalence contracts for derived reuse or parity claims
- the exact digests and diagnostics that prove identical derived meaning

This phase exists to stop cache coincidence or scheduling order from being
mistaken for semantic parity.

Implementation targets:

- `worth-topo`
  - define parity reports and equivalence bases for materialized and
    interpreted topology
- `forge-runtime-bridge`
  - support historical truth evaluation inputs needed by the parity contract

### Phase 7: Freeze fallback and diagnostic surfaces

Define what happens when Milestone-2 derivation cannot stay narrow.

This phase must freeze:

- whole-view fallback classes
- fallback admission rules
- fallback counters
- diagnostics for invalidation breadth, rebuild breadth, changed
  interpretation scope, and replay mismatch scope

This phase exists to stop expensive or broad behavior from hiding behind one
generic "recomputed" label.

Implementation targets:

- `worth-topo`
  - emit named counters and diagnostic records for invalidation, rebuild, and
    fallback behavior
- `worth-schema`
  - preserve any shared fallback or diagnostic vocabulary that must survive
    closeout artifacts

### Phase 8: Implement corpus-shaped certification and closeout

Prove the derived layer over the admitted Milestone-1 families.

This phase must freeze:

- the canonical derived-workflow corpus
- the bridge-proof corpus
- the replay and branch-local derived parity corpus
- the machine-checkable closeout artifact for Milestone 2

This phase exists to stop Milestone 2 from being "one materialized view works."

Implementation targets:

- `worth-topo`
  - extend the certification harness and fixtures for derived topology closeout
- `worth-schema`
  - support any additional fixture vocabulary needed for honest Milestone-2
    scenario generation

## Must Ship

- one explicit proof-bearing derived read basis
- one explicit truth-to-derived invalidation mapping pack for topology and
  naming aspect changes
- one materialized topology view layer that is rebuildable from authoritative
  truth alone
- one interpreted topology layer for shell, wire, boundary, and radial meaning
- one derived-validator subsystem over interpreted topology
- one explicit equivalence-contract surface for reuse, suppression, parity, and
  rebuild claims
- one branch-local and replay parity surface for derived topology
- one explicit fallback and diagnostic surface for whole-view or broad rebuild
  behavior
- one corpus-shaped bridge proof over admitted Milestone-1 families
- one Milestone-2 closeout artifact emitted as direct machine-checkable output

### Milestone-lock decisions that must exist by closeout

- derived read basis frozen
- invalidation target vocabulary frozen
- materialization/interpretation/validation ownership frozen
- derived equivalence basis frozen
- replay and branch-local equivalence basis frozen
- fallback classes frozen
- bridge proof corpus frozen
- closeout output vocabulary frozen

## Must Preserve

- Milestone-1 authoritative truth remains the only topology authority
- Milestone-1 commit-boundary legality remains upstream of all derived work
- persistent naming truth remains authoritative rather than cache-owned
- branch-local history semantics remain inherited from authoritative truth
- bridge routing owns causality, not topo-local helpers
- derived state remains destroyable and rebuildable from authority alone
- later milestones remain free to widen geometry and editing semantics without
  renegotiating what derived topology means

## Acceptance Evidence

Milestone 2 is not closed by one recompute demo or one topology view digest.

### Required workload surface

Run deterministic derived-topology workloads containing:

- primitive-corpus coverage for the Milestone-1 admitted families under
  materialization, interpretation, validation, replay, and branch-local reads,
  including for each family:
  - `Smallest`
  - `Generic`
  - `HostileAdmitted`
  - `OutOfClass`
- arbitrary admitted seeded topology truth
- arbitrary admitted local authoritative truth mutations from Milestone 1's
  admitted authority workflows
- arbitrary admitted shell, wire, and radial counts within the Milestone-1
  admitted class
- mainline reads, branch-local reads, replayed reads, and historical
  branch-head evaluations of the same truth history
- bridge-routed invalidation from topology and naming truth changes into
  derived topology scopes
- explicit whole-view fallback cases where narrow invalidation is not admitted

The bridge proof corpus must include at minimum:

- one `WireBranch(k)` member
- one `SheetPatch(f)` member
- one `SolidShell(f)` member
- one `NmtEdgeFan(k)` member
- one smaller wire-family member

### Must verify

- identical truth histories produce identical materialized topology views
- identical truth histories produce identical interpreted topology summaries
- identical truth histories produce identical derived-validator outcomes
- bridge invalidation remains deterministic and attributable to declared truth
  aspects
- branch-local and replayed derived reads preserve the same meaning for the
  same authoritative history
- derived validator and interpretation coverage remain attributable by admitted
  primitive family, so one family cannot hide behind another family's totals
- whole-view fallbacks are explicit, counted, and diagnostically localized
- reuse and rebuild suppression claims are justified by an explicit
  equivalence-contract artifact
- no derived cache or materialized view becomes the only surviving source of
  topology meaning

### Required machine-checkable outputs

- `materialized_topology_digest`
- `interpreted_topology_digest`
- `derived_validation_digest`
- `derived_truth_basis_digest`
- `bridge_routing_digest`
- `bridge_historical_evaluation_digest`
- `derived_family_coverage_matrix`
- `derived_family_parity_matrix`
- `derived_validator_coverage_report`
- `derived_invalidation_report`
- `derived_rebuild_report`
- `derived_equivalence_contract_report`
- `derived_fallback_report`
- `derived_failure_locality_report`
- `derived_branch_local_parity_report`
- `derived_replay_parity_report`
- `derived_bridge_family_coverage_report`
- `milestone_2_counter_report`

These outputs must be emitted as direct closeout surfaces for the milestone,
not only as nested helper artifacts.
Derived validator coverage must remain attributable by admitted primitive
family, validator family, and derived phase so a missing validator path cannot
hide behind family-level success totals.

### Non-closure conditions

Milestone 2 is not closed if any of the following remain true:

- materialization and interpretation are fused into one opaque helper
- invalidation scope is still partly implicit or discovered ad hoc at runtime
- replay parity is proven only on one branch or one tiny edit history
- whole-view fallback exists but is not counted and localized explicitly
- bridge proof depends on one showcase family instead of a corpus-shaped set
- reuse or suppression claims exist without a machine-checkable equivalence
  basis
- a later milestone would still need to redefine what a derived topology read
  means
- a derived topology artifact cannot be rebuilt solely from authority plus the
  declared bridge contracts

## Architectural Notes

- Milestone 2 must make implicit derived-topology assumptions explicit rather
  than leaving them encoded in helper order, cache shape, or callsite
  conventions.
- Semantic purity matters more than convenience here:
  - materialization may accelerate reading
  - interpretation may summarize meaning
  - validation may classify derived consistency
  - none of them may redefine authoritative truth
- This milestone should prefer proof-bearing derived types and declared
  contracts over helper bags or boolean flags.
- The test architecture must continue following
  [forge_test_architecture.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_test_architecture.md):
  - phase-oriented fixtures
  - certification grammar separated from domain meaning
  - milestone closeout as thin declaration over reusable proof infrastructure

## Sequencing Notes

This belongs immediately after Milestone 1 because:

- topology editing should consume a trustworthy derived topology layer instead
  of inventing one ad hoc
- geometry binding should land on top of explicit derived topology semantics,
  not on top of hidden materializer assumptions
- regeneration and later geometry analysis need bridge-causal and replay-safe
  derived behavior before their own workflow surfaces widen

Milestone 2 therefore solves the hard read-side structural problem first:

- not more topology authority
- not more operator coverage
- but honest derivation, causality, and rebuildability over the authority we
  already froze
