from __future__ import annotations

import json

from runner.graph_runtime.execution_authority.stability_canary import run_stability_canary


def run_stability_canary_command(args) -> int:
    del args
    report = run_stability_canary()
    print(json.dumps(report, indent=2))
    return 0 if report["healthy"] else 1
