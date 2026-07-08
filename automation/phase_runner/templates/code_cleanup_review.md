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

Return only meaningful findings that would make this phase structurally
incomplete. For each finding, name the root issue, the affected surface, and the
cleanup outcome needed.

If meaningful structural work remains, report findings in chat and finish with:

`RUNNER_EVENT: {"event_type":"review_failed","payload":{"notes":{"findings":["structural cleanup findings remain"]}}}`

If the phase cleanup is structurally complete, say so in chat and finish with:

`RUNNER_EVENT: {"event_type":"review_passed","payload":{"notes":{"verification":["structural cleanup review passed"]}}}`

Phase-specific review focus:
{phase.qa_focus}

{contract}

