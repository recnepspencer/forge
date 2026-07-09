Repair the structural review findings for cleanup phase {phase.id}: {phase.title}.

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

Use the review findings as the repair target. Improve the underlying structure:

- clarify ownership
- improve naming
- move code to the responsibility that owns it
- narrow or reshape facades
- extract named classifiers or transition steps
- split overloaded functions into named semantic steps so orchestration reveals
  the transition sequence
- seal construction boundaries where the proof depends on construction order
- update evidence where the repair changes a boundary or behavior

After repair, summarize the structural improvement in chat and return to
structural review.

Phase-specific instructions:
{phase.instructions}

Finish with:

`RUNNER_EVENT: {"event_type":"repair_completed","payload":{"notes":{"done":["structural cleanup repair completed"]}}}`

{contract}

