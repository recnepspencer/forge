# Milestone 12 Engineering Spec: Aspect-Causal Invalidation

> **Status:** Planned
>
> **Architecture parents:**
> - [signal_architecture2.md](./signal_architecture2.md), especially `S2.2` and `S9.16.3`
> - [s9_16_acceptance_map.md](./s9_16_acceptance_map.md)
>
> **Inherited closeout:** [milestone-d-closeout.md](./milestone-d-closeout.md)
>
> **Successor:** [milestone-13-plan.md](./milestone-13-plan.md)

## 1. Goal And Roadmap Placement

Milestone 12 makes aspect invalidation causally correct across every dependency
hop in `worth-signal`.

The milestone reopens the semantic portion of `S9.16.3`. The existing runtime
is precise for many direct subscribers, but its transitive frontier copies the
original seed aspect through every descendant. That is not a lawful
interpretation of aspect truth: an aspect identifies part of one producer's
output contract and cannot be reinterpreted as a descendant producer's output
without a realized evaluation result.

Milestone 12 therefore establishes one rule:

> Downstream aspect authority comes only from the immediate dependency's
> committed output delta.

Milestone 13 may trust that rule when it changes frontier mechanics. Milestone
12 must certify that rule while it is implemented; semantic certification is
not deferred to a later milestone.

## 2. Current Boundary

The current invalidation path has three materially different facts:

1. a source mutation declares one changed source aspect
2. a direct subscriber is admitted using its contract and the source delta
3. descendants are structurally reachable from that subscriber

The first two facts can carry aspect meaning. The third fact cannot. Today,
the transitive application path treats reachability as if it proved that the
original source aspect changed on every intermediate producer. It then stores
that copied aspect in each descendant's `dirty_aspects` and condition admission
consumes that mask.

This permits a false clean/deferred outcome. In the smallest hostile graph:

```text
source produces A
  -> middle consumes A and produces B
     -> leaf consumes B and is gated by AspectFilter(B)
```

the leaf can receive `A` as its dirty aspect. `AspectFilter(B)` then defers the
leaf even after the middle's `B` output changed.

The present surfaces that encode the problem include:

- `FrontierWavePlan.aspect`, which assigns one aspect to a whole wave
- `TransitiveFrontierRoot.aspect`, which carries the seed aspect forward
- `execute_transitive_wave`, which marks every reachable descendant with that
  aspect
- `NodeEntry.dirty_aspects`, which does not preserve the immediate dependency
  that authorized each aspect fact
- condition eligibility, which assumes the stored mask is valid local cause

Code is evidence of present reality, not authority over the destination.

## 3. Adversarial Financial Courtroom

Milestone 12 must expand the existing production-shaped fintech financial
world under `tests/domains/fintech`; a parallel generic graph world is not
accepted as closeout evidence. Generic focused tests remain useful for local
mechanics, but the milestone claim belongs to named financial scenarios that
enter through the real runtime mutation/evaluation composition root.

Each scenario owns a causally complete baseline, one named financial delta,
the expected financial truth, an independently declared necessity set, a
plausible defect, and a mutation probe. A scale or seed generator may vary a
scenario, but generated topology is not itself a scenario and cannot replace
the named courtroom.

The required semantic scenarios are:

| Scenario | Financial delta and required truth | Defect it must expose |
|---|---|---|
| `quote_to_risk_aspect_translation` | A primary market `PRICE` change is normalized, repriced, translated into `RISK`, and observed by matched `AspectFilter(RISK)` and unmatched `AspectFilter(ALERT)` twins | copying the root `PRICE` aspect through the price-to-risk hop or treating reachability as output change |
| `tolerance_suppressed_repricing` | one within-tolerance quote move leaves committed price and downstream risk unchanged; a larger twin changes both | propagating after comparator suppression or proving unchanged truth from missing execution |
| `producer_local_factor_slot_collision` | two market dependencies reuse an aspect slot while representing distinct FX/curve meanings; only the economically affected path changes | flattening dependency identity into one ambiguous aspect mask |
| `partitioned_curve_bucket_bump` | a rates `bucket-0` shock changes the rates detail and coarse book views while the credit partition remains unchanged | widening or relabeling partition/detail scope at a hop |
| `gated_repricing_release` | a price/risk dependency remains pending while its threshold or on-demand condition blocks, then becomes eligible and reaches the required descendant | evaluating `AspectFilter` against unresolved ancestor evidence and stranding the descendant |
| `instrument_dependency_rewire` | an instrument moves from one declared market factor/model dependency to another between committed evaluations, including cycle preflight rejection | using stale dependency snapshots or treating topology reachability as current cause |
| `branch_shock_restore_replay` | the same market shock runs on main and analysis branches, restores from checkpoint, and replays to identical financial truth; an attached async-capable audit node preserves independent lifecycle truth | reconstructing causal authority from replay/diagnostics or leaking branch-local invalidation state |

Every scenario must observe committed financial outputs, output versions, node
lifecycle, dependency snapshots, invalidation causes, condition/suppression
decisions, and the relevant branch/restore/replay conclusions.

Two independent oracles are required:

1. `FreshFinancialRecompute` reconstructs the declared financial world from
   authoritative market and portfolio inputs and evaluates it without dirty
   masks, the incremental frontier, incremental condition classification, or
   ready queues.
2. `FinancialNecessityManifest` enumerates economically affected nodes from
   scenario-owned positions, factor subscriptions, aspect translations, and
   partition ownership. It may not call the production routing planner or
   incremental classifiers.

For every scenario step, incremental and fresh execution must agree on
committed outputs and dependency snapshots, and the observed semantic work
must agree with the scenario necessity manifest. Deleting, bypassing,
inverting, or stale-reusing immediate-dependency cause authority must turn at
least one named scenario red.

## 4. Product Decision Lock

### 4.1 Aspects Are Producer-Local Output Meaning

An aspect is interpreted against the producer whose output contract declares
it. Equal aspect indices on different producers are not evidence of equal
meaning.

An invalidation cause must therefore retain:

- the immediate dependency that produced the change
- the changed output aspect mask for that dependency
- the narrowed partition/detail scopes
- the committed output version or equivalent freshness proof
- source-seed references for explanation only, never for local aspect meaning

### 4.2 Reachability And Changed Output Are Different Truths

Structural reachability may establish that a dependency chain must be ordered
or revalidated. It may not populate a descendant's changed-aspect mask.

The runtime must represent unresolved dependency revalidation separately from
realized dependency output change. A node blocked behind unresolved
dependencies remains unresolved; it is not allowed to become clean or to be
permanently deferred by evaluating an aspect condition against ancestor-local
evidence.

### 4.3 Output Commit Is The Transitive Authority Boundary

Initial host mutations may seed source-local aspect deltas. After the direct
edge, further semantic propagation occurs only when evaluation/apply commits a
realized output delta.

Comparator match, unchanged output identity, unchanged continuity, or another
lawful suppression verdict emits no changed-output delta. It may still emit
diagnostic evidence, but that evidence cannot dirty consumers.

### 4.4 Conditions Consume Resolved Cause

`AspectFilter` may admit or reject only from resolved immediate-dependency
change evidence. Pending dependency revalidation is a separate eligibility
state and cannot be encoded as a mismatched aspect.

Temporal, previous-value, on-demand, async-capability, and custom conditions
remain orthogonal policy axes. This milestone must compose with them without
rewriting their authority.

### 4.5 Dirty Masks Are Derived Aggregates

A node-level dirty-aspect mask may remain as a compact hot-path aggregate only
if it is derived from retained immediate-dependency causes and cannot become a
second causal authority. Destroying and rebuilding the aggregate from current
pending causes must reproduce it exactly.

### 4.6 Full Recompute Is An Independent Oracle

The full-recompute path must not call the disputed frontier planner,
incremental condition classifier, dirty-state transition, or output-delta
propagator to determine expected results. Shared public contracts, evaluator
implementations, and authoritative host inputs are allowed; shared disputed
classification is not.

### 4.7 Certification Is An Implementation Gate

Certification forms are built and sealed inside this milestone. A phase cannot
close while the financial scenarios assigned to it are red, incomplete, or
running only through a test-only composition root. Correctness and locality
remain separate verdicts: Milestone 12 owns semantic equivalence and Milestone
13 owns structural locality.

### 4.8 The Runtime Remains Domain-Agnostic

The fintech financial world is the mandatory courtroom, not runtime ontology.
Production invalidation causes, deltas, reports, and sealed runs use generic
graph/aspect/scope vocabulary. Market, quote, curve, instrument, portfolio,
desk, and risk names remain under `tests/domains/fintech`. A future geometry,
imaging, simulation, or other domain courtroom must be able to consume the same
generic certification forms without renaming or wrapping core authority.

## 5. Required Proof-Bearing Forms

The implementation must establish canonical equivalents of these forms:

```rust
pub struct ProducedAspectDelta {
    producer: NodeId,
    changed_aspects: AspectMask,
    changed_scopes: PartitionScopeSet,
    committed_output_version: AspectVersion,
}

pub struct DependencyInvalidationCause {
    dependency: NodeId,
    changed_aspects: AspectMask,
    changed_scopes: PartitionScopeSet,
    committed_output_version: AspectVersion,
    source_seed_refs: CanonicalSeedRefs,
}

pub struct PendingDependencyRevalidation {
    node: NodeId,
    unresolved_dependencies: DedupedNodeBatch,
}

pub struct InvalidationCertificationCase { /* identity, trace, policies */ }
pub struct InvalidationEquivalenceReport { /* independent semantic verdict */ }
pub struct AspectCausalityCertificationRun { /* sealed M12 evidence */ }
```

Names may change only if the replacement preserves the distinctions visibly.
The implementation may not collapse realized output change and unresolved
reachability into one enum variant with optional fields.

Authority direction is fixed:

```text
host/source mutation
  -> source-local seed
  -> direct dependency cause
  -> evaluation and output comparison
  -> committed ProducedAspectDelta
  -> next direct dependency cause
```

Diagnostics, lineage, execution summaries, and replay presentations derive
from this chain. They do not mint it.

## 6. Architectural Destination

Milestones 12-13 commit to this destination topology:

```text
crates/worth-signal/src/
  data/proof/
    invalidation/                         [created directory]
      mod.rs                              [created facade]
      seed.rs                             [moved/replaced from admission forms]
      causality.rs                        [created in Milestone 12]
      plan.rs                             [moved/replaced]
      execution.rs                        [moved/replaced]
      certification/                     [created across Milestones 12-13]
        mod.rs
        case.rs                           [generic case identity, Milestone 12]
        equivalence.rs                    [Milestone 12]
        causality_run.rs                  [Milestone 12]
        cost.rs                           [Milestone 13]
        strategy.rs                       [Milestone 13]
        decision.rs                       [Milestone 13]
        locality_run.rs                   [Milestone 13]
  logic/invalidation/
    mod.rs                                [existing, stable facade]
    subscription.rs                       [existing direct-edge evidence]
    causality/                            [created in Milestone 12]
      mod.rs
      source_seed.rs
      dependency_cause.rs
      output_delta.rs
      pending_revalidation.rs
    routing/                              [existing mechanism boundary]
      mod.rs
      planning.rs
      application.rs
      counters.rs
      evidence.rs
      seeds.rs
    scheduling/                           [committed Milestone 13 sibling]
      mod.rs
      ready_work.rs
      topological_order.rs
  tests/domains/fintech/
    invalidation/                         [created financial proof family]
      mod.rs
      quote_to_risk_aspect_translation.rs [Milestone 12 scenario]
      tolerance_suppressed_repricing.rs   [Milestone 12 scenario]
      producer_local_factor_slot_collision.rs
      partitioned_curve_bucket_bump.rs
      gated_repricing_release.rs
      instrument_dependency_rewire.rs
      branch_shock_restore_replay.rs
      sparse_book_fanout.rs               [Milestone 13 scenario]
      convergent_factor_batch.rs          [Milestone 13 scenario]
      dense_market_close.rs               [Milestone 13 scenario]
    certification.rs                     [existing stable domain facade]
    certification/
      workflow/                           [existing workflow family migrated]
        adapter.rs
        artifact_matrix.rs
        independent_oracle.rs
        scenario.rs
        session.rs
      invalidation/                       [created certification family]
        mod.rs
        financial_scenario.rs
        fresh_recompute.rs
        necessity_manifest.rs
        causality_run.rs
        locality_run.rs                   [Milestone 13]
        cost_slope.rs                     [Milestone 13]
        strategy_decision.rs              [Milestone 13]
```

The dominant axes are:

- `data/proof/invalidation`: immutable proof and summary vocabulary
- `logic/invalidation/causality`: semantic authority derivation
- `logic/invalidation/routing`: application of already-decided invalidation
- `logic/invalidation/scheduling`: replaceable ready-work mechanics
- `tests/domains/fintech/invalidation`: financial scenario ownership and
  adversarial observations
- `tests/domains/fintech/certification`: stable certification facade separating
  workflow and invalidation proof families
- `tests/domains/fintech/certification/invalidation`: independent oracle,
  necessity, and sealed invalidation-evidence construction

The stable operational facade remains `logic/invalidation/mod.rs`. External
callers must not depend on causality or scheduling internals. The existing flat
proof files may be re-exported through `data::proof`, but duplicate ordinary
implementations or compatibility authority lanes are forbidden.

Forbidden placements include new invalidation semantics in `helpers`,
`common`, generic planner files, diagnostics, async lifecycle modules, or the
public facade. A second generic certification world beside the fintech world is
also forbidden; focused graph fixtures must be subordinate to a named financial
scenario and cannot become a competing closeout authority.

Financial names are equally forbidden in production invalidation, proof,
scheduling, or public-facade modules. They belong only to the fintech test
domain and its documentation.

## 7. Ordered Implementation Phases

### M12.0 - Contract And False-Closeout Freeze

- mark `S9.16.3` reopened
- freeze producer-local aspect meaning and the adversarial courtroom
- expand the financial world with failing
  `quote_to_risk_aspect_translation` matched/unmatched twins
- identify every consumer of `dirty_aspects`
- phase gate: the baseline world is causally valid and the translation scenario
  fails for the known inherited-aspect defect

### M12.1 - Causal Proof Forms

- establish source seed, pending revalidation, dependency cause, and produced
  delta as separate forms
- move invalidation proof vocabulary into its committed topology
- prevent diagnostics or summaries from constructing operational causes
- phase gate: `producer_local_factor_slot_collision` distinguishes dependency
  identity and its mutation probe fails if causes are flattened

### M12.2 - Output-Commit Propagation

- make evaluation/apply produce the only transitive output delta
- stop propagation on unchanged/suppressed output
- preserve immediate-dependency identity and narrowed scopes
- phase gate: `quote_to_risk_aspect_translation`,
  `tolerance_suppressed_repricing`, and `partitioned_curve_bucket_bump` agree
  with fresh recomputation

### M12.3 - Planner And Condition Integration

- make planning and condition admission consume resolved local causes
- keep unresolved predecessor state distinct and non-clean
- preserve temporal, async, partition, replay, and branch semantics
- phase gate: `gated_repricing_release` and `instrument_dependency_rewire`
  agree with both independent oracles

### M12.4 - Branch Composition And Sealed Semantic Certification

- finish `FreshFinancialRecompute` and `FinancialNecessityManifest`
- establish the `certification/workflow` and `certification/invalidation`
  sibling topology while preserving existing workflow certification behavior
- run deterministic financial mutation traces through incremental and oracle
  runtimes
- prove `branch_shock_restore_replay`, including diagnostic-tier parity and
  async-capability orthogonality
- add mutation-sensitive negative controls
- seal `AspectCausalityCertificationRun`; construction rejects missing,
  duplicate, stale, or mismatched scenario evidence
- update architecture, test requirements, and acceptance ownership

## 8. Documentation Deliverables

Milestone 12 must revise:

- `signal_architecture2.md`: producer-local aspect meaning and reopened
  `S9.16.3` status
- `s9_16_acceptance_map.md`: replace reachability-only acceptance with causal
  aspect correctness
- `test-requirements.md`: bind the translation twins and independent oracle
- the fintech domain README and scenario catalog: bind every required financial
  baseline, delta, oracle, and mutation probe
- any caller-facing condition documentation that describes `AspectFilter`

The continuing audience is runtime implementers and advanced callers who use
aspect contracts and conditions. Examples must show an aspect-changing
intermediate node, not only same-aspect direct edges.

## 9. Must Ship And Must Preserve

Must ship:

- immediate-dependency causal evidence
- output-commit-driven transitive propagation
- unresolved revalidation distinct from changed aspect
- independent full-recompute differential proof
- scenario-owned financial necessity manifests
- sealed semantic certification produced during implementation
- branch, restore, replay, condition, comparator, partition, and async
  composition coverage

Must preserve:

- `Clean | MaybeStale | Dirty` as graph lifecycle vocabulary
- transaction rollback and commit-bounded observation
- deterministic ordering where requested
- async lifecycle truth as an orthogonal subsystem
- hot/cold diagnostic separation
- public facade compatibility unless a public semantic lie is discovered

## 10. Explicit Exclusions

Milestone 12 does not:

- optimize frontier traversal
- add a priority queue or order-maintenance timestamps
- adopt a tree-only traversal model
- claim scale-local execution
- change host-defined aspect meanings
- let a test-only API bypass the production composition root

## 11. Acceptance Evidence

Milestone 12 closes only when:

- the `A -> B` matched filter recomputes and the unmatched twin does not
- unchanged middle output creates no downstream semantic invalidation
- condition-blocked intermediates cannot strand a required descendant
- producer-local aspect collisions across dependencies remain distinguishable
- partition scopes remain narrowed across each realized hop
- incremental committed outputs and dependency snapshots equal the independent
  full-recompute oracle for every required financial scenario
- observed semantic work equals the independent financial necessity manifest
- branch restore and replay produce the same causal results
- every implementation phase closes only after its assigned financial
  scenarios and mutation probes are green
- `AspectCausalityCertificationRun` rejects missing, duplicate, stale, or
  mismatched scenario evidence
- removing immediate-dependency identity or reintroducing root-aspect copying
  makes the courtroom fail
- focused tests, the complete `worth-signal` suite, boundary checks, context
  checks, formatting, and dirty Rust line-cap checks pass

## 12. Successor Handoff

Milestone 13 may assume that every realized work item carries valid
immediate-dependency change evidence. It may not reinterpret aspects, infer
changed outputs from reachability, weaken either Milestone 12 oracle, or defer
financial certification to a later milestone.
