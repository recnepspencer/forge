from __future__ import annotations

import json

from runner.facade.runtime_inspection import active_runs


def run_active_command(args) -> int:
    del args
    print(json.dumps(active_runs(), indent=2))
    return 0
