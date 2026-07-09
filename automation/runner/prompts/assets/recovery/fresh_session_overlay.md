Fresh recovery session context:

The durable runner intentionally dropped the previous persistent agent session before this turn.
Reason: {fresh_recovery.reason}
Observed QA/repair cycle count: {fresh_recovery.cycle_count} (threshold: {fresh_recovery.threshold})

You are a fresh agent stepping into a stuck phase. First rebuild context from the spec, projection,
event log, current phase, recent findings, recent repair summaries, and touched files. Look for the
deeper repeated structural cause before editing. This is not permission to bypass the current turn:
complete the current turn honestly and emit the normal RUNNER_EVENT for this turn when done.
