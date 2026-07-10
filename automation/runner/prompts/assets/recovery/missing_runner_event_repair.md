The previous agent turn finished without emitting the required runner outcome marker.

Run id: {run_id}
Projection file: {projection_file}
Event log file: {event_log_file}
Current cursor: {current_cursor}
Expected turn instance id: {expected_turn_instance_id}
{artifact_block}Do not redo the review. Reconstruct the correct outcome for the already-completed turn from the prompt artifact,
contract artifact, projection, event log, recent code changes, and the existing thread context.

Your entire response must be exactly one compact line in this format:
RUNNER_EVENT: {"event_type":"event_name","payload":{"turn_instance_id":"{expected_turn_instance_id}"}}

The payload must include exactly "turn_instance_id":"{expected_turn_instance_id}".
Do not include prose before or after the marker.
