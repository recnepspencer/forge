from __future__ import annotations

from typing import Any, Callable


PUBLIC_EXAMPLE_COMMAND = [
    "cargo", "check", "--manifest-path", "workspaces/worth-ui/Cargo.toml",
    "-p", "worth-ui", "--example", "text_platform",
]


def execute_if_required(
    requirement: str,
    execute: Callable[[list[str], str], tuple[Any, int]],
) -> dict[str, object] | None:
    if requirement != "P4-FONT-COLLECTION-01":
        return None
    result, duration_ms = execute(list(PUBLIC_EXAMPLE_COMMAND), "public-example")
    return {
        "command": list(PUBLIC_EXAMPLE_COMMAND),
        "duration_ms": duration_ms,
        "exit_code": result.returncode,
    }
