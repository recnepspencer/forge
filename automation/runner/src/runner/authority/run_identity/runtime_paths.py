from __future__ import annotations

import ctypes
import json
import os
from contextlib import contextmanager
from dataclasses import dataclass
from pathlib import Path


def runtime_root_from_env(env_var: str, default: str) -> Path:
    override = os.environ.get(env_var)
    if not override:
        return Path(default)
    return Path(override)


CANONICAL_RUNTIME_ROOT = runtime_root_from_env("AUTOMATION_RUNNER_RUNTIME_ROOT", "automation/runner/runtime")
RUNTIME_SUBDIRECTORIES = ("events", "executions", "projections", "checkpoints", "instantiations", "notifications", "telegram", "logs", "locks", "archives")
AUTHORITY_RUNTIME_SUBDIRECTORIES = ("events", "executions")
DERIVED_RUNTIME_SUBDIRECTORIES = ("projections", "instantiations", "notifications", "telegram", "logs")
CONTINUITY_RUNTIME_SUBDIRECTORIES = ("checkpoints",)
PROCESS_CONTROL_RUNTIME_SUBDIRECTORIES = ("locks",)


@dataclass(frozen=True)
class RuntimePaths:
    run_id: str

    @property
    def runtime_root(self) -> Path:
        return resolve_runtime_root(self.run_id)

    @property
    def events(self) -> Path:
        return self.runtime_root / "events" / f"{self.run_id}.jsonl"

    @property
    def projection(self) -> Path:
        return self.runtime_root / "projections" / f"{self.run_id}.json"

    @property
    def checkpoints(self) -> Path:
        return self.runtime_root / "checkpoints" / self.run_id

    @property
    def instantiations(self) -> Path:
        return self.runtime_root / "instantiations" / self.run_id

    @property
    def executions(self) -> Path:
        return self.runtime_root / "executions" / self.run_id

    @property
    def notifications(self) -> Path:
        return self.runtime_root / "notifications" / f"{self.run_id}.jsonl"

    @property
    def notification_delivery(self) -> Path:
        return self.runtime_root / "notifications" / f"{self.run_id}.delivery.jsonl"

    @property
    def telegram_alerts(self) -> Path:
        return self.runtime_root / "telegram" / f"{self.run_id}.jsonl"

    @property
    def telegram_receipts(self) -> Path:
        return self.runtime_root / "telegram" / "inbound-receipts.jsonl"

    @property
    def log(self) -> Path:
        return self.runtime_root / "logs" / f"{self.run_id}.jsonl"

    @property
    def archive(self) -> Path:
        return self.runtime_root / "archives" / self.run_id

    @property
    def locks_dir(self) -> Path:
        return self.runtime_root / "locks"

    @property
    def active_lock(self) -> Path:
        return self.locks_dir / f"{self.run_id}.active.lock"

    @property
    def event_lock(self) -> Path:
        return self.locks_dir / f"{self.run_id}.events.lock"

    @property
    def stop_request(self) -> Path:
        return self.locks_dir / f"{self.run_id}.stop"


def resolve_runtime_root(run_id: str) -> Path:
    return CANONICAL_RUNTIME_ROOT


def ensure_runtime_dirs() -> None:
    for relative in RUNTIME_SUBDIRECTORIES:
        (CANONICAL_RUNTIME_ROOT / relative).mkdir(parents=True, exist_ok=True)


def runtime_lane_descriptions(run_id: str) -> tuple[dict[str, str], ...]:
    paths = RuntimePaths(run_id)
    return (
        runtime_lane_description("events", "authoritative_event_ledger", paths.events),
        runtime_lane_description("executions", "authoritative_provider_execution_receipts", paths.executions),
        runtime_lane_description("projections", "derived_operator_projection", paths.projection),
        runtime_lane_description("checkpoints", "execution_continuity_only", paths.checkpoints),
        runtime_lane_description("instantiations", "derived_prompt_instantiation", paths.instantiations),
        runtime_lane_description("notifications", "derived_operator_signal_log", paths.notifications),
        runtime_lane_description("telegram", "derived_operator_reply_routing", paths.telegram_alerts),
        runtime_lane_description("logs", "observational_provider_capture", paths.log),
        runtime_lane_description("locks", "process_control_only", paths.locks_dir),
        runtime_lane_description("archives", "retained_run_archive_bundle", paths.archive),
    )


def runtime_lane_description(lane: str, meaning: str, path: Path) -> dict[str, str]:
    return {
        "lane": lane,
        "meaning": meaning,
        "path": str(path.resolve()),
    }


@contextmanager
def acquire_active_run_lock(paths: RuntimePaths):
    with acquire_lock(paths.active_lock, f"run {paths.run_id} is already active"):
        yield


@contextmanager
def acquire_event_append_lock(paths: RuntimePaths):
    with acquire_lock(paths.event_lock, f"run {paths.run_id} event log is locked"):
        yield


@contextmanager
def acquire_lock(path: Path, message: str):
    ensure_runtime_dirs()
    path.parent.mkdir(parents=True, exist_ok=True)
    while True:
        try:
            descriptor = os.open(path, os.O_CREAT | os.O_EXCL | os.O_WRONLY)
            break
        except FileExistsError:
            if clear_stale_lock(path):
                continue
            raise RuntimeError(message)
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8") as handle:
            json.dump({"pid": os.getpid()}, handle)
        yield
    finally:
        try:
            path.unlink()
        except FileNotFoundError:
            pass


def clear_stale_lock(path: Path) -> bool:
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except (FileNotFoundError, json.JSONDecodeError):
        try:
            path.unlink()
        except FileNotFoundError:
            pass
        return True
    pid = payload.get("pid")
    if not isinstance(pid, int) or pid <= 0 or pid_is_running(pid):
        return False
    try:
        path.unlink()
    except FileNotFoundError:
        return False
    return True


def pid_is_running(pid: int) -> bool:
    process = ctypes.windll.kernel32.OpenProcess(0x1000, False, pid)
    if process == 0:
        return False
    ctypes.windll.kernel32.CloseHandle(process)
    return True


def mark_stop_requested(paths: RuntimePaths) -> None:
    ensure_runtime_dirs()
    paths.stop_request.parent.mkdir(parents=True, exist_ok=True)
    paths.stop_request.write_text("stop\n", encoding="utf-8")


def clear_stop_requested(paths: RuntimePaths) -> None:
    try:
        paths.stop_request.unlink()
    except FileNotFoundError:
        pass


def stop_requested(paths: RuntimePaths) -> bool:
    return paths.stop_request.exists()
