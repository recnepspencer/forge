Now lets create an in-chat plan to fix the phase {phase.id}: {phase.title}
done-check issues. Make sure it is principled, follows our arch laws, follows
our perf laws, and respects our current APIs.

Then go implement that plan.

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

Open done-check summary from projection:
{phase.notes.findings}

Use the detailed findings from the previous chat turn as the real repair input.
The projection summary is only a pointer, not the artifact of record.

Repair rules:

- Fix the cause, not the symptom.
- Do not weaken tests or rename debt to make findings disappear.
- Do not keep old authority alive through adapters, shims, wrappers, bridges, or
  compatibility facades unless they are mechanically barred from ordinary
  production authority.
- Keep the repair scoped to making this phase actually done.
- Put the repair plan, implementation explanation, and any important evidence in
  chat, not in runner authority files.

After repair, finish with:

`RUNNER_EVENT: {"event_type":"repair_completed","payload":{"notes":{"done":["..."],"verification":["..."]}}}`

Do not stop at architectural analysis. If you can name the real seam, implement
it in this turn and advance the runner honestly.

Phase-specific instructions:
{phase.instructions}

{contract}
