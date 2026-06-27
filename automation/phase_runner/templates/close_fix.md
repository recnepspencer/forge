Implement the close test-quality repair plan for phase {phase.id}: {phase.title}.

Do not run a new code-quality QA pass in this turn. This turn is only for
implementing the already-planned test-quality fixes and verifying them.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

Use the previous close plan as the work order. Keep the implementation scoped to
phase-relevant test-quality and missing-production-surface fixes. If a finding
requires broader work, name it as residue in chat rather than smuggling it into
this phase.

Run the focused verification commands named by the plan. Summarize the result in
chat. In the JSON state, add only compact `notes.done` and `notes.verification`
markers, keep `status: complete` and `qa_status: passed`, and advance the
cursor to `close_quality_qa`.

Do not put logs, artifacts, command tails, long findings, or plans into the
JSON.

{contract}
