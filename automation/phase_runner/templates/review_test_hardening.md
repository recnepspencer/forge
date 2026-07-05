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

If the phase is still in a mixed cutover state, the primary finding is still
that the cutover is incomplete. But do not stop at one sentence. Enumerate the
full set of independent, load-bearing cutover leaks in the same ownership
family that must be fixed together for the phase to become mechanically honest.

Do not pad the review with cosmetic or derivative findings. Do batch the real
ones that share the same root family, especially:

- surviving ordinary callers of displaced lanes
- public re-exports that preserve legacy authority
- constructor or facade seams that still bypass the new lane
- stale compile-fail or boundary proofs that no longer fence the real surface
- any remaining ownership split where the new lane still depends on displaced
  implementation ownership

Do not treat each sibling leak as a separate future turn if it is already
visible now. The repair turn should receive the widest honest fix set it can
substantively close in one pass.

Use the qa-loop skill and review the real implementation against:

- the spec and this phase's acceptance evidence
- relevant public APIs touched by the phase
- arch laws, perf laws, composition laws, domain structure laws, and DX laws
- missed edge cases and incomplete production surfaces

Review only. Do not fix yet.

Root-cause review rule:

- Do not only list the next visible defect.
- First name the shared root defect if findings point to the same bad boundary.
- Classify each finding as one of: local bug, missing test, wrong ownership
  boundary, forgeable authority, certification-overreach, count/projection
  pretending to be proof, or adoption not tied to a production API.
- If two or more findings share a class, write one root repair direction that
  would remove the whole class of failure rather than asking repair to patch
  each symptom.
- For repeated review failures in the same phase, assume the phase may need an
  authority-topology repair. Review for the owning law surface: which lower
  crate should define the sealed witness, which crate may consume it, and which
  public constructors must become impossible.

If the phase is not actually done, finish with:

`RUNNER_EVENT: {"event_type":"review_failed","payload":{"notes":{"findings":["..."]}}}`

If the phase is actually done, finish with:

`RUNNER_EVENT: {"event_type":"review_passed","payload":{"notes":{"done":["..."],"verification":["..."]}}}`

Phase-specific review focus:
{phase.qa_focus}

{contract}
