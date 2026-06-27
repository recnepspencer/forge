Implement the close structural code-quality repair plan for phase {phase.id}:
{phase.title}.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

Use the previous close quality plan as the work order. Keep fixes scoped to this
phase's structural quality and public-boundary proof. Do not invent another QA
loop. If broader aerospace-grade work remains, record it as named residue in
chat and a compact `notes.remaining` marker.

Run the focused verification commands named by the plan and the phase acceptance
checks needed for closeout confidence. Summarize the result in chat.

When this close-hardening sequence is complete, update only lightweight JSON
state:

- keep this phase `status: complete` and `qa_status: passed`
- add at most short `notes.done`, `notes.remaining`, and `notes.verification`
  markers
- if a later phase exists, advance to that phase at turn `plan`
- if this was the last phase, set `current` to null and set `completed_at`

Do not put logs, artifacts, command tails, long findings, or plans into the
JSON.

{contract}
