Repair the cleanup closeout blocker for phase {phase.id}: {phase.title}.

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

Use the closeout finding as the repair target. Improve the structure or
evidence needed for phase closeout:

- complete the intended cleanup boundary
- clarify topology
- make proof transitions explicit
- improve or narrow public surfaces
- clarify authority status for production, certification, and test support
- decompose overloaded functions that still hide cleanup-relevant semantic
  steps
- add or clarify evidence where the closeout needs it

After repair, summarize the closeout improvement in chat.

Finish with:

`RUNNER_EVENT: {"event_type":"code_quality_repair_completed","payload":{"notes":{"done":["cleanup closeout repair completed"]}}}`

{contract}

