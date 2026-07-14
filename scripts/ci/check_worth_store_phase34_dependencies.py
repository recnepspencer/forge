#!/usr/bin/env python3
"""Enforce Worth Store's physical-owner -> layout-adapter -> courtroom direction."""

from __future__ import annotations

import pathlib
import re
import sys
import tomllib


ROOT = pathlib.Path(__file__).resolve().parents[2]
STORE = ROOT / "workspaces" / "worth-store"
CRATES = STORE / "crates"

PHYSICAL_OWNERS = {
    "worth-store-lsm-authority",
    "worth-store-physical-backend",
    "worth-store-physical-format",
    "worth-store-physical-integrity",
    "worth-store-physical-isolation",
    "worth-store-recovery-physics",
    "worth-store-wal",
}
UPWARD_CRATES = {
    "worth-store-certification",
    "worth-store-layout-indexes",
    "worth-store-test-support",
}
LAYOUT_LOWER_OWNERS = {
    "worth-store-blob-chunks",
    "worth-store-tiering",
}
LAYOUT_OBSERVATION_CONSUMERS = {
    "worth-store-maintenance": {"observation"},
    "worth-store-operations": {
        "access_planning",
        "bootstrap",
        "declarations",
        "integrity",
        "materialization",
        "observation",
    },
}
LAYOUT_IMPORT = re.compile(r"worth_store_layout_indexes::([A-Za-z_][A-Za-z0-9_]*)")
LAYOUT_HARNESS_ROOTS = (
    CRATES / "worth-store-test-support" / "src" / "harness" / "layout",
    CRATES / "worth-store-test-support" / "src" / "harness" / "lsm_execution_fixture.rs",
)
FORBIDDEN_LAYOUT_HARNESS_AUTHORITY = (
    "BaselineBTreeExactCounterWitness::",
    "AdmittedPhysicalReadRequest {",
    "AdmittedPhysicalRecoveryRequest {",
    "AdmittedPhysicalMutationRequest {",
    "from_planned_counter_envelope(",
    "select_with_budget(",
)


def manifest(path: pathlib.Path) -> dict:
    with path.open("rb") as source:
        return tomllib.load(source)


def production_dependencies(document: dict) -> set[str]:
    names: set[str] = set()
    for table_name in ("dependencies", "build-dependencies"):
        names.update(document.get(table_name, {}))
    for target in document.get("target", {}).values():
        for table_name in ("dependencies", "build-dependencies"):
            names.update(target.get(table_name, {}))
    return names


def main() -> int:
    violations: list[str] = []
    crate_dependencies: dict[str, set[str]] = {}

    for path in sorted(CRATES.glob("*/Cargo.toml")):
        document = manifest(path)
        name = document.get("package", {}).get("name")
        if name:
            crate_dependencies[name] = production_dependencies(document)

    for owner in sorted(PHYSICAL_OWNERS):
        forbidden = crate_dependencies.get(owner, set()) & UPWARD_CRATES
        for dependency in sorted(forbidden):
            violations.append(f"{owner} must not depend on {dependency}")

    layout_forbidden = crate_dependencies.get("worth-store-layout-indexes", set()) & {
        "worth-store-certification",
        "worth-store-test-support",
    }
    for dependency in sorted(layout_forbidden):
        violations.append(f"worth-store-layout-indexes must not depend on {dependency}")

    for owner in sorted(LAYOUT_LOWER_OWNERS):
        if "worth-store-layout-indexes" in crate_dependencies.get(owner, set()):
            violations.append(f"{owner} must not depend upward on worth-store-layout-indexes")

    for crate, allowed_modules in LAYOUT_OBSERVATION_CONSUMERS.items():
        source_root = CRATES / crate / "src"
        for path in sorted(source_root.rglob("*.rs")):
            source = path.read_text(encoding="utf-8")
            for match in LAYOUT_IMPORT.finditer(source):
                imported = match.group(1)
                if imported not in allowed_modules:
                    line_number = source.count("\n", 0, match.start()) + 1
                    relative = path.relative_to(STORE)
                    violations.append(
                        f"{relative}:{line_number} imports layout owner module {imported}; "
                        f"consumer crates may import only {sorted(allowed_modules)}"
                    )

            if "layout_projection" in path.parts and path.name != "tests.rs":
                for forbidden in ("fn admit_", "fn readmit_", "fn execute_", "fn issue_"):
                    if forbidden in source:
                        relative = path.relative_to(STORE)
                        violations.append(
                            f"{relative} exposes authority-shaped operation {forbidden.strip()} "
                            "from an observation projection module"
                        )

    for root in LAYOUT_HARNESS_ROOTS:
        paths = root.rglob("*.rs") if root.is_dir() else (root,)
        for path in paths:
            source = path.read_text(encoding="utf-8")
            for forbidden in FORBIDDEN_LAYOUT_HARNESS_AUTHORITY:
                if forbidden in source:
                    relative = path.relative_to(STORE)
                    violations.append(
                        f"{relative} constructs displaced layout authority through {forbidden}"
                    )

    if violations:
        print("Phase 34 dependency-direction violations:", file=sys.stderr)
        for violation in violations:
            print(f"- {violation}", file=sys.stderr)
        return 1

    print(f"Phase 34 dependency direction verified across {len(crate_dependencies)} crates.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
