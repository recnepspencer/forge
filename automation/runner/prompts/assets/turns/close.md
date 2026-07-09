Phase {phase.id}: {phase.title} has passed the required done-check loop.

This is the legacy bundled close turn. Prefer the segmented close-hardening
prompt set when the state file includes these turns:

- `close_qa`
- `close_plan`
- `close_fix`
- `close_quality_qa`
- `close_quality_plan`
- `close_quality_fix`

If those turns exist in this state, do not run QA, plan, and fixes in this same
turn. Update only lightweight JSON state by moving this phase cursor to
`close_qa`, then stop.

If this state has not yet been migrated, run the smallest closeout verification
needed to avoid losing progress, keep the phase `status: complete` and
`qa_status: passed`, add only compact tracking markers, and advance to the next
phase at `plan` or set `current: null` if this was the final phase.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

Acceptance evidence:
{phase.acceptance}

Do not put logs, artifacts, command tails, long QA lists, or plans into the
JSON. The JSON is only progress tracking.

{contract}
