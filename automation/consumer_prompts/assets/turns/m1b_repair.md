Repair the concrete review findings for phase {phase.id}: {phase.title}.

Confirm each finding applies to current acceptance, make the smallest complete
fix, preserve unrelated work, and run focused verification. Do not widen the
phase to address optional hardening or later milestones.

Finish with:
`RUNNER_EVENT: {"event_type":"repair_completed","payload":{"notes":{"done":["..."],"verification":["..."]}}}`

{contract}
