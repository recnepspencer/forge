from __future__ import annotations

from contextlib import nullcontext
from pathlib import Path
import json
import os
import queue
import subprocess
import tempfile
import threading
import time
from typing import Any
from collections.abc import Callable

DEFAULT_TURN_TIMEOUT_SECONDS = 7200
DEFAULT_IDLE_TIMEOUT_SECONDS = 900
PROGRESS_WATCHDOG_POLL_SECONDS = 5
TIMEOUT_EXIT_CODE = 124
WALL_TIMEOUT_FAMILY = "wall_timeout"
IDLE_TIMEOUT_FAMILY = "idle_timeout"


def run_agent(
    state: dict[str, Any],
    build_command,
    prompt: str,
    log_path: Path | None,
    stop_requested_fn=None,
    progress_watchdog_fn: Callable[[], dict[str, str] | None] | None = None,
) -> tuple[int, dict[str, Any]]:
    capture: dict[str, Any] = {"agent_messages": []}
    prompt_file: Path | None = None
    provider = state["session"].get("provider", "codex")
    try:
        if provider == "grok":
            with tempfile.NamedTemporaryFile(
                "w",
                encoding="utf-8",
                errors="replace",
                delete=False,
                suffix=".txt",
            ) as handle:
                handle.write(prompt)
                prompt_file = Path(handle.name)
            stdin = subprocess.DEVNULL
        else:
            stdin = subprocess.PIPE
        process = subprocess.Popen(
            build_command(state, prompt_file),
            cwd=state["project"]["cwd"],
            stdin=stdin,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            encoding="utf-8",
            errors="replace",
            env=build_process_env(state),
        )
        if provider != "grok":
            assert process.stdin is not None
            process.stdin.write(prompt)
            process.stdin.close()
        return collect_process_output(
            process,
            capture,
            log_path,
            state,
            stop_requested_fn,
            progress_watchdog_fn,
        )
    finally:
        if prompt_file is not None:
            try:
                prompt_file.unlink()
            except OSError:
                pass


def collect_process_output(
    process: subprocess.Popen[str],
    capture: dict[str, Any],
    log_path: Path | None,
    state: dict[str, Any],
    stop_requested_fn=None,
    progress_watchdog_fn: Callable[[], dict[str, str] | None] | None = None,
) -> tuple[int, dict[str, Any]]:
    assert process.stdout is not None
    line_queue: queue.Queue[str | None] = queue.Queue()
    reader = threading.Thread(target=stream_stdout, args=(process.stdout, line_queue), daemon=True)
    reader.start()
    timeouts = runner_timeouts(state)
    started_at = time.monotonic()
    last_output_at = started_at
    next_progress_watch_at = started_at
    with open_log_handle(log_path) as log:
        while True:
            if stop_requested_fn and stop_requested_fn():
                process.terminate()
                capture["failure_reason"] = "operator stop requested"
            try:
                line = line_queue.get(timeout=1)
            except queue.Empty:
                line = None
            now = time.monotonic()
            if line is not None:
                last_output_at = now
                if log:
                    log.write(line)
                    log.flush()
                update_capture_from_stream_line(capture, line)
            if process.poll() is not None and line_queue.empty():
                break
            maybe_timeout_process(process, capture, started_at, last_output_at, now, timeouts)
            if progress_watchdog_fn is not None and now >= next_progress_watch_at:
                maybe_abort_for_missing_progress(process, capture, progress_watchdog_fn)
                next_progress_watch_at = now + PROGRESS_WATCHDOG_POLL_SECONDS
        exit_code = process.wait()
    capture.setdefault("thread_id", capture.get("session_id"))
    return exit_code, capture


def stream_stdout(stdout, line_queue: queue.Queue[str | None]) -> None:
    try:
        for line in stdout:
            line_queue.put(line)
    finally:
        line_queue.put(None)


def runner_timeouts(state: dict[str, Any]) -> dict[str, int]:
    runner_control = state.get("runner_control", {})
    return {
        "turn_timeout_seconds": runner_control.get("turn_timeout_seconds", DEFAULT_TURN_TIMEOUT_SECONDS),
        "idle_timeout_seconds": runner_control.get("idle_timeout_seconds", DEFAULT_IDLE_TIMEOUT_SECONDS),
    }


def timeout_reason(elapsed: float, idle_for: float, timeouts: dict[str, int]) -> str:
    if elapsed >= timeouts["turn_timeout_seconds"]:
        return f"agent turn timed out after {timeouts['turn_timeout_seconds']} seconds"
    if idle_for >= timeouts["idle_timeout_seconds"]:
        return f"agent turn produced no output for {timeouts['idle_timeout_seconds']} seconds"
    return "agent turn stopped before a timeout reason was reached"


def maybe_timeout_process(
    process: subprocess.Popen[str],
    capture: dict[str, Any],
    started_at: float,
    last_output_at: float,
    now: float,
    timeouts: dict[str, int],
) -> None:
    turn_timeout = timeouts["turn_timeout_seconds"]
    idle_timeout = timeouts["idle_timeout_seconds"]
    if turn_timeout > 0 and now - started_at > turn_timeout:
        process.terminate()
        capture["failure_reason"] = f"turn timeout after {turn_timeout} seconds"
        capture["failure_family"] = WALL_TIMEOUT_FAMILY
    elif idle_timeout > 0 and now - last_output_at > idle_timeout:
        process.terminate()
        capture["failure_reason"] = f"idle timeout after {idle_timeout} seconds"
        capture["failure_family"] = IDLE_TIMEOUT_FAMILY


def maybe_abort_for_missing_progress(
    process: subprocess.Popen[str],
    capture: dict[str, Any],
    progress_watchdog_fn: Callable[[], dict[str, str] | None],
) -> None:
    if capture.get("failure_reason"):
        return
    progress_fault = progress_watchdog_fn()
    if progress_fault is None:
        return
    process.terminate()
    capture["failure_reason"] = progress_fault["reason"]
    capture["failure_family"] = progress_fault["failure_family"]


def build_process_env(state: dict[str, Any]) -> dict[str, str]:
    env = os.environ.copy()
    session_env = state["session"].get("env", {})
    if isinstance(session_env, dict):
        env.update(session_env)
    return env


def update_capture_from_stream_line(capture: dict[str, Any], line: str) -> None:
    stripped = line.strip()
    if not stripped:
        return
    try:
        payload = json.loads(stripped)
    except json.JSONDecodeError:
        capture.setdefault("agent_messages", []).append(stripped)
        return
    message_type = payload.get("type")
    if message_type in {"session.created", "session.resumed"}:
        capture["session_id"] = payload.get("session_id")
        return
    if message_type == "assistant.message":
        text = extract_message_text(payload)
        if text:
            capture.setdefault("agent_messages", []).append(text)
        return
    if message_type == "item.completed":
        item = payload.get("item")
        if isinstance(item, dict) and item.get("type") == "agent_message":
            text = extract_message_text(item)
            if text:
                capture.setdefault("agent_messages", []).append(text)
        return
    if message_type == "thread_id":
        thread_id = payload.get("thread_id")
        if isinstance(thread_id, str) and thread_id:
            capture["thread_id"] = thread_id
        return
    if payload.get("thread_id"):
        capture["thread_id"] = payload["thread_id"]


def extract_message_text(payload: dict[str, Any]) -> str:
    text = payload.get("text")
    if isinstance(text, str):
        return text
    message = payload.get("message")
    if isinstance(message, str):
        return message
    if isinstance(message, dict):
        content = message.get("content")
        if isinstance(content, str):
            return content
        if isinstance(content, list):
            parts = []
            for item in content:
                if isinstance(item, dict):
                    text = item.get("text")
                    if isinstance(text, str):
                        parts.append(text)
            return "\n".join(parts)
    return ""


def open_log_handle(log_path: Path | None):
    if log_path is None:
        return nullcontext()
    log_path.parent.mkdir(parents=True, exist_ok=True)
    return log_path.open("a", encoding="utf-8")
