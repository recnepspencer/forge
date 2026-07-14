Repair phase {phase.id}: {phase.title} against its admitted done-check findings.

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

Open finding summary from projection:
{phase.notes.findings}

Read the detailed findings artifact, but treat each finding as a claim to
adjudicate against the current phase specification—not as automatic authority.

Before editing, state briefly in chat:

- the exact current-phase obligation
- each admitted blocking finding
- each rejected `phase-scope mismatch` or non-blocking hardening item
- the smallest file and test surface you intend to touch

Repair rules:

- The phase specification, including Warnings and explicit exclusions, is the
  scope ceiling.
- Fix only admitted blockers required for the current phase's named acceptance.
- Prefer one smallest common-cause correction when admitted findings share a
  real root cause.
- Do not expand into adjacent phases, generalized framework construction, or
  compiler-completeness work merely because a hypothetical bypass can be named.
- Do not add adapters, wrappers, aliases, or test-only seams that preserve the
  actual admitted defect.
- Do not weaken tests. Exercise the real production path named by acceptance.
- Use focused verification during repair. Run broad suites only when current
  acceptance explicitly names them as closeout evidence.
- If every received finding is a scope mismatch or follow-up hardening item,
  make no code change, explain the adjudication, and complete the turn normally.

Completion rule:

- stop when the current phase's named acceptance and required hostile proofs pass
- do not keep widening the repair to eliminate later-phase or trusted-internal
  hardening concerns
- leave the next review a concise statement of what changed and what was
  intentionally rejected as out of scope

Put plans, reasoning, and evidence in chat, not runner authority files. Never
edit runner runtime artifacts directly.

Finish with:

`RUNNER_EVENT: {"event_type":"repair_completed","payload":{"notes":{"done":["..."],"verification":["..."]}}}`

Phase-specific instructions:
{phase.instructions}

{contract}
