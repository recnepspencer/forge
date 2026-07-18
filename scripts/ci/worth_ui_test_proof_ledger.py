import csv
import re
from collections import Counter
from pathlib import Path
from typing import Any

from worth_ui_test_topology_config import (
    Violation,
    load_json,
    load_toml,
    required_string,
    required_string_values,
)


def ledger_violations(root: Path, config: dict[str, Any]) -> list[Violation]:
    path = root / required_string(config, "proof_ledger")
    if not path.is_file():
        return [Violation("proof-ledger", f"missing {path.relative_to(root).as_posix()}")]
    with path.open(encoding="utf-8", newline="") as source:
        rows = list(csv.DictReader(source))
    required = {
        "legacy_package",
        "legacy_target",
        "proof_count",
        "fixture_count",
        "final_lane",
        "final_target",
        "disposition",
    }
    if not rows:
        return [Violation("proof-ledger", "ledger has no migration rows")]
    if not required.issubset(rows[0]):
        missing = sorted(required - set(rows[0]))
        return [Violation("proof-ledger", f"missing columns: {', '.join(missing)}")]

    identities: set[tuple[str, str]] = set()
    violations: list[Violation] = []
    lane_targets = {lane: set(targets) for lane, targets in config["proof_lanes"].items()}
    suite_targets = integration_suite_targets(root, config)
    replacement_proofs = config.get("module_proof_replacements", {})
    if not isinstance(replacement_proofs, dict):
        raise ValueError("module_proof_replacements must be an object")
    used_replacements: set[str] = set()

    for row in rows:
        identity = (row["legacy_package"], row["legacy_target"])
        if identity in identities:
            violations.append(Violation("proof-ledger", f"duplicate legacy target: {identity}"))
        identities.add(identity)
        if row["disposition"] not in {
            "module_preserved",
            "batched",
            "explicit_lane",
            "support_owned",
        }:
            violations.append(Violation("proof-ledger", f"invalid disposition for {identity}"))
        reconcile_lane_ownership(row, identity, lane_targets, violations)
        if row["disposition"] == "module_preserved":
            replacement_key = f"{row['legacy_package']}:{row['legacy_target']}"
            proof_paths = preserved_proof_paths(
                root, config, row, replacement_key, replacement_proofs, used_replacements
            )
            reconcile_preserved_modules(
                root, row, identity, proof_paths, suite_targets, violations
            )
        reconcile_counts(row, identity, violations)

    for replacement_key in sorted(set(replacement_proofs) - used_replacements):
        violations.append(
            Violation("proof-ledger", f"unused module proof replacement: {replacement_key}")
        )
    reconcile_baseline_counts(root, config, rows, violations)
    return violations


def reconcile_lane_ownership(row, identity, lane_targets, violations) -> None:
    lane = row["final_lane"]
    target = f"{row['legacy_package']}:{row['final_target']}"
    if lane not in lane_targets:
        violations.append(Violation("proof-ledger", f"{identity}: unknown lane {lane}"))
    elif target not in lane_targets[lane]:
        violations.append(
            Violation("proof-ledger", f"{identity}: {target} is not owned by lane {lane}")
        )


def preserved_proof_paths(
    root, config, row, replacement_key, replacement_proofs, used_replacements
):
    configured_proofs = replacement_proofs.get(replacement_key)
    if configured_proofs is None:
        package_manifest = root / required_string(
            config["packages"][row["legacy_package"]], "manifest"
        )
        return [package_manifest.parent / "tests" / f"{row['legacy_target']}.rs"]
    used_replacements.add(replacement_key)
    return [
        root / path
        for path in required_string_values(
            configured_proofs, f"module_proof_replacements.{replacement_key}"
        )
    ]


def reconcile_preserved_modules(
    root, row, identity, proof_paths, suite_targets, violations
) -> None:
    suite = suite_targets.get((row["legacy_package"], row["final_target"]))
    if suite is None:
        violations.append(Violation("proof-ledger", f"{identity}: final target is missing"))
        return
    included_modules = included_suite_modules(suite)
    for proof_path in proof_paths:
        if not proof_path.is_file():
            relative = proof_path.relative_to(root).as_posix()
            violations.append(
                Violation("proof-ledger", f"{identity}: preserved proof is missing: {relative}")
            )
        elif proof_path.resolve() not in included_modules:
            violations.append(
                Violation(
                    "proof-ledger",
                    f"{identity}: {proof_path.name} is not included by {suite.name}",
                )
            )


def reconcile_counts(row, identity, violations) -> None:
    for field in ("proof_count", "fixture_count"):
        try:
            count = int(row[field])
        except ValueError:
            violations.append(Violation("proof-ledger", f"{identity}: {field} is not an integer"))
            continue
        if count < 0:
            violations.append(Violation("proof-ledger", f"{identity}: {field} is negative"))


def reconcile_baseline_counts(root, config, rows, violations) -> None:
    baseline = load_json(root / required_string(config, "baseline_inventory"))
    baseline_counts = baseline.get("integration_targets", {})
    actual_counts = Counter(row["legacy_package"] for row in rows)
    expected_packages = set(config["packages"])
    if set(actual_counts) != expected_packages:
        violations.append(
            Violation(
                "proof-ledger-parity",
                f"ledger packages {sorted(actual_counts)} do not equal {sorted(expected_packages)}",
            )
        )
    for package in sorted(expected_packages):
        expected = baseline_counts.get(package)
        if not isinstance(expected, int):
            violations.append(Violation("proof-ledger-parity", f"baseline missing {package}"))
        elif actual_counts[package] != expected:
            violations.append(
                Violation(
                    "proof-ledger-parity",
                    f"{package}: {actual_counts[package]} rows does not preserve {expected}",
                )
            )


def integration_suite_targets(root: Path, config: dict[str, Any]):
    suites = {}
    for package, package_config in config["packages"].items():
        manifest_path = root / required_string(package_config, "manifest")
        manifest = load_toml(manifest_path)
        for target in manifest.get("test", []):
            suites[(package, required_string(target, "name"))] = (
                manifest_path.parent / required_string(target, "path")
            )
    return suites


def included_suite_modules(suite: Path) -> set[Path]:
    pattern = re.compile(r'#\[path = "([^"]+)"\]')
    return {
        (suite.parent / relative_module).resolve()
        for relative_module in pattern.findall(suite.read_text(encoding="utf-8"))
    }
