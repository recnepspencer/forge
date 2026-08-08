# Gate 8.5 — Turn 2

Turn 1 is the strongest first turn of any gate in this phase. I verified the
architectural centre by reading signatures, and it holds.

## Confirmed by signature, not by test

**R8.45 — you got the hard part right.**

- `evaluate_divergence` is a method on `WorthQueryLinearLineageChain` taking
  `bound_head` as a parameter, commented "Linear-lane policy." The policy lives
  in the lane.
- `WorthQueryRedoIntent` has accessors and `derive` — and **no `is_valid`, no
  `authorize`, no `into_admission`, no replay payload.** Possession authorizes
  nothing, which is R8.42 enforced rather than asserted. I checked the impl
  block, not just the struct, because a field-free type with one wrong method
  would have failed silently.
- `WorthQueryAftermathParentCausalityEdge { parent, child }` — no branch field,
  no merge placeholder, no reserved slot. You satisfied "admits a
  branch-shaped successor as a leaf addition" **without** adding an empty
  placeholder, which is the R8.53 over-correction I warned about. That
  distinction is easy to get wrong in the safe-looking direction and you did
  not.
- Zero branch/merge/alternate-lineage residue.
- `WorthQueryForbiddenOrdinaryChainPosture` names exactly the six, each with a
  positive twin lowering into `SingularContinuity`.
- `ordinary_redo_path_has_no_replay_import_residue` reads module sources and
  asserts no `worth_query_replay` reference — mechanical, as the spec demands,
  not convention.

Your answer to "could a 9.18 rebasing lane reuse this type unchanged?" is
correct and the code supports it.

## Why the gate does not close

You listed the residual honestly, and it is the one row I said I would audit
hardest — the third instance of the requirement class that both previous gates
initially got wrong.

**R8.43 / A9 has no world-drift test.** Six of the eight causes are proved
through the Bank production boundary: `CopiedIntent`, `ForeignPrincipal`,
`ChangedOperationMeaning`, `DuplicateRedo`, and `DivergenceInvalidation` in two
shapes. Two are not:

- **`Stale` has no assertion anywhere in the Bank tests.** Not a weak one — none.
- **`NewlyUnauthorized` is not proved in the world-drift shape.** Your note says
  it is "covered by production `map_recovery_denial` on the bank path and
  foreign/terminal admits." Those prove the mapping exists. They do not prove
  that a redo whose intent is entirely honest denies because *the world* changed.

That is the whole point of the row. The proved undo is the **derivation**
precondition for the intent; it is not authority. The only test that separates
"redo re-admits" from "redo trusts the proved undo" is:

> Drift the world after the undo — expire the grant, revoke the capability —
> leave the intent completely honest, and prove redo denies on current policy.

You already wrote exactly this test in Gate 8.4:
`undo_denies_on_current_policy_after_world_drift_with_honest_receipt` sets grant
validity, advances the clock past it, asserts `CurrentPolicyDenied`, and then
asserts the receipt is unchanged to prove the denial came from the world and not
from a corrupted input. Reproduce that shape here, asserting the intent is
unchanged at the end.

Add the `Stale` twin in the same shape — a clock-expiry path after a proved
undo, through the disbursement world, denying `Stale` at the Bank boundary.

## What turn 2 owes

1. **A9 / X3** — world-drift redo denial, `NewlyUnauthorized`, intent proved
   honest afterwards.
2. **X2** — `Stale` through the Bank production path with its own cause.
3. Positive twins for both.

That is all. Everything else in this gate is proved.

## One thing to note, not to fix

`WorthQueryRedoIntent::derive` returns `Result<Self, &'static str>`, as do
`undo_intent`, `external_effect/correlation`, and `dispatch`. Every
consumer-visible denial in Phase 8 is a typed kind — `WorthQueryRedoDenialKind`,
`WorthQueryUndoDenialKind`, `WorthQueryRecoveryHandleDenialKind` — so these
string-typed internal derivation errors are inconsistent with the phase's own
discipline.

They are internal (digest preparation failures), not consumer-visible, and they
predate this gate, so this is **not** a Gate 8.5 defect and I am not asking you
to fix it here. Record it in the closure ledger as a finding owed at 8.6's
residue sweep, so it is not discovered a third time.

## Standard

Standing verification set, every row, each target by name, `--lib` five runs all
reported. Update the closure ledger in-slice — and correct the Gate 8.5 status
line, which currently reads `CLOSED (turn 1)`. It is not closed until A9 and X2
land. Do not start Gate 8.6.
