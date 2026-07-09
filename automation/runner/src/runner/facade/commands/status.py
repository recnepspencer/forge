from __future__ import annotations

import json

from runner.facade.runtime_state import refresh_projection_for_run, runtime_artifact_surface


def run_status_command(args) -> int:
    projection = refresh_projection_for_run(args.run_id)
    print(json.dumps(status_view(projection), indent=2))
    return 0


def status_view(projection: dict) -> dict:
    artifact_surface = runtime_artifact_surface(projection["run_id"])
    return {
        "run_id": projection["run_id"],
        "authority": {
            "config_path": projection["config_path"],
            "event_log": artifact_surface["authority"],
        },
        "derived": {
            "projection_path": projection.get("projection_path"),
            "artifacts": artifact_surface["derived"],
        },
        "continuity": {
            "artifacts": artifact_surface["continuity"],
        },
        "current": projection["current"],
        "completed_at": projection["completed_at"],
        "stopped": projection["stopped"],
        "stop_reason": projection["stop_reason"],
        "thread_id": projection["session"]["thread_id"],
        "latest_summary": projection["latest_summary"],
        "last_event": projection["last_event"],
        "phases": [
            {
                "id": phase["id"],
                "title": phase["title"],
                "program_id": phase.get("program_id"),
                "status": phase["status"],
                "qa_status": phase["qa_status"],
            }
            for phase in projection["phases"]
        ],
    }
