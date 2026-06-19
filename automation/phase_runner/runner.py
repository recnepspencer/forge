#!/usr/bin/env python3
from __future__ import annotations

import argparse
import sys
import time
from pathlib import Path

from codex_cli import run_codex
from prompts import render_prompt
from state import (
    append_history,
    cursor_reason,
    load_state,
    save_state,
)
from validation import validate_state


def run_once(
    state_path: Path,
    dry_run: bool,
    log_path: Path | None,
    forced_phase: str | None,
    forced_turn: str | None,
) -> bool:
    state = load_state(state_path)
    validate_or_exit(state, state_path)

    if forced_phase is not None:
        state.setdefault("current", {})["phase"] = int(forced_phase)
    if forced_turn is not None:
        state.setdefault("current", {})["turn"] = forced_turn

    reason = cursor_reason(state)

    if state.get("current") is None:
        if dry_run:
            print(reason)
            return False
        fresh = load_state(state_path)
        append_history(fresh, "runner_stop", reason)
        save_state(state_path, fresh)
        print(reason)
        return False

    prompt = render_prompt(state, state_path)
    if dry_run:
        print(prompt)
        return True

    append_history(state, "runner_prompt_selected", reason)
    save_state(state_path, state)

    exit_code, capture = run_codex(state, prompt, log_path)
    merge_after_codex(state_path, reason, exit_code, capture)

    if exit_code != 0:
        print(f"codex exited with {exit_code}", file=sys.stderr)
        return False

    return True


def merge_after_codex(
    state_path: Path,
    reason: str,
    exit_code: int,
    capture: dict[str, str],
) -> None:
    fresh = load_state(state_path)
    session = fresh.setdefault("session", {})
    if capture.get("thread_id") and not session.get("thread_id"):
        session["thread_id"] = capture["thread_id"]
        session["thread_started_at"] = capture.get("thread_started_at")
    append_history(fresh, "codex_turn_completed", reason, exit_code)
    save_state(state_path, fresh)


def validate_or_exit(state: dict, state_path: Path) -> None:
    errors = validate_state(state, state_path)
    if errors:
        for error in errors:
            print(f"validation error: {error}", file=sys.stderr)
        raise SystemExit(2)


def validate_command(state_path: Path) -> int:
    state = load_state(state_path)
    errors = validate_state(state, state_path)
    if errors:
        for error in errors:
            print(f"validation error: {error}", file=sys.stderr)
        return 2
    print("state file is valid")
    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Run a reusable JSON-backed phase plan.")
    parser.add_argument("state_file", type=Path)
    parser.add_argument("--loop", action="store_true")
    parser.add_argument("--max-turns", type=int)
    parser.add_argument("--sleep-seconds", type=int, default=10)
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--validate", action="store_true")
    parser.add_argument("--log", type=Path)
    parser.add_argument("--phase", type=int, help="render or run a specific phase id")
    parser.add_argument("--turn", choices=["plan", "implement", "review", "repair", "close"])
    return parser.parse_args()


def main() -> int:
    if hasattr(sys.stdout, "reconfigure"):
        sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    if hasattr(sys.stderr, "reconfigure"):
        sys.stderr.reconfigure(encoding="utf-8", errors="replace")

    args = parse_args()
    if args.validate:
        return validate_command(args.state_file)

    turns = 0
    while True:
        should_continue = run_once(
            args.state_file,
            args.dry_run,
            args.log,
            args.phase,
            args.turn,
        )
        turns += 1
        if not args.loop or not should_continue:
            return 0
        if args.max_turns is not None and turns >= args.max_turns:
            print(f"stopped after max turns: {args.max_turns}")
            return 0
        time.sleep(args.sleep_seconds)


if __name__ == "__main__":
    raise SystemExit(main())
