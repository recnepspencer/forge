# Gate 8.3 — Turn 4 (focused)

Turn 3 is verified. I re-ran every target myself rather than reading your
report, and every claim held:

- Bank `ordinary_mutations` **49 passed** (was 42).
- Query consumers **313 / 37 / 22** — exactly baseline.
- `compile_certification` **14 passed** — Q8.6 closed.
- Line cap: no touched file over 400. Dead code retired.
- The re-blessed `.stderr` diff removes **only** the stale
  `WorthQueryCompensationCapability` suggestion; `E0432` and the "no such
  symbol in `facade::domain`" assertion are preserved. Nothing was laundered —
  that was the right way to do it.
- The eleven `*_denies_distinctly` drift tests each have a real positive twin
  (`truth.check(&matching).expect("positive twin admits")`) before the one-axis
  mutation, and each asserts its own denial kind.
- All three `axis_probe` constructors are `#[cfg(test)] pub(crate)` — Gate
  8.1's exported-fixture defect was **not** repeated.
- `recorded_inverse_aftermath_admits_reconcile_and_denies_compensate` (T4b) is
  the strongest test in the gate: one installed contract, reconcile admits on
  the authority axis, compensate denies on the mechanism axis, both through the
  production runtime under a real rail fault.
- Four terminal leak paths: consumed, disposed, expired, force-terminated.

One item remains, and it is the last instance of the pattern this whole gate
has been about.

## The admitted read is unforgeable but unbound

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedIdempotencyRead {
    resolution: WorthQueryApplicationIdempotencyResolution,
    _private: (),
}
```

`resolve_recovery_handle` takes it by value, which is right. But the proof
carries **no binding to the idempotency identity it was read for**, and it
derives `Clone`.

The effect authority is slot-checked against the handle. The read is not. So a
caller legitimately holding two recovery handles can resolve handle B while
presenting handle A's read, and `resolve` returns A's resolution as B's answer.
`Clone` makes the same read replayable indefinitely.

This is not a cross-principal escalation — it needs real authority for B. But
it is exactly the defect class the gate exists to remove: *a value standing in
for the evidence that should have produced it*. Turn 1 had it as
`capability_currently_grants: bool`, turn 2 had it as a bare taxonomy
parameter, and this is the last corner of it.

**Fix:**

1. Bind the read to the `WorthQueryApplicationIdempotencyBinding` it was read
   for, and have `resolve_recovery_handle` deny when that binding is not the
   handle's own. Give it its own denial kind — this is a distinct fact from
   `IdempotencyMismatch` on the binding axis, because here the *handle* is
   right and the *read* is foreign.
2. Drop `Clone`. A one-shot read result that can be duplicated is not one-shot.
3. Two tests: the negative (read from handle A cannot resolve handle B, denying
   with the new kind) and its positive twin (the matching read resolves).

## Then update the ledger and stop

Mark Gate 8.3's rows in
`_docs/WORTH-query/milestone-9.16-runtime-phase-8-closure-ledger.md`, including
R8.63's own row — this is the first gate to update that ledger *at its own
closure* rather than retroactively, which is the whole point of the
requirement.

Leave Q8.3 as the dated `PARTIAL` you recorded; that disposition is acceptable
and I am not asking you to finish it in this gate.

Do not start Gate 8.4. Re-run the full named set when you are done: bank
`ordinary_mutations`, the three Query consumer targets, `compile_certification`,
`worth-query-execution --lib`, `boundary-check`, `agent-context`, the line cap,
and a warning-clean build.
