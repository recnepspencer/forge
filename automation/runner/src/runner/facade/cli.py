from __future__ import annotations

import argparse

from runner.facade.commands.active import run_active_command
from runner.facade.commands.archive import run_archive_command
from runner.facade.commands.artifacts import run_artifacts_command
from runner.facade.commands.doctor import run_doctor_command
from runner.facade.commands.import_legacy import run_import_legacy_command
from runner.facade.commands.import_legacy_runtime import run_import_legacy_runtime_command
from runner.facade.commands.inject import run_inject_command
from runner.facade.commands.plan import run_plan_command
from runner.facade.commands.report import run_report_command
from runner.facade.commands.resume import run_resume_command
from runner.facade.commands.start import run_start_command
from runner.facade.commands.status import run_status_command
from runner.facade.commands.stop import run_stop_command
from runner.facade.commands.validate import run_validate_command
from runner.facade.commands.generate import run_generate_command


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Canonical automation runner")
    subcommands = parser.add_subparsers(dest="command", required=True)

    validate = subcommands.add_parser("validate")
    validate.add_argument("config")

    generate = subcommands.add_parser("generate")
    generate.add_argument("kind", choices=("milestone", "single_prompt", "handoff"))
    generate.add_argument("--name", required=True)
    generate.add_argument("--project-root", default=".")
    generate.add_argument("--spec", required=True)
    generate.add_argument("--force", action="store_true")
    generate.add_argument("--telegram", action="store_true")

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

    report = subcommands.add_parser("report")
    report.add_argument("run_id")
    report.add_argument("--json", action="store_true")

    doctor = subcommands.add_parser("doctor")
    doctor.add_argument("run_id")

    artifacts = subcommands.add_parser("artifacts")
    artifacts.add_argument("run_id")

    archive = subcommands.add_parser("archive")
    archive.add_argument("run_id")
    archive.add_argument("--prune-derived", action="store_true")

    subcommands.add_parser("active")

    stop = subcommands.add_parser("stop")
    stop.add_argument("run_id")
    stop.add_argument("--reason", default="operator stop")

    inject = subcommands.add_parser("inject")
    inject.add_argument("run_id")
    inject.add_argument("--message", required=True)
    inject.add_argument("--phase", type=int)
    inject.add_argument("--turn")

    plan = subcommands.add_parser("plan")
    plan_subcommands = plan.add_subparsers(dest="plan_command", required=True)

    plan_diff = plan_subcommands.add_parser("diff")
    plan_diff.add_argument("run_id")
    plan_diff.add_argument("--config", required=True)

    plan_revise = plan_subcommands.add_parser("revise")
    plan_revise.add_argument("run_id")
    plan_revise.add_argument("--config", required=True)
    plan_revise.add_argument("--reason", default="operator plan revision")
    plan_revise.add_argument("--allow-current-restart", action="store_true")

    plan_fork = plan_subcommands.add_parser("fork")
    plan_fork.add_argument("run_id")
    plan_fork.add_argument("--config", required=True)
    plan_fork.add_argument("--new-run-id", required=True)
    plan_fork.add_argument("--reason", default="plan revision fork")

    prompt_override = plan_subcommands.add_parser("override-prompt")
    prompt_override.add_argument("run_id")
    prompt_override.add_argument("--phase-key", required=True)
    prompt_override.add_argument("--turn")
    prompt_override.add_argument("--asset-id")
    prompt_override.add_argument("--assembly-id")
    prompt_override.add_argument("--reason", default="operator prompt override")

    mark_external = plan_subcommands.add_parser("mark-external")
    mark_external.add_argument("run_id")
    mark_external.add_argument("--phase-key", required=True)
    mark_external.add_argument("--agent", required=True)
    mark_external.add_argument("--summary", required=True)
    mark_external.add_argument("--evidence", action="append", default=[])

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
    if args.command == "generate":
        return run_generate_command(args)
    if args.command == "start":
        return run_start_command(args)
    if args.command == "resume":
        return run_resume_command(args)
    if args.command == "status":
        return run_status_command(args)
    if args.command == "report":
        return run_report_command(args)
    if args.command == "doctor":
        return run_doctor_command(args)
    if args.command == "artifacts":
        return run_artifacts_command(args)
    if args.command == "archive":
        return run_archive_command(args)
    if args.command == "active":
        return run_active_command(args)
    if args.command == "stop":
        return run_stop_command(args)
    if args.command == "inject":
        return run_inject_command(args)
    if args.command == "plan":
        return run_plan_command(args)
    if args.command == "import-legacy":
        return run_import_legacy_command(args)
    if args.command == "import-legacy-runtime":
        return run_import_legacy_runtime_command(args)
    raise ValueError(f"unsupported command {args.command!r}")


if __name__ == "__main__":
    raise SystemExit(main())
