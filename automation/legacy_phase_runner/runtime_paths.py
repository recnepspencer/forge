from __future__ import annotations

import json
import os
from contextlib import contextmanager
from dataclasses import dataclass
from pathlib import Path
import ctypes


RUNTIME_ROOT = Path("automation/legacy_phase_runner/runtime")


@dataclass(frozen=True)
class RuntimePaths:
    run_id: str

    @property
    def events(self) -> Path:
        return RUNTIME_ROOT / "events" / f"{self.run_id}.jsonl"

    @property
    def projection(self) -> Path:
        return RUNTIME_ROOT / "projections" / f"{self.run_id}.json"

    @property
    def log(self) -> Path:
        return RUNTIME_ROOT / "logs" / f"{self.run_id}.jsonl"

    @property
    def locks_dir(self) -> Path:
        return RUNTIME_ROOT / "locks"

    @property
    def active_lock(self) -> Path:
        return self.locks_dir / f"{self.run_id}.active.lock"

    @property
    def event_lock(self) -> Path:
        return self.locks_dir / f"{self.run_id}.events.lock"

    @property
    def stop_request(self) -> Path:
        return self.locks_dir / f"{self.run_id}.stop"


def ensure_runtime_dirs() -> None:
    for relative in ("events", "projections", "logs", "locks"):
        (RUNTIME_ROOT / relative).mkdir(parents=True, exist_ok=True)


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
    paths.stop_request.write_text("stop\n", encoding="utf-8")


def clear_stop_requested(paths: RuntimePaths) -> None:
    try:
        paths.stop_request.unlink()
    except FileNotFoundError:
        pass


def stop_requested(paths: RuntimePaths) -> bool:
    return paths.stop_request.exists()
