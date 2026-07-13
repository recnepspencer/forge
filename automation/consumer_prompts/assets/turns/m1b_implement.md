Implement phase {phase.id}: {phase.title} using the current plan.

Stay within the phase scope, follow the repository laws, preserve unrelated
work, and use the real production API. Add the focused tests required by the
acceptance evidence and run enough verification to support completion.

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

Finish with:
`RUNNER_EVENT: {"event_type":"implementation_completed","payload":{"notes":{"done":["..."],"verification":["..."]}}}`

{contract}
