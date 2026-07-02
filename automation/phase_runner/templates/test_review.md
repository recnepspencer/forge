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

Cutover gate for test review:

Do not perform deep hostile test review against a mechanically incomplete
migration. If the new lane does not yet fully own ordinary behavior, the honest
conclusion is that test review is premature. Call out the incomplete cutover as
the load-bearing issue and return the phase to repair/implement instead of
hardening tests around a mixed state.

Your job is to produce a findings-first review of the tests:

- everything weak, synthetic, overly mocked, or proof-light
- where tests are bypassing real production paths
- where a subtly dishonest implementation could still pass
- which production surfaces are missing to support honest tests
- which verification lanes are too shallow to support the phase claims

Be concrete. Use file/line references when possible.

Aggression rules for this review:

- Find the smallest set of load-bearing dishonest seams that explain the test
  weakness. Do not produce a long list of cosmetic or sibling-level findings
  when they collapse to one root cause.
- Prefer "this test bypasses the real production seam" over weaker variants of
  the same complaint.
- If one real production surface is missing and would make several tests honest,
  call that out as the primary finding instead of scattering the problem across
  many test-only complaints.
- Keep the review tight enough that the next turn can fix the seam directly.

If the tests need hardening, update the JSON state file directly:

- short `notes.findings`
- cursor turn `test_repair_plan`
- do not mark the phase `complete/passed` and leave `current` on
  `test_review`, `test_repair_plan`, or `test_repair_implement`

If the tests are already honest enough, update the JSON state file directly:

- short `notes.done`
- cursor turn `code_quality_review`
- if the phase row is `status: complete` and `qa_status: passed`, you must
  advance to `code_quality_review` now
- never leave `current` on any same-phase `test_*` turn after deciding the test
  lane is honest enough

{contract}
