Before planning phase {phase.id}: {phase.title}, briefly identify the boundary
the implementation must preserve.

Read the phase scope, acceptance evidence, relevant code, and milestone spec.
State what this phase owns, what remains outside its scope, and the main mistake
the implementation should avoid. Keep the brief proportional to the phase and
do not write the implementation plan yet.

Finish with:
`RUNNER_EVENT: {"event_type":"boundary_review_completed","payload":{"notes":{"plan":["..."]}}}`

{contract}
