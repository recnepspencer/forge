from __future__ import annotations

import json
from runner.authority.run_identity import RuntimePaths

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
        "awaiting_operator": projection.get("awaiting_operator"),
        "stop_reason": projection["stop_reason"],
        "thread_id": projection["session"].get("thread_id"),
        "session_threads": projection["session"].get("threads", {}),
        "latest_summary": projection["latest_summary"],
        "last_event": projection["last_event"],
        "notification_delivery_failure": latest_notification_delivery_failure(projection["run_id"]),
        "telegram": telegram_status(projection["run_id"]),
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


def latest_notification_delivery_failure(run_id: str) -> dict | None:
    path = RuntimePaths(run_id).notification_delivery
    if not path.exists():
        return None
    try:
        lines = [line for line in path.read_text(encoding="utf-8").splitlines() if line]
        return json.loads(lines[-1]) if lines else None
    except (OSError, json.JSONDecodeError):
        return {"unreadable": True, "path": str(path)}


def telegram_status(run_id: str) -> dict:
    paths = RuntimePaths(run_id)
    health_path = paths.runtime_root / "telegram" / "poller-health.json"
    receipt_path = paths.telegram_receipts
    result = {"poller_health": None, "latest_inbound_receipt": None}
    for key, path in (("poller_health", health_path), ("latest_inbound_receipt", receipt_path)):
        if not path.exists():
            continue
        try:
            lines = [line for line in path.read_text(encoding="utf-8").splitlines() if line]
            result[key] = json.loads(lines[-1]) if lines else None
        except (OSError, json.JSONDecodeError):
            result[key] = {"unreadable": True, "path": str(path)}
    return result
