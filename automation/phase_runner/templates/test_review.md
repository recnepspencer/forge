[$qa-tests](C:\Users\Esther\.codex\skills\qa-tests\SKILL.md) now review the
tests for phase {phase.id}: {phase.title}.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

This turn is test QA only. Do not code yet. Do not make a repair plan yet.

Your job is to produce a findings-first review of the tests:

- everything weak, synthetic, overly mocked, or proof-light
- where tests are bypassing real production paths
- where a subtly dishonest implementation could still pass
- which production surfaces are missing to support honest tests
- which verification lanes are too shallow to support the phase claims

Be concrete. Use file/line references when possible.

If the tests need hardening, update JSON only:

- short `notes.findings`
- cursor turn `test_repair_plan`

If the tests are already honest enough, update JSON only:

- short `notes.done`
- cursor turn `code_quality_review`

{contract}
