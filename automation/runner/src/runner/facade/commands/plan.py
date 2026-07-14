from __future__ import annotations

import json
from pathlib import Path

from runner.facade.plan_revision import (
    diff_plan,
    fork_plan,
    revise_plan,
)
from runner.facade.plan_operator_actions import record_external_completion, record_prompt_override


def run_plan_command(args) -> int:
    if args.plan_command == "diff":
        print(json.dumps(diff_plan(args.run_id, Path(args.config)), indent=2))
        return 0
    if args.plan_command == "revise":
        result = revise_plan(
            args.run_id,
            Path(args.config),
            allow_current_restart=args.allow_current_restart,
            reason=args.reason,
        )
        print(json.dumps(result, indent=2))
        return 0
    if args.plan_command == "fork":
        result = fork_plan(
            args.run_id,
            Path(args.config),
            args.new_run_id,
            args.reason,
            resume_phase_key=args.resume_phase_key,
            resume_turn=args.resume_turn,
        )
        print(json.dumps(result, indent=2))
        return 0
    if args.plan_command == "override-prompt":
        record_prompt_override(
            args.run_id,
            args.phase_key,
            prompt_binding(args),
            turn=args.turn,
            reason=args.reason,
        )
        return 0
    if args.plan_command == "mark-external":
        record_external_completion(args.run_id, args.phase_key, args.agent, args.summary, args.evidence)
        return 0
    raise ValueError(f"unsupported plan command {args.plan_command!r}")


def prompt_binding(args) -> dict[str, str]:
    if args.asset_id and args.assembly_id:
        raise ValueError("pass only one of --asset-id or --assembly-id")
    if args.asset_id:
        return {"asset_id": args.asset_id}
    if args.assembly_id:
        return {"assembly_id": args.assembly_id}
    raise ValueError("pass --asset-id or --assembly-id")
