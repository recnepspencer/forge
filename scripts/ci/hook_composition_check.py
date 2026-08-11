#!/usr/bin/env python3
"""Edit-time composition feedback for agent harnesses.

Reads a PostToolUse payload on stdin, runs the composition guards against the
file that was just written, and emits the result as `additionalContext` so the
model sees it immediately instead of at review time.

Deliberately advisory: it never blocks an edit. The gates are CI, the
pre-commit hook, and `cargo test`. This is the fast path, and its only job is
to stop a six-parameter privileged function or a 500-line file from surviving
until someone reads the diff.

Pure Python on purpose — no `jq`, no shell quoting, no `python3`-vs-`python`
probing. The interpreter running this file is by definition present, and the
guard logic is imported rather than shelled out to.

Wired from `.claude/settings.json`. Any harness that can pipe the payload to a
command can use it identically.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts" / "ci"))


def main() -> int:
    try:
        payload = json.load(sys.stdin)
    except (json.JSONDecodeError, ValueError):
        return 0

    tool_input = payload.get("tool_input") or {}
    tool_response = payload.get("tool_response") or {}
    raw = tool_input.get("file_path") or tool_response.get("filePath") or ""
    if not raw or not raw.endswith(".rs"):
        return 0

    path = Path(raw)
    if not path.is_absolute():
        path = ROOT / path
    if not path.is_file():
        return 0
    try:
        path.relative_to(ROOT)
    except ValueError:
        return 0  # outside this repository

    try:
        from check_composition_advisories import Findings, analyse, load_allowlist
    except ImportError:
        return 0

    findings = Findings()
    analyse(path, findings, load_allowlist())

    items = findings.hard + sorted(findings.advisory, key=lambda i: -i["actual"])
    if not items:
        return 0

    lines = [
        ("FAIL: " if item in findings.hard else "ADVISORY: ") + item["message"]
        for item in items
    ]
    context = (
        f"Composition guards on the file you just edited "
        f"({len(items)} finding(s)):\n\n"
        + "\n".join(lines)
        + "\n\n400-line files are a hard cap. 60-line functions and 5+ parameters are "
        "advisory — a long parameter list on a privileged function is usually a set "
        "of values the runtime could derive instead of accept. Address them now or "
        "state why they stand."
    )

    json.dump(
        {
            "hookSpecificOutput": {
                "hookEventName": "PostToolUse",
                "additionalContext": context,
            }
        },
        sys.stdout,
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception:
        # Never fail an edit because the guard had a bad day.
        sys.exit(0)
