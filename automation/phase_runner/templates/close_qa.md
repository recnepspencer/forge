Phase {phase.id}: {phase.title} has passed the required done-check loop.

Run only the test-quality QA discovery pass now. Do not create a fix plan. Do
not edit production or test code. Do not repair issues in this turn.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

Acceptance evidence:
{phase.acceptance}

Use this skill:

[$qa-tests](C:\Users\Esther\.codex\skills\qa-tests\SKILL.md)

Review this phase's tests with a hostile Forge-quality posture. Find everything
weak, synthetic, too self-referential, fixture-theater-shaped, or insufficiently
bound to real production behavior. Then tell which production surfaces are
missing to support those tests honestly.

Output the findings in chat. In the JSON state, record only short findings
markers in `notes.findings`, keep `status: complete` and `qa_status: passed`,
and advance the cursor to `close_plan`.

Do not put logs, artifacts, command tails, long QA lists, or plans into the
JSON. The JSON is only progress tracking.

{contract}
