from __future__ import annotations

from pathlib import Path

from runner.facade.lifecycle import start_run


def run_start_command(args) -> int:
    log_path = Path(args.log) if args.log else None
    return start_run(
        Path(args.config),
        args.run_id,
        args.loop,
        args.sleep_seconds,
        log_path,
    )
