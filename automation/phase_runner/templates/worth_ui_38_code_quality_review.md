[$code-quality-qa](C:\Users\Esther\.codex\skills\code-quality-qa\SKILL.md)
review the structural code quality of phase {phase.id}: {phase.title}.

Config file: {config_file}
Projection file: {projection_file}
Event log file: {event_log_file}
Spec file: {spec_file}
Run id: {run_id}
Cursor: phase {current.phase}, turn {current.turn}

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

Review only. This is a gate: concrete composition-law, domain-structure-law,
file-size, directory-topology, public-facade, `mod.rs`, helper-placement, or
ownership-boundary violations fail the phase and return it to repair.

Verify that receipt identity, invalidation classification, stream policy,
replan selection, freshness transitions, evidence, and inspection each have
clear responsibility-shaped homes. Reject host, renderer, gesture, helper, or
certification modules that decide allocation semantics or maintain a second
dependency graph. Reject reachable generic "changed" invalidation, generic
debounce as semantic policy, root fallback, or candidate-to-committed
conversion in ordinary production code.

Report concrete issues with file/line references. Do not pass while known
structural defects remain. Run focused acceptance proof and include concise
verification in the event.

Finish with either:

`RUNNER_EVENT: {"event_type":"code_quality_review_failed","payload":{"notes":{"findings":["..."]}}}`

or:

`RUNNER_EVENT: {"event_type":"code_quality_review_passed","payload":{"notes":{"done":["..."],"remaining":["..."],"verification":["..."]}}}`

{contract}
