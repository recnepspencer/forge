[$qa-loop](C:\Users\Esther\.codex\skills\qa-loop\SKILL.md) first, make sure
phase {phase.id}: {phase.title} is 100% done. Let's make sure that we didn't
leave any gaps. Then make sure our approach was thorough and principled, that it
follows our perf and arch laws, and look for missed edge cases.

Config file: {config_file}
Projection file: {projection_file}
Event log file: {event_log_file}
Spec file: {spec_file}
Run id: {run_id}
Cursor: phase {current.phase}, turn {current.turn}

Phase scope:
{phase.scope}

Acceptance evidence (re-run focused proof for this phase; broad closeout suites
are opt-in only when explicitly named):
{phase.acceptance}

This is the only runner step that must loop. The review question is: is the
phase actually done?

Before declaring review findings, first ask:
Is this phase mechanically cut over, or is the new lane still sharing ordinary
behavior with displaced helpers or legacy callers?

If the phase is still in a mixed cutover state, the primary finding is that the
cutover is incomplete. Do not spend the review turn generating secondary
proof-level findings against that intermediate state. Send it back to repair so
the cutover finishes first.

Use the qa-loop skill and review the real implementation against:

- the spec and this phase's acceptance evidence
- relevant public APIs touched by the phase
- arch laws, perf laws, composition laws, domain structure laws, and DX laws
- missed edge cases and incomplete production surfaces

Review only. Do not fix yet.

If the phase is not actually done, finish with:

`RUNNER_EVENT: {"event_type":"review_failed","payload":{"notes":{"findings":["..."]}}}`

If the phase is actually done, finish with:

`RUNNER_EVENT: {"event_type":"review_passed","payload":{"notes":{"done":["..."],"verification":["..."]}}}`

Phase-specific review focus:
{phase.qa_focus}

{contract}
