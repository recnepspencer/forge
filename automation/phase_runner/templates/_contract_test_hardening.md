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
Read `_docs\more_guidelines\dx_laws.md` when planning or changing public caller
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

The only mandatory loop is the phase-done loop: `review` may send the phase to
`repair`, and `repair` returns to `review`. Test hardening and code-quality
review are follow-up passes after phase done-ness, not reasons to reopen the
phase loop unless they prove the phase was never actually done.

## Cursor rules

The runner sends exactly the turn named by `current`. Available turns: {turns}

Iteration evidence is not the same thing as closeout evidence. During `plan`,
`implement`, `review`, `repair`, `test_review`, `test_repair_plan`,
`test_repair_implement`, and `code_quality_review`, prefer the narrowest command
that proves the phase claim: `cargo check`, `cargo test --no-run`, the touched
module tests, the touched integration target, and the named compile-fail target
that protects the public boundary. Do not run whole-crate or whole-workspace
suites such as `cargo test -p forge-query --tests`, `cargo test -p worth-spatial --tests`,
or the `worth-spatial` `public_api_contract` umbrella unless the phase
acceptance explicitly names that exact command as required evidence.

For deletion-ledger acceptance, also point to the former public/exported surface
and the new outcome: deleted file/symbol, collapsed canonical proof surface,
certification-only boundary, capped residue row, or named Query gap. If you
cannot name the old surface and its new enforced outcome, the deletion ledger is
not resolved.

Advance like this:

- after `plan`: same phase, turn `implement`
- after `implement`: same phase, turn `review` if implementation is ready for
  the phase-done check; otherwise stay on `implement`
- after `review`: same phase, turn `repair` if the phase is not actually done;
  turn `test_review` if the phase is actually done
- after `repair`: same phase, turn `review`
- after `test_review`: same phase, turn `test_repair_plan` if test findings
  need fixes; turn `code_quality_review` if test hardening is not needed
- after `test_repair_plan`: same phase, turn `test_repair_implement`
- after `test_repair_implement`: same phase, turn `code_quality_review`
- after `code_quality_review`: next phase at turn `plan`, or `current: null`
  and `completed_at` if this was the last phase

Only `code_quality_review` advances to the next phase in this prompt set.
