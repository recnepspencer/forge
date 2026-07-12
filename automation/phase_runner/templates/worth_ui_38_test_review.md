[$qa-tests](C:\Users\Esther\.codex\skills\qa-tests\SKILL.md) review tests
for phase {phase.id}: {phase.title}.

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

This is test QA only: do not code or create a repair plan. If the phase is
still mechanically mixed, say test review is premature and identify that
cutover as the primary issue.

Determine whether the tests exercise the ordinary production lane, a partially
ordinary lane with synthetic gaps, or only a certification/fixture/helper seam.
Find the smallest load-bearing dishonest seams: tests that pre-build authority
the runtime should derive, bypass admitted graph or Query-consumption paths,
avoid host/receipt boundaries, or let broad replan and candidate-to-committed
coercion pass unnoticed.

For applicable 3.8 scenarios, assess whether the tests make replay, typed
denial, neighborhood boundedness, and inspection evidence observable. Do not
ask for cosmetic coverage; name the missing production seam that would make
several tests honest.

Finish with either:

`RUNNER_EVENT: {"event_type":"test_review_failed","payload":{"notes":{"findings":["..."]}}}`

or:

`RUNNER_EVENT: {"event_type":"test_review_passed","payload":{"notes":{"done":["..."]}}}`

{contract}
