Review phase {phase.id}: {phase.title} against its specification and acceptance
evidence. Check the actual implementation and focused tests. Report only
concrete issues that block this phase; keep later-phase improvements separate.
Do not edit code in this turn.

If blockers exist, finish with:
`RUNNER_EVENT: {"event_type":"review_failed","payload":{"notes":{"findings":["..."]}}}`

If the phase is complete, finish with:
`RUNNER_EVENT: {"event_type":"review_passed","payload":{"notes":{"done":["..."]}}}`

{contract}
