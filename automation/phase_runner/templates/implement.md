Now go implement the phase {phase.id}: {phase.title} plan.

State file: {state_file}
Spec file: {spec_file}
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

Do not put logs, command output tails, artifacts, long plans, or review findings
into the state payload. Put substantive implementation explanation in chat. The
committed state is only progress tracking.

When done, commit the phase outcome through `state_tool.py apply` with only
short progress markers:

- `status: complete` and `qa_status: needed` if implementation is ready for the
  phase-done check.
- `status: in_progress` if implementation remains, with a short `notes.remaining`
  marker.
- `status: blocked` only for a precise blocker.

Advance the cursor to `review` only when implementation is ready for the
phase-done check; otherwise stay on `implement`.

Phase-specific instructions:
{phase.instructions}

{contract}
