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

- Operator correction for this repair:
  - Do not invent or demand a bespoke Query-owned "gap ledger" artifact.
  - Use Query surfaces for what they actually mean:
    - support and admission posture prove whether a Query capability is supported, deferred, or unsupported
    - consumer residue audit proves whether Worth is rebuilding Query proof locally
  - Do not collapse those into one fake unified artifact.
  - If Worth still has a surviving wrapper, alias, adapter, local scanner, local manifest, or local second-ontology lane, classify that as Worth-local explicit residue and fix it here.
  - If a migration is blocked because Query does not support a required ordinary-path capability, name the exact missing capability and treat it as a blocker to fix, not as a standing debt ledger.
  - If you cannot name the exact missing Query capability, do not manufacture a Query-gap row. Treat it as Worth-local residue or a bad test assumption and fix the model accordingly.

- Fix the cause, not the symptom.
- Fix the full visible finding family, not just the first file that failed.
- Do not weaken tests or rename debt to make findings disappear.
- Do not keep old authority alive through adapters, shims, wrappers, bridges, or
  compatibility facades unless they are mechanically barred from ordinary
  production authority.
- If review named multiple independent leaks in the same cutover family, close
  them in this turn unless one of them is genuinely blocked by a deeper seam
  you are already replacing.
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
