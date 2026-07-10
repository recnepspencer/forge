#!/usr/bin/env python3
from __future__ import annotations

import argparse
import subprocess
import sys
import time
from pathlib import Path

from state import current_cursor, load_state
from validation import validate_state


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Run one milestone state file to completion, then hand off to the next."
    )
    parser.add_argument("current_state", type=Path)
    parser.add_argument("next_state", type=Path)
    parser.add_argument("--sleep-seconds", type=int, default=30)
    parser.add_argument("--retry-sleep-seconds", type=int, default=900)
    parser.add_argument("--dry-run", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    current_state = args.current_state.resolve()
    next_state = args.next_state.resolve()

    validate_state_file(current_state)
    validate_state_file(next_state)

    if args.dry_run:
        describe_chain(current_state, next_state)
        return 0

    run_until_complete(
        current_state,
        sleep_seconds=args.sleep_seconds,
        retry_sleep_seconds=args.retry_sleep_seconds,
    )

    ensure_fresh_session(next_state)

    run_until_complete(
        next_state,
        sleep_seconds=args.sleep_seconds,
        retry_sleep_seconds=args.retry_sleep_seconds,
    )
    return 0


def describe_chain(current_state: Path, next_state: Path) -> None:
    print(f"current: {current_state.name}")
    print(f"current complete: {is_honestly_complete(current_state)}")
    print(f"next: {next_state.name}")
    state = load_state(next_state)
    session = state.get("session", {})
    has_thread = isinstance(session, dict) and bool(session.get("thread_id"))
    print(f"next has persisted thread: {has_thread}")
    print("chain action: run current to honest completion, clear next session metadata, then run next")


def run_until_complete(
    state_path: Path,
    sleep_seconds: int,
    retry_sleep_seconds: int,
) -> None:
    if is_honestly_complete(state_path):
        print(f"{state_path.name}: already complete")
        return

    while True:
        print(f"{state_path.name}: starting runner loop")
        exit_code = subprocess.run(
            [
                sys.executable,
                str(Path(__file__).with_name("runner.py")),
                str(state_path),
                "--loop",
                "--sleep-seconds",
                str(sleep_seconds),
                "--log",
                str(state_path.with_suffix(".runner.out.log")),
            ],
            cwd=state_path.parent.parent.parent,
        ).returncode
        print(f"{state_path.name}: runner exited with {exit_code}")

        validate_state_file(state_path)
        if is_honestly_complete(state_path):
            print(f"{state_path.name}: completion verified")
            return

        print(
            f"{state_path.name}: not complete yet; sleeping {retry_sleep_seconds}s before retry"
        )
        time.sleep(retry_sleep_seconds)


def is_honestly_complete(state_path: Path) -> bool:
    state = load_state(state_path)
    if current_cursor(state) is not None:
        return False

    phases = state.get("phases", [])
    if not isinstance(phases, list) or not phases:
        return False

    for phase in phases:
        if phase.get("status") != "complete":
            return False
        if phase.get("qa_status") != "passed":
            return False

    return True


def ensure_fresh_session(state_path: Path) -> None:
    state = load_state(state_path)
    session = state.get("session")
    if not isinstance(session, dict):
        raise ValueError(f"{state_path} session is not a mapping")

    if "thread_id" in session or "thread_started_at" in session:
        session.pop("thread_id", None)
        session.pop("thread_started_at", None)
        from state import save_state  # local import to avoid circular write-only use

        save_state(state_path, state)
        print(f"{state_path.name}: cleared prior session thread metadata")


def validate_state_file(state_path: Path) -> None:
    state = load_state(state_path)
    errors = validate_state(state, state_path)
    if errors:
        joined = "\n".join(errors)
        raise ValueError(f"{state_path} failed validation:\n{joined}")


if __name__ == "__main__":
    raise SystemExit(main())
