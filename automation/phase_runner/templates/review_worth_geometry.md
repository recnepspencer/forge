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

Acceptance evidence:
{phase.acceptance}

Review only. Do not fix yet.

This is the only runner step that must loop. The review question is: is the
phase actually done?

Before declaring findings, first ask whether the phase is mechanically cut over:
is the new owner lane the ordinary behavior path, or is it still sharing
authority with displaced helpers, fixtures, wrappers, legacy callers, local
second ontologies, or certification-only seams?

If the phase is still mixed, the primary finding is incomplete cutover. But do
not stop at one sentence. Enumerate the full set of independent, load-bearing
cutover leaks in the same ownership family that must be fixed together.

For Worth geometry work, review the real implementation against:

- the spec and this phase's acceptance evidence
- the relevant public APIs touched by this phase
- the lower owning law/witness surfaces
- ordinary caller paths and downstream handoff consumers
- touched graph, query, index, and aspect authority boundaries
- replay/checkpoint/hostile proof paths
- arch laws, perf laws, composition laws, domain structure laws, and DX laws
- missed edge cases and incomplete production surfaces

Root-cause review rule:

- Do not only list the next visible defect.
- First name the shared root defect if findings point to the same bad boundary.
- Classify each finding as one of: local bug, missing test, wrong ownership
  boundary, forgeable authority, certification-overreach, projection/counter
  pretending to be proof, fixture-owned proof, mixed ordinary lane, or adoption
  not tied to a production API.
- If two or more findings share a class, write one root repair direction that
  would remove the whole class of failure rather than asking repair to patch
  each symptom.
- For repeated review failures in the same phase, assume the phase may need an
  authority-topology repair. Review for the owning law surface: which lower
  crate should define the sealed witness, which crate may consume it, and which
  public constructors must become impossible.

If the phase fails, include a `Repair manifest` section in chat. The manifest
must list the concrete surfaces the repair turn should close together:

- lower owning crate/API surface
- kernel/certification caller or facade path
- public constructor/export/compile-fail boundary
- ordinary production seam
- hostile/focused proof
- downstream handoff proof

Do not pad the review with cosmetic or derivative findings. Do not drip-feed
sibling leaks that are already visible. Do not reward issue-by-issue patching
when the real problem is one missing authority seam.

If the phase is not actually done, finish with:

`RUNNER_EVENT: {"event_type":"review_failed","payload":{"notes":{"findings":["..."]}}}`

If the phase is actually done, finish with:

`RUNNER_EVENT: {"event_type":"review_passed","payload":{"notes":{"done":["..."],"verification":["..."]}}}`

Phase-specific review focus:
{phase.qa_focus}

{contract}
