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

Do not edit the JSON state file directly. Do not use ad hoc PowerShell, Python,
or text replacement to patch runner state.

The only legal mutation surface is:

```powershell
python automation\phase_runner\state_tool.py apply {state_file} -
```

Pass one JSON payload on stdin describing the semantic outcome of the turn.
The model decides the phase truth. The state tool commits that truth safely.

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
`repair`, and `repair` returns to `review`. Do not create loops because tests
could be stronger, directories could be prettier, or aerospace-grade is not yet
claimable. Those are close-pass hardening inputs, not runner loop conditions,
unless they prove the phase itself is not actually done.

## Cursor rules

The runner sends exactly the turn named by `current`. Available turns: {turns}

Iteration evidence is not the same thing as closeout evidence. During `plan`,
`implement`, `review`, and `repair`, prefer the narrowest command that proves
the phase claim: `cargo check`, `cargo test --no-run`, the touched module tests,
the touched integration target, and the named compile-fail target that protects
the public boundary. Do not run whole-crate or whole-workspace suites such as
`cargo test -p forge-query --tests`, `cargo test -p worth-spatial --tests`, or
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
  turn `close_qa` if the phase is actually done and this prompt set provides
  segmented close-hardening turns; otherwise turn `close`
- after `repair`: same phase, turn `review`
- after legacy `close`: next phase at turn `plan`, or `current: null` and
  `completed_at` if this was the last phase

Segmented close-hardening turns advance like this:

- after `close_qa`: same phase, turn `close_plan`
- after `close_plan`: same phase, turn `close_fix`
- after `close_fix`: same phase, turn `close_quality_qa`
- after `close_quality_qa`: same phase, turn `close_quality_plan`
- after `close_quality_plan`: same phase, turn `close_quality_fix`
- after `close_quality_fix`: next phase at turn `plan`, or `current: null` and
  `completed_at` if this was the last phase

Only `close` or `close_quality_fix` advances to the next phase.

Do not leave the cursor on the same turn you just finished. Commit the turn
result through `state_tool.py apply`, with the next turn set explicitly. Never
hand-edit `current`.
