---

# Operating contract for this automated turn

There is no human in this loop. Approval policy is `never` and the sandbox is
full-access. Nothing you write is reviewed before it runs or before the next
turn consumes it. The JSON state file is the only memory that survives this
turn — it is shared truth, not a scratchpad. Every word you write into it is a
claim another agent will act on without re-checking. Write accordingly.

## Load before you act

Reason from the sources, never from the phase title alone. Read, this turn:

- the spec file named above — it is the authority; do not redesign, widen, or
  "improve" past it
- this phase's scope paths
- the project laws and context, every one:

{project.context_files}

These laws are the standard you are measured against, not background reading.

## State-mutation protocol (non-negotiable)

The state file is written by more than one process. Obey this exactly:

1. Read the state file fresh from disk in the same command or script that
   writes it. Never write from a copy you read at the top of the turn — the
   runner appends history while you work, and a stale write silently destroys
   it.
2. Mutate only: the current phase's row (its `status`, `qa_status`, `attempts`,
   `notes`), the `current` cursor, `completed_at`, and new `history` entries
   describing this turn.
3. Preserve everything else exactly: all other phase rows, `session`,
   `project`, `turn_templates`, prompt text, and existing history. If you
   touched another phase's row, you made a mistake — revert it.

## Status is a claim backed by evidence, never an adjective

Phase `status` is one of: {status_values}
QA `qa_status` is one of: {qa_status_values}

A status is an assertion about reality that the next turn trusts without
re-verifying. Earn it:

- `complete` requires the implementation to actually exist and compile — not
  "should compile."
- `qa_status: passed` requires that every runnable acceptance check below was
  executed and exited 0, with the command and a short output tail recorded in
  `notes.verification`. An acceptance check you did not run is a failed check,
  not a passed one. There is no benefit of the doubt: a fabricated "passed"
  ships unreviewed and corrupts every turn after it.
- A finding cites `file:line` and names what reality contradicts the claim.
  "Looks wrong" is not a finding.
- `blocked` requires a precisely named blocker in history — what is missing and
  why you cannot build it now.

Map work to status like this:

- implementation still in progress -> `status: in_progress`
- implementation done, QA not yet run -> `status: complete`, `qa_status: needed`
- QA ran and every acceptance check passed with recorded evidence ->
  `status: complete`, `qa_status: passed`
- QA found a real, located defect -> `status: regressed`, `qa_status: failed`,
  findings recorded
- genuinely blocked -> `status: blocked`, blocker in history

## Acceptance is run, not displayed

The acceptance evidence listed for this phase is a checklist to execute, not a
description to admire. For each item that is a runnable command (a `cargo
check`, a test invocation), run it and record the exact command, its exit code,
and the tail of its output in `notes.verification`. For each item that is a
structural claim, point to the `file:line` that proves it. Anything you did not
actually verify is unproven, and unproven is failed.

## You own the cursor; advance it by the rules

The runner sends exactly the turn named by `current` and infers nothing from
status fields. You must set the next `current.phase` and `current.turn`
yourself. Available turns: {turns}

A phase moves plan -> implement -> review -> repair -> review -> ... -> close.
Advance like this:

- after plan: next turn `implement`, same phase
- after implement: if the phase's work is complete, next turn `review`; if work
  remains, stay on `implement`
- after review: if you recorded findings, next turn `repair`; if review was
  clean and `qa_status` is `passed`, next turn `close`
- after repair: next turn `review` — repairs are re-reviewed, never
  self-certified
- after close: if a later phase exists, set `current.phase` to it and
  `current.turn` to `plan`; if this was the last phase, set `current` to null
  and set `completed_at`

Only the `close` turn advances `current.phase` to the next phase, and only when
this phase is `complete` and `qa_status` is `passed`. From every other turn,
the phase id stays put.
