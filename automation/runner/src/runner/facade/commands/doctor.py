from __future__ import annotations

import json

from runner.facade.runtime_inspection import doctor_report


def run_doctor_command(args) -> int:
    report = doctor_report(args.run_id)
    print(json.dumps(report, indent=2))
    return 0 if report["healthy"] else 1
