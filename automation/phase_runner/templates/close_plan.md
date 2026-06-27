Create an in-chat plan to fix the test-quality issues found in the previous
close QA turn for phase {phase.id}: {phase.title}.

Do not edit code in this turn.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

The plan must:

- use the previous close QA findings as the repair input
- make missing production surfaces explicit instead of hiding them in tests
- follow arch, composition, domain-structure, perf, and DX laws
- respect current public APIs and ownership boundaries
- name the exact files/modules expected to change
- name focused verification commands

After posting the plan in chat, update only lightweight JSON state: add a short
`notes.plan` marker, keep `status: complete` and `qa_status: passed`, and
advance the cursor to `close_fix`.

Do not put the full plan, logs, artifacts, or command tails into the JSON.

{contract}
