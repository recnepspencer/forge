Review the tests for phase {phase.id}: {phase.title}. Verify that they exercise
the real production behavior and would fail for a materially incorrect
implementation. Report only current-phase blockers. Do not edit code.

If test work is required, finish with:
`RUNNER_EVENT: {"event_type":"test_review_failed","payload":{"notes":{"findings":["..."]}}}`

Otherwise finish with:
`RUNNER_EVENT: {"event_type":"test_review_passed","payload":{"notes":{"done":["..."]}}}`

{contract}
