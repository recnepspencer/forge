#!/usr/bin/env python3
from __future__ import annotations

import argparse
import traceback
import sys
import time
from pathlib import Path
from typing import Any

from codex_cli import run_codex
from prompts import render_prompt
from state import (
    append_history,
    backup_path,
    cursor_reason,
    load_state,
    save_state,
)
from validation import validate_state


class RunnerFailure(Exception):
    pass


def run_once(
    state_path: Path,
    dry_run: bool,
    log_path: Path | None,
    forced_phase: str | None,
    forced_turn: str | None,
) -> bool:
    state = load_state(state_path)
    validate_or_raise(state, state_path)

    if forced_phase is not None or forced_turn is not None:
        if not isinstance(state.get("current"), dict):
            state["current"] = {}
    if forced_phase is not None:
        state["current"]["phase"] = int(forced_phase)
    if forced_turn is not None:
        state["current"]["turn"] = forced_turn

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

    if should_stop_before_phase(state):
        stop_reason = stop_before_phase_reason(state)
        if dry_run:
            print(stop_reason)
            return False
        fresh = load_state(state_path)
        append_history(fresh, "runner_stop_before_phase", stop_reason)
        save_state(state_path, fresh)
        print(stop_reason)
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
        raise RunnerFailure(f"codex exited with {exit_code}")

    return True


def should_stop_before_phase(state: dict[str, Any]) -> bool:
    current = state.get("current")
    if not isinstance(current, dict):
        return False
    stop_before_phase = state.get("runner_control", {}).get("stop_before_phase")
    if stop_before_phase is None:
        return False
    try:
        return int(current.get("phase")) >= int(stop_before_phase)
    except (TypeError, ValueError):
        return False


def stop_before_phase_reason(state: dict[str, Any]) -> str:
    current = state.get("current", {})
    stop_before_phase = state.get("runner_control", {}).get("stop_before_phase")
    label = state.get("runner_control", {}).get("stop_reason")
    if not label:
        label = "configured stop-before-phase gate"
    return (
        f"{label}: current phase {current.get('phase')} "
        f"{current.get('turn')} reached stop_before_phase {stop_before_phase}"
    )


def run_once_with_recovery(
    state_path: Path,
    dry_run: bool,
    log_path: Path | None,
    forced_phase: str | None,
    forced_turn: str | None,
    recover: bool,
) -> bool:
    try:
        return run_once(state_path, dry_run, log_path, forced_phase, forced_turn)
    except Exception as error:
        if not recover or dry_run:
            raise
        details = traceback.format_exc()
        print(details, file=sys.stderr)
        return request_recovery(
            state_path,
            log_path,
            f"{type(error).__name__}: {error}",
            details,
        )


def request_recovery(
    state_path: Path,
    log_path: Path | None,
    reason: str,
    details: str,
) -> bool:
    state = load_state_for_recovery(state_path)
    recovery_reason = f"{reason}: {first_line(details)}"
    append_history(state, "runner_recovery_requested", recovery_reason)
    save_state(state_path, state)

    prompt = recovery_prompt(state, state_path, reason, details)
    exit_code, capture = run_codex(state, prompt, log_path)
    merge_after_codex(state_path, "runner recovery", exit_code, capture)
    if exit_code != 0:
        print(f"codex recovery exited with {exit_code}", file=sys.stderr)
        return False
    return True


def load_state_for_recovery(state_path: Path) -> dict[str, Any]:
    try:
        return load_state(state_path)
    except Exception:
        backup = backup_path(state_path)
        if not backup.exists():
            raise
        state = load_state(backup)
        state["_recovery_source_path"] = str(backup)
        return state


def first_line(text: str) -> str:
    for line in text.splitlines():
        stripped = line.strip()
        if stripped:
            return stripped[:240]
    return "no details"


def recovery_prompt(
    state: dict[str, Any],
    state_path: Path,
    reason: str,
    details: str,
) -> str:
    current = state.get("current")
    return f"""The automated phase runner failed before it could send the next normal turn.

This is a recovery turn in the same persistent Codex session. Fix the cause, then
leave the JSON state valid so the runner can continue automatically.

State file: {state_path.resolve()}
Recovery state source: {state.get("_recovery_source_path", str(state_path.resolve()))}
Current cursor before recovery: {current}
Failure reason: {reason}

Failure details:
```text
{details[-6000:]}
```

Recovery rules:
- Read the state file fresh immediately before writing it.
- Preserve the current cursor unless the state already contains completed work
  that objectively requires a cursor correction.
- Preserve all existing phase notes, history, session, project, and template
  fields except the malformed field or directly related recovery history.
- If a notes field is supposed to be one of the standard note buckets, make it a
  JSON array: plan, done, remaining, findings, verification.
- Do not mark QA passed during recovery. Recovery only makes the runner able to
  continue; normal review/repair/close turns do the certification work.
- After fixing, run:
  python automation\\phase_runner\\runner.py {state_path} --validate
  and record the result in history.
"""


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


def validate_or_raise(state: dict, state_path: Path) -> None:
    errors = validate_state(state, state_path)
    if errors:
        raise RunnerFailure("\n".join(f"validation error: {error}" for error in errors))


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
    parser.add_argument(
        "--no-recover",
        action="store_true",
        help="stop on runner validation/render failures instead of asking Codex to repair state",
    )
    parser.add_argument("--phase", type=int, help="render or run a specific phase id")
    parser.add_argument(
        "--turn",
        help=(
            "render or run a specific turn; must exist in the state's "
            "turn_templates map"
        ),
    )
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
        should_continue = run_once_with_recovery(
            args.state_file,
            args.dry_run,
            args.log,
            args.phase,
            args.turn,
            not args.no_recover,
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
