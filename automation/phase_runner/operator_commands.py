from __future__ import annotations

import argparse
import json
from pathlib import Path

from config_schema import load_config, validate_config
from orchestrator import (
    config_path_for_run,
    drive_run,
    import_legacy_run,
    refresh_projection,
    resume_run,
    start_run,
    stop_run,
)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Durable phase runner")
    subcommands = parser.add_subparsers(dest="command", required=True)

    validate = subcommands.add_parser("validate")
    validate.add_argument("config", type=Path)

    start = subcommands.add_parser("start")
    start.add_argument("config", type=Path)
    start.add_argument("--run-id")
    start.add_argument("--loop", action="store_true")
    start.add_argument("--sleep-seconds", type=int, default=30)
    start.add_argument("--log", type=Path)

    resume = subcommands.add_parser("resume")
    resume.add_argument("run_id")
    resume.add_argument("--loop", action="store_true")
    resume.add_argument("--sleep-seconds", type=int, default=30)
    resume.add_argument("--log", type=Path)
    resume.add_argument("--reset-thread", action="store_true")

    status = subcommands.add_parser("status")
    status.add_argument("run_id")

    stop = subcommands.add_parser("stop")
    stop.add_argument("run_id")
    stop.add_argument("--reason", default="operator stop")

    legacy = subcommands.add_parser("import-legacy")
    legacy.add_argument("old_state", type=Path)
    legacy.add_argument("config", type=Path)
    legacy.add_argument("--run-id")
    return parser


def dispatch(args: argparse.Namespace) -> int:
    if args.command == "validate":
        config = load_config(args.config)
        errors = validate_config(config, args.config)
        if report_validation_errors(errors):
            return 2
        print("config is valid")
        return 0

    if args.command == "start":
        config = load_config(args.config)
        if report_validation_errors(validate_config(config, args.config)):
            return 2
        return start_run(args.config, args.run_id, args.loop, args.sleep_seconds, args.log)

    if args.command == "resume":
        return resume_run(args.run_id, args.loop, args.sleep_seconds, args.log, args.reset_thread)

    if args.command == "status":
        projection = refresh_projection(config_path_for_run(args.run_id), args.run_id)
        print(json.dumps(status_view(projection), indent=2))
        return 0

    if args.command == "stop":
        stop_run(args.run_id, args.reason)
        return 0

    if args.command == "import-legacy":
        config = load_config(args.config)
        if report_validation_errors(validate_config(config, args.config)):
            return 2
        run_id = import_legacy_run(args.old_state, args.config, args.run_id)
        print(run_id)
        return 0

    raise ValueError(f"unsupported command {args.command!r}")


def report_validation_errors(errors: list[str]) -> bool:
    if not errors:
        return False
    for error in errors:
        print(f"validation error: {error}")
    return True


def status_view(projection: dict) -> dict:
    return {
        "run_id": projection["run_id"],
        "current": projection["current"],
        "completed_at": projection["completed_at"],
        "stopped": projection["stopped"],
        "stop_reason": projection["stop_reason"],
        "thread_id": projection["session"]["thread_id"],
        "latest_summary": projection["latest_summary"],
        "last_event": projection["last_event"],
        "phases": [
            {
                "id": phase["id"],
                "title": phase["title"],
                "status": phase["status"],
                "qa_status": phase["qa_status"],
            }
            for phase in projection["phases"]
        ],
    }
