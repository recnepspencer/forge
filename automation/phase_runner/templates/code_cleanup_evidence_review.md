Review the cleanup evidence for phase {phase.id}: {phase.title}.

Config file: {config_file}
Projection file: {projection_file}
Event log file: {event_log_file}
Spec file: {spec_file}
Run id: {run_id}
Cursor: phase {current.phase}, turn {current.turn}

Phase scope:
{phase.scope}

Cleanup evidence:
{phase.acceptance}

This is an evidence review for a cleanup phase. Evaluate whether the evidence
matches the kind of cleanup performed.

Look for:

- structural diffs that demonstrate improved topology
- public API diffs that demonstrate narrower or clearer facades
- named transition functions, classifiers, or decision tables where proof flow
  was cleaned up
- compile-fail coverage where construction or visibility boundaries changed
- runtime tests where behavior changed or hostile behavior had to be preserved
- focused verification commands for touched crates/modules
- a clear explanation for pure topology cleanup when no executable behavior
  changed

If evidence is missing or mismatched to the cleanup, report the missing evidence
in chat and finish with:

`RUNNER_EVENT: {"event_type":"test_review_failed","payload":{"notes":{"findings":["cleanup evidence is incomplete"]}}}`

If the evidence is appropriate, say so in chat and finish with:

`RUNNER_EVENT: {"event_type":"test_review_passed","payload":{"notes":{"verification":["cleanup evidence review passed"]}}}`

Phase-specific review focus:
{phase.qa_focus}

{contract}

