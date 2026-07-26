---

# Operating contract for this automated turn

There is no human in this loop. Approval policy is `never` and the sandbox is
full-access. The JSON state file is only lightweight progress state. The chat
transcript is where plans, reviews, QA lists, command output summaries, and
implementation explanations belong.

## Load before you act

Reason from the sources, never from the phase title alone. Read the spec file,
this phase's scope paths, the relevant public APIs, and the project laws/context:

{project.context_files}

Read `_docs\coding_guidelines\MENTALITY.md` and
`_docs\coding_guidelines\arch_laws.md` with special attention on every turn.
Read `_docs\coding_guidelines\dx_laws.md` when planning or changing public caller
experience.

## JSON is progress state, not an evidence bundle

Do not put logs, artifacts, command output tails, long plans, long findings,
full QA reports, or proof transcripts into the JSON state. Keep those in chat.

The JSON may contain only short tracking markers:

- `status`
- `qa_status`
- `current`
- `completed_at`
- short entries in `notes.plan`, `notes.done`, `notes.remaining`,
  `notes.findings`, and `notes.verification`
- small runner history entries

Every note entry should be a compact pointer, not a report.

## State-mutation protocol

The state file may be written by more than one process. Obey this exactly:

1. Read the state file fresh from disk in the same command or script that writes
   it. Never write from a stale copy.
2. Mutate only the current phase row, the `current` cursor, `completed_at`, and
   small history entries describing this turn.
3. Preserve everything else exactly: all other phase rows, `session`, `project`,
   `turn_templates`, prompt text, and existing history.

## Authority and reconciliation

Phase rows are the source of truth. `current` is only the next-turn cursor.

- If a phase row says `status: complete` and `qa_status: passed`, that phase is
  already closed.
- Never leave `current` on the same phase at `review`, `repair`,
  `test_review`, `test_repair_plan`, `test_repair_implement`, or
  `code_quality_review` after marking that phase `complete/passed`.
- If `current` disagrees with the phase rows, repair `current` from the phase
  rows before doing any other work.
- `current.phase` should point at the first not-fully-finished phase unless the
  milestone is complete, in which case set `current: null` and set
  `completed_at`.

Before every write, run this reconciliation check:

1. Did I just mark this phase `complete/passed`?
2. If yes, did I advance `current` to the next phase `plan`, or to `null` plus
   `completed_at` if this was the last phase?
3. Does `current` still point at a same-phase repair/test turn even though the
   phase row is already `complete/passed`?
4. If so, fix `current` now and record a short history note about the repair.

## Status values

Phase `status` is one of: {status_values}
QA `qa_status` is one of: {qa_status_values}

Use them this way:

- implementation still in progress -> `status: in_progress`
- implementation done, phase-done QA not yet run -> `status: complete`,
  `qa_status: needed`
- phase-done QA passed -> `status: complete`, `qa_status: passed`
- phase-done QA found a real gap -> `status: regressed`, `qa_status: failed`
- genuinely blocked -> `status: blocked`

  The phase-done loop and structural code-quality loop are mandatory gates.
  `review` may send the phase to `repair`, and `repair` returns to `review`.
  `code_quality_review` sends the phase to `code_quality_repair` when it finds
  concrete composition-law, domain-structure-law, file-size, directory-topology,
  public-facade, `mod.rs` business-logic, helper-placement, missed-abstraction,
  or ownership-boundary violations. Vague perfection concerns do not loop, but
  concrete structural-law violations are phase defects and must not be recorded
  as optional residue. Do not route structural findings through generic
  `repair`; semantic repair, test repair, and structural repair are separate
  turns.

## Cursor rules

The runner sends exactly the turn named by `current`. Available turns: {turns}

Iteration evidence is not the same thing as closeout evidence. During `plan`,
`implement`, `review`, and `repair`, prefer the narrowest command that proves
the phase claim: `cargo check`, `cargo test --no-run`, the touched module tests,
the touched integration target, and the named compile-fail target that protects
the public boundary. Do not run whole-crate or whole-workspace suites such as
`cargo test -p worth-query --tests`, `cargo test -p worth-spatial --tests`, or
the `worth-spatial` `public_api_contract` umbrella unless the phase acceptance
explicitly names that exact command as required closeout evidence. Heavy
metaboss, public API umbrella, architectural closeout, and broad compile-fail
suites are closeout lanes, not ordinary iteration lanes. If a listed acceptance
item is too broad for iteration, replace it with focused proof plus test
compilation (`--no-run`) and record the deferred broad command as explicit
closeout-only evidence rather than burning the loop.

For deletion-ledger acceptance, also point to the former public/exported surface
and the new outcome: deleted file/symbol, collapsed canonical proof surface,
certification-only boundary, capped residue row, or named Query gap. If you
cannot name the old surface and its new enforced outcome, the deletion ledger is
not resolved.

Default phase turns advance like this:

- after `plan`: same phase, turn `implement`
- after `implement`: same phase, turn `review`
- after `review`: same phase, turn `repair` if the phase is not actually done;
  turn `test_review` if the phase is actually done
- after `repair`: same phase, turn `review`
- after `test_review`: same phase, turn `test_repair_plan` if test findings
  need fixes; turn `code_quality_review` if test hardening is not needed
- after `test_repair_plan`: same phase, turn `test_repair_implement`
- after `test_repair_implement`: same phase, turn `test_review` if fixes need
  re-review; turn `code_quality_review` if tests are now honest enough
- after `code_quality_review`: same phase, turn `code_quality_repair` if
  structural QA found concrete law violations; otherwise next phase at turn
  `plan`, or `current: null` and `completed_at` if this was the last phase
- after `code_quality_repair`: same phase, turn `code_quality_review`

Only passing `code_quality_review` advances to the next phase in this prompt
set.

## Stale-cursor recovery example

If you read the state and see:

- phase 11 row -> `status: complete`, `qa_status: passed`
- `current` -> `phase: 11`, `turn: test_repair_implement`

that means the cursor is stale. Do not continue `test_repair_implement`.
Repair `current` first:

- set `current` to phase 12 at turn `plan` if phase 12 is the next unfinished
  phase
- or set `current: null` and `completed_at` if phase 11 was the final phase

Then append a short history note describing the cursor repair, validate the
state, and continue from the repaired cursor.
