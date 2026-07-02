from __future__ import annotations

from datetime import datetime, timezone
import json
import queue
import subprocess
import threading
import time
from pathlib import Path
from typing import Any

DEFAULT_TURN_TIMEOUT_SECONDS = 7200
DEFAULT_IDLE_TIMEOUT_SECONDS = 900
TIMEOUT_EXIT_CODE = 124


def build_command(state: dict[str, Any]) -> list[str]:
    session = state["session"]
    command = session.get("command", session.get("codex_command", "codex"))
    model = session["model"]
    effort = session["reasoning_effort"]
    thread_id = session.get("thread_id")
    config_args: list[str] = []

    for key, value in session.get("config", {}).items():
        config_args.extend(["-c", f"{key}={json.dumps(value)}"])

    if thread_id:
        return [
            command,
            "exec",
            "resume",
            "--json",
            "-m",
            model,
            "-c",
            f'model_reasoning_effort="{effort}"',
            *config_args,
            thread_id,
            "-",
        ]

    return [
        command,
        "exec",
        "--json",
        "-m",
        model,
        "-c",
        f'model_reasoning_effort="{effort}"',
        *config_args,
        "-C",
        state["project"]["cwd"],
        "-",
    ]


def run_codex(
    state: dict[str, Any],
    prompt: str,
    log_path: Path | None,
    stop_requested_fn=None,
) -> tuple[int, dict[str, Any]]:
    capture: dict[str, Any] = {"agent_messages": []}
    process = subprocess.Popen(
        build_command(state),
        cwd=state["project"]["cwd"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        encoding="utf-8",
        errors="replace",
    )

    assert process.stdin is not None
    process.stdin.write(prompt)
    process.stdin.close()

    return collect_process_output(process, capture, log_path, state, stop_requested_fn)


def collect_process_output(
    process: subprocess.Popen[str],
    capture: dict[str, Any],
    log_path: Path | None,
    state: dict[str, Any],
    stop_requested_fn=None,
) -> tuple[int, dict[str, Any]]:
    assert process.stdout is not None
    line_queue: queue.Queue[str | None] = queue.Queue()
    reader = threading.Thread(
        target=stream_stdout,
        args=(process.stdout, line_queue),
        daemon=True,
    )
    reader.start()

    timeouts = runner_timeouts(state)
    started_at = time.monotonic()
    last_output_at = started_at

    while True:
        if stop_requested_fn is not None and stop_requested_fn():
            terminate_process(process)
            capture["failure_reason"] = "operator stop requested"
            return TIMEOUT_EXIT_CODE, capture
        timeout_seconds = min(
            timeouts["turn_timeout_seconds"] - (time.monotonic() - started_at),
            timeouts["idle_timeout_seconds"] - (time.monotonic() - last_output_at),
        )
        timeout_seconds = min(timeout_seconds, 1.0)
        if timeout_seconds <= 0:
            break
        try:
            line = line_queue.get(timeout=timeout_seconds)
        except queue.Empty:
            continue
        if line is None:
            return process.wait(), capture
        last_output_at = time.monotonic()
        print(line, end="")
        if log_path:
            with log_path.open("a", encoding="utf-8") as log_file:
                log_file.write(line)
        capture_thread_id(capture, line)

    elapsed = time.monotonic() - started_at
    idle_for = time.monotonic() - last_output_at
    reason = timeout_reason(elapsed, idle_for, timeouts)
    terminate_process(process)
    capture["failure_reason"] = reason
    return TIMEOUT_EXIT_CODE, capture


def capture_thread_id(capture: dict[str, Any], line: str) -> None:
    try:
        event = json.loads(line)
    except json.JSONDecodeError:
        return

    if isinstance(event, dict) and event.get("type") == "thread.started":
        capture["thread_id"] = event["thread_id"]
        capture["thread_started_at"] = now_iso()
        return

    item = event.get("item") if isinstance(event, dict) else None
    if not isinstance(item, dict):
        return
    if item.get("type") == "agent_message":
        text = item.get("text")
        if isinstance(text, str):
            capture.setdefault("agent_messages", []).append(text)


def now_iso() -> str:
    return datetime.now(timezone.utc).isoformat()


def stream_stdout(stdout, line_queue: queue.Queue[str | None]) -> None:
    try:
        for line in stdout:
            line_queue.put(line)
    finally:
        line_queue.put(None)


def runner_timeouts(state: dict[str, Any]) -> dict[str, int]:
    runner_control = state.get("runner_control", {})
    return {
        "turn_timeout_seconds": positive_timeout(
            runner_control.get("turn_timeout_seconds"),
            DEFAULT_TURN_TIMEOUT_SECONDS,
        ),
        "idle_timeout_seconds": positive_timeout(
            runner_control.get("idle_timeout_seconds"),
            DEFAULT_IDLE_TIMEOUT_SECONDS,
        ),
    }


def positive_timeout(value: Any, default: int) -> int:
    return value if isinstance(value, int) and value > 0 else default


def timeout_reason(elapsed: float, idle_for: float, timeouts: dict[str, int]) -> str:
    if elapsed >= timeouts["turn_timeout_seconds"]:
        return f"codex turn timed out after {timeouts['turn_timeout_seconds']} seconds"
    return f"codex turn produced no output for {timeouts['idle_timeout_seconds']} seconds"


def terminate_process(process: subprocess.Popen[str]) -> None:
    try:
        process.kill()
    except OSError:
        pass
    process.wait()
