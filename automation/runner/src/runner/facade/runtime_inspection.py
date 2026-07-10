from __future__ import annotations

import json
import shutil
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from runner.authority.events import load_events
from runner.authority.run_identity import CANONICAL_RUNTIME_ROOT, RuntimePaths
from runner.facade.runtime_state import config_path_for_run, refresh_projection_for_run, runtime_artifact_surface
from runner.facade.commands.status import status_view


@dataclass(frozen=True)
class RuntimeArtifactRecord:
    lane: str
    retention_class: str
    path: str
    exists: bool
    bytes: int


def run_report(run_id: str) -> dict[str, Any]:
    projection = refresh_projection_for_run(run_id)
    events = load_events(RuntimePaths(run_id).events)
    status = status_view(projection)
    return {
        "run_id": run_id,
        "state": classify_run_state(projection),
        "current": projection.get("current"),
        "completed_at": projection.get("completed_at"),
        "stopped": projection.get("stopped"),
        "latest_summary": projection.get("latest_summary"),
        "event_count": len(events),
        "last_event": events[-1] if events else None,
        "telegram": status["telegram"],
        "notification_delivery_failure": status["notification_delivery_failure"],
        "next_operator_action": next_operator_action(status),
    }


def doctor_report(run_id: str) -> dict[str, Any]:
    projection = refresh_projection_for_run(run_id)
    status = status_view(projection)
    findings = doctor_findings(status)
    return {
        "run_id": run_id,
        "healthy": not any(finding["severity"] == "error" for finding in findings),
        "state": classify_run_state(projection),
        "findings": findings,
    }


def artifact_inventory(run_id: str) -> dict[str, Any]:
    records = [
        artifact_record(lane["lane"], retention_class_for_lane(lane["lane"]), Path(lane["path"]))
        for group in runtime_artifact_surface(run_id).values()
        for lane in group
    ]
    records.append(artifact_record("archives", "archive", RuntimePaths(run_id).archive.resolve()))
    return {"run_id": run_id, "artifacts": [record.__dict__ for record in records]}


def active_runs() -> dict[str, Any]:
    run_ids = sorted(run_id_from_event_path(path) for path in event_root().glob("*.jsonl"))
    active = []
    for run_id in run_ids:
        try:
            projection = refresh_projection_for_run(run_id)
        except Exception as error:
            active.append({"run_id": run_id, "state": "unreadable", "reason": str(error)})
            continue
        state = classify_run_state(projection)
        if state in {"active", "stopped"}:
            active.append({"run_id": run_id, "state": state, "current": projection.get("current")})
    return {"active": active}


def archive_run(run_id: str, *, prune_derived: bool = False) -> dict[str, Any]:
    paths = RuntimePaths(run_id)
    report = run_report(run_id)
    if prune_derived and report["state"] != "completed":
        raise ValueError("archive --prune-derived requires a completed run")
    config_path = config_path_for_run(run_id)
    archive_root = paths.archive
    archive_root.mkdir(parents=True, exist_ok=True)
    copied = {
        "events": copy_file(paths.events, archive_root / "events.jsonl"),
        "projection": copy_file(paths.projection, archive_root / "projection.json"),
        "config": copy_file(config_path, archive_root / "config.json"),
    }
    (archive_root / "report.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    manifest = {
        "run_id": run_id,
        "archive_root": str(archive_root.resolve()),
        "copied": copied,
        "pruned": prune_derived_artifacts(paths) if prune_derived else [],
    }
    (archive_root / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    return manifest


def classify_run_state(projection: dict[str, Any]) -> str:
    if projection.get("completed_at") is not None:
        return "completed"
    if projection.get("stopped"):
        return "stopped"
    if projection.get("current") is not None:
        return "active"
    return "idle"


def next_operator_action(status: dict[str, Any]) -> str:
    if status["completed_at"] is not None:
        return "archive when inspection is complete"
    if status["stopped"]:
        return "resume or inspect stop_reason"
    if status["notification_delivery_failure"] is not None:
        return "inspect notification delivery failure"
    telegram = status["telegram"]
    health = telegram.get("poller_health")
    if isinstance(health, dict) and health.get("healthy") is False:
        return "restart or inspect Telegram poller"
    return "continue monitoring current run"


def doctor_findings(status: dict[str, Any]) -> list[dict[str, str]]:
    findings: list[dict[str, str]] = []
    if status["notification_delivery_failure"] is not None:
        findings.append({"severity": "error", "code": "notification_delivery_failed", "message": "latest notification sink delivery failed"})
    telegram = status["telegram"]
    health = telegram.get("poller_health")
    if isinstance(health, dict) and health.get("healthy") is False:
        findings.append({"severity": "error", "code": "telegram_poller_unhealthy", "message": str(health.get("error"))})
    last_event = status.get("last_event")
    if status["current"] is not None and last_event is None:
        findings.append({"severity": "warning", "code": "active_without_events", "message": "projection has a cursor but no event evidence"})
    if status["stopped"]:
        findings.append({"severity": "info", "code": "run_stopped", "message": str(status.get("stop_reason"))})
    if not findings:
        findings.append({"severity": "info", "code": "no_findings", "message": "no obvious runner health issue found"})
    return findings


def retention_class_for_lane(lane: str) -> str:
    if lane == "events":
        return "authority"
    if lane == "checkpoints":
        return "continuity"
    if lane in {"logs", "instantiations"}:
        return "observation"
    if lane in {"projections", "notifications", "telegram"}:
        return "derived"
    if lane == "locks":
        return "process_control"
    if lane == "archives":
        return "archive"
    return "unknown"


def artifact_record(lane: str, retention_class: str, path: Path) -> RuntimeArtifactRecord:
    return RuntimeArtifactRecord(
        lane=lane,
        retention_class=retention_class,
        path=str(path),
        exists=path.exists(),
        bytes=artifact_size(path),
    )


def artifact_size(path: Path) -> int:
    if not path.exists():
        return 0
    if path.is_file():
        return path.stat().st_size
    return sum(child.stat().st_size for child in path.rglob("*") if child.is_file())


def copy_file(source: Path, target: Path) -> str | None:
    if not source.exists():
        return None
    target.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source, target)
    return str(target.resolve())


def prune_derived_artifacts(paths: RuntimePaths) -> list[str]:
    pruned: list[str] = []
    for candidate in run_scoped_prune_candidates(paths):
        if remove_artifact(candidate):
            pruned.append(str(candidate.resolve()))
    return pruned


def run_scoped_prune_candidates(paths: RuntimePaths) -> tuple[Path, ...]:
    return (
        paths.projection,
        paths.checkpoints,
        paths.instantiations,
        paths.notifications,
        paths.notification_delivery,
        paths.telegram_alerts,
        paths.log,
    )


def remove_artifact(path: Path) -> bool:
    if not path.exists():
        return False
    if path.is_dir():
        shutil.rmtree(path)
    else:
        path.unlink()
    return True


def event_root() -> Path:
    return CANONICAL_RUNTIME_ROOT / "events"


def run_id_from_event_path(path: Path) -> str:
    return path.stem
