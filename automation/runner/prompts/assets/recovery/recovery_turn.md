The automated durable phase runner hit a failure on the current turn.

Run id: {run_id}
Projection file: {projection_file}
Event log file: {event_log_file}
Current cursor: {current_cursor}
Failure reason: {failure_reason}
{recovery_route_guidance}

Continue in the same persistent agent session when available. Re-read the current phase context if needed,
then finish the same turn honestly. Do not mutate any runner files directly.

If the prior agent turn already completed the work, do not redo the work. Emit the correct
typed RUNNER_EVENT for that already-completed turn.

Expected turn instance id: {expected_turn_instance_id}
Your RUNNER_EVENT payload must include exactly "turn_instance_id":"{expected_turn_instance_id}".

Your final line must be exactly one of the allowed compact markers below:
{expected_runner_event_markers}
