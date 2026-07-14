from __future__ import annotations

from pathlib import Path

from runner.facade.lifecycle import resume_run


def run_resume_command(args) -> int:
    log_path = Path(args.log) if args.log else None
    return resume_run(
        args.run_id,
        args.loop,
        args.sleep_seconds,
        log_path,
    )
