# Fintech Causality Courtroom

This directory owns the financial test domain used to certify Signal's
invalidation semantics. It is not a production pricing library, but its
financial truth is real enough to prevent version-counter theater: market
inputs and positions are fixed-point values, formulas compute economic values,
and Signal revisions are only the incremental runtime projection of those
values.

## Authoritative World

`FinancialWorldDefinition` is the source of truth. It owns a deterministic
market, typed factors (quotes, FX, curve buckets, and volatility), positions,
factor subscriptions, condition policies, consumer comparators, and producer
output-equivalence policies. `CompiledFinancialWorld` lowers that definition
into one Signal graph and establishes a baseline before any mutation is
admitted.

The compiler keeps three boundaries explicit:

- financial inputs and formula outputs are authoritative fixed-point values;
- semantic projection revisions describe meaningful changes in those values;
- Signal `Aspect` values are producer-local runtime slots, not financial
  identities.

Baseline provenance includes the deterministic seed, definition, compiled
economic snapshot, dependency revisions, established dependency snapshots,
and reproduction policy. A test may not manufacture a convenient graph version
and call it a market move.

## Independent Oracles

Every certification scenario compares the incremental run with two owners that
do not consult Signal's scheduling decision:

- `FreshFinancialRecompute` rebuilds the economic result from the mutated
  financial definition.
- `FinancialNecessityManifest` derives the work that is economically necessary
  from definition and mutation semantics.

The runtime passes only when its final economic snapshot equals fresh truth and
its observed work equals the necessity manifest. A shared expected-version
constant, the runtime's own dirty set, or its own trace is not an independent
oracle.

## Scenario Catalog

The sealed `FinancialAspectCausalityCertificationRun` requires exactly one
claim for each scenario:

- `quote_to_risk_aspect_translation`: a quote-side change becomes a distinct
  risk-side committed aspect before a risk condition sees it.
- `heterogeneous_consumer_comparators`: exact, tolerance, and installed
  consumers disagree for declared policy reasons.
- `tolerance_suppressed_repricing`: producer output equivalence suppresses a
  small candidate but publishes a larger economic move coherently.
- `producer_local_factor_slot_collision`: different producers reuse the same
  numeric slot without sharing semantic identity.
- `partitioned_curve_bucket_bump`: scoped curve detail commits accumulate and
  release through a gated risk consumer.
- `gated_repricing_release`: a financial threshold holds and then releases
  necessary repricing.
- `instrument_dependency_rewire`: a position changes curve dependency through
  the production topology authority, including revision and cycle rules.
- `branch_shock_restore_replay`: unresolved causes cross branch, checkpoint,
  replay, diagnostics, and async admission before settlement.

The reproduction tuple records scenario identity, seed, comparator profile,
producer output-equivalence policy, actual mutation steps, economic delta, and
dependency revision. A tuple from another scenario or policy must be rejected,
even if its final numbers happen to match.

## Lifecycle And Execution Lanes

The courtroom exercises ordinary transactions, scheduled reads, async
capability admission, rollback, branch isolation, checkpoint restore, replay,
and diagnostic retention. The same certification suite compiles under the
parallel feature; grouped parallel order/effect proofs remain separate runtime
evidence. The library must also compile for the WASM target without inventing a
second invalidation authority.

## Mutation Probes

Adversarial controls are scenario-based. They include missing or duplicate
claims; wrong scenario, policy, revision, seed, oracle, work, or lifecycle
evidence; copied root aspects; producer-slot collisions; aspect/scope swaps;
scope loss across multiple commits; deferred-producer settlement; stale cause
bindings; and topology rewires or checkpoint data that contradict live
authority. Each probe must make its assigned scenario or seal fail for the
specific causal reason.

The older `FintechWorld` stress fixtures remain useful for scale, audit-cache,
artifact, and executor pressure. They are supplemental and cannot replace the
definition/compiler/oracle courtroom for Milestone 12 certification.
