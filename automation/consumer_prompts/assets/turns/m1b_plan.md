Plan phase {phase.id}: {phase.title}.

Read the milestone spec, current phase scope, acceptance evidence, and relevant
code. Produce a practical implementation plan with the files to change, the
main design choice, and focused verification. Keep the plan proportional to
the phase; do not expand into later phases.

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

Finish with:
`RUNNER_EVENT: {"event_type":"plan_posted","payload":{"notes":{"plan":["..."]}}}`

{contract}
