[$code-quality-qa](C:\Users\Esther\.codex\skills\code-quality-qa\SKILL.md) now
review the code quality of phase {phase.id}: {phase.title}.

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

Review only. Do not fix yet.

Look for:

- composition-law violations
- domain-structure-law violations
- files or modules that are too broad
- facade-shaped modules that implement rather than aggregate
- hidden helper buckets
- file length and directory shape problems
- weak ownership boundaries
- places where the implementation works but teaches the wrong architecture

This turn does not reopen the phase loop. It is the closeout honesty pass for
directory shape, file size discipline, and ownership boundaries after the phase
is already functionally done.

If you find issues, report them in chat with file/line references, but still
close the phase honestly rather than pretending it is aerospace-grade.

Re-run the acceptance checks and record concise command evidence in
`payload.notes.verification`. Then finish with:

`RUNNER_EVENT: {"event_type":"code_quality_review_passed","payload":{"notes":{"done":["..."],"remaining":["..."],"verification":["..."]}}}`

{contract}
