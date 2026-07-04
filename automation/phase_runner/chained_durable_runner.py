#!/usr/bin/env python3
from __future__ import annotations

import argparse
import sys
import time
from pathlib import Path

from config_schema import load_config, validate_config
from orchestrator import config_path_for_run, refresh_projection, start_run
from runtime_paths import RuntimePaths


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Wait for one durable run to complete, then start the next durable run."
    )
    parser.add_argument("current_run_id")
    parser.add_argument("next_config", type=Path)
    parser.add_argument("next_run_id")
    parser.add_argument("--poll-seconds", type=int, default=60)
    parser.add_argument("--sleep-seconds", type=int, default=30)
    parser.add_argument("--log", type=Path)
    parser.add_argument("--loop", action="store_true")
    return parser


def main() -> int:
    if hasattr(sys.stdout, "reconfigure"):
        sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    if hasattr(sys.stderr, "reconfigure"):
        sys.stderr.reconfigure(encoding="utf-8", errors="replace")
    args = build_parser().parse_args()
    next_config = args.next_config.resolve()
    config = load_config(next_config)
    errors = validate_config(config, next_config)
    if errors:
        for error in errors:
            print(f"validation error: {error}", flush=True)
        return 2

    print(
        f"handoff armed: waiting for {args.current_run_id} before starting {args.next_run_id}",
        flush=True,
    )
    while True:
        if next_run_exists(args.next_run_id):
            print(f"next run already exists: {args.next_run_id}", flush=True)
            return 0

        projection = refresh_projection(config_path_for_run(args.current_run_id), args.current_run_id)
        if projection["stopped"]:
            print(
                f"handoff aborted: {args.current_run_id} stopped ({projection['stop_reason']})",
                flush=True,
            )
            return 1
        if projection["completed_at"] is not None:
            print(
                f"starting next run: {args.next_run_id} from {next_config}",
                flush=True,
            )
            return start_run(
                next_config,
                args.next_run_id,
                args.loop,
                args.sleep_seconds,
                args.log,
            )
        print(
            f"waiting: {args.current_run_id} is at {projection['current']}",
            flush=True,
        )
        time.sleep(args.poll_seconds)


def next_run_exists(run_id: str) -> bool:
    return RuntimePaths(run_id).events.exists()


if __name__ == "__main__":
    raise SystemExit(main())
