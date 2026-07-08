from __future__ import annotations

from datetime import datetime, timezone
import json
import os
import queue
import subprocess
import sys
import threading
import time
import tempfile
from pathlib import Path
from typing import Any

DEFAULT_TURN_TIMEOUT_SECONDS = 7200
DEFAULT_IDLE_TIMEOUT_SECONDS = 900
TIMEOUT_EXIT_CODE = 124


def build_command(state: dict[str, Any], prompt_file: Path | None = None) -> list[str]:
    session = state["session"]
    provider = session.get("provider", "codex")
    if provider == "grok":
        if prompt_file is None:
            raise ValueError("grok provider requires a prompt file")
        return build_grok_command(state, prompt_file)
    if provider == "cursor":
        return build_cursor_command(state)
    return build_codex_command(state)


def build_codex_command(state: dict[str, Any]) -> list[str]:
    session = state["session"]
    command = session.get("command") or session.get("codex_command") or "codex"
    model = session["model"]
    effort = session.get("reasoning_effort", "medium")
    thread_id = session.get("thread_id") if session.get("reuse_session", True) else None
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


def build_cursor_command(state: dict[str, Any]) -> list[str]:
    session = state["session"]
    thread_id = session.get("thread_id") if session.get("reuse_session", True) else None
    command = session.get("command")
    command_args = session.get("command_args", [])
    if command:
        base = [command, *command_args]
    else:
        base = [sys.executable, str(cursor_agent_entry_path())]

    result = [
        *base,
        "--print",
        "--output-format",
        "stream-json",
        "--trust",
        "--workspace",
        state["project"]["cwd"],
        "--model",
        session["model"],
    ]

    for arg in cursor_permission_args(session.get("config", {})):
        result.append(arg)

    if thread_id:
        result.extend(["--resume", thread_id])
    return result


def build_grok_command(state: dict[str, Any], prompt_file: Path) -> list[str]:
    session = state["session"]
    command = session.get("command") or "grok"
    command_args = session.get("command_args", [])
    thread_id = session.get("thread_id") if session.get("reuse_session", True) else None
    config = session.get("config", {})
    result = [
        command,
        *command_args,
        "--no-auto-update",
        "--no-alt-screen",
        "--always-approve",
        "--output-format",
        "streaming-json",
        "--cwd",
        state["project"]["cwd"],
        "--model",
        session["model"],
        "--prompt-file",
        str(prompt_file),
    ]
    if config.get("sandbox_mode") == "danger-full-access":
        result.extend(["--sandbox", "off"])
    effort = session.get("reasoning_effort")
    if isinstance(effort, str) and effort:
        result.extend(["--effort", effort])
    if thread_id:
        result.extend(["--resume", thread_id])
    return result


def cursor_permission_args(config: dict[str, Any]) -> list[str]:
    args: list[str] = []
    approval_policy = config.get("approval_policy")
    if approval_policy == "never":
        args.append("--force")
    sandbox_mode = config.get("sandbox_mode")
    if sandbox_mode == "danger-full-access":
        args.extend(["--sandbox", "disabled"])
    if config.get("approve_mcps") is True:
        args.append("--approve-mcps")
    return args


def cursor_agent_entry_path() -> Path:
    return Path(__file__).with_name("cursor_agent_entry.py")


def run_agent(
    state: dict[str, Any],
    prompt: str,
    log_path: Path | None,
    stop_requested_fn=None,
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

        return collect_process_output(process, capture, log_path, state, stop_requested_fn)
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
        capture_agent_event(capture, line)

    elapsed = time.monotonic() - started_at
    idle_for = time.monotonic() - last_output_at
    reason = timeout_reason(elapsed, idle_for, timeouts)
    terminate_process(process)
    capture["failure_reason"] = reason
    return TIMEOUT_EXIT_CODE, capture


def capture_agent_event(capture: dict[str, Any], line: str) -> None:
    try:
        event = json.loads(line)
    except json.JSONDecodeError:
        return

    if not isinstance(event, dict):
        return
    event_type = event.get("type")
    if event_type == "text":
        text = event.get("data")
        if isinstance(text, str) and text:
            capture["grok_text"] = capture.get("grok_text", "") + text
        return

    if event_type == "end":
        grok_text = capture.get("grok_text")
        if isinstance(grok_text, str) and grok_text:
            capture.setdefault("agent_messages", []).append(grok_text)
        session_id = event.get("sessionId")
        if isinstance(session_id, str) and session_id:
            capture["thread_id"] = session_id
            capture["thread_started_at"] = now_iso()
        return

    if event_type == "error":
        message = event.get("message")
        if isinstance(message, str) and message:
            capture["failure_reason"] = message
        return

    if event_type == "thread.started":
        capture["thread_id"] = event["thread_id"]
        capture["thread_started_at"] = now_iso()
        return

    if event_type == "system" and event.get("subtype") == "init":
        session_id = event.get("session_id")
        if isinstance(session_id, str) and session_id:
            capture["thread_id"] = session_id
            capture["thread_started_at"] = now_iso()
        return

    item = event.get("item")
    if isinstance(item, dict) and item.get("type") == "agent_message":
        text = item.get("text")
        if isinstance(text, str):
            capture.setdefault("agent_messages", []).append(text)
        return

    if event_type == "assistant":
        message = event.get("message")
        if isinstance(message, dict):
            append_cursor_content(capture, message.get("content"))


def append_cursor_content(capture: dict[str, Any], content: Any) -> None:
    if not isinstance(content, list):
        return
    for item in content:
        if not isinstance(item, dict):
            continue
        if item.get("type") != "text":
            continue
        text = item.get("text")
        if isinstance(text, str) and text:
            capture.setdefault("agent_messages", []).append(text)


def build_process_env(state: dict[str, Any]) -> dict[str, str]:
    env = os.environ.copy()
    session_env = state["session"].get("env", {})
    if isinstance(session_env, dict):
        for key, value in session_env.items():
            if isinstance(key, str) and isinstance(value, str):
                env[key] = value
    return env


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
        return f"agent turn timed out after {timeouts['turn_timeout_seconds']} seconds"
    return f"agent turn produced no output for {timeouts['idle_timeout_seconds']} seconds"


def terminate_process(process: subprocess.Popen[str]) -> None:
    try:
        process.kill()
    except OSError:
        pass
    process.wait()
