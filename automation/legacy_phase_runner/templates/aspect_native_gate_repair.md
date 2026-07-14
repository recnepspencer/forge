Now create an in-chat plan to fix the phase {phase.id}: {phase.title}
done-check issues for the Worth Store Aspect-Native Workspace Gate.

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
- Keep the repair scoped to making this phase actually done.
- Do not weaken tests, broaden allowlists, or rename debt to make findings
  disappear.
- Do not keep JSON/serde authority alive through adapters, shims, wrappers,
  bridges, or compatibility facades unless they are mechanically barred from
  ordinary production authority and this phase explicitly owns the quarantine.
- Do not let terminal projection text re-enter authority without explicit
  Store readmission.
- Put the repair plan, implementation explanation, and important evidence in
  chat, not in the JSON.

After repair, update the JSON state only with short progress markers:

- `status: complete`, `qa_status: needed`, and cursor turn `review` if the
  phase is ready to re-check.
- `status: in_progress` and cursor turn `implement` only if substantial
  implementation remains.
- `status: blocked` only for a precise blocker.

Phase-specific instructions:
{phase.instructions}

{contract}
