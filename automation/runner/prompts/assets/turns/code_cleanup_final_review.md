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

If meaningful structural work remains, report the blocker and the required
cleanup outcome in chat, then finish with:

`RUNNER_EVENT: {"event_type":"code_quality_review_failed","payload":{"notes":{"findings":["cleanup closeout blocker remains"]}}}`

If the cleanup phase is complete, say so in chat and finish with:

`RUNNER_EVENT: {"event_type":"code_quality_review_passed","payload":{"notes":{"verification":["cleanup closeout passed"]}}}`

Phase-specific review focus:
{phase.qa_focus}

{contract}

