Close out cleanup phase {phase.id}: {phase.title}.

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

Confirm the phase now has:

- the intended cleanup boundary completed
- clearer topology than before
- explicit proof-flow transitions where applicable
- overloaded functions decomposed or justified as cohesive semantic units
- narrower or better-shaped public surfaces
- clear production/certification/test authority boundaries
- appropriate evidence for the kind of cleanup performed

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

If meaningful structural work remains, report all phase-scoped blockers grouped
by area and the required cleanup outcomes in chat. Do not collapse multiple
blocking issues into one representative example. Then finish with:

`RUNNER_EVENT: {"event_type":"code_quality_review_failed","payload":{"notes":{"findings":["cleanup closeout blocker remains"]}}}`

If the cleanup phase is complete, say so in chat and finish with:

`RUNNER_EVENT: {"event_type":"code_quality_review_passed","payload":{"notes":{"verification":["cleanup closeout passed"]}}}`

Phase-specific review focus:
{phase.qa_focus}

{contract}
