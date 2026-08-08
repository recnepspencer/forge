# Task Brief: Runtime Phase 8 Gate 8.6 — Bank Cutover, Publication, Certification

Gates 8.1 through 8.5 are closed and independently audited. Bank
`ordinary_mutations` **72**, consumers **313 / 37 / 22**, certification **14**,
execution `--lib` **578 × 3**. This is the closing gate.

## Mandatory reading

Re-read if your context has rolled: `AGENTS.md`, all of
`_docs/coding_guidelines/`, and
`workspaces/worth-query/crates/worth-query/docs/AI_README.md`.

Governing specification: `_docs/WORTH-query/milestone-9.16-runtime-phase-8.md`,
§9 Gate 8.6 binding. Also constraining you directly:

- **§11 Courtroom** — all fifteen scenarios, plus the test-form obligations.
- **§13** — the platform-boundary defects PB1, PB2, PB4 (R8.59-R8.61).
- **§8** — the cost contract, which R8.51 closes.
- **§10** — R8.63's ledger must be *complete* when this gate closes.

Closure ledger:
`_docs/WORTH-query/milestone-9.16-runtime-phase-8-closure-ledger.md`.

## Mandatory skill

Execute `skills/implementation-batch/SKILL.md`. Satisfy
`skills/code-quality-qa/SKILL.md` and `skills/qa-tests/SKILL.md`. Do not read
or use `skills/spec-designer/SKILL.md`.

## This gate has two jobs

Its own five requirements, **and** truing up every `PARTIAL` row Phase 8 has
been carrying. A gate that proves R8.47-R8.51 while R8.0, R8.14, or R8.61 stay
partial has not closed Phase 8.

The carried rows, from the ledger: R8.0, R8.3, R8.10, R8.11, R8.14, R8.15,
R8.59, R8.60, R8.61, R8.63, R8.65, Q8.3, and the `&'static str` derivation-error
finding recorded at 8.5 for this gate's residue sweep.

**A warning about the closing gate's specific temptation.** It is much easier to
true up the *ledger* than the *code* — to mark rows `PROVED` because the gate is
ending rather than because evidence changed. I will read the evidence column,
not the status column. A row whose status moved without its evidence moving is a
regression in the ledger's meaning, and the ledger is the durable artifact this
phase produces.

## R8.48 is the sharpest row in the gate

Publication must preserve authorization, disclosure, and inherited branch
affinity across four surfaces — outcome, explanation, recovery posture, and
receipt-linked lineage — and Phase 7 noninterference applies unchanged.

Three leak channels, and the third is the one that matters:

1. Aftermath **explanation** — paired worlds differing only in a protected fact
   must produce equal explanations.
2. Lineage **shape** — equal edge counts and topology.
3. **Next-action availability** — and this is the channel that looks like
   correct behaviour.

Think carefully about the third. If `undo` is offered in one world and withheld
in another *because of a protected fact*, then the shape of the API is the leak.
Withholding undo when undo is unsafe is exactly what the system should do; it is
also exactly how the fact escapes to a consumer who can only observe which
methods exist. Paired worlds must offer identical next actions.

And check your own fixture: if the paired worlds do not genuinely differ in the
protected fact, all three tests are vacuous. Prove the difference exists before
asserting the equality.

## R8.50 — "removed or privatized" is two different things

For each superseded path — monolith, bank-local, generic rollback — say in your
report **which one you did**. Privatizing is weaker than removing: a
`pub(crate)` path is still a second authority that internal code can call, and
R8.0 wants exact retirement, not reduced visibility.

Where you privatize rather than remove, prove the path is unreachable rather
than merely unexported.

Remember Q8.6, which cost this phase a red test surviving three gate closures: a
retirement that leaves an orphaned authority witness or a stale facade fixture
is not exact. Warning-clean build **and** `compile_certification` green are both
part of proving M7.

## R8.51 — measure against a real baseline

"Ordinary commit cost is unchanged when no external or recovery work is
required" must be proved against something real — a pre-Phase-8 baseline, or an
operation that provably takes no aftermath work.

Use the Gate 8.2 discipline that made R8.4 convincing: that test asserted the
transport was **live** before asserting the counters were zero. Do the
equivalent here — prove the aftermath machinery is present and available, then
prove the ordinary path pays nothing for it.

## R8.61 — recording is not fixing

PB1, PB2, and PB4 must be entered in
`worth-query-bank-world/docs/front-door-closure-ledger.md`.

But note the deadline in the ledger: **PB1's rename must land before Phase 9's
facade snapshot**, or it becomes a permanent migration surface. If the rename is
in scope here, do it. If you judge it out of scope, say so explicitly and record
the deadline and owner — do not let "recorded" read as "handled."

## §11 — all fifteen courtroom scenarios

Rows 1-6 and 13-15 are largely covered by 8.1-8.4, rows 7-12 by 8.5 and this
gate. Confirm each of the fifteen exists, is named, and fails closed. Row 12 is
the fan-out row and closes R8.14: **10 vs 1000 postings, 1 vs 100 lineage
edges** — two sizes, because a single-size assertion proves nothing about slope.

Test-form obligations: Gate 8.2's rows are integration/e2e and must be named as
such; the double-entry oracle stays independent of the production accounting
path; every negative case has a positive twin; residue and import checks are
mechanical, not reviewed.

## Standard

Standing verification set, every row, each target by name, `--lib` five runs all
reported.

When you believe Phase 8 is closeable, say so and list which rows you consider
proved by which evidence. I will re-run all of it and read the ledger's evidence
column against the code.
