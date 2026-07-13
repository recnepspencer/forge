from __future__ import annotations

import json
import os
import signal
import subprocess
import time

from runner.authority.run_identity.runtime_paths import RuntimePaths, pid_is_running

PROCESS_EXIT_GRACE_SECONDS = 3.0


def stop_owned_processes(paths: RuntimePaths) -> None:
    runner_pid = active_runner_pid(paths)
    if runner_pid is None or not pid_is_running(runner_pid):
        return
    provider_pids = active_provider_pids(paths, runner_pid)
    for pid in provider_pids:
        terminate_process_tree(pid)
    wait_for_exit(provider_pids, PROCESS_EXIT_GRACE_SECONDS)
    if runner_pid is not None and runner_pid != os.getpid():
        terminate_process_tree(runner_pid)
        wait_for_exit([runner_pid], PROCESS_EXIT_GRACE_SECONDS)
    remaining = [pid for pid in [*provider_pids, runner_pid] if pid and pid_is_running(pid)]
    if remaining:
        raise RuntimeError(f"runner stop left owned processes alive: {remaining}")


def active_provider_pids(paths: RuntimePaths, runner_pid: int) -> list[int]:
    if not paths.executions.exists():
        return []
    active: list[int] = []
    for receipt_path in paths.executions.glob("*.json"):
        try:
            payload = json.loads(receipt_path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            continue
        pid = payload.get("provider_pid")
        if (
            payload.get("state") == "launched"
            and payload.get("runner_pid") == runner_pid
            and isinstance(pid, int)
            and pid > 0
        ):
            active.append(pid)
    return active


def active_runner_pid(paths: RuntimePaths) -> int | None:
    try:
        payload = json.loads(paths.active_lock.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None
    pid = payload.get("pid")
    return pid if isinstance(pid, int) and pid > 0 else None


def terminate_process_tree(pid: int) -> None:
    if not pid_is_running(pid):
        return
    if os.name == "nt":
        subprocess.run(
            ["taskkill", "/PID", str(pid), "/T", "/F"],
            capture_output=True,
            check=False,
            text=True,
        )
        return
    try:
        os.kill(pid, signal.SIGTERM)
    except ProcessLookupError:
        pass


def wait_for_exit(pids: list[int], timeout_seconds: float) -> None:
    deadline = time.monotonic() + timeout_seconds
    while time.monotonic() < deadline:
        if not any(pid_is_running(pid) for pid in pids):
            return
        time.sleep(0.05)
