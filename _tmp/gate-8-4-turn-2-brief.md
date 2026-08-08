# Gate 8.4 — Turn 2

Turn 1 is a genuine vertical slice and your "not closure" verdict is right. I
verified the load-bearing pieces by reading code and re-running checks.

## Confirmed good

- **C2.** `WorthQueryPrimaryMutationWorkEvidence::from_commit` derives
  `touched_records` from the commit's `changed_records`; the constructor is
  `pub(in crate::domain_computation::primary_graph)`; identities are typed
  `WorthQueryTouchedRecordIdentity::from_commit_record(RecordRef)`, not strings.
  Correct standard, correctly applied.
- **Q8.9 at cause — you took the better fix.** The registry is instance-scoped;
  `reset_for_integration_test`, `reset_for_test`, `lock_for_test`, and the
  `test-support` feature are gone with zero residue. I asked for this before
  8.4/8.5 grew more registry-touching suites and you did it rather than
  deferring.
- **Q8.10.** Spec entry rewritten to "This gate builds its own entry
  condition," matching Gate 8.2's shape.
- **A10 — the row I said I would audit hardest — is right.**
  `undo_denies_on_current_policy_after_world_drift_with_honest_receipt` sets
  grant validity to 600, commits at 500, advances the clock to 601, and denies
  with `CurrentPolicyDenied` — then asserts `receipt.idempotency_binding() ==
  binding_before` to prove the receipt stayed honest. That is drifting the
  *world*, not the receipt, which is exactly the distinction that matters.
- **`publication.rs` was correctly adapted, not weakened.** Whole-equality on
  `mutation_work()` genuinely breaks once it carries EntityIds that differ
  across separately provisioned worlds; comparing each counter plus
  `touched_record_count` plus a non-empty assertion is strictly stronger.

Re-run independently: bank `ordinary_mutations` **51 × 3 consecutive runs**, so
your "first run failed" was the pre-fix state rather than a flake. Good that you
reported it either way.

## One defect in turn 1's own work

The A10 test's match has a permissive arm:

```rust
BankEstateProgressionDenial::Authorization(_) => {}
```

Any authorization denial satisfies it. The test would pass if undo denied for an
unrelated authorization reason having nothing to do with the expired grant —
which is the failure mode the test exists to exclude. Either assert the specific
cause, or, if three denial shapes are genuinely reachable here, assert that each
is a current-policy fact. An empty arm in an otherwise precise test is where a
future regression will hide.

## What turn 2 owes

Your own list, which I agree with, in the order I would take it:

**1. R8.2 consumption — retain the pre-image into the receipt.** Installation
already declares the demand (R8.18). The consumption side must actually read
the declared pre-image and carry it, and the inverse derivation must use it.
Watch the standing trap: a pre-image that is *required by signature* but not
consumed passes type-checking and proves nothing.

**2. R8.9 — Bridge correspondence resolution at install.** The destination
inverse contract must reference the installed correspondence as a typed value,
not by name.

**3. R8.38 — money, with the independent oracle.** Compensating debit *and*
credit; both original journals preserved; exactly one compensating transfer;
equivalent retry compensates once.

Two things I will check specifically: count **committed journal rows**, not
requests — a request-layer count of one while the provider emits two is the
defect this row exists to catch. And the double-entry oracle must not share code
with the production accounting path; if it does, it proves production agrees
with itself. I will read the oracle's imports.

**4. R8.39 — eight denials, no fallback mutation.** Legal, audit, approval,
released estate, escaped effect, stale, conflicted, already-consumed. Typed
cause each, all distinguishable, each with a positive twin.

Prove **nothing was written**, not that nothing survived. A path that writes
then rolls back leaves an identical graph afterwards and fails this row anyway.
Released estate additionally needs no `undo` method reachable from its outcome
type — type-level absence, inherited from R8.21.

**5. R8.40 fan-out twins.** Vary the fan-out — 10 vs 1000 postings, 1 vs 100
lineage edges — rather than asserting a single number once. One identity
derivation regardless.

**6. R8.41.** Foundational may describe the completed relationship afterward
and cannot substitute for the fresh admission. Needs the positive case as well
as the negative.

**7. The ordinary progression for the derived request (A11).** R8.37 says undo
is "an ordinary operation with an unusual input." Your status file lists this
as owed and it is the one I care most about after R8.38: the derived
inverse/compensation must traverse the **same production entry point** as any
other mutation. A parallel undo-only path that resembles the progression is not
the progression, no matter how faithfully it resembles it.

## Standard

Standing verification set, every row, each target by name, `--lib` five runs all
reported. Do not start Gate 8.5.
