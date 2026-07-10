Create an in-chat plan to fix the structural code-quality issues found in the
previous close quality QA turn for phase {phase.id}: {phase.title}.

Do not edit code in this turn.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

The plan must:

- use the previous code-quality QA findings as the repair input
- keep fixes phase-relevant and ownership-safe
- follow arch, composition, domain-structure, perf, and DX laws
- name files/modules expected to change
- name focused verification commands
- explicitly refuse to treat concrete structural-law violations as optional
  residue

After posting the plan in chat, update only lightweight JSON state: add a short
`notes.plan` marker, keep the phase `status: regressed` and `qa_status: failed`
when the plan addresses concrete structural-law findings, and advance the cursor
to `close_quality_fix`.

Do not put the full plan, logs, artifacts, or command tails into the JSON.

{contract}
