Now lets create an in-chat plan to fix the phase {phase.id}: {phase.title}
done-check issues. Make sure it is principled, follows our arch laws, follows
our perf laws, and respects our current APIs.

Then go implement that plan.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

Open done-check summary from JSON:
{phase.notes.findings}

Use the detailed findings from the previous chat turn as the real repair input.
The JSON summary is only a pointer, not the artifact of record.

Repair rules:

- Fix the cause, not the symptom.
- Do not weaken tests or rename debt to make findings disappear.
- Do not keep old authority alive through adapters, shims, wrappers, bridges, or
  compatibility facades unless they are mechanically barred from ordinary
  production authority.
- Keep the repair scoped to making this phase actually done.
- Put the repair plan, implementation explanation, and any important evidence in
  chat, not in the JSON.

After repair, update the JSON state file directly with short progress markers:

- `status: complete`, `qa_status: needed`, and cursor turn `review` if the phase
  is ready to re-check.
- `status: in_progress` and cursor turn `implement` only if substantial
  implementation remains.
- `status: blocked` only for a precise blocker.

Do not stop at architectural analysis. If you can name the real seam, implement
it in this turn and advance the JSON cursor before finishing.

Phase-specific instructions:
{phase.instructions}

{contract}
