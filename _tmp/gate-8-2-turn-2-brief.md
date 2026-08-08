# Gate 8.2 — Turn 2

Your turn 1 was audited under `skills/qa-loop/SKILL.md` against a ledger
written before your code was inspected.

**The hard half succeeded.** `bank-external-rail` is a genuine external
boundary and its strongest property is structural: its dependency list is
`serde`, `serde_json`, `tokio` and nothing else, so it *cannot* reach the
runtime's truth source. That is a better proof than any test. All 7 exit-proof
tests pass, including the PID-inequality check and a success twin. Gate 8.2's
entry condition is now genuinely met.

**Your conclusion was correct and I verified it independently:** the gate is not
closed. Reporting that honestly rather than wiring an in-process shortcut was
the right call.

No regression: 313 consumer tests and 13 Gate 8.1 aftermath tests still pass.

## What remains — this is the whole job for turn 2

### 1. Wire the Query side (it is currently dead code)

`cargo check` reports `classify_transport_fault` and `decision_time` as never
used in
`worth-query-execution/src/domain_computation/application_aftermath/external_effect/classification.rs`.
The classification exists and nothing calls it. Connect it to production
dispatch.

### 2. The proof that actually closes this gate

A rail fault must travel through **Query's production dispatch path** and come
out as the correct typed posture. Right now the rail proves the *rail* behaves;
nothing proves Query classifies it.

For each of the five faults, an end-to-end test driving real Query dispatch
against the real spawned rail process:

| Rail fault | Required Query posture |
|---|---|
| CommitThenLoseResponse | unknown / indeterminate — never `Completed` |
| AcknowledgeWithoutCompleting | acknowledged, distinct from completed |
| CompleteAfterDelay | timeout, then late completion reconciles — no duplicate effect |
| DuplicateAcknowledgement | posture does not advance twice |
| DisappearMidDispatch | unknown — not a guess, not a failure-as-success |
| Succeed (twin) | completed |

**I will check which code path these tests traverse.** A test that exercises
the fault against a transport double while the real rail process sits unused
does not close this gate — that is the same defect as an in-process fake,
wearing a real process as decoration.

### 3. Outbox co-commit — R8.4 / R8.25 / R8.55 (ledger O1-O4)

- dispatch intent co-commits with the mutation in one transaction (O1)
- an operation with **no domain mutation** still commits its dispatch record,
  because that record is its only anchor (O2)
- operations declaring no external effect pay **exactly zero** — zero dispatch
  records, zero dispatch counters, zero added commit work (O3)
- prove O3 against a real bank money-movement operation, not against a path
  that has no external effect declared anywhere (O4)

Precedent for the co-commit: `provider/idempotency.rs` already writes a
Query-owned entity into the operation's own `MutationIntent`.

### 4. Correlation carried, not re-derived — R8.26 (K2/K3)

`WorthQueryProviderCompareAndCommitOutcome::Indeterminate(failure)` already
carries evidence, and the session protocol already distinguishes
`CommitRecoveryRequired` from `AbortRecoveryRequired`. Both are discarded at
the application boundary today. Carry them up. Do not recompute them.

### 5. DEFECT — the time source was aliased, not generalized (M1 / PB3)

```rust
WorthQueryAuthorizationTimeSource as WorthQueryRuntimeTimeSource,
...
pub use time_source::{WorthQueryAuthorizationTimeSource, ...};  // still exported
```

You did not fork the source — correct, and that was the dangerous failure.
But an alias is not a rename. Both names are now public for one type in one
module, and architectural law 10 requires every public type to have exactly one
meaning. PB3 asked for the owner to stop being authorization-scoped, not to
gain a second label.

Required: a real rename, the old name gone, the owner no longer living under
`authorization/` if it now serves dispatch timeouts too. One name, one meaning.

### 6. State the CDC decision explicitly (E1, R8.8)

Say in your report whether dispatch delivery uses Relational CDC. If it does,
prove no CDC subscriber makes an authority or disclosure decision and that a
CDC checkpoint cannot be readmitted as a Query dispatch posture. If it does
not, confirm you built no second change stream over Relational.

### 7. Counters — N2-N4

- new dispatch/causality event: exactly 1 basis preparation, 1 digest
  derivation, 0 text materializations
- delivery, acknowledgement, timeout classification: 0 and 0 — they carry the
  identity
- ordinary commit with no external effect: 0 across the board

The `external_dispatch` slot you added is correct; assert exact values through
it.

## Hard boundaries — unchanged

Gate 8.2 only. No recovery handle (8.3), no undo, no redo. No `_docs/` edits.
No PB1/PB2/PB4 repairs. 400-line cap.

## Verification

Real output: the new end-to-end fault tests, `bank-external-rail` (7),
`worth-query-installation application_aftermath` (13), the three consumer
targets (313 / 37 / 22), the real
`scripts/ci/check_workspace_rust_line_caps.sh dirty`, boundary-check, and
agent-context.

If you cannot finish, say precisely what is unwired. Your last two reports were
accurate; that is worth more to me than a closed gate.
