# Provisional State And Invariant Execution

## What This Feature Is

Provisional state lets a provider stage graph changes in an isolated overlay so
Query can inspect the exact proposed post-state before it becomes authoritative.
Invariant execution then loads real state from that proposal and runs every
installed validator. Use this when a domain mutation must prove its complete
installed invariant posture before a later publication boundary may consider
it.

The proposal is inspectable but non-authoritative. The invariant progression is
proof of validation, not commit authority.

## Why You Use It

- Keep ordinary reads on committed truth while testing a proposed change.
- Bind staged effects to one fresh decision read-set and provider session.
- Revise or discard a proposal without leaking overlay state.
- Ensure validators observe the real proposed post-state.
- Require the exact complete installed invariant set before progression.
- Preserve violation, indeterminate, exhaustion, and cleanup outcomes.

## Stable Entry Points

Consumer progression through
`worth_query_host::facade::installed::domain_computation`:

- `WorthQuerySessionEffectAuthority::lower_provisional_program(...)`
- `WorthQuerySessionBoundReadsAndEffects::begin_provisional_attempt(...)`
- `WorthQueryProvisionalAttempt::materialize_proposed_state()`
- `WorthQueryProposedPostState::inspect()`
- `WorthQueryProposedStateInspection::revise(...)`
- `WorthQueryProposedStateInspection::discard()`
- `WorthQueryProposedStateInspection::select_installed_invariant(...)`
- `WorthQuerySelectedInstalledInvariant::admit_state_load_plan(...)`
- invariant execution `execute()`
- `WorthQueryProposedStateInspection::admit_invariant_progression(...)`
- `WorthQueryInvariantProgressionAuthority`

Host integration through
`worth_query_host::facade::installed::provider_session`:

- `WorthQueryProvisionalGraphProvider`
- `WorthQueryInvariantExecutionProvider`

## Core Mental Model

The provisional program is lowered from the exact staged session and fresh
decision read-set:

```text
staged session + fresh decision read-set + declared effect steps
  -> lowered provisional program
  -> provider overlay
  -> proposed post-state
  -> inspection
  -> revision or invariant execution
  -> discard, or complete invariant progression for a later boundary
```

Provider overlay evidence is bound to provider identity and generation,
session token, basis, program identity, and attempt generation. The provider
cannot substitute an overlay from another attempt.

Ordinary graph reads remain authoritative and do not see the proposal.
Proposed-state identity, candidate scores, decision records, and comparison
evidence describe the proposal; none can publish it.

Invariant execution is another typed sequence:

```text
installed invariant requirement
  -> selected invariant
  -> admitted state-load plan
  -> provider-loaded proposed state
  -> validator execution
  -> passed | advisory | violated | indeterminate | exhausted receipt
```

A selected invariant or installed provider capability is not a verdict. The
validator must execute against the exact proposal. State-load evidence carries
provider, session, basis, proposal, attempt, plan, scope, and physical execution
identity.

For an installed application operation, the proposal inherits the same typed
branch and branch-qualified basis that entered obligation selection and the
managed session. A snapshot or version from another branch is foreign even if
its numeric version is equal. Successful invariant progression seals a real
Relational validated candidate; the application-operation provider consumes
that candidate inside compare-and-commit before publication can exist.

`admit_invariant_progression` requires one exact receipt for every installed
invariant slot. Blocking requirements need passed receipts. Advisory
requirements need advisory receipts. Missing, duplicate, foreign, violated,
indeterminate, or exhausted receipts cannot progress.

## How It Executes

```text
lower effects from fresh decision authority
  -> validate program dimensions and proposal basis
  -> provider stages isolated overlay
  -> Query validates overlay evidence
  -> materialize and inspect proposed state
  -> optionally revise through a new typed attempt generation
  -> load exact state for each installed invariant
  -> execute each registered validator
  -> admit the complete exact receipt set
  -> explicitly discard, or retain progression for a later supported boundary
```

Every provisional stage has a consuming discard. Provider failures and panics
return the actual overlay and session recovery posture.

## Small Example

```rust
let inspection = staged
    .begin_provisional_attempt(fresh_read_set, provisional_program)?
    .materialize_proposed_state()
    .inspect();

inspect_proposed_state(&inspection);
let discard = inspection.discard();
```

Inspection does not expose a commit method. Discard consumes the proposal and
cleans both overlay and provider session.

## Real Example

```rust
let receipt = inspection
    .select_installed_invariant("no-overlapping-allocations")?
    .admit_state_load_plan(state_load)?
    .execute()?;

let progression = inspection.admit_invariant_progression([receipt])?;

inspect_receipt_identities(progression.receipt_identities());
inspect_proposed_state_identity(progression.proposed_state_identity());

let discard = inspection.discard();
handle_discard(discard);
```

A real operation with multiple installed invariants executes every slot and
passes the complete receipt collection. One successful validator is
insufficient when more requirements are installed.

## How It Relates To Other Features

- [Provider Sessions And Decision Read-Sets](./provider-sessions-and-decision-read-sets.md)
  supplies the staged session, fresh facts, and effect authority.
- [Installed Computation Artifact Contracts](./installed-computation-artifact-contracts.md)
  governs proposal artifacts, counters, and decision evidence.
- [Relational Truth And Invariants](./invariants/capability-gaps-and-invariant-denials.md)
  explains invariant support and denial posture. This guide covers actual
  validator execution.
- [Effects](../execution/effects.md) describes the broader effect vocabulary;
  general effects do not authorize a provider overlay.

## Inspection And Debugging

Inspect:

- program and decision read-set identities;
- provider and session token generations;
- attempt generation and proposal-basis dimensions;
- proposed-state and overlay identities;
- stage, revision, and discard counters;
- invariant slot, enforcement posture, affected scope, and validator provider;
- state-load plan and evidence identities;
- passed, advisory, violated, indeterminate, or exhausted disposition;
- complete progression receipt identities.

An incomplete or foreign receipt set should fail without changing the proposed
state or authoritative graph.

## Anti-Patterns

- Constructing a provisional program or proposal basis locally.
- Treating staged provider state as committed graph truth.
- Reading the proposal through the ordinary authoritative read lane.
- Publishing candidate or decision evidence as approval.
- Reusing a fresh read-set for another session or proposal generation.
- Treating invariant selection or provider support as execution.
- Running validators without an admitted state-load plan.
- Minting progression from one passed invariant when more are installed.
- Treating advisory, violated, indeterminate, or exhausted as blocking success.
- Assuming overlay or session cleanup succeeded without inspecting its outcome.

## Current Limits

- Proposed state is non-authoritative.
- This low-level provisional-state feature does not expose commit. The higher
  installed application-operation progression consumes its sealed validated
  candidate through provider compare-and-commit.
- Multi-authority publication requires a later boundary with genuine atomic
  authority or explicit compensation semantics.
- Invariant progression proves only the exact proposal and attempt generation
  named by its receipts.

## Related Docs

- [Canonical Graph Obligation Progression](./canonical-graph-obligation-progression.md)
- [Provider Sessions And Decision Read-Sets](./provider-sessions-and-decision-read-sets.md)
- [Installed Computation Artifact Contracts](./installed-computation-artifact-contracts.md)
- [Capability Gaps And Invariant Denials](./invariants/capability-gaps-and-invariant-denials.md)
- [Aspects And Authority Lanes](../modeling/aspects-and-authority-lanes.md)
