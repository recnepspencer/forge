Create an implementation plan for cleanup phase {phase.id}: {phase.title}.

Config file: {config_file}
Projection file: {projection_file}
Event log file: {event_log_file}
Spec file: {spec_file}
Run id: {run_id}
Cursor: phase {current.phase}, turn {current.turn}

Phase scope:
{phase.scope}

Cleanup evidence:
{phase.acceptance}

Use the audit findings and milestone spec. Plan the target structure
explicitly.

Include:

1. The relevant context read and what it constrains.
2. The phase cleanup boundary.
3. The target directory skeleton.
4. The intended public facade shape.
5. The proof-flow grammar after cleanup.
6. The source authority and evidence inputs.
7. The classified cases or transition states.
8. The verification step.
9. The receipt/counter construction point.
10. The next capability exposed to callers.
11. The evidence needed to close the phase.

For any overloaded function in scope, plan the target function shape
explicitly:

- the orchestration function
- named evidence collection step
- named classifier or decision-table step
- named verification/transition step
- named receipt/counter construction step
- named denial/result assembly step where applicable

Make the plan concrete enough that implementation can follow it without
rediscovering the architecture.

Phase-specific instructions:
{phase.instructions}

After posting the plan in chat, finish with:

`RUNNER_EVENT: {"event_type":"plan_posted","payload":{"notes":{"plan":["cleanup plan posted"]}}}`

{contract}

