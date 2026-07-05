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

Do an ordinary-lane audit before writing findings:

- are the tests proving the real ordinary production lane?
- are they proving a certification-only seam, fixture seam, helper seam, or
  synthetic constructor instead?
- could the production path still be missing while these tests pass?
- is any test pre-solving the authority artifact the phase is supposed to
  derive?

If the tests certify the wrong seam, treat that as a primary finding even if
coverage looks strong.

State explicitly in chat whether the current phase proves:

- the ordinary lane
- a partially ordinary lane with synthetic gaps
- only a certification seam

If it is not the ordinary lane, say exactly what production caller, authority
path, or boundary crossing is still missing.

Be concrete. Use file/line references when possible.

If the tests need hardening, finish with:

`RUNNER_EVENT: {"event_type":"test_review_failed","payload":{"notes":{"findings":["..."]}}}`

If the tests are already honest enough, finish with:

`RUNNER_EVENT: {"event_type":"test_review_passed","payload":{"notes":{"done":["..."]}}}`

{contract}
