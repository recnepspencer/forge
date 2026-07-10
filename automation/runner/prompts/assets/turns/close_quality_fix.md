Implement the close structural code-quality repair plan for phase {phase.id}:
{phase.title}.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

Use the previous close quality plan as the work order. Keep fixes scoped to this
phase's structural quality and public-boundary proof. Concrete structural-law
findings must be fixed before the phase can pass. Do not hide bad topology,
missed abstractions, inline policy, `mod.rs` business logic, facade dumps,
helper buckets, or production/test authority mixing as vague aerospace-grade
residue.

Run the focused verification commands named by the plan and the phase acceptance
checks needed for closeout confidence. Summarize the result in chat.

When this close-hardening sequence is complete, update only lightweight JSON
state:

- set this phase `status: complete` and `qa_status: passed` only after the
  structural findings are fixed
- add at most short `notes.done`, `notes.remaining`, and `notes.verification`
  markers
- if a later phase exists, advance to that phase at turn `plan`
- if this was the last phase, set `current` to null and set `completed_at`

Do not put logs, artifacts, command tails, long findings, or plans into the
JSON.

{contract}
