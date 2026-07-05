[$qa-loop](C:\Users\Esther\.codex\skills\qa-loop\SKILL.md) first, make sure
phase {phase.id}: {phase.title} is 100% done. Let's make sure that we didn't
leave any gaps. Then make sure our approach was thorough and principled, that it
follows our perf and arch laws, and look for missed edge cases.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

Phase scope:
{phase.scope}

Acceptance evidence (re-run focused proof for this phase; broad closeout suites
are opt-in only when explicitly named):
{phase.acceptance}

This is the only runner step that must loop. The review question is: is the
phase actually done?

Use the qa-loop skill and review the real implementation against:

- the spec and this phase's acceptance evidence
- relevant public APIs touched by the phase
- arch laws, perf laws, composition laws, domain structure laws, and DX laws
- missed edge cases and incomplete production surfaces

Before you write findings, do a root-cause check privately and let it shape the
review:

- is the implementation closing the ordinary production lane, or only a
  certification/test seam?
- is any required authority still synthetic, caller-minted, test-minted, or
  reconstruction-based?
- if this implementation passed as-is, what adjacent failure would most likely
  appear in the next phase?
- does the current design fix a local symptom, or does it remove the boundary
  mistake that caused the symptom?

If the deeper issue is architectural rather than local, say so directly.
Do not stop at the nearest observable defect if it is only a manifestation of a
broader missing lane.

For each finding, include a root-cause class:

- `local bug`
- `missing ordinary lane`
- `synthetic authority`
- `projection dishonesty`
- `test-only proof`
- `phase-scope mismatch`
- `boundary collapse`

Do not reward issue-by-issue patching if multiple findings are caused by the
same missing authority seam. Prefer one deeper finding over several shallow
findings when they share a root cause.

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
- cursor turn `test_review`

Do not store logs, artifacts, command tails, long findings, or review prose in
the JSON. The JSON is purely to keep track of where we are.

Phase-specific review focus:
{phase.qa_focus}

{contract}
