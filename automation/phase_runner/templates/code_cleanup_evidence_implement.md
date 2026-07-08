Implement the evidence repair plan for cleanup phase {phase.id}: {phase.title}.

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

Implement only the missing evidence identified by the evidence review and plan.
Keep the evidence matched to the cleanup that actually happened.

After implementation, summarize the evidence added or clarified in chat.

Finish with:

`RUNNER_EVENT: {"event_type":"test_repair_completed","payload":{"next_turn":"test_review","notes":{"done":["cleanup evidence repair completed"]}}}`

{contract}

