# Task Brief: Runtime Phase 8 Gate 8.4 — Fresh Undo, Inverse Operations, Compensation

Gate 8.3 is closed and independently audited: linear recovery handle, eleven
per-axis drift denials with positive twins, four terminal leak paths, the C1
receipt repair, and `worth-query-execution --lib` green across five consecutive
loaded runs. You are continuing into Gate 8.4.

## Mandatory reading

Re-read if your context has rolled: `AGENTS.md`, all of
`_docs/coding_guidelines/`, and
`workspaces/worth-query/crates/worth-query/docs/AI_README.md`.

Governing specification: `_docs/WORTH-query/milestone-9.16-runtime-phase-8.md`,
§9 Gate 8.4 binding. Also constraining you directly:

- **§10** Self-Support Obligations — R8.63, R8.64, R8.65 all land on you.
- **§11 Courtroom** rows 1-5 and 13-15 are undo/compensation scenarios.
- **§5 G1 / R8.2** — the inverse pre-image demand. You own the *consumption*
  side; installation landed at 8.1.
- **§8** — the counter contract. `undo_admission` is populated here.

The closure ledger is
`_docs/WORTH-query/milestone-9.16-runtime-phase-8-closure-ledger.md`. Read its
**Standing verification set** section before you start; it is new since your
last gate and it defines what "green" now means.

## Mandatory skill

Execute `skills/implementation-batch/SKILL.md` — four ordered stages, boundary
review and plan before any code. Satisfy `skills/code-quality-qa/SKILL.md` and
`skills/qa-tests/SKILL.md`. Do not read or use `skills/spec-designer/SKILL.md`.

## Your entry condition is misstated — correct it

Gate 8.4's entry reads "G1 resolution implemented (R8.2). G8 typed (R8.9)."
Both are listed in the closure ledger as **owed by 8.4**. So the entry names as
preconditions two things this gate must itself build — the same defect §10 was
written to prevent, occurring inside a spec that now forbids it.

You own R8.2's consumption side and R8.9 either way. Fix the spec's entry
wording to say so, the way Gate 8.2's entry now does, and note it in the ledger
as **Q8.10**. Do not silently proceed as though the entry were satisfied.

## Start here: C2 (R8.1, this gate's obligation)

`WorthQueryPrimaryMutationWorkEvidence` currently carries six counts —
`decision_facts`, `proposed_facts`, `invariant_state_facts`,
`invariant_work_units`, `relational_invariant_executions`,
`relational_invariant_results` — **and no identities**.

An inverse cannot be derived from a count. Do this before deriving anything:

1. Mutation work names the touched records.
2. Names are **derived from the commit**, never caller-supplied. Same standard
   you correctly applied to C1 in Gate 8.3.
3. The constructor stays non-public; R8.1's unforgeability survives
   strengthening.
4. If adding the names does not break every construction site, they are not
   required — check that it did.
5. The inverse derivation must actually **consume** them, not store them
   alongside a derivation that keys off something else.

Watch the cost contract while you do it: naming records must not make undo's
identity derivation scale with how many there are (R8.40).

## What this gate actually is

Gate 8.3's centre was a linear resource. Gate 8.4's centre is a **fresh
admission**. R8.37: undo "re-enters the full current progression: capability,
purpose, disclosure, conflict, touched-graph, invariant, idempotency, provider,
and compare-and-commit. It is an ordinary operation with an unusual input."

Every serious defect in Phase 8 so far has been the same one: **a caller
supplying what should have been evidence.** Turn 1 of Gate 8.3 had it as
`capability_currently_grants: bool`; turn 2 as a bare taxonomy value; turn 4 as
an unbound read proof. You fixed all three well.

Undo gives that defect its best hiding place yet, because undo legitimately
*holds a receipt that was once authorized*. The tempting shortcut is to treat
that receipt as evidence of current authority for some part of the progression.
It is not. It is evidence of what happened, and nothing about now.

Concretely, the row I will be auditing hardest:

> **No step of the progression is skipped because the receipt was once
> authorized.**

And the test shape that proves it: revoke or drift the **world** after commit,
leave the receipt entirely honest, and prove undo denies on current policy. A
test that corrupts the stored receipt proves the opposite of what is needed —
it proves undo notices a *bad receipt*, not that it re-admits against a
*changed world*.

Related: R8.37 says undo is an ordinary operation. Make it traverse the same
production entry point as any other mutation. A parallel undo-only path that
merely resembles the progression is not the progression.

## The rest of the requirements

R8.36, R8.38 through R8.41. They are precise in the spec; notes below only
where they are easy to satisfy dishonestly.

**R8.36 — derivation.** Undo derives a *request*, never an effect. It must not
mutate history and must not call the provider to repair state. Which of
inverse / compensation / reconciliation it derives must key off the **installed
axes** — mechanism for inverse vs compensation, authority for reconciliation —
exactly as Gate 8.3's transitions do. Not off a posture name.

**R8.38 — money.** Compensating debit *and* credit, not a net adjustment. Both
original journals preserved. Exactly one compensating transfer per undo, and an
equivalent retry compensates **once**. Count committed journal rows, not
requests — a request-layer count of one while the provider emits two is the
failure this row exists to catch.

The independent double-entry oracle must not share code with the production
accounting path (§11). If it does, it proves production agrees with itself.

**R8.39 — denials.** Eight cases: legal, audit, approval, released estate,
escaped effect, stale, conflicted, already-consumed. Each with a typed cause,
each distinguishable, **and no fallback mutation**. Prove nothing was written,
not that nothing survived — a path that writes then rolls back fails this row
even though the graph looks right afterwards.

Released estate additionally has **no `undo` method reachable** from its
outcome type (§11 row 4, inherited from R8.21) — type-level absence.

**R8.40 — identity.** One new bounded intent identity carrying the original
committed and aftermath identities. Prove independence by *varying* fan-out —
10 vs 1000 postings, 1 vs 100 lineage edges — not by asserting a single number
once. Populate the `undo_admission` slot with exact values.

**R8.41 — Foundational.** May describe the completed relationship afterward;
cannot substitute for the fresh admission that produced it. Negative case: no
undo authorized from Foundational material alone.

## Three inherited obligations

**R8.64** — extend the named `phase8_cross_gate` suite with a scenario driving
undo **through** Gate 8.3's handle, Gate 8.2's real rail, and Gate 8.1's
installed aftermath. Through them, not beside them.

**Q8.9 at cause** — the ledger records the process-global recovery registry as
owed **before Gates 8.4 and 8.5 grow more registry-touching suites**. That is
now. Scope the registry to the runtime instance so isolation falls out of
construction, then retire `reset_for_integration_test` and the test lock. If
you conclude this is genuinely too large for this gate, say so explicitly with
your reasoning — do not let it slide silently into 8.5.

**R8.63** — update the closure ledger as part of this gate's closure, not
after.

## Verification — the standard changed

Run and report **every** row of the Standing verification set, each by name:

- `cargo test -p bank-server --test ordinary_mutations`
- the three Query consumer targets
- `cargo test -p worth-query-certification --test compile_certification`
- `cargo test -p worth-query-execution --lib` — **five runs, report all five**
- warning-clean build, `boundary-check`, `agent-context`, dirty line-cap
- no test affordance reachable without `test-support`

Two reading rules, both learned the expensive way in Gate 8.3: name the target
you ran rather than describing it, and never report the best of several runs.

Report honestly. Your self-reports through Gate 8.3 were accurate about what
you built, and that made every audit faster.
