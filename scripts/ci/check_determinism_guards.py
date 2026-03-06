#!/usr/bin/env python3
"""
Determinism guardrails for forge-topo.

Checks:
1) Ban `println!` / `dbg!` in non-test Rust sources.
2) Ban `HashMap` / `HashSet` usage in observable determinism-sensitive paths.
"""

from __future__ import annotations

import pathlib
import re
import sys


REPO_ROOT = pathlib.Path(__file__).resolve().parents[2]
TOPO_SRC = REPO_ROOT / "crates" / "forge-topo" / "src"

RUST_FILES = list(TOPO_SRC.rglob("*.rs"))

IGNORE_SUBSTRINGS = [
    "/tests/",
    "/src/tests.rs",
    "/target/",
]

OBSERVABLE_PATH_HINTS = [
    "/crates/forge-topo/src/provenance/",
    "/crates/forge-topo/src/persistent_naming/",
    "/crates/forge-topo/src/transactions/",
    "/crates/forge-topo/src/semantic_attributes/",
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

RADIAL_SET_ALLOWED_SUFFIXES = [
    "/crates/forge-topo/src/b_rep/data/mesh/half_edge.rs",
    "/crates/forge-topo/src/b_rep/logic/topo_ops/radial_ring.rs",
    "/crates/forge-topo/src/b_rep/data/storage/cache_runtime.rs",
]

GLOBAL_INVALIDATE_ALLOWED_SUFFIXES = [
    "/crates/forge-topo/src/b_rep/data/storage/cache_runtime.rs",
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

    if failures:
        print("Determinism guards FAILED", file=sys.stderr)
        for msg in failures:
            print(f"  - {msg}", file=sys.stderr)
        return 1

    print("Determinism guards passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
