from __future__ import annotations

from pathlib import Path
from typing import Any


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
    if session.get("goal_mode"):
        # Goal mode appends grok's headless self-verification loop so the turn
        # drives itself to completion before handing back to review.
        result.append("--check")
    if thread_id:
        result.extend(["--resume", thread_id])
    return result
