#!/usr/bin/env python3
"""
Subscriber discipline guardrails for signal-unified lifecycle wiring.

Checks:
1) Ban direct replay/hash/version lifecycle writes outside subscriber modules.
2) Enforce typed IDs in feature/topo operation event payload enums.
"""

from __future__ import annotations

import pathlib
import re
import sys


REPO_ROOT = pathlib.Path(__file__).resolve().parents[2]
TOPO_SRC = REPO_ROOT / "crates" / "forge-topo" / "src"
KERNEL_SRC = REPO_ROOT / "crates" / "forge-kernel" / "src"

RUST_FILES = list(TOPO_SRC.rglob("*.rs")) + list(KERNEL_SRC.rglob("*.rs"))

IGNORE_SUBSTRINGS = [
    "/tests/",
    "/src/tests.rs",
    "_tests.rs",
    "/target/",
]

ALLOWED_DISCIPLINE_PATHS = [
    "/crates/forge-topo/src/transactions/logic/subscribers/",
]

BANNED_CALL_PATTERNS = [
    re.compile(r"\.log_operation_start\s*\("),
    re.compile(r"\.finalize_last\s*\("),
    re.compile(r"\.set_last_cache_refresh_trace\s*\("),
    re.compile(r"\.bump_topology_version\s*\("),
    re.compile(r"\.bump_geometry_version\s*\("),
    re.compile(r"\.set_topology_hash\s*\("),
]

RAW_NUMERIC_ID_PATTERN = re.compile(r"\b[a-zA-Z_]*id\s*:\s*(u64|usize)\b")


def rel(path: pathlib.Path) -> str:
    return str(path.relative_to(REPO_ROOT))


def is_ignored(path: pathlib.Path) -> bool:
    s = str(path)
    if any(token in s for token in IGNORE_SUBSTRINGS):
        return True
    return s.endswith("/tests.rs")


def is_allowed_discipline_path(path: pathlib.Path) -> bool:
    s = str(path)
    return any(token in s for token in ALLOWED_DISCIPLINE_PATHS)


def enum_raw_numeric_id_violations(path: pathlib.Path, enum_name: str) -> list[tuple[int, str]]:
    lines = path.read_text(encoding="utf-8").splitlines()
    violations: list[tuple[int, str]] = []

    in_enum = False
    brace_depth = 0
    for line_no, raw_line in enumerate(lines, start=1):
        stripped = raw_line.strip()
        code_only = raw_line.split("//", 1)[0]

        if not in_enum:
            if re.search(rf"\bpub\s+enum\s+{re.escape(enum_name)}\b", stripped):
                in_enum = True
                brace_depth += code_only.count("{") - code_only.count("}")
            continue

        brace_depth += code_only.count("{") - code_only.count("}")
        if RAW_NUMERIC_ID_PATTERN.search(code_only):
            violations.append((line_no, code_only.strip()))
        if brace_depth <= 0:
            in_enum = False

    return violations


def main() -> int:
    failures: list[str] = []

    for path in RUST_FILES:
        if is_ignored(path):
            continue
        lines = path.read_text(encoding="utf-8").splitlines()

        in_test_module = False
        pending_test_attr = False
        test_depth = 0

        for line_no, raw_line in enumerate(lines, start=1):
            stripped = raw_line.strip()
            if "#[cfg(test)]" in stripped:
                pending_test_attr = True

            if pending_test_attr and re.search(r"\bmod\s+tests\b", stripped):
                in_test_module = True
                pending_test_attr = False
                test_depth = raw_line.count("{") - raw_line.count("}")
                continue

            if in_test_module:
                test_depth += raw_line.count("{") - raw_line.count("}")
                if test_depth <= 0:
                    in_test_module = False
                continue

            code_only = raw_line.split("//", 1)[0]
            if not code_only.strip():
                continue

            if not is_allowed_discipline_path(path):
                for pattern in BANNED_CALL_PATTERNS:
                    if pattern.search(code_only):
                        failures.append(
                            f"{rel(path)}:{line_no}: banned lifecycle write `{pattern.pattern}` outside subscriber modules"
                        )
                        break

    topo_event_file = TOPO_SRC / "transactions" / "data" / "operation_event.rs"
    kernel_event_file = KERNEL_SRC / "engine" / "transaction" / "data" / "feature_event.rs"

    for line_no, snippet in enum_raw_numeric_id_violations(topo_event_file, "TopoOperationEvent"):
        failures.append(
            f"{rel(topo_event_file)}:{line_no}: raw numeric ID in TopoOperationEvent payload is banned: `{snippet}`"
        )

    for line_no, snippet in enum_raw_numeric_id_violations(kernel_event_file, "KernelFeatureEvent"):
        failures.append(
            f"{rel(kernel_event_file)}:{line_no}: raw numeric ID in KernelFeatureEvent payload is banned: `{snippet}`"
        )

    if failures:
        print("Subscriber discipline guards FAILED", file=sys.stderr)
        for msg in failures:
            print(f"  - {msg}", file=sys.stderr)
        return 1

    print("Subscriber discipline guards passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
