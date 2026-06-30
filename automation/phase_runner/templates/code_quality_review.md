[$code-quality-qa](C:\Users\Esther\.codex\skills\code-quality-qa\SKILL.md) now
review the code quality of phase {phase.id}: {phase.title}.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

Review only. Do not fix yet.

Look for:

- composition-law violations
- domain-structure-law violations
- files or modules that are too broad
- facade-shaped modules that implement rather than aggregate
- hidden helper buckets
- file length and directory shape problems
- weak ownership boundaries
- places where the implementation works but teaches the wrong architecture

If you find issues, report them in chat with file/line references, commit the
phase outcome through `state_tool.py apply` using only short `notes.findings` /
`notes.remaining` markers, and still close the phase honestly rather than
pretending it is aerospace-grade.

Re-run the acceptance checks and record command, exit code, and output tail in
`notes.verification`. Then commit the lightweight final outcome through
`state_tool.py apply`:

- keep this phase `status: complete`
- keep `qa_status: passed`
- add at most short `notes.done` / `notes.remaining` markers
- if a later phase exists, advance to that phase at turn `plan`
- if this was the last phase, set `current` to null and set `completed_at`

{contract}
