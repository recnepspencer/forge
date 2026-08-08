# Gate 8.7 — Turn 2 Corrective

Turn 1 is good work. The architecture is right, the layering held, the single
classification site held, and the rail's idempotent-receiver design is the
correct pattern. Four findings, one of them serious. Read this whole brief
before editing.

Your governing texts are unchanged: `_docs/WORTH-query/milestone-9.16-runtime-phase-8.md`
§9 Gate 8.7 and §11 rows 16–18, plus `skills/implementation-batch/SKILL.md`,
`skills/code-quality-qa/SKILL.md`, `skills/qa-tests/SKILL.md`. Do **not** edit
anything under `_docs/`.

---

## F1 (High) — `redispatch_admitted_external_effect` needs no recovery handle

This is the finding that matters. Current signature:

```rust
pub fn redispatch_admitted_external_effect<Operation, Input, Scope>(
    &self,
    admission: &WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
    outbox: &WorthQueryDispatchOutboxRecord,
) -> Result<WorthQueryAdmittedExternalRedispatch, WorthQueryExternalRedispatchDenial>
```

It takes an admission and a **caller-supplied outbox record**. It does not take
the handle, and it does not take `WorthQueryRecoveryEffectAuthority`.

`WorthQueryApplicationCommitReceipt::dispatch_outbox()` is `pub`, and
`WorthQueryDispatchOutboxRecord` is exported through
`worth-query-execution/src/facade.rs`. So any consumer holding a receipt and a
current admission can emit the external effect **with no recovery handle in
existence at all** — never minting one, never consuming one, never touching the
Gate 8.3 lifecycle.

Three consequences:

1. **R8.69 is not proved.** Your three "denies before transport" tests pass
   because `safe_retry_commit_recovery` happens to call
   `admit_recovery_effect_authority` before `redispatch_admitted_external_effect`.
   That is caller discipline, not a structural guarantee. A second consumer
   written in the other order re-dispatches first and learns the handle expired
   afterwards — after the effect escaped.
2. **R8.30's linear lifecycle is bypassable.** "A second transition on the same
   handle denies" is enforced on the handle. This path never consumes one, so a
   caller can emit repeatedly and the handle stays live.
3. **It is the Gate 8.3 turn-1 defect class again** — the caller supplies what
   should have been evidence. There, it was `capability_currently_grants: bool`.
   Here, it is the outbox record: that record must come from a live handle's
   binding, not from a caller's hand.

**Fix.** Take the handle and the authority; read the outbox from the binding:

```rust
pub fn redispatch_admitted_external_effect<Operation, Input, Scope>(
    &self,
    handle: &WorthQueryRecoveryHandle,
    authority: &WorthQueryRecoveryEffectAuthority,
    admission: &WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
) -> Result<WorthQueryAdmittedExternalRedispatch, WorthQueryExternalRedispatchDenial>
```

Call `require_fresh_effect_authority(handle, authority)` **first**, before the
transport is reached, then take the outbox from
`handle.binding().dispatch_outbox()`. The authority type is minted only by
`admit_recovery_effect_authority`, which already performs `ensure_live`,
`deny_if_expired`, `ensure_admission_belongs_to_runtime`, and
`check_binding_axes` — so requiring it makes "authority precedes dispatch" a
property of the signature rather than of the call order.

This also lets `safe_retry_recovery_handle` drop its outbox-vs-binding equality
check, since the record can no longer come from anywhere else. Keep the check
only if you can state what it still rules out.

Simplify `safe_retry_commit_recovery` in
`bank-server/src/estate_progression/recovery.rs` accordingly — the manual
`dispatch_outbox().cloned().ok_or_else(...)` block goes away.

**Prove it, do not just fix it.** Add a compile-fail case: re-dispatch is
unreachable without a handle and authority. That is the negative case R8.69 has
been missing.

## F2 (High) — exactly-once is only proved where the answer was already known

`phase8_safe_retry.rs` covers two dispatch outcomes:

- `DisappearMidDispatch` → the rail never wrote a ledger record. The effect
  provably did not happen, so retrying it is the easy case.
- `Succeed` then retry → the effect provably did happen, and Query knows it.

Neither is the case this machinery exists for. **`CommitThenLoseResponse` is the
scenario that matters**: the rail admitted the effect, the response was lost,
and Query holds `Unresolved` — it does not know whether the effect occurred.
Retrying under genuine indeterminacy is where a duplicate emission would
actually happen, and it is untested.

Add it: commit under `CommitThenLoseResponse`, assert the posture is
`Unresolved` and `admission_count() == 1` (the rail admitted, Query does not
know), then safe-retry and assert `admission_count()` is **still 1** and the
ledger status is `Completed`. That is §11 row 16's real content, and it is the
row 5 shape ("lost response after commit… moves no money") applied to retry.

Keep the existing two — they are legitimate boundary cases. This one is the
load-bearing one.

## F3 (Medium) — the R8.66 compile-fail proves arity, not the boundary

`safe_retry_requires_admitted_redispatch.stderr` expects **E0061: this function
takes 4 arguments but 3 arguments were supplied**. That proves the parameter
exists. It would pass unchanged if `WorthQueryAdmittedExternalRedispatch` had a
public constructor — which is the thing R8.66 actually claims is impossible.

The implementation *is* correct (private fields, `pub(crate) mint`). The
evidence does not bind it. This is the Q8.7 class: evidence that names something
adjacent to the claim.

Add a compile-fail case that attempts to **construct**
`WorthQueryAdmittedExternalRedispatch` from outside the crate — struct literal
and `mint` — and expect the privacy error. Keep the arity case and the positive
twin; they are fine as far as they go.

## F4 (Low) — the foreign-principal assertion accepts four outcomes

`assert_foreign_or_authorization` admits `ForeignPrincipal`,
`FreshAuthorityDenied`, `CurrentPolicyDenied`, **or** any
`BankEstateProgressionDenial::Authorization`. A test that accepts four causes
cannot tell you which one fired, and would still pass if the denial moved to an
unrelated reason. Pin it to the cause you expect. If more than one is genuinely
reachable depending on admission order, say which and why in a comment — but
narrow it.

## Also fix

`WorthQueryAdmittedExternalRedispatch` carries a `_private: ()` field alongside
fields that are already private. Struct-literal construction from outside the
module is impossible without it. Drop it — it is ceremony that reads as
protection, and the same pattern was removed from `dispose.rs` in an earlier
pass. If you believe it is load-bearing, state what it prevents that field
privacy does not.

## Do not change

- The layering. Aftermath stays Signal-free and Bridge-free.
- `dispatch_external_effect` as the single classification site.
- The rail's idempotent-receiver design — checking the ledger before running the
  fault script is correct and realistic.
- Q8.14's scope. No drain, no scanner, no Store dependency.
- `WorthQueryDispatchOutboxDurabilityPosture` as a typed posture.

## Verification

The full standing set again, every target by name, five `--lib` runs all
reported. I independently confirmed `-p bank-server --test ordinary_mutations`
green at **87 passed** before this turn; report the new count and say what the
delta is.

No `#[allow(...)]`. Report honestly what you did not finish or could not prove.
