# Gate 8.6 — Turn 3 (one test, then Phase 8 closes)

Turn 2 did both repairs correctly, and the courtroom map earned its keep
immediately — it found a row that has been marked proved on the wrong evidence
since Gate 8.2.

## Turn 2 confirmed

- **The exit condition is now true.** One named carry (Q8.3), with owner and a
  deadline, and a stated reason the carry is sound. The false "two PARTIAL /
  earlier-than-close" clause is gone.
- **The fifteen-row courtroom map exists**, each scenario mapped to named
  tests — and you reported two honest gaps instead of stretching nearby tests
  to fit. That was the right call, and one of those gaps turns out to matter.

## The finding your own map surfaced

You flagged §11 row 14 as: *"co-commit / lost-response / exactly-once covered;
no named mutation-free (O2) e2e."* I checked, and it is sharper than that.

**Every `co_committed_dispatch_outbox()` assertion in the repository is on a
mutating operation** — `notify_death` or disbursement. There is no mutation-free
case anywhere.

That matters because of what R8.25 actually says:

> The dispatch intent is co-committed with the mutation (D1). **This is
> structural, not a cost optimization**: under R8.55 every escaping effect is
> anchored by a committed local fact, so **an operation with no domain mutation
> still commits its dispatch record and that record is its anchor.**

The mutation-free case is not an edge case of R8.25 — it is the clause that
makes the anchoring structural. If a dispatch record only co-commits when there
is already a mutation to ride along with, then co-committing is an optimization
of an existing write, and R8.55's law has a hole exactly where it is hardest to
see: an operation that escapes to the world while writing nothing locally.

Gate 8.2's row O2 was marked `PROVED` on evidence that actually proves O1. The
ledger inherits that: R8.25 reads `PROVED` citing rows O1-O4.

This is a Gate 8.2 defect surfacing at Gate 8.6, which §9 handles as an
append-only corrective that blocks unfinished dependents. Phase 8 is the
unfinished dependent, so it lands here.

## What turn 3 owes — one test

A Bank end-to-end scenario for §11 row 14, in the shape the row names:

1. An operation that performs **no domain mutation** and declares **one
   external effect**.
2. Its dispatch record **commits** — `co_committed_dispatch_outbox()` is true
   while `changed_record_count()` is zero. Assert both in the same test, so the
   mutation-free precondition cannot silently drift.
3. A **lost response resolves by idempotency**, returning the same semantic
   result.
4. A **retry emits the external effect exactly once** — counted at the rail,
   not at the request layer.

Use the real `bank-external-rail`, as Gate 8.2's fault scenarios do.

If the Bank domain has no mutation-free operation with an external effect, add
the smallest honest one rather than simulating the condition — an operation
that notifies the rail and writes nothing is a legitimate domain shape, and it
is the shape R8.25 exists to protect.

If you conclude after trying that this cannot be built without distorting the
Bank domain, say so explicitly with the reasoning, and I will record R8.25 as
`PARTIAL` with a named owner rather than let it read `PROVED` on the wrong
evidence. **An honest `PARTIAL` is a better closing artifact than a `PROVED`
that does not hold.**

## Then update the ledger

- Record this as a new finding (**Q8.11**) with its Gate 8.2 origin, so the
  ledger shows the corrective rather than quietly restating R8.25.
- Correct R8.25's and R8.55's evidence columns to name the new test.
- Update courtroom row 14 in the map.
- Row 11's gap (multi-node session/queue is Bank Phase 5) is a legitimate
  downstream boundary — leave it as the honest gap you recorded.

## Standard

Standing verification set, every row by name, `--lib` five runs all reported.
Nothing else. After this, Phase 8 closes.
