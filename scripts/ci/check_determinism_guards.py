#!/usr/bin/env python3
"""
Determinism guardrails for live workspace crates.

This guard used to enforce worth-topo-specific policies. After worth-topo
removal, it skips cleanly when that legacy crate is absent instead of failing
the workspace on a dead path.
"""

from __future__ import annotations

import pathlib
import re
import sys


REPO_ROOT = pathlib.Path(__file__).resolve().parents[2]
TOPO_SRC = REPO_ROOT / "crates" / "worth-topo" / "src"

RUST_FILES = list(TOPO_SRC.rglob("*.rs")) if TOPO_SRC.exists() else []

IGNORE_SUBSTRINGS = [
    "/tests/",
    "/src/tests.rs",
    "/target/",
]

OBSERVABLE_PATH_HINTS = [
    "/crates/worth-topo/src/provenance/",
    "/crates/worth-topo/src/persistent_naming/",
    "/crates/worth-topo/src/transactions/",
    "/crates/worth-topo/src/semantic_attributes/",
]

BAN_PRINT_PATTERNS = [
    re.compile(r"\bprintln!\s*\("),
    re.compile(r"\bdbg!\s*\("),
]

BAN_HASH_PATTERNS = [
    re.compile(r"\bHashMap\b"),
    re.compile(r"\bHashSet\b"),
]

BAN_RADIAL_SET_PATTERN = re.compile(r"\.set_radial_next\s*\(")
BAN_GLOBAL_INVALIDATE_PATTERN = re.compile(r"\bTopoCacheEffect::GlobalInvalidate\b")
BAN_RAW_EVENT_ID_PATTERN = re.compile(r":\s*(u64|usize)\b")

RADIAL_SET_ALLOWED_SUFFIXES = [
    "/crates/worth-topo/src/b_rep/data/mesh/half_edge.rs",
    "/crates/worth-topo/src/b_rep/logic/topo_ops/radial_ring.rs",
    "/crates/worth-topo/src/b_rep/data/storage/cache_runtime.rs",
]

GLOBAL_INVALIDATE_ALLOWED_SUFFIXES = [
    "/crates/worth-topo/src/b_rep/data/storage/cache_runtime.rs",
]


def is_ignored(path: pathlib.Path) -> bool:
    s = str(path)
    return any(token in s for token in IGNORE_SUBSTRINGS)


def in_observable_path(path: pathlib.Path) -> bool:
    s = str(path)
    return any(token in s for token in OBSERVABLE_PATH_HINTS)

def radial_set_allowed(path: pathlib.Path) -> bool:
    s = str(path)
    if any(s.endswith(suffix) for suffix in RADIAL_SET_ALLOWED_SUFFIXES):
        return True
    return "/tests/" in s or s.endswith("/src/tests.rs")


def global_invalidate_allowed(path: pathlib.Path) -> bool:
    s = str(path)
    if any(s.endswith(suffix) for suffix in GLOBAL_INVALIDATE_ALLOWED_SUFFIXES):
        return True
    return "/tests/" in s or s.endswith("/src/tests.rs")


def rel(path: pathlib.Path) -> str:
    return str(path.relative_to(REPO_ROOT))


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
            line = raw_line
            stripped = line.strip()

            if "#[cfg(test)]" in stripped:
                pending_test_attr = True

            if pending_test_attr and re.search(r"\bmod\s+tests\b", stripped):
                in_test_module = True
                pending_test_attr = False
                test_depth = line.count("{") - line.count("}")
                continue

            if in_test_module:
                test_depth += line.count("{") - line.count("}")
                if test_depth <= 0:
                    in_test_module = False
                continue

            # Ignore trailing comments to avoid false positives from docs/comments.
            code_only = line.split("//", 1)[0]

            for pattern in BAN_PRINT_PATTERNS:
                if pattern.search(code_only):
                    failures.append(
                        f"{rel(path)}:{line_no}: banned debug print in production path"
                    )
                    break

            if in_observable_path(path):
                for pattern in BAN_HASH_PATTERNS:
                    if pattern.search(code_only):
                        failures.append(
                            f"{rel(path)}:{line_no}: nondeterministic container token `{pattern.pattern}` in observable path"
                        )
                        break

            if BAN_RADIAL_SET_PATTERN.search(code_only) and not radial_set_allowed(path):
                failures.append(
                    f"{rel(path)}:{line_no}: direct .set_radial_next(...) is banned; use arena.set_half_edge_radial_next(...)"
                )

            if BAN_GLOBAL_INVALIDATE_PATTERN.search(code_only) and not global_invalidate_allowed(path):
                failures.append(
                    f"{rel(path)}:{line_no}: TopoCacheEffect::GlobalInvalidate is banned outside sanctioned cache runtime paths"
                )

            # Topo operation lifecycle events must use typed IDs.
            if str(path).endswith("/crates/worth-topo/src/transactions/data/operation_event.rs"):
                if BAN_RAW_EVENT_ID_PATTERN.search(code_only):
                    failures.append(
                        f"{rel(path)}:{line_no}: raw numeric ID type in TopoOperationEvent payload is banned; use typed IDs"
                    )

    if not TOPO_SRC.exists():
        print("Determinism guards skipped: worth-topo is not part of this workspace")
        return 0

    if failures:
        print("Determinism guards FAILED", file=sys.stderr)
        for msg in failures:
            print(f"  - {msg}", file=sys.stderr)
        return 1

    print("Determinism guards passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
