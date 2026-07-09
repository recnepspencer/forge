[$qa-loop](C:\Users\Esther\.codex\skills\qa-loop\SKILL.md) first, make sure
phase {phase.id}: {phase.title} of the WORTH Store Aspect-Native Workspace
Gate is actually done.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

This is the only runner step that must loop. The review question is: is this
phase actually done according to the gate spec?

Use the qa-loop skill and review the real implementation against:

- the gate spec and this phase's acceptance evidence
- relevant public APIs touched by the phase
- arch laws, perf laws, composition laws, domain-structure laws, and DX laws
- whether JSON/serde authority was actually denied, not merely renamed
- whether terminal projections are quarantined and cannot re-enter authority
- whether tests prove production surfaces honestly rather than passing through
  synthetic fixture-only proof
- missed edge cases and incomplete production surfaces

Do not loop on whole-gate perfection, broad test purity, directory polish, or
aerospace-grade status unless the issue proves this phase itself is not done.
Those are close-pass hardening inputs, not loop conditions.

If the phase is not actually done, report the findings in chat with file/line
references and update the JSON state only with a short marker:

- `status: regressed`
- `qa_status: failed`
- a short `notes.findings` summary
- cursor turn `repair`

If the phase is actually done, say so in chat and update the JSON state only
with a short marker:

- `status: complete`
- `qa_status: passed`
- cursor turn `close`

Do not store logs, artifacts, command tails, long findings, or review prose in
the JSON. The JSON is purely to keep track of where we are.

Phase-specific review focus:
{phase.qa_focus}

{contract}
