#!/usr/bin/env python3
"""Apply the reviewed Worth Store function rename inventory."""

from __future__ import annotations

import csv
import re
from collections import defaultdict
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
CRATES_ROOT = REPO_ROOT / "workspaces/worth-store/crates"
INVENTORY = REPO_ROOT / "_docs/worth-store/worth-store-function-rename-inventory.csv"


def replace_identifier(text: str, old: str, new: str) -> str:
    return re.sub(rf"\b{re.escape(old)}\b", new, text)


def replace_once(text: str, old: str, new: str, path: Path) -> str:
    count = text.count(old)
    if count < 1:
        raise RuntimeError(f"expected {old!r} in {path}")
    return text.replace(old, new, 1)


def apply_overloaded_renames(files: dict[Path, str]) -> None:
    isolation = CRATES_ROOT / "worth-store-io-scheduler/src/admission/isolation.rs"
    text = files[isolation]
    text = replace_once(
        text,
        "pub const fn required_from_s5() -> [Self; 4]",
        "pub const fn required_physical_stability_assumptions() -> [Self; 4]",
        isolation,
    )
    text = replace_once(
        text,
        "pub const fn required_from_s5() -> [Self; 5]",
        "pub const fn required_unsupported_qos_non_claims() -> [Self; 5]",
        isolation,
    )
    text = text.replace(
        "IoSchedulerPhysicalStabilityAssumption::required_from_s5()",
        "IoSchedulerPhysicalStabilityAssumption::required_physical_stability_assumptions()",
    )
    text = text.replace(
        "IoSchedulerUnsupportedQosNonClaim::required_from_s5()",
        "IoSchedulerUnsupportedQosNonClaim::required_unsupported_qos_non_claims()",
    )
    files[isolation] = text

    complexity = CRATES_ROOT / "worth-store-physical-format/src/binary_format/operation_complexity.rs"
    text = files[complexity]
    text = replace_once(
        text,
        "pub const fn s1_required() -> [Self; 8]",
        "pub const fn required_physical_operations() -> [Self; 8]",
        complexity,
    )
    text = replace_once(
        text,
        "pub const fn s1_required(operation: PhysicalOperationKind) -> Self",
        "pub const fn required_complexity_contract(operation: PhysicalOperationKind) -> Self",
        complexity,
    )
    files[complexity] = text

    for path, text in list(files.items()):
        text = text.replace(
            "PhysicalOperationKind::s1_required",
            "PhysicalOperationKind::required_physical_operations",
        )
        text = text.replace(
            "PhysicalOperationComplexityContract::s1_required",
            "PhysicalOperationComplexityContract::required_complexity_contract",
        )
        files[path] = text

    replay = CRATES_ROOT / "worth-store-physical-format/src/facade/replay_artifact.rs"
    files[replay] = replace_identifier(files[replay], "reopen_s1", "reopen_physical_format")
    files[replay] = files[replay].replace(
        "reopen::reopen_physical_format", "reopen::reopen_from_verified_layout"
    )

    reopen = CRATES_ROOT / "worth-store-physical-format/src/facade/reopen.rs"
    text = files[reopen]
    text = replace_once(text, "pub fn reopen_s1(", "pub fn reopen(", reopen)
    text = replace_once(text, "pub(crate) fn reopen_s1(", "pub(crate) fn reopen_from_verified_layout(", reopen)
    text = text.replace("replay_artifact.reopen_s1(", "replay_artifact.reopen_physical_format(")
    files[reopen] = text

    for path, text in list(files.items()):
        text = text.replace("PlatformPhysicalFacade::reopen_s1", "PlatformPhysicalFacade::reopen")
        text = text.replace("replay_artifact.reopen_s1", "replay_artifact.reopen_physical_format")
        text = text.replace("reopen::reopen_s1", "reopen::reopen_from_verified_layout")
        files[path] = text


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as handle:
        rows = list(csv.DictReader(handle))

    proposals: dict[str, set[str]] = defaultdict(set)
    for row in rows:
        proposals[row["current_name"]].add(row["proposed_name"])
    unambiguous = {
        old: next(iter(names)) for old, names in proposals.items() if len(names) == 1
    }

    paths = sorted(CRATES_ROOT.rglob("*.rs"))
    files = {path: path.read_text(encoding="utf-8") for path in paths}
    apply_overloaded_renames(files)

    for path, text in list(files.items()):
        for old, new in sorted(unambiguous.items(), key=lambda item: -len(item[0])):
            text = replace_identifier(text, old, new)
        text = replace_identifier(text, "s1_required", "physical_format_required")
        text = replace_identifier(text, "s2_readiness", "physical_substrate_readiness")
        files[path] = text

    readiness = CRATES_ROOT / "worth-store-readiness/src/physical_integrity/readiness.rs"
    text = files[readiness]
    marker = "#[cfg(test)]"
    production, tests = text.split(marker, 1)
    tests = replace_identifier(
        tests, "physical_substrate_readiness", "physical_substrate_readiness_fixture"
    )
    files[readiness] = production + marker + tests

    changed = 0
    for path in paths:
        original = path.read_text(encoding="utf-8")
        if files[path] != original:
            path.write_text(files[path], encoding="utf-8", newline="")
            changed += 1
    print(f"updated {changed} Rust files")


if __name__ == "__main__":
    main()
