Review the completed cleanup for phase {phase.id}: {phase.title}.

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

Judge whether the phase now satisfies the milestone's cleanup goal.

Assess:

- whether the directory tree teaches the lifecycle or authority boundary
- whether files are cohesive semantic units
- whether public facades are narrow and lifecycle-shaped
- whether proof transitions are visible and named
- whether classifiers or decision tables make cases easy to audit
- whether receipts/counters come from verified outcomes
- whether certification/test support has clear authority status
- whether the next valid API call is apparent from returned types
- whether important functions still hide multiple semantic steps in one body,
  especially evidence collection, validation, classification, mutation, proof
  construction, counter updates, and result assembly

Before deciding this phase passes, do a deep-structure closeout check.

A phase does not pass just because the public facade, `lib.rs`, `mod.rs`,
exports, or directory names are cleaner. It passes only if the implementation
beneath that surface is also easier for a future model or human to reason about.

Check every changed production and test-support area in this phase for:

- lifecycle-shaped implementation modules, not just lifecycle-shaped facades
- narrow orchestration functions rather than god functions
- explicit proof/state transitions instead of scattered predicates
- named classification/decision cases where branches matter
- receipt/proof construction separated from evidence collection and verification
- helper/test-support code placed by domain responsibility, not convenience
- public surfaces that route, while domain modules own behavior

If the cleanup mainly moved files or grouped exports while underlying flows
still require a reader to reconstruct the logic from broad functions, copied
fields, or implicit branches, fail the phase and request repair.

Return only meaningful findings that would make this phase structurally
incomplete. For each finding, name the root issue, the affected surface, and the
cleanup outcome needed. When failing, list all phase-scoped structural issues
that block pass, grouped by area. Do not collapse multiple blocking issues into
one representative example.

If meaningful structural work remains, report findings in chat and finish with:

`RUNNER_EVENT: {"event_type":"review_failed","payload":{"notes":{"findings":["structural cleanup findings remain"]}}}`

If the phase cleanup is structurally complete, say so in chat and finish with:

`RUNNER_EVENT: {"event_type":"review_passed","payload":{"notes":{"verification":["structural cleanup review passed"]}}}`

Phase-specific review focus:
{phase.qa_focus}

{contract}
