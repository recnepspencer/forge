from __future__ import annotations

from pathlib import Path
import sys
from typing import Any


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
    result.extend(cursor_permission_args(session.get("config", {})))
    if thread_id:
        result.extend(["--resume", thread_id])
    return result


def cursor_permission_args(config: dict[str, Any]) -> list[str]:
    args: list[str] = []
    if config.get("approval_policy") == "never":
        args.append("--force")
    if config.get("sandbox_mode") == "danger-full-access":
        args.extend(["--sandbox", "disabled"])
    if config.get("approve_mcps") is True:
        args.append("--approve-mcps")
    return args


def cursor_agent_entry_path() -> Path:
    return (Path(__file__).resolve().parent.parent / "cursor_agent_entry.py").resolve()
