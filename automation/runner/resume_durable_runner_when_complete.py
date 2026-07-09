#!/usr/bin/env python3
from __future__ import annotations

import argparse
import sys
import time
from pathlib import Path


def _ensure_runner_src_on_path() -> None:
    root = Path(__file__).resolve().parent
    sys.path.insert(0, str(root / "src"))
    current = sys.modules.get("runner")
    if current is not None and not hasattr(current, "__path__"):
        del sys.modules["runner"]


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Wait for one durable run to complete, then resume another durable run."
    )
    parser.add_argument("current_run_id")
    parser.add_argument("next_run_id")
    parser.add_argument("--poll-seconds", type=int, default=60)
    parser.add_argument("--sleep-seconds", type=int, default=30)
    parser.add_argument("--log", type=Path)
    parser.add_argument("--loop", action="store_true")
    return parser


def main() -> int:
    _ensure_runner_src_on_path()
    from runner.authority.run_identity import RuntimePaths
    from runner.facade.completion_handoff import resume_completion_handoff_target
    from runner.facade.lifecycle import resume_run_with_reason
    from runner.facade.runtime_state import config_path_for_run, refresh_projection_for_run

    if hasattr(sys.stdout, "reconfigure"):
        sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    if hasattr(sys.stderr, "reconfigure"):
        sys.stderr.reconfigure(encoding="utf-8", errors="replace")
    args = build_parser().parse_args()

    if not RuntimePaths(args.next_run_id).events.exists():
        print(f"resume target does not exist: {args.next_run_id}", flush=True)
        return 2

    print(
        f"resume handoff armed: waiting for {args.current_run_id} before resuming {args.next_run_id}",
        flush=True,
    )

    while True:
        current = refresh_projection_for_run(args.current_run_id)
        if current["stopped"]:
            print(
                f"resume handoff aborted: {args.current_run_id} stopped ({current['stop_reason']})",
                flush=True,
            )
            return 1
        if current["completed_at"] is not None:
            next_projection = refresh_projection_for_run(args.next_run_id)
            if next_projection["completed_at"] is not None:
                print(f"resume target already complete: {args.next_run_id}", flush=True)
                return 0
            if not next_projection["stopped"]:
                print(f"resume target already active: {args.next_run_id}", flush=True)
                return 0
            print(f"resuming next run: {args.next_run_id}", flush=True)
            return resume_completion_handoff_target(
                {
                    "next_run_id": args.next_run_id,
                    "loop": args.loop,
                    "sleep_seconds": args.sleep_seconds,
                    "log": str(args.log) if args.log else None,
                },
                lambda config_path, run_id: refresh_projection_for_run(run_id),
                config_path_for_run,
                resume_run_with_reason,
                RuntimePaths,
                polling_run_id=args.current_run_id,
            )
        print(f"waiting: {args.current_run_id} is at {current['current']}", flush=True)
        time.sleep(args.poll_seconds)


if __name__ == "__main__":
    raise SystemExit(main())
