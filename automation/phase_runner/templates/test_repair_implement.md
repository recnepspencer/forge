Now implement the test hardening plan for phase {phase.id}: {phase.title}.

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

Open plan summary from projection:
{phase.notes.plan}

Use the detailed plan from the previous chat turn as the real implementation
input. The projection summary is only a pointer, not the artifact of record.

Implement the planned production and test changes needed to make the tests
honest.

Rules:

- Prefer real production paths over test-only seams.
- Do not make the tests pass by weakening assertions.
- Do not add fake helper surfaces where the production runtime should own the
  meaning.
- Re-run the focused verification needed to prove the hardened tests are real.

After implementation, finish with one of:

`RUNNER_EVENT: {"event_type":"test_repair_completed","payload":{"next_turn":"test_review","notes":{"done":["..."],"verification":["..."]}}}`

or

`RUNNER_EVENT: {"event_type":"test_repair_completed","payload":{"next_turn":"code_quality_review","notes":{"done":["..."],"verification":["..."]}}}`

{contract}
