Now go implement the phase {phase.id}: {phase.title} plan.

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

Use the in-chat plan from the previous runner turn as the working plan. If the
plan is missing or clearly stale, rebuild the missing part inline in chat before
editing. Keep following our arch laws, perf laws, composition laws, domain
structure laws, and DX laws.

Implementation rules:

- Work phase-relevant code only, but follow the real API boundaries wherever the
  phase leads.
- Prefer principled production surfaces over adapters, shims, compatibility
  bridges, fixture-only proof, or renamed debt.
- Make invalid states unrepresentable where the codebase gives you a reasonable
  way to do that.
- Keep directories and files shaped to the explicit skeleton from the plan.
- Keep touched code and test files under the workspace line cap unless an
  explicit exemption already exists.
- Verify what you changed enough to know whether the phase implementation is
  actually ready for done-check QA.
- For parallel cutover phases, finish the mechanical cutover before broad proof.
  Use `cargo check`, focused compile-fail fences, import breakage, and type
  errors as the main guide until the new lane fully owns ordinary behavior.

Do not put logs, command output tails, artifacts, long plans, or review
findings into the runner payload. Put substantive implementation explanation in
chat.

When done, finish with:

`RUNNER_EVENT: {"event_type":"implementation_completed","payload":{"notes":{"done":["..."],"verification":["..."]}}}`

Phase-specific instructions:
{phase.instructions}

{contract}
