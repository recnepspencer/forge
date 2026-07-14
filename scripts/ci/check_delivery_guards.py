#!/usr/bin/env python3
"""
Delivery guardrails for WORTH.

Purpose:
- Make partial implementations harder to merge.
- Enforce placeholder bans in production paths (with explicit allowlists).
- Optionally validate epic delivery checklists are complete and evidence-backed.

Usage:
  python3 scripts/ci/check_delivery_guards.py
  python3 scripts/ci/check_delivery_guards.py --checklist path/to/CHECKLIST.md
"""

from __future__ import annotations

import argparse
import os
import pathlib
import re
import sys
from dataclasses import dataclass


REPO_ROOT = pathlib.Path(__file__).resolve().parents[2]

RUST_FILE_EXTS = {".rs"}

# Placeholder language that should not appear in production implementation paths.
PLACEHOLDER_PATTERNS = [
    re.compile(r"\btodo!\s*\(", re.IGNORECASE),
    re.compile(r"\bunimplemented!\s*\(", re.IGNORECASE),
    re.compile(r"\bnot implemented\b", re.IGNORECASE),
    re.compile(r"\bplaceholder\b", re.IGNORECASE),
]

# The user explicitly wants curved placeholders for now.
ALLOWLIST_PLACEHOLDER_PATH_SUBSTRINGS = [
    "/curved_merge/",
]

# Ignore obvious test/docs/generated/target paths.
IGNORE_PATH_SUBSTRINGS = [
    "/target/",
    "/.git/",
    "/.venv/",
    "/tests/",
    "/src/tests.rs",
    "/_tests.rs",
    "/benches/",
]

# Optional hygiene checks in production kernel/IO code.
STRINGLY_AUDIT_PATTERN = re.compile(r'format!\s*\(\s*"\{\:\?\}"')
STRINGLY_AUDIT_PATH_HINTS = [
    "/src/audit/",
]


@dataclass
class Finding:
    path: pathlib.Path
    line_no: int
    message: str


def rel(p: pathlib.Path) -> str:
    return str(p.relative_to(REPO_ROOT))


def should_ignore_for_placeholder_scan(path: pathlib.Path) -> bool:
    s = str(path)
    return any(token in s for token in IGNORE_PATH_SUBSTRINGS)


def is_allowlisted_placeholder_path(path: pathlib.Path) -> bool:
    s = str(path)
    return any(token in s for token in ALLOWLIST_PLACEHOLDER_PATH_SUBSTRINGS)


def iter_repo_files() -> list[pathlib.Path]:
    out: list[pathlib.Path] = []
    skip_dirs = {".git", "target", ".venv", ".pytest_cache"}
    for root, dirs, files in os.walk(REPO_ROOT):
        dirs[:] = [d for d in dirs if d not in skip_dirs]
        root_path = pathlib.Path(root)
        for name in files:
            p = root_path / name
            if p.suffix not in RUST_FILE_EXTS:
                continue
            out.append(p)
    return out


def scan_placeholder_bans() -> list[Finding]:
    findings: list[Finding] = []
    for path in iter_repo_files():
        if should_ignore_for_placeholder_scan(path):
            continue
        if is_allowlisted_placeholder_path(path):
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        for i, line in enumerate(text.splitlines(), start=1):
            # Ignore comments in docs/tests? No: placeholders in production comments are often drift markers.
            for pat in PLACEHOLDER_PATTERNS:
                if pat.search(line):
                    findings.append(Finding(path, i, f"placeholder pattern banned in production path: `{pat.pattern}`"))
                    break
    return findings


def scan_stringly_audit_patterns() -> list[Finding]:
    findings: list[Finding] = []
    for path in iter_repo_files():
        s = str(path)
        if not any(token in s for token in STRINGLY_AUDIT_PATH_HINTS):
            continue
        text = path.read_text(encoding="utf-8")
        for i, line in enumerate(text.splitlines(), start=1):
            if STRINGLY_AUDIT_PATTERN.search(line):
                findings.append(Finding(path, i, "stringly debug formatting in audit path; use typed serializable error/provenance fields"))
    return findings


CHECKBOX_RE = re.compile(r"^- \[( |x|X)\] (.+)$")
EVIDENCE_RE = re.compile(r"^\s*Evidence:\s+(.+)$")


def validate_checklist(checklist_path: pathlib.Path) -> list[str]:
    errors: list[str] = []
    if not checklist_path.exists():
        return [f"Checklist not found: {checklist_path}"]

    lines = checklist_path.read_text(encoding="utf-8").splitlines()
    seen_any = False
    pending_checked_item: tuple[int, str] | None = None
    for idx, line in enumerate(lines, start=1):
        m = CHECKBOX_RE.match(line)
        if m:
            seen_any = True
            checked = m.group(1).lower() == "x"
            item_text = m.group(2).strip()
            pending_checked_item = (idx, item_text) if checked else None
            if not checked:
                errors.append(f"{rel(checklist_path)}:{idx}: unchecked checklist item: {item_text}")
            continue
        if pending_checked_item:
            em = EVIDENCE_RE.match(line)
            if em:
                evidence = em.group(1).strip()
                if not evidence or evidence.lower() in {"tbd", "todo"}:
                    li, item = pending_checked_item
                    errors.append(f"{rel(checklist_path)}:{li}: checked item missing concrete evidence: {item}")
                pending_checked_item = None
            elif line.strip() and not line.startswith("  "):
                li, item = pending_checked_item
                errors.append(f"{rel(checklist_path)}:{li}: checked item must include indented `Evidence:` line: {item}")
                pending_checked_item = None
    if pending_checked_item:
        li, item = pending_checked_item
        errors.append(f"{rel(checklist_path)}:{li}: checked item must include trailing indented `Evidence:` line: {item}")
    if not seen_any:
        errors.append(f"{rel(checklist_path)}: no checklist items found (`- [ ]` / `- [x]`)")
    return errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--checklist", type=str, default=None, help="Optional checklist markdown file to validate")
    args = parser.parse_args()

    findings: list[Finding] = []
    findings.extend(scan_placeholder_bans())
    findings.extend(scan_stringly_audit_patterns())

    checklist_errors: list[str] = []
    if args.checklist:
        checklist_errors = validate_checklist((REPO_ROOT / args.checklist).resolve() if not pathlib.Path(args.checklist).is_absolute() else pathlib.Path(args.checklist))

    if findings or checklist_errors:
        print("Delivery guard check FAILED", file=sys.stderr)
        for f in findings:
            print(f"  - {rel(f.path)}:{f.line_no}: {f.message}", file=sys.stderr)
        for e in checklist_errors:
            print(f"  - {e}", file=sys.stderr)
        return 1

    print("Delivery guard check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
