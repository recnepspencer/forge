[$qa-tests](C:\Users\Esther\.codex\skills\qa-tests\SKILL.md) now review the
tests for phase {phase.id}: {phase.title}.

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

This turn is test QA only. Do not code yet. Do not make a repair plan yet.

Your job is to produce a findings-first review of the tests:

- everything weak, synthetic, overly mocked, or proof-light
- where tests are bypassing real production paths
- where a subtly dishonest implementation could still pass
- which production surfaces are missing to support honest tests
- which verification lanes are too shallow to support the phase claims

Be concrete. Use file/line references when possible.

If the tests need hardening, finish with:

`RUNNER_EVENT: {"event_type":"test_review_failed","payload":{"notes":{"findings":["..."]}}}`

If the tests are already honest enough, finish with:

`RUNNER_EVENT: {"event_type":"test_review_passed","payload":{"notes":{"done":["..."]}}}`

{contract}
