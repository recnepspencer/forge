from __future__ import annotations

import json
import subprocess
from pathlib import Path
from typing import Any

from state import now_iso


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
) -> tuple[int, dict[str, Any]]:
    capture: dict[str, Any] = {}
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

    assert process.stdout is not None
    for line in process.stdout:
        print(line, end="")
        if log_path:
            with log_path.open("a", encoding="utf-8") as log_file:
                log_file.write(line)
        capture_thread_id(capture, line)

    return process.wait(), capture


def capture_thread_id(capture: dict[str, Any], line: str) -> None:
    try:
        event = json.loads(line)
    except json.JSONDecodeError:
        return

    if isinstance(event, dict) and event.get("type") == "thread.started":
        capture["thread_id"] = event["thread_id"]
        capture["thread_started_at"] = now_iso()
