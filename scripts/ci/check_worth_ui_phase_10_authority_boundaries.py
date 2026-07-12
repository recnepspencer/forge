from __future__ import annotations

import argparse
import json
import re
import sys
from collections import Counter
from dataclasses import dataclass
from pathlib import Path
from typing import Any


DEFAULT_MANIFEST = Path("scripts/ci/worth_ui_phase_10_authority_boundaries.json")


@dataclass(frozen=True)
class Violation:
    rule_id: str
    message: str


def load_manifest(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise ValueError("boundary manifest must be a JSON object")
    if not isinstance(value.get("roots"), list) or not value["roots"]:
        raise ValueError("boundary manifest roots must be a non-empty list")
    if not isinstance(value.get("rules"), list) or not value["rules"]:
        raise ValueError("boundary manifest rules must be a non-empty list")
    return value


def check_manifest(workspace: Path, manifest: dict[str, Any]) -> list[Violation]:
    sources = source_files(workspace, manifest["roots"])
    violations: list[Violation] = []
    seen_ids: set[str] = set()
    for rule in manifest["rules"]:
        rule_id = required_string(rule, "id")
        if rule_id in seen_ids:
            violations.append(Violation(rule_id, "duplicate rule id"))
            continue
        seen_ids.add(rule_id)
        pattern = re.compile(required_string(rule, "pattern"))
        allowed = normalized_allowed(rule_id, rule.get("allowed"), violations)
        actual = match_counts(workspace, sources, pattern)
        if actual != allowed:
            violations.extend(compare_counts(rule_id, allowed, actual))
    return violations


def source_files(workspace: Path, roots: list[Any]) -> list[Path]:
    files: set[Path] = set()
    for raw_root in roots:
        if not isinstance(raw_root, str) or not raw_root:
            raise ValueError("boundary manifest root entries must be non-empty strings")
        root = workspace / raw_root
        if not root.is_dir():
            raise ValueError(f"boundary manifest root does not exist: {raw_root}")
        files.update(root.rglob("*.rs"))
    return sorted(files)


def match_counts(workspace: Path, sources: list[Path], pattern: re.Pattern[str]) -> Counter[str]:
    counts: Counter[str] = Counter()
    for source in sources:
        text = source.read_text(encoding="utf-8")
        count = len(pattern.findall(text))
        if count:
            counts[source.relative_to(workspace).as_posix()] = count
    return counts


def normalized_allowed(
    rule_id: str, raw: Any, violations: list[Violation]
) -> Counter[str]:
    if not isinstance(raw, dict):
        violations.append(Violation(rule_id, "allowed must be an object"))
        return Counter()
    allowed: Counter[str] = Counter()
    for path, count in raw.items():
        if not isinstance(path, str) or not path:
            violations.append(Violation(rule_id, "allowed path must be a non-empty string"))
            continue
        if not isinstance(count, int) or count < 0:
            violations.append(Violation(rule_id, f"invalid allowed count for {path!r}"))
            continue
        if count:
            allowed[Path(path).as_posix()] = count
    return allowed


def compare_counts(rule_id: str, expected: Counter[str], actual: Counter[str]) -> list[Violation]:
    violations: list[Violation] = []
    for path in sorted(set(expected) | set(actual)):
        expected_count = expected[path]
        actual_count = actual[path]
        if expected_count != actual_count:
            violations.append(
                Violation(
                    rule_id,
                    f"{path}: expected {expected_count} occurrence(s), found {actual_count}",
                )
            )
    return violations


def required_string(value: dict[str, Any], key: str) -> str:
    item = value.get(key)
    if not isinstance(item, str) or not item:
        raise ValueError(f"boundary rule {key} must be a non-empty string")
    return item


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Check Worth UI Phase 10 authority edges")
    parser.add_argument("--root", type=Path, default=Path.cwd())
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    workspace = args.root.resolve()
    manifest_path = args.manifest
    if not manifest_path.is_absolute():
        manifest_path = workspace / manifest_path
    try:
        manifest = load_manifest(manifest_path)
        violations = check_manifest(workspace, manifest)
    except (OSError, ValueError, json.JSONDecodeError) as error:
        print(f"[worth-ui-phase-10-boundaries] invalid configuration: {error}", file=sys.stderr)
        return 2
    if violations:
        print("[worth-ui-phase-10-boundaries] dirty-edge freeze violated:", file=sys.stderr)
        for violation in violations:
            print(f"  {violation.rule_id}: {violation.message}", file=sys.stderr)
        return 1
    print(f"[worth-ui-phase-10-boundaries] {len(manifest['rules'])} rules match the exact manifest")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
