from __future__ import annotations

import json

from runner.facade.runtime_inspection import run_report


def run_report_command(args) -> int:
    report = run_report(args.run_id)
    if args.json:
        print(json.dumps(report, indent=2))
    else:
        print(human_report(report))
    return 0


def human_report(report: dict) -> str:
    current = report.get("current") or {}
    lines = [
        f"Run {report['run_id']}: {report['state']}",
        f"Events: {report['event_count']}",
        f"Current: phase {current.get('phase')} {current.get('turn')}" if current else "Current: none",
        f"Latest: {report.get('latest_summary') or '(none)'}",
        f"Next: {report['next_operator_action']}",
    ]
    return "\n".join(lines)
