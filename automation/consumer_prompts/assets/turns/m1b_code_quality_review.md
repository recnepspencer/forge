Review the code changed for phase {phase.id}: {phase.title} for concrete
structural problems: unclear ownership, overly broad files, misplaced logic,
public-surface mistakes, or line-cap violations. Do not fail on style preference
alone and do not edit code in this turn.

If blocking structural issues exist, finish with:
`RUNNER_EVENT: {"event_type":"code_quality_review_failed","payload":{"notes":{"findings":["..."]}}}`

Otherwise finish with:
`RUNNER_EVENT: {"event_type":"code_quality_review_passed","payload":{"notes":{"done":["..."],"verification":["..."]}}}`

{contract}
