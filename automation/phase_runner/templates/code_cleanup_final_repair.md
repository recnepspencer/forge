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

Repair all phase-scoped blocking structural issues from the closeout review.
The repair should improve implementation shape directly:

- split god functions into named semantic steps
- move behavior into lifecycle/domain modules
- introduce classifiers for meaningful decision branches
- separate collect/classify/verify/build responsibilities
- make next valid proof/capability obvious from the returned type
- keep facades as routing surfaces, not business logic homes

Do not stop after making `lib.rs`, `mod.rs`, exports, or file placement look
cleaner if actual proof flows remain hard to audit.

After repair, summarize the closeout improvement in chat. Include each blocking
issue and the concrete structural change that resolves it.

Finish with:

`RUNNER_EVENT: {"event_type":"code_quality_repair_completed","payload":{"notes":{"done":["cleanup closeout repair completed"]}}}`

{contract}
