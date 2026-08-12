# Milestone 13 Engineering Spec: Locality-First Frontier Execution

> **Status:** Planned
>
> **Prerequisite:** [milestone-12-plan.md](./milestone-12-plan.md)
>
> **Architecture parent:** [signal_architecture2.md](./signal_architecture2.md), `S9.16.3`
>
> **Successor:** [milestone-14-plan.md](./milestone-14-plan.md)

## 1. Goal And Roadmap Placement

Milestone 13 makes the physical cost of invalidation scale with the realized
semantic frontier rather than the complete reachable subscriber closure.

Milestone 12 establishes which dependency changes are true. Milestone 13
establishes how those truths become bounded ready work without giving traversal
mechanics authority to widen or reinterpret them.

Together, Milestones 12 and 13 genuinely close `S9.16.3`.
Milestone 13 certifies locality and strategy readiness while it implements the
frontier; there is no later invalidation-certification milestone.

## 2. Current Boundary

The current direct-edge planner checks aspect and partition contracts, but the
transitive application path then:

- seeds every subscriber of each direct root
- visits every reachable descendant
- marks each descendant `MaybeStale`
- enqueues all subscribers regardless of edge contract, aspect, or partition
- reports reachability as the inclusion basis

This can be output-correct in same-aspect examples while still violating the
locality promise. A source-local delta can touch work proportional to the broad
graph even when only one narrow chain is semantically relevant.

The existing acceptance language that transitive waves stay within nodes
"reachable from planned roots" is insufficient. Reachability is an upper
bound on possible topology, not the semantic frontier.

## 3. Adversarial Financial Courtroom

Milestone 13 expands the same fintech financial world certified by Milestone
12. Locality evidence must be expressed as named financial work, not synthetic
graph shapes with financial labels attached after construction.

The required locality scenarios are:

| Scenario | Financial workload and scale axes | Defect it must expose |
|---|---|---|
| `sparse_book_fanout` | one primary quote-to-risk chain of depth 16 is relevant while each level has `10^3`, `10^4`, then `10^5` instruments or audit projections subscribed only to disjoint aspects | complete subscriber-closure walking, late filtering, or hidden non-semantic visits |
| `partitioned_curve_universe` | one rates bucket changes among 16, 256, and 1,024 declared curve/credit regions while overlap density varies independently | partition widening, post-enqueue rejection, or a locality claim that hides region scans |
| `convergent_factor_batch` | price, FX, curve, and volatility shocks overlap on one portfolio risk aggregate, with duplicate causes and insertion-order permutations | deduplication that loses dependency provenance or performs duplicate work |
| `dense_market_close` | a broad market-close revaluation lawfully changes sparse, medium, and dense fractions of the book at `10^3`, `10^4`, and `10^5` nodes | a strategy that wins only on sparse work, violates dense-work cost, or drops necessary nodes |
| `portfolio_dependency_churn` | instruments change desk, factor, and pricing-model dependencies between commits, including cycle rejection and repeated rewiring | stale ready work, graph-order assumptions, or counters that omit topology churn |
| `branch_restore_locality_replay` | the same narrow and dense traces run after branch capture, checkpoint restore, replay, and deterministic rerun | retained frontier state, nondeterministic deduplication, or work moved into reconstruction |

The scheduled lane owns the largest scales; the ordinary change gate includes
small instances of every named scenario, one sparse slope, and one dense slope.
Correctness may not depend only on scheduled execution. Every case records its
scenario identity, seed, financial scale tuple, mutation trace, runtime policy,
diagnostic tier, and cold/warm posture.

Run the same financial deltas at increasing irrelevant fanout, partition count,
and semantic-frontier density.

Required structural result:

- evaluated work equals the semantic necessity set
- admitted ready work equals realized immediate-edge matches
- disjoint subscribers are rejected before enqueue
- duplicate justifications collapse without losing provenance
- graph-size growth in unreachable or contract-disjoint branches does not
  create proportional node visits
- deterministic and optimized modes commit equivalent truth

The courtroom must convict:

- any complete subscriber-closure walk
- filtering only after a node has been enqueued or marked stale
- counters derived from planned estimates rather than realized work
- a queue that silently deduplicates away distinct causal evidence
- a traversal strategy that rechecks contracts or reconstructs semantic intent
- elapsed-time-only performance claims
- a generated graph whose expected necessity set is derived from the same
  subscription traversal being tested
- a standalone certification pass that can turn green after an implementation
  phase has already closed with red financial scenarios

## 4. Product Decision Lock

### 4.1 Semantic Admission Precedes Scheduling

The causality owner from Milestone 12 decides whether an immediate subscriber
receives a work item. Scheduling may order, batch, and deduplicate admitted
items; it may not discover changed aspects, widen scopes, or admit a rejected
edge.

### 4.2 Transitive Work Is Realized Incrementally

There is no precomputed semantic transitive cone. Each committed
`ProducedAspectDelta` admits only its direct matching subscribers. Further work
exists only after another producer commits another delta.

Topology preflight may validate graph legality through a separately named
costed lane. It may not mutate node dirty state or masquerade as semantic
frontier execution.

### 4.3 Ordering Is WORTH Topology Ordering

Ready work must respect the existing lowered stage/topology guarantees,
deterministic mode, cycle law, transaction boundaries, and async/temporal
admission. Milestone 13 does not import tree traversal timestamps or assume a
single rooted hierarchy.

### 4.4 One Canonical Work Item

The implementation must establish a canonical equivalent of:

```rust
pub struct InvalidationWorkItem {
    consumer: NodeId,
    causes: CanonicalDependencyCauseSet,
    narrowed_scopes: PartitionScopeSet,
    readiness: DependencyReadiness,
}

pub struct ReadyInvalidationBatch {
    stage: StageId,
    entries: Vec<InvalidationWorkItem>,
}
```

Deduplication merges causes for the same consumer and readiness epoch. It must
not flatten distinct dependencies into one ambiguous aspect mask.

### 4.5 Traversal Strategy Is Mechanical

The scheduling boundary may expose an internal sealed strategy contract whose
inputs are already-admitted ready batches and whose outputs preserve all work
items exactly once in lawful order.

The first implementation may use the existing bitset/arena strengths where
they remain profitable. The architecture must make a later ready-queue strategy
additive, not require moving semantic authority.

### 4.6 Cost Truth Is Canonical

Canonical execution summaries must report realized counters for at least:

- source deltas consumed
- dependency edges examined
- candidates rejected by aspect
- candidates rejected by partition/detail scope
- work items admitted
- work items merged by deduplication
- ready items enqueued and popped
- nodes evaluated
- produced deltas emitted
- propagation stops from unchanged output
- non-semantic nodes visited by routing or scheduling
- maximum ready-frontier width

Predicted counters may remain as planning estimates only if named separately
and never substituted for realized proof.

### 4.7 Locality Certification Is Sealed Here

Milestone 13 owns the cost-slope, same-work-stream strategy, and final locality
reports formerly separable from implementation. The financial necessity
manifest from Milestone 12 is the independent semantic-work oracle; it may be
extended with scale metadata but may not import routing or scheduling logic.

Correctness and cost remain separate verdicts. A semantically correct case can
fail locality; a cheap case can fail equivalence. The milestone closes only
when both verdicts are green for all required scenario/scale lanes.

Canonical equivalents of these forms are required:

```rust
pub struct InvalidationCostSlopeReport { /* structural work by declared scale */ }
pub struct InvalidationStrategyReport { /* identical admitted work stream */ }
pub struct FrontierLocalityCertificationRun { /* sealed M13 evidence */ }

pub enum TraversalStrategyDecision {
    CurrentStrategyCertified,
    OrderedReadyWorkCandidate(OptimizationEvidence),
    InsufficientEvidence(MeasurementGap),
}
```

The typed decision records measured strategy readiness only. It cannot weaken
the current strategy, prescribe a tree algorithm, or delay the parallel
execution roadmap.

The financial scenario identity and necessity manifest remain test-domain
evidence. Production cost, strategy, decision, and sealed-run forms stay
domain-agnostic so future domain courtrooms consume the same contract.

## 5. Architectural Destination

Milestone 13 populates the destination committed by Milestone 12:

```text
logic/invalidation/
  causality/
    output_delta.rs                       [Milestone 12 authority]
    dependency_cause.rs                   [Milestone 12 authority]
  routing/
    planning.rs                           [narrow admission orchestration]
    application.rs                        [applies admitted direct work only]
    counters.rs                           [realized operational counters]
    evidence.rs                           [derived summaries/traces]
  scheduling/
    mod.rs                                [stable internal facade]
    ready_work.rs                         [queue/batch mechanics]
    topological_order.rs                  [stage/order preservation]

data/proof/invalidation/
  plan.rs                                 [semantic admitted plan]
  execution.rs                            [realized execution truth]
  certification/
    case.rs                               [generic Milestone 12 authority]
    equivalence.rs                        [Milestone 12 authority]
    causality_run.rs                      [Milestone 12 authority]
    cost.rs                               [Milestone 13]
    strategy.rs                           [Milestone 13]
    decision.rs                           [Milestone 13]
    locality_run.rs                       [Milestone 13]

tests/domains/fintech/
  invalidation/
    sparse_book_fanout.rs
    partitioned_curve_universe.rs
    convergent_factor_batch.rs
    dense_market_close.rs
    portfolio_dependency_churn.rs
    branch_restore_locality_replay.rs
  certification/invalidation/
    financial_scenario.rs                 [Milestone 12 authority]
    fresh_recompute.rs                    [Milestone 12 oracle]
    necessity_manifest.rs                 [Milestone 12 oracle]
    locality_run.rs
    cost_slope.rs
    strategy_decision.rs
```

`causality` owns meaning. `routing` converts valid deltas to direct admitted
work. `scheduling` owns ready-work mechanics. `evidence` derives cold or summary
views. Dependency direction follows that order and may not reverse.

The public invalidation facade remains stable. Scheduling implementations are
not exported as caller policy in this milestone.

## 6. Ordered Implementation Phases

### M13.0 - Cost Contract Freeze

- replace reachability acceptance with semantic-frontier acceptance
- define realized counter names and complexity variables
- add failing small/medium `sparse_book_fanout` and
  `partitioned_curve_universe` slopes before changing traversal
- phase gate: financial necessity manifests are independent and the inherited
  closure walk violates the realized counter envelope

### M13.1 - Direct-Hop Realization

- remove pre-marking of the transitive subscriber closure
- make committed output deltas generate only direct admitted work
- reject disjoint aspect/scope candidates before enqueue
- phase gate: sparse fanout and partition-universe scenarios reject disjoint
  work before dirty mutation and enqueue

### M13.2 - Ready-Work Scheduling Boundary

- establish canonical work item and ready batch forms
- preserve topological and deterministic order
- keep scheduling incapable of minting or widening causes
- phase gate: `dense_market_close` preserves every necessary node at sparse,
  medium, and dense frontier densities

### M13.3 - Deduplication And Multi-Source Churn

- merge overlapping justifications canonically
- preserve per-dependency causal identity
- prove stable results across seed order and rewiring permutations
- phase gate: `convergent_factor_batch` and `portfolio_dependency_churn` agree
  with financial truth and necessity oracles across permutations

### M13.4 - Counter And Summary Cutover

- derive execution summaries from realized work
- separate predicted estimates from realized counters
- remove or rename reachability-based metrics that overclaim locality
- phase gate: cost-slope reports expose edge, queue, evaluation, suppression,
  non-semantic visit, memory, and topology-churn work without log scraping

### M13.5 - Strategy Decision And Sealed Locality Closeout

- run every named financial courtroom at ordinary and scheduled scale lanes
- prove deterministic/optimized equivalence
- prove `branch_restore_locality_replay` plus temporal, condition, and async
  composition
- compare mechanical strategies only with identical admitted work streams
- seal `FrontierLocalityCertificationRun` and emit exactly one typed traversal
  strategy decision; reject missing, duplicate, stale, or mismatched evidence
- update `S9.16.3` only after Milestones 12 and 13 are both accepted

## 7. Complexity Contracts

The ordinary invalidation lane must target:

```text
O(source_deltas
  + examined_immediate_subscriber_edges
  + admitted_work_items log-or-better ready-order cost
  + evaluated_nodes)
```

The bound must not contain total reachable descendants or total graph nodes
unless the semantic delta actually reaches them.

Cycle or topology validation that lawfully has a broader bound must be named,
measured, and kept separate from semantic invalidation counters. Background or
amortized work cannot launder an unbounded queue.

## 8. Documentation Deliverables

Milestone 13 must revise:

- `signal_architecture2.md`: locality means semantic reach, not graph reach
- `s9_16_acceptance_map.md`: exact realized counter ownership
- `test-requirements.md`: fanout slope and negative-space requirements
- fintech domain documentation: named locality scenarios, scale lanes, oracle
  ownership, and reproduction metadata
- performance contract registries for every touched hot path

The documentation must explain which counters are predicted, realized,
semantic, and mechanical. A single "frontier width" number is insufficient if
it can conceal rejected or auxiliary work.

## 9. Must Ship And Must Preserve

Must ship:

- direct-hop-only semantic realization
- pre-enqueue aspect/partition narrowing
- canonical ready-work boundary
- stable causal deduplication
- realized structural counters
- scale-sensitive disjoint-fanout proof
- financial cost-slope and strategy reports produced during implementation
- sealed locality certification and typed traversal-strategy decision

Must preserve:

- Milestone 12 causal authority and oracle
- transaction rollback and commit-bounded observation
- cycle rejection before unlawful state commit
- deterministic execution semantics
- branch, replay, temporal, condition, and async lifecycle guarantees
- cold diagnostic policy independence

## 10. Explicit Exclusions

Milestone 13 does not:

- implement an order-maintenance data structure
- assume a tree-shaped graph or fixed global traversal timestamp
- expose a caller-selectable queue implementation
- select a strategy from unmeasured heuristics
- claim geometry or any other domain readiness from financial evidence alone
- replace correctness counters with wall-clock thresholds

## 11. Acceptance Evidence

Milestone 13 closes only when:

- the complete reachable-closure walk is removed from the ordinary lane
- disjoint subscribers are rejected before dirty mutation and enqueue
- irrelevant fanout growth does not create proportional visited-node growth
- the `10^3`, `10^4`, and `10^5` scheduled financial scale sweeps report
  structural slopes for sparse, medium, and dense frontiers
- all admitted work retains immediate-dependency causes from Milestone 12
- multi-source deduplication is deterministic and causally complete
- predicted and realized counters are distinguishable and independently tested
- deterministic and optimized modes commit equivalent results
- the full-recompute oracle remains green
- observed semantic work equals the independent financial necessity manifest
- every implementation phase closes only after its assigned financial
  scenarios and mutation probes are green
- `FrontierLocalityCertificationRun` rejects missing, duplicate, stale, or
  mismatched reports and its strategy decision matches the measured envelope
- mutation probes that restore closure walking, late filtering, or counter
  laundering fail the courtroom
- focused tests, the complete `worth-signal` suite, boundary checks, context
  checks, formatting, and dirty Rust line-cap checks pass

## 12. Successor Handoff

[Milestone 14 - Deterministic Parallel Execution Foundation](./milestone-14-plan.md)
inherits a financially certified causal frontier and measured sparse/dense
envelopes. Parallel execution may consume the canonical work stream, but it
may not weaken pre-enqueue narrowing, causal evidence, deterministic ordering,
either independent oracle, or the typed strategy conclusion.

If and only if the conclusion is `OrderedReadyWorkCandidate`, a separate
WORTH-native traversal specification may be added. It begins from dynamic DAG
topology and the measured financial regimes; it does not delay or silently
enter the parallel foundation.
