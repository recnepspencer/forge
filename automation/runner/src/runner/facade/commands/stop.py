from __future__ import annotations

from runner.facade.lifecycle import stop_run


def run_stop_command(args) -> int:
    stop_run(args.run_id, args.reason)
    return 0
