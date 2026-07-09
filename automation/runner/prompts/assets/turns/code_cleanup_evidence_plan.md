Create a focused evidence repair plan for cleanup phase {phase.id}: {phase.title}.

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

Use the evidence review findings. Plan only the missing evidence needed to close
this cleanup phase.

Evidence may be:

- a directory skeleton diff
- a public API diff
- a removed export or visibility proof
- a named decision table or classifier
- compile-fail coverage for a construction boundary
- runtime coverage for changed behavior
- a focused verification command
- a short closeout note explaining why topology-only cleanup did not require
  executable tests

Post the evidence repair plan in chat, then finish with:

`RUNNER_EVENT: {"event_type":"test_repair_plan_posted","payload":{"notes":{"plan":["cleanup evidence repair plan posted"]}}}`

{contract}

