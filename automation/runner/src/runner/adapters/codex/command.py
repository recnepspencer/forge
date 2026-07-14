from __future__ import annotations

import json
from typing import Any


def build_codex_command(state: dict[str, Any]) -> list[str]:
    session = state["session"]
    command = session.get("command") or session.get("codex_command") or "codex"
    model = session["model"]
    effort = session.get("reasoning_effort", "medium")
    thread_id = session.get("thread_id") if session.get("reuse_session", True) else None
    config_args: list[str] = []
    for key, value in session.get("config", {}).items():
        config_args.extend(["-c", f"{key}={json.dumps(value)}"])
    # Goal mode enables codex's experimental `goals` feature so the turn drives
    # itself to completion before handing back to review.
    goal_args = ["--enable", "goals"] if session.get("goal_mode") else []
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
            *goal_args,
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
        *goal_args,
        "-C",
        state["project"]["cwd"],
        "-",
    ]
