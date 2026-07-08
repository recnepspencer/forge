Complete the phase {phase.id}: {phase.title} as a single-prompt phase.

Config file: {config_file}
Projection file: {projection_file}
Event log file: {event_log_file}
Spec file: {spec_file}
Run id: {run_id}
Cursor: phase {current.phase}, turn {current.turn}

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

This phase does not use the standard plan/implement/review loop.

Do the exact work described by the phase instructions in one tightly scoped
pass. Keep the work inside the named scope and finish honestly.

Do not mutate runner files directly.

When done, finish with:

`RUNNER_EVENT: {"event_type":"{phase.success_event_type}","payload":{"notes":{"done":["..."],"verification":["..."]}}}`

Phase-specific instructions:
{phase.instructions}

{contract}
