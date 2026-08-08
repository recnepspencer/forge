# Gate 8.6 — Turn 2 (closing)

Turn 1 is strong. I verified the gate's hardest rows by reading code, and they
hold.

## Confirmed

- **R8.48, including the channel I warned about.**
  `paired_protected_account_detail_worlds_publish_equal_aftermath_surfaces`
  opens with an explicit **"Vacuity guard"** — `assert_ne!` proving the paired
  worlds genuinely differ in the protected foreign-account status — then asserts
  seven equalities including `next_action_kind` and `undo_admitted` in both.
  That closes P4, P5, **P6**, and P7. You did not just avoid the leak; you
  proved the test could detect one.
- **R8.50 — removed, not privatized**, and proved mechanically.
  `phase8_residue` asserts the monolith directory is absent with the message
  "must be removed, not privatized," and that `enum EstateAftermath` is gone.
- **R8.14 at two sizes.** `[(10, 1), (1000, 100)]` in both `redo_intent` and
  `undo_admission_tests`, plus Bank lineage 1 vs 100 — courtroom row 12 as
  written, not a single-size assertion.
- **R8.61 is better than recorded.** Each of PB1, PB2, PB4 carries intake
  category, Phase 9 owner, the pre-snapshot deadline, and an **executable
  consequence command**. And you stated plainly that Phase 8 does not rename,
  rather than letting "recorded" read as "handled."
- **The ledger's evidence columns actually moved.** Every carried row cites new
  specific evidence (`phase8_residue::r8_3_*`, `phase8_fanout_courtroom`).
  That was the closing gate's temptation and you did not take it.

Independently re-run so far: bank `ordinary_mutations` **79**.

## Two things block the close, both about the durable artifact

Neither is an architecture defect. Both are the Q8.7 class — the evidence list,
not the evidence.

### 1. The exit condition contradicts itself

```
Phase 8 closes when every `R8.*` row reads `PROVED`, every `Q8.*` finding
reads `CLOSED`, and no gate's evidence rests on a claim this ledger records
as `PARTIAL`.

Two `PARTIAL` rows are carried deliberately ... each has a named owner and a
deadline earlier than Phase 8's own close
```

Three problems:

- **Q8.3 reads `PARTIAL`**, so the main clause is false as written.
- It says **"Two"**; Q8.9 closed at Gate 8.4, so there is one.
- It claims each carried row has "a deadline earlier than Phase 8's own close."
  Q8.3's deadline is **not** earlier — it is a deliberate carry *past* Phase 8.

This matters more than its size suggests. The exit condition is the sentence
that declares Phase 8 finished. If it is self-contradictory, "Phase 8 is closed"
is not a checkable claim, and this ledger is the artifact the next phase
inherits.

Rewrite it so it is true: state the closure rule, then name Q8.3 explicitly as a
deliberate carry with its owner and the phase that will resolve it, and say why
carrying it is sound — the residual internal constructibility is bounded in
writing and no consumer can reach it. A carried exception that is named and
justified is fine. One that contradicts the rule above it is not.

### 2. There is no §11 courtroom traceability map

§11 names fifteen scenarios that "must exist and must fail closed." Exactly one
test is named for its row — `courtroom_row_12_...`.

The substance is very likely spread across 8.1-8.6, and I believe it is there.
But "very likely there" is not the standard this phase has held for anything
else, and nobody can confirm all fifteen without re-deriving the mapping from
memory. That is precisely how Q8.6 survived three gate closures: not a missing
guarantee, a missing entry in the list of things checked.

Add a table to the closure ledger mapping each of §11's fifteen rows to the
test(s) that satisfy it, by name. Where a row is genuinely not covered, say so
rather than stretching a nearby test to fit — an honest gap I can see is worth
more than a mapping I cannot check.

## Then Phase 8 closes

Nothing else. Re-run the standing verification set, every row by name, `--lib`
five runs all reported.
