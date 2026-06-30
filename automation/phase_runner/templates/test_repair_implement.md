Now implement the test hardening plan for phase {phase.id}: {phase.title}.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

Open plan summary from JSON:
{phase.notes.plan}

Use the detailed plan from the previous chat turn as the real implementation
input. The JSON summary is only a pointer, not the artifact of record.

Implement the planned production and test changes needed to make the tests
honest.

Rules:

- Prefer real production paths over test-only seams.
- Do not make the tests pass by weakening assertions.
- Do not add fake helper surfaces where the production runtime should own the
  meaning.
- Re-run the focused verification needed to prove the hardened tests are real.

After implementation, update the JSON state file directly:

- short `notes.done`
- short `notes.verification`
- cursor turn `test_review` if the hardened tests need re-review
- cursor turn `code_quality_review` if the test lane is ready to close
- do not leave `current` on `test_repair_implement` after writing the state
- if this implementation makes the test lane honest enough and the phase row is
  `status: complete` and `qa_status: passed`, you must move to `test_review` or
  `code_quality_review` in the same write
- if `current` still points at `test_repair_implement` after you decide the
  hardening work is complete, that is a stale cursor and you must repair it

{contract}
