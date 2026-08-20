---
name: qa-loop
description: Review and correct completed WORTH implementation work through a requirement-and-evidence closure ledger and a fresh independent agent or CLI critic. Use after a coherent implementation phase or slice to audit specification fidelity and causally relevant risks, obtain outside-eyes review, fix root causes, reopen affected guarantees, and continue until the implementation is genuinely proved.
---

# QA Loop

Attempt to falsify both the implementation and the completeness of the ledger.
The specification is authoritative, but it may not exhaust the system's
intended obligations. Green tests, completed fixes, and finding counts do not
define completion.

## Independent review

Launch a fresh, read-only independent reviewer at the start of the QA loop.
The critic is an independent evidence source for missed defects.

If the user names a reviewer, model, or CLI, use it. Supported choices may
include a fresh Codex agent, Claude CLI, Cursor Agent, Grok, or another
user-supplied non-interactive review command. Do not invent a command, install a
tool, authenticate an account, or substitute a similarly named service. Verify
that the selected reviewer is available before relying on it.

If the user does not name a reviewer, default to Luna Max and proceed without
blocking for a choice. If neither Luna Max nor a user-selected reviewer can be
launched, report that the independent-review requirement is blocked; do not
relabel a self-review as outside review.

Give the critic a compact, source-bound review packet containing only the raw
task-local evidence it needs:

- repository instructions and governing specification
- the frozen revision or deterministic source fingerprint
- the scoped diff and changed files
- relevant producer, consumer, authority, persistence, and facade boundaries
- any search-coverage already recorded; do not delay launch to finish a
  complete manifest
- test commands, failures, logs, and evidence artifacts
- a neutral request to find correctness, authority, lifecycle, recovery,
  performance, topology, DX, and proof defects that are causally relevant

Do not give the critic prior findings, intended fixes, suspected bugs, closure
ledger conclusions, or a preferred answer before its first pass.
Keep the critic read-only. It may inspect and report; edits, verification, and
final judgment remain outside the read-only review.

Record the critic identity, model or command, source revision, scope, prompt,
and complete findings in the audit history. Verify every external finding
against repository evidence. Reject unsupported claims explicitly rather than
accepting them for consensus, and add any newly exposed guarantee to the ledger.

## Establish scope

Read the repository instructions, governing specification, changed files, and
causally relevant engineering laws. Inspect adjacent producers, consumers,
authority owners, persistence boundaries, public facades, and test support
where they can invalidate the implementation's guarantees.

Freeze the source state from which the audit begins and preserve unrelated
worktree changes.

## Generate the closure ledger

Build the ledger from the implementation's full intended contract, not only
the specification's explicit clauses.

Use the specification, engineering laws, public promises, architectural
boundaries, actual authority and state model, adjacent integrations, cleanup
obligations, and credible failure modes as evidence of what must remain true.
Infer guarantees that the system's purpose requires even when no document
states them verbatim.

Use judgment. Do not mechanically expand a fixed taxonomy or produce rows for
their own sake. Write the semantic guarantees needed to expose an
implementation that satisfies the wording while defeating the intent.

For each row, record:

- a stable identifier
- the exact closure claim
- the evidence needed to prove it
- the current result and evidence
- `OPEN`, `PROVED`, `DEFECT`, `BLOCKED`, or justified `N/A`

Start uncertain rows as `OPEN`. Existing tests are candidate evidence until
their world, boundary, oracle, consequential state, and fault sensitivity have
been inspected.

The ledger is a living model of the implementation's obligations. Discovery
may reveal that a guarantee is missing, too weak, or framed at the wrong
boundary. Add or revise rows when that happens, preserve the history, and
reopen every affected claim.

The ledger enumerates guarantees, not review categories, implementation tasks,
or test cases. One piece of evidence may prove several rows, and one row may
require several forms of evidence.

Before closure, attack the ledger itself:

> What meaningful defect could still exist while every current row passes?

If a credible defect can survive, the ledger is incomplete. Improve the ledger
before using it to claim completion.

## Build the risk map

Choose review lenses according to the guarantees and failure surfaces
represented in the ledger. Possible lenses include:

- semantic and specification correctness
- authority, security, privacy, and disclosure
- architecture, phase progression, lifecycle, and dependency direction
- failure, cancellation, concurrency, recovery, and migration
- performance and resource behavior
- test and fixture honesty
- composition and domain topology
- public DX and operability

For each possible lens, ask whether a defect in that category could plausibly
invalidate an in-scope guarantee. Apply the lens deeply when it is causally
relevant, lightly when only a boundary check is warranted, and omit it when it
has no meaningful connection to the work.

Do not privilege any category or mechanically multiply categories across
ledger rows. The implementation's actual claims and risks determine the
review.

Audit total future construction cost. Distinguish harmless local repetition
from duplicated meaning that can drift, and distinguish useful organization
from abstraction that only adds navigation. Test whether committed successors
can be added, tested, replaced, and scaled through predictable boundaries
without duplicating authority or changing unrelated responsibilities.

## Plan proof economically

Reuse one causally valid canonical world with isolated semantic deltas. Group
equivalent inputs by the production behavior they exercise.

Prefer the cheapest honest proof:

- type and dependency enforcement for impossible structure
- compile-fail evidence for inaccessible public authority
- pure or property tests for broad validation spaces
- one-axis metamorphic tests for causal behavior
- targeted runtime tests for lifecycle and consequential state
- pairwise cases where two axes genuinely interact
- exhaustive matrices only for small, closed decision lattices
- a small number of realistic integrated journeys for cross-phase composition
- dedicated race tests only for real shared-state concurrency

Test combinations only when production behavior couples the axes. Every added
test must detect a distinct plausible fault. Consolidate redundant fixtures,
scenarios, compiler sessions, and integration journeys.

## Discover before declaring closure

Trace real execution, denial, failure, cancellation, and teardown paths across
the whole ledger. Do not stop discovery after finding the first defect.

Look for violated invariants, unearned authority, premature mutation, hidden
effects, incomplete lifecycle behavior, dishonest fallback, stale derived
state, incompatible evolution, resource escape, fixture theatre, and
implementation that satisfies wording while defeating intent.

Record every finding against the ledger rows it invalidates. State:

1. severity and affected guarantees
2. the concrete defect and evidence
3. the governing requirement or invariant
4. the required root-cause correction
5. the proof that would close it

Do not report preferences or speculative concerns as defects.

The independent critic runs from the start and may change the risk map and
ledger. Reconcile all review evidence against source and runtime evidence.
Agreement does not prove a claim; disagreement requires direct evidence.

When a defect was not represented by an existing row, treat that as two
failures: the implementation defect and a ledger-completeness defect. Add the
missing guarantee, inspect the surrounding semantic family, and determine
which other claims were falsely allowed to remain proved.

## Correct and reopen

Fix root causes rather than padding tests or adding local guards around a
broken authority model. Search the affected semantic family for the same
defect.

After a correction:

- retain the finding as audit history
- reopen every row whose evidence or assumptions changed
- reopen causally downstream rows when phase progression changed
- update fixtures and tests whose earlier proof became stale
- rerun the affected evidence family
- reassess the corrected production path directly

Do not rerun the Cartesian product of the entire suite when only a bounded
proof family was affected. Do not leave downstream guarantees proved merely
because their old tests remain green.

## Complete the loop

Continue discovery, correction, reopening, and verification until every ledger
row is resolved and no known in-scope defect remains.

Before declaring completion:

- verify the final source state, not an earlier snapshot
- run the required functional, compiler, architectural, and repository checks
- confirm relevant warm paths and test costs remain sane
- preserve resolved findings and their closure evidence
- record any unrelated repository failure as an explicit scoped caveat
- ensure the ledger contains no stale `OPEN` or `DEFECT` rows
- obtain a closure pass from a new critic instance when material production,
  authority, lifecycle, persistence, or proof changes were made during QA
- verify that the closure critic reviewed the final source revision and did not
  inherit the first critic's conclusions

If the critic finds a material defect, fix it, reopen affected guarantees, and
repeat the affected semantic-family searches before launching a new final-source
critic. Never keep one critic running as a search assistant across correction
turns. The replacement critic defaults to Luna Max unless the user specified
another reviewer.

Report closure from the ledger. Never infer closure from the absence of new
findings in a single pass. Summarize which independent reviewer was used, which
findings were accepted or rejected and why, and whether the final outside-eyes
pass found any unresolved defect.
