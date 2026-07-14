Use [$qa-loop](C:\Users\Esther\.codex\skills\qa-loop\SKILL.md) to review
Phase {phase.id}: {phase.title}. Review only; do not edit.

Spec: {spec_file}
Scope: {phase.scope}
Acceptance: {phase.acceptance}
Focus: {phase.qa_focus}

Assume the underlying code may have deep structural defects. Before judging
local correctness, trace the authority boundary end-to-end: canonical owner,
constructors, exports, callers, tests, counters, and ordinary production path.
Treat the existing graph, planning, receipt, Query, host, and evidence substrate
as part of that boundary. Do not dismiss a missing or broken required substrate
as "not this phase's problem"; identify its owning seam and require the
principled extension or repair needed for this phase to use it honestly.

If a structural defect exists, report one batched root finding: name the owning
artifact, every visible bypass/sibling seam, and the complete cutover required.
Do not drip-feed symptoms that repository search can reveal now. Do not fail
this phase for work explicitly assigned to a later phase.

End with exactly one:
`RUNNER_EVENT: {"event_type":"review_failed","payload":{"notes":{"findings":["..."]}}}`
or
`RUNNER_EVENT: {"event_type":"review_passed","payload":{"notes":{"done":["..."],"verification":["..."]}}}`
