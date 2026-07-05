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

- Before choosing a fix, classify the problem as one of:
  - `local fix`
  - `structural fix`
  - `phase-scope mismatch`
- Choose `local fix` only if the defect is isolated and fixing it will not
  leave the same authority gap behind.
- Choose `structural fix` if the finding reveals a missing ordinary lane,
  synthetic authority, fake proof path, or boundary collapse.
- Choose `phase-scope mismatch` only if the review is demanding a surface this
  phase does not honestly own.
- State the classification first in chat.
- If the correct classification is `structural fix`, do not propose a narrow
  patch that only silences the current finding.
- Fix the cause, not the symptom.
- Do not weaken tests or rename debt to make findings disappear.
- Do not keep old authority alive through adapters, shims, wrappers, bridges, or
  compatibility facades unless they are mechanically barred from ordinary
  production authority.
- Keep the repair scoped to making this phase actually done.
- Put the repair plan, implementation explanation, and any important evidence in
  chat, not in runner authority files.

In the plan or implementation explanation, say briefly:

- what the real root cause is
- why the chosen fix removes that root cause
- what adjacent future finding this fix is intended to prevent

Do not reward issue-by-issue patching if multiple findings are caused by the
same missing authority seam. Prefer one deeper fix over several shallow fixes
when they share a root cause.

After repair, finish with:

`RUNNER_EVENT: {"event_type":"repair_completed","payload":{"notes":{"done":["..."],"verification":["..."]}}}`

Phase-specific instructions:
{phase.instructions}

{contract}
