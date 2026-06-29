#!/usr/bin/env python3
from __future__ import annotations

import argparse
import sys
import time
from pathlib import Path

from runner_runtime import recovery_prompt, run_once_with_recovery, validate_command


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
        should_continue = run_once_with_recovery(
            args.state_file,
            args.dry_run,
            args.log,
            args.phase,
            args.turn,
            not args.no_recover,
            recovery_prompt,
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
