from __future__ import annotations

import argparse

from runner.facade.commands.import_legacy import run_import_legacy_command
from runner.facade.commands.import_legacy_runtime import run_import_legacy_runtime_command
from runner.facade.commands.inject import run_inject_command
from runner.facade.commands.resume import run_resume_command
from runner.facade.commands.start import run_start_command
from runner.facade.commands.status import run_status_command
from runner.facade.commands.stop import run_stop_command
from runner.facade.commands.validate import run_validate_command


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Canonical automation runner")
    subcommands = parser.add_subparsers(dest="command", required=True)

    validate = subcommands.add_parser("validate")
    validate.add_argument("config")

    start = subcommands.add_parser("start")
    start.add_argument("config")
    start.add_argument("--run-id")
    start.add_argument("--loop", action="store_true")
    start.add_argument("--sleep-seconds", type=int, default=30)
    start.add_argument("--log")

    resume = subcommands.add_parser("resume")
    resume.add_argument("run_id")
    resume.add_argument("--loop", action="store_true")
    resume.add_argument("--sleep-seconds", type=int, default=30)
    resume.add_argument("--log")

    status = subcommands.add_parser("status")
    status.add_argument("run_id")

    stop = subcommands.add_parser("stop")
    stop.add_argument("run_id")
    stop.add_argument("--reason", default="operator stop")

    inject = subcommands.add_parser("inject")
    inject.add_argument("run_id")
    inject.add_argument("--message", required=True)
    inject.add_argument("--phase", type=int)
    inject.add_argument("--turn")

    legacy = subcommands.add_parser("import-legacy")
    legacy.add_argument("old_state")
    legacy.add_argument("config")
    legacy.add_argument("--run-id")

    legacy_runtime = subcommands.add_parser("import-legacy-runtime")
    legacy_runtime.add_argument("run_id")
    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    if args.command == "validate":
        return run_validate_command(args)
    if args.command == "start":
        return run_start_command(args)
    if args.command == "resume":
        return run_resume_command(args)
    if args.command == "status":
        return run_status_command(args)
    if args.command == "stop":
        return run_stop_command(args)
    if args.command == "inject":
        return run_inject_command(args)
    if args.command == "import-legacy":
        return run_import_legacy_command(args)
    if args.command == "import-legacy-runtime":
        return run_import_legacy_runtime_command(args)
    raise ValueError(f"unsupported command {args.command!r}")
