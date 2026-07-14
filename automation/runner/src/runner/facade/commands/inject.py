from __future__ import annotations

from pathlib import Path

from runner.facade.lifecycle import inject_operator_override


def run_inject_command(args) -> int:
    inject_operator_override(
        args.run_id,
        args.message,
        phase_id=args.phase,
        turn=args.turn,
    )
    return 0
