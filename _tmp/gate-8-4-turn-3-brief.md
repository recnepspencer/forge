# Gate 8.4 — Turn 3

Turn 2 landed real substance. I verified the two things I told you I would read
rather than take from a result, and both hold:

- **The double-entry oracle is genuinely independent.** `independent_oracle_agrees`
  sums `AccountActivityItem` amounts from activity rows with its own arithmetic
  and asserts absolute expected values (1_000 / 0). It does not ask production
  accounting what the answer should be, so it cannot prove production merely
  agrees with itself.
- **"Exactly one compensating transfer" counts committed rows.**
  `committed_disbursement_and_reversal_ids(&fixture).len() == 2` after an
  equivalent retry — committed journal IDs, not requests. That is the right
  counting layer, and the retry twin is there.

Also confirmed: the A10 permissive arm is fixed and now asserts
`CapabilityAuthorizationMissing`; the original journal is proved preserved via
`reversal_of() == Some(None)`; bank `ordinary_mutations` **56 × 2**, consumers
**313 / 37 / 22**.

## One test is not evidence

```rust
fn eight_undo_denial_kinds_are_distinguishable() {
    let kinds = [ /* eight variants */ ];
    let mut unique = kinds.to_vec();
    // dedup
    assert_eq!(unique.len(), 8, "R8.39 requires eight distinguishable causes");
}
```

This builds an array of the eight enum variants, deduplicates it, and asserts
there are eight. **The compiler already guarantees that.** The test cannot fail
for any reason connected to undo, and it occupies the slot where R8.39's actual
proof belongs — which makes it worse than no test, because the row reads as
covered.

R8.39 requires eight *scenarios*, each reaching its own cause through the
production path. Delete this one and replace it with real cases.

## What turn 3 owes

**1. The eight denial scenarios, for real.** Legal, audit, approval, released
estate, escaped effect, stale, conflicted, already-consumed. Each driven
through the production undo path, each asserting its own typed cause at the
public boundary, each with a positive twin.

Your own list already flags Conflicted and AlreadyConsumed as open. Stale is
the third: I do not see a scenario for it either.

**2. Distinguish "installs irreversible" from "undo wrote nothing".** Three of
your four denial tests are named `..._installs_irreversible_without_entering_mutation`
and `..._install_without_mutation`. Proving an operation *installs* as
irreversible is an installation property — R8.21 and Gate 8.1 already own it.

R8.39's claim is different and stronger: when undo is *attempted* on such an
operation, the attempt denies with a typed cause and **writes nothing**. That
needs an actual undo attempt, then proof the graph is unchanged.

Prove it by capturing state before the attempt and comparing after — journal
IDs and account activity, as `escaped_effect_..._writes_nothing_on_disbursement_world`
already does. That test has the right shape; the other three do not yet.

Remember the trap from your brief: a path that writes and then rolls back
leaves an identical graph afterwards. If any denial can reach a transaction,
prove it never opened one rather than that it left nothing behind.

**3. RecordedInverse end-to-end consuming the retained pre-image.** Your own
open item. R8.2's consumption side is only proved when a real inverse
derivation reads the retained pre-image and uses it — not when the pre-image is
merely present on the receipt.

This is also where R8.38's M5 gets decided: `RecordedInverse` and `Compensation`
must actually diverge in behaviour. Gate 8.3's T4b is the model — find the
configuration where routing both to compensation would be visibly wrong, and
test that one.

## Standard

Standing verification set, every row, each target by name, `--lib` five runs all
reported. Update the closure ledger in-slice (R8.63). Do not start Gate 8.5.

When you believe the gate is closeable, say so explicitly and list which rows
you consider proved by which evidence — I will re-run all of it.
