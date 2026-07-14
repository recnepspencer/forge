Repair Phase {phase.id}: {phase.title}.

Spec: {spec_file}
Scope: {phase.scope}
Acceptance: {phase.acceptance}
Current finding: {phase.notes.findings}

Execute the repair plan already posted for this finding. If repository evidence
contradicts it, correct the plan in chat before editing. Implement the complete
root-boundary cutover, not merely the reported symptom.

Use and, where necessary, repair the owning existing substrate. Do not reject a
required graph, planning, receipt, Query, host, or evidence defect as "not my
problem," and do not mask it with a phase-local proxy.

Do not patch only the reported symptom. Do not emit `repair_completed` while a
visible sibling bypass, proxy authority, stale export, or duplicate owner
survives. Keep work inside this phase; leave later-phase behavior as a narrow
typed handoff, never a proxy implementation.

Run focused proof for the complete cutover. End with exactly:
`RUNNER_EVENT: {"event_type":"repair_completed","payload":{"notes":{"done":["..."],"verification":["..."]}}}`
