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

Repair all phase-scoped blocking structural issues from the review. The repair
should improve implementation shape directly:

- split god functions into named semantic steps
- move behavior into lifecycle/domain modules
- introduce classifiers for meaningful decision branches
- separate collect/classify/verify/build responsibilities
- make next valid proof/capability obvious from the returned type
- keep facades as routing surfaces, not business logic homes

Do not stop after making `lib.rs`, `mod.rs`, exports, or file placement look
cleaner if actual proof flows remain hard to audit.

After repair, summarize the structural improvement in chat and return to
structural review. Include each blocking issue and the concrete structural
change that resolves it.

Phase-specific instructions:
{phase.instructions}

Finish with:

`RUNNER_EVENT: {"event_type":"repair_completed","payload":{"notes":{"done":["structural cleanup repair completed"]}}}`

{contract}
