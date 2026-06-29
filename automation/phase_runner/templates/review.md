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
