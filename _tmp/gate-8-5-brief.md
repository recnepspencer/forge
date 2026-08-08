# Task Brief: Runtime Phase 8 Gate 8.5 — Fresh Redo Intent And Linear Lineage

Gate 8.4 is closed and independently audited: C2, R8.2 consumption end-to-end,
eight real denial scenarios with no-write proofs, money compensation with an
independent double-entry oracle, and an instance-scoped recovery registry. Bank
`ordinary_mutations` **62**, execution `--lib` **562 × 3**. You are continuing
into Gate 8.5.

## Mandatory reading

Re-read if your context has rolled: `AGENTS.md`, all of
`_docs/coding_guidelines/`, and
`workspaces/worth-query/crates/worth-query/docs/AI_README.md`.

Governing specification: `_docs/WORTH-query/milestone-9.16-runtime-phase-8.md`,
§9 Gate 8.5 binding. Also constraining you directly:

- **§7** destination topology — the lineage edge type lives here.
- **§10** self-support obligations — R8.63, R8.64, R8.65.
- **§11** courtroom rows 7, 8, 9.
- **§5 G9** — the linear Foundational continuity kinds.
- **§8** — `redo_admission` is populated here.

Closure ledger:
`_docs/WORTH-query/milestone-9.16-runtime-phase-8-closure-ledger.md`. Its
**Standing verification set** section defines what "green" means.

## Mandatory skill

Execute `skills/implementation-batch/SKILL.md` — four ordered stages, boundary
review and plan before any code. Satisfy `skills/code-quality-qa/SKILL.md` and
`skills/qa-tests/SKILL.md`. Do not read or use `skills/spec-designer/SKILL.md`.

## What this gate is really about

Gate 8.3's centre was a linear resource. Gate 8.4's was a fresh admission.
**Gate 8.5's centre is a type that must not learn its lane's policy.**

Read R8.45 carefully — it is unusual, and it is what this gate turns on:

> Invalidation-on-divergence is a **policy of the linear lane**, located where a
> lane policy lives, and is **not a property of the redo intent type**: rebasing
> a redo onto a diverged head is routine in CAD and EDA, and a 9.18 lane that
> permits it must not have to unpick this type to do so. Symmetrically, the
> lineage edge type must admit a branch-shaped successor as a leaf addition
> without reshaping the linear chain.

So the predictable defect here is **not** that redo does the wrong thing. It is
that redo is **correct and unreusable** — linearity welded into the type where
it belongs in the lane. That passes every behavioural test in this gate and
costs a rewrite in 9.18.

Two questions I will answer by reading your signatures, not your tests:

1. If `RedoIntent` itself consults the current head to decide validity, the
   policy is in the type. Where does the invalidation decision actually live?
2. Could a 9.18 rebasing lane reuse this redo intent type **unchanged**? Answer
   it concretely against the real signature, in your report.

This is the general-purpose platform requirement doing real work. WORTH must
serve finance, medical, CAD, and chip simulation. Finance wants linear undo;
CAD and EDA rebase onto diverged heads routinely. The type has to survive both.

**And the symmetric trap.** R8.45 also says the lineage edge type must *admit* a
branch-shaped successor as a leaf addition. Do not satisfy that by adding an
empty branch placeholder — R8.53 forbids empty placeholders for uncommitted
possibilities, and domain structure law 2 requires the smallest populated form.
The axis is the commitment; a future domain adds one leaf beneath a durable
axis. Admitting a leaf is not the same as reserving a slot for one.

## R8.43 — expect this to be the third instance

R8.43 requires fresh capability, policy, conflict, touched-graph, invariant,
idempotency, provider, and compare-and-commit admission.

This is the same requirement shape as R8.31 (Gate 8.3) and R8.37 (Gate 8.4).
**Both earlier gates initially shipped a version where the caller supplied what
should have been evidence** — a `bool` in 8.3, and in 8.4 the risk that a
receipt once authorized would stand in for current authority.

Redo's version of that shortcut: *possession of a proved undo* becomes the
authorization. It is not. A proved undo is the **derivation** precondition for
the intent, and says nothing about whether redo is authorized now.

The test shape that settles it is the one you already got right in Gate 8.4:
drift the **world** after the undo, leave the intent completely honest, and
prove redo denies on current policy. A test that corrupts the intent proves the
wrong thing.

## The rest

**R8.42 — descriptive.** No runtime authority, no replay state. Check this
against the `impl` block, not just the struct: a field-free intent with an
`into_admission()` method is not descriptive. Possession must authorize nothing.

**R8.44 — one chain, one head.** Exactly one parent-causality edge per original,
per undo, per redo outcome. Count **committed lineage rows**, not requests —
the Gate 8.4 lesson about counting at the right layer.

**R8.46 — Foundational continuity.** Lowers only after each Query transition
completes, and only into the linear kinds. Six postures may not be relabeled as
the ordinary chain: replayed, reconstructed, restored, branch-local, partial,
promoted. Six negative cases, each with a positive twin.

**Exit proof — eight scenarios.** Lawful, stale, newly unauthorized, copied
intent, foreign principal, changed operation meaning between undo and redo,
duplicate redo, divergence invalidation. Eight distinct causes.

Say it plainly because it cost a turn in Gate 8.4: **eight scenarios reaching
their own cause through the production path.** Not an array of enum variants
deduplicated and counted. That test proves what the compiler already
guarantees.

Also: test divergence invalidation for more than one shape of divergence. If
the head moved by an ordinary operation is the only case, the policy is proved
once and assumed everywhere else — try an intervening undo or redo too.

**Certification replay.** May verify the evidence; must not appear in the
ordinary redo path — and the spec says explicitly this is proved **by import
residue check, not by convention**. Build the mechanical check.

## Standard

Standing verification set, every row, each target by name, `--lib` five runs all
reported. Update the closure ledger in-slice (R8.63). Contribute the cross-gate
scenario (R8.64) driving redo through 8.4's undo, 8.3's handle, 8.2's rail, and
8.1's aftermath. Do not start Gate 8.6.

Your self-reports have been accurate through four gates, including when they
said "not closed." Keep that. When you think it is closeable, say so and list
which rows you consider proved by which evidence — I re-run all of it.
