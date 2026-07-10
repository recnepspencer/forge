from __future__ import annotations

from pathlib import Path
from typing import Any

from runner.adapters.codex import build_codex_command
from runner.adapters.cursor import (
    build_cursor_command,
    cursor_agent_entry_path,
    cursor_permission_args,
)
from runner.adapters.grok import build_grok_command
from runner.adapters.process_runtime import (
    DEFAULT_IDLE_TIMEOUT_SECONDS,
    DEFAULT_TURN_TIMEOUT_SECONDS,
    TIMEOUT_EXIT_CODE,
    run_agent as run_process_agent,
    runner_timeouts,
    timeout_reason,
    update_capture_from_stream_line,
)


def build_command(state: dict[str, Any], prompt_file: Path | None = None) -> list[str]:
    provider = state["session"].get("provider", "codex")
    if provider == "grok":
        if prompt_file is None:
            raise ValueError("grok provider requires a prompt file")
        return build_grok_command(state, prompt_file)
    if provider == "cursor":
        return build_cursor_command(state)
    return build_codex_command(state)


def run_agent(
    state: dict[str, Any],
    prompt: str,
    log_path: Path | None,
    stop_requested_fn=None,
    progress_watchdog_fn=None,
) -> tuple[int, dict[str, Any]]:
    return run_process_agent(
        state,
        build_command=build_command,
        prompt=prompt,
        log_path=log_path,
        stop_requested_fn=stop_requested_fn,
        progress_watchdog_fn=progress_watchdog_fn,
    )


__all__ = [
    "DEFAULT_IDLE_TIMEOUT_SECONDS",
    "DEFAULT_TURN_TIMEOUT_SECONDS",
    "TIMEOUT_EXIT_CODE",
    "build_codex_command",
    "build_command",
    "build_cursor_command",
    "build_grok_command",
    "cursor_agent_entry_path",
    "cursor_permission_args",
    "run_agent",
    "runner_timeouts",
    "timeout_reason",
    "update_capture_from_stream_line",
]
