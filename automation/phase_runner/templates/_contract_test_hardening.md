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

## Bias toward action

This runner exists to finish milestones, not to admire them.

- After you have read enough to identify the real seam, move to code quickly.
- Do not spend the turn enumerating multiple plausible approaches if one
  approach already matches the spec, the architecture laws, and the existing
  code shape.
- When several findings share one root cause, fix the root cause in one assertive
  pass instead of nibbling through one symptom per turn.
- Prefer replacing a dishonest seam with the real production seam over adding
  another helper, another adapter, or another certification-only detour.
- If a file is clearly the wrong owner, create or use the correct parallel lane
  and cut imports over; do not keep thinking inside the displaced file.
- Keep chat concise and implementation-heavy. The transcript should explain the
  chosen path, not narrate every possibility you decided against.

Default posture by turn:

- `plan`: read the governing surfaces, then produce one executable plan that
  can be implemented literally next turn
- `implement`: make the production cutover and the narrow proof reruns needed
  to know whether it works
- `review`: find only load-bearing gaps; do not manufacture style findings
- `repair`: close every finding in the same family before handing the phase back
- `test_review`: identify the deepest dishonest seam, not every minor testing
  imperfection
- `test_repair_plan`: produce one plan that removes that dishonest seam at the
  production boundary
- `test_repair_implement`: implement the real seam replacement, not a temporary
  test convenience
- `code_quality_review`: verify structure and file ownership boundaries, then
  advance

Avoid these failure modes:

- rereading the same files multiple turns in a row without changing the owning
  seam
- fixing one hostile test by adding a narrower synthetic test seam
- reopening the same phase with smaller and smaller findings instead of
  collapsing them into one decisive repair
- spending most of a turn on prose when the next code edit is already obvious

## Cutover-first rule

For parallel-lane migration phases, do not stop in a mixed state and then spend
turns reviewing or hardening that mixed state. First finish the mechanical
cutover so the new lane is the only ordinary behavior path and the displaced
lane is wrapper-only, capped, or disconnected from ordinary callers.

During this cutover work:

- let compiler errors, import failures, and type-boundary failures lead you
- prefer finishing the cutover over proving an intermediate state
- do not treat a partially migrated lane as ready for deep QA just because some
  focused tests pass
- mixed states should be finished, not reviewed

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
- after `implement`: same phase, turn `review`
- after `review`: same phase, turn `repair` if the phase is not actually done;
  turn `test_review` if the phase is actually done
- after `repair`: same phase, turn `review`
- after `test_review`: same phase, turn `test_repair_plan` if test findings
  need fixes; turn `code_quality_review` if test hardening is not needed
- after `test_repair_plan`: same phase, turn `test_repair_implement`
- after `test_repair_implement`: same phase, turn `test_review` if fixes need
  re-review; turn `code_quality_review` if tests are now honest enough
- after `code_quality_review`: next phase at turn `plan`, or `current: null`
  and `completed_at` if this was the last phase

Only `code_quality_review` advances to the next phase in this prompt set.

## Turn-completion rule

This runner cannot infer your intent from chat alone. A turn is only complete if
you leave the JSON state in the next correct position before you finish.

- Do not end a turn with the same cursor unless the template explicitly told you
  to keep the same turn, which these milestone prompts do not.
- If you found the fix and implemented it, write the status and cursor advance
  now.
- If you concluded the phase still has findings, write the regressed/failed
  state and move to the required repair turn now.
- If you only wrote analysis in chat and did not advance the JSON, the runner
  will treat that as an incomplete turn and send you back to repair it.

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
