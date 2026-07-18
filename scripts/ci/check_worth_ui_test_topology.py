from __future__ import annotations

import argparse
import csv
import json
import re
import sys
import tomllib
from collections import Counter
from pathlib import Path
from typing import Any

from worth_ui_ci_contract import ci_contract_violations
from worth_ui_test_proof_ledger import ledger_violations
from worth_ui_test_topology_config import (
    Violation,
    load_json,
    load_toml,
    required_int,
    required_string,
    required_string_list,
)


DEFAULT_BUDGET = Path("scripts/ci/worth_ui_test_topology_budget.json")


def normalized(path: Path) -> str:
    return path.as_posix()


def package_violations(
    root: Path, config: dict[str, Any]
) -> tuple[list[Violation], int, set[str]]:
    violations: list[Violation] = []
    compile_contract_targets = 0
    integration_targets: set[str] = set()
    for package, package_config in config["packages"].items():
        manifest_path = root / required_string(package_config, "manifest")
        manifest = load_toml(manifest_path)
        package_table = manifest.get("package", {})
        if package_table.get("autotests") is not False:
            violations.append(
                Violation("explicit-targets", f"{package}: package.autotests must be false")
            )
        targets = manifest.get("test", [])
        if not isinstance(targets, list):
            raise ValueError(f"{package}: [[test]] entries must be a list")
        maximum = required_int(package_config, "max_integration_targets")
        if len(targets) > maximum:
            violations.append(
                Violation("target-budget", f"{package}: {len(targets)} targets exceeds {maximum}")
            )
        seen_names: set[str] = set()
        for target in targets:
            name = required_string(target, "name")
            path = required_string(target, "path")
            if name in seen_names:
                violations.append(Violation("target-identity", f"{package}: duplicate {name}"))
            seen_names.add(name)
            integration_targets.add(f"{package}:{name}")
            target_path = manifest_path.parent / path
            if not target_path.is_file():
                violations.append(
                    Violation("target-path", f"{package}: missing {normalized(target_path.relative_to(root))}")
                )
            if name == "compile_contracts":
                compile_contract_targets += 1
        expected_names = set(required_string_list(package_config, "expected_integration_targets"))
        for name in sorted(seen_names - expected_names):
            violations.append(
                Violation("target-regression", f"{package}: unexpected integration target {name}")
            )
        for name in sorted(expected_names - seen_names):
            violations.append(
                Violation("target-regression", f"{package}: missing integration target {name}")
            )
    return violations, compile_contract_targets, integration_targets


def lane_violations(
    config: dict[str, Any], integration_targets: set[str]
) -> list[Violation]:
    configured_targets = Counter(
        target for targets in config["proof_lanes"].values() for target in targets
    )
    violations: list[Violation] = []
    for target, count in sorted(configured_targets.items()):
        if count != 1:
            violations.append(
                Violation("lane-convergence", f"{target} is owned by {count} proof lanes")
            )

    external_targets = set(required_string_list(config, "external_proof_targets"))
    configured_integration_targets = set(configured_targets) - external_targets
    for target in sorted(configured_integration_targets - integration_targets):
        violations.append(
            Violation("lane-convergence", f"lane owns unknown integration target {target}")
        )
    for target in sorted(integration_targets - configured_integration_targets):
        violations.append(
            Violation("lane-convergence", f"integration target has no proof lane: {target}")
        )
    for target in sorted(external_targets - set(configured_targets)):
        violations.append(
            Violation("lane-convergence", f"external proof target has no lane: {target}")
        )
    return violations


def rust_test_sources(root: Path, config: dict[str, Any]) -> list[Path]:
    sources: list[Path] = []
    for package_config in config["packages"].values():
        manifest = root / required_string(package_config, "manifest")
        tests = manifest.parent / "tests"
        if tests.is_dir():
            sources.extend(
                source
                for source in tests.rglob("*.rs")
                if not is_embedded_topology_fixture(tests, source)
            )
    return sorted(set(sources))


def is_embedded_topology_fixture(tests: Path, source: Path) -> bool:
    parts = source.relative_to(tests).parts
    return len(parts) > 2 and parts[0] == "fixtures" and parts[1].startswith("topology_")


def source_violations(root: Path, config: dict[str, Any]) -> list[Violation]:
    violations: list[Violation] = []
    allowed_sessions = {
        normalized(Path(path)) for path in config.get("allowed_trybuild_sessions", [])
    }
    actual_sessions: set[str] = set()
    constructor_count = 0
    nested_cargo_patterns = (
        re.compile(r'Command::new\(\s*"cargo"\s*\)'),
        re.compile(r"Command::new\(\s*cargo_binary\(\)\s*\)"),
        re.compile(r"\.arg\(\s*\"--manifest-path\"\s*\)"),
    )
    for source in rust_test_sources(root, config):
        text = source.read_text(encoding="utf-8")
        relative = normalized(source.relative_to(root))
        source_constructor_count = text.count("trybuild::TestCases::new")
        constructor_count += source_constructor_count
        if source_constructor_count:
            actual_sessions.add(relative)
        if source_constructor_count > 1:
            violations.append(
                Violation(
                    "trybuild-session",
                    f"{relative}: {source_constructor_count} TestCases sessions share one binary",
                )
            )
        if "RUSTFLAGS" in text:
            violations.append(
                Violation("warning-fingerprint", f"{relative}: runtime RUSTFLAGS mutation is forbidden")
            )
        if any(pattern.search(text) for pattern in nested_cargo_patterns):
            violations.append(
                Violation("nested-cargo", f"{relative}: Cargo subprocess inside an ordinary test")
            )
    unexpected = actual_sessions - allowed_sessions
    missing = allowed_sessions - actual_sessions
    for path in sorted(unexpected):
        violations.append(Violation("trybuild-session", f"unexpected session owner: {path}"))
    for path in sorted(missing):
        violations.append(Violation("trybuild-session", f"configured session owner missing: {path}"))
    maximum_sessions = required_int(config, "max_trybuild_sessions")
    if constructor_count > maximum_sessions:
        violations.append(
            Violation(
                "trybuild-session-budget",
                f"{constructor_count} TestCases sessions exceeds {maximum_sessions}",
            )
        )
    return violations


def source_inventory_violations(root: Path, config: dict[str, Any]) -> list[Violation]:
    owner = root / required_string(config, "workspace_source_inventory_owner")
    suite = root / required_string(config, "workspace_source_inventory_suite")
    violations: list[Violation] = []
    if not owner.is_file():
        return [Violation("source-inventory", f"missing {normalized(owner.relative_to(root))}")]
    if not suite.is_file():
        return [Violation("source-inventory", f"missing {normalized(suite.relative_to(root))}")]

    suite_text = suite.read_text(encoding="utf-8")
    for marker in ("LazyLock<WorkspaceSourceInventory>", "workspace_source_inventory()"):
        if marker not in suite_text:
            violations.append(
                Violation("source-inventory", f"{normalized(suite.relative_to(root))}: missing {marker}")
            )

    path_directive = re.compile(r'#\[path = "([^"]+)"\]')
    for relative_module in path_directive.findall(suite_text):
        module = (suite.parent / relative_module).resolve()
        module_text = module.read_text(encoding="utf-8")
        if "read_dir(" in module_text or "fs::read_to_string" in module_text:
            violations.append(
                Violation(
                    "source-inventory",
                    f"{normalized(module.relative_to(root))}: topology suite module bypasses the shared source inventory",
                )
            )

    topology_root = owner.parent
    for source in topology_root.rglob("*.rs"):
        if source == owner:
            continue
        text = source.read_text(encoding="utf-8")
        if "read_dir(" in text:
            violations.append(
                Violation(
                    "source-inventory",
                    f"{normalized(source.relative_to(root))}: recursive source discovery bypasses the inventory owner",
                )
            )
    return violations


def required_source_violations(root: Path, config: dict[str, Any]) -> list[Violation]:
    violations: list[Violation] = []
    for configured_path in config.get("required_source_paths", []):
        source = root / configured_path
        if not source.is_file():
            violations.append(
                Violation("fresh-checkout-source", f"missing required source {configured_path}")
            )
    return violations


def compile_reconciliation_violations(
    root: Path, config: dict[str, Any]
) -> list[Violation]:
    inventory_path = root / required_string(config, "worth_ui_compile_inventory")
    execution_path = root / required_string(config, "worth_ui_compile_execution")
    violations: list[Violation] = []
    inventory = read_compile_rows(inventory_path)
    execution = read_compile_rows(execution_path)
    expected_inventory = required_int(config, "worth_ui_compile_inventory_count")
    expected_execution = required_int(config, "worth_ui_compile_execution_count")
    if len(inventory) != expected_inventory:
        violations.append(
            Violation(
                "compile-reconciliation",
                f"inventory has {len(inventory)} rows; expected {expected_inventory}",
            )
        )
    if len(execution) != expected_execution:
        violations.append(
            Violation(
                "compile-reconciliation",
                f"execution has {len(execution)} rows; expected {expected_execution}",
            )
        )

    inventory_by_path = {row["path"]: row for row in inventory}
    execution_by_path = {row["path"]: row for row in execution}
    if len(inventory_by_path) != len(inventory):
        violations.append(Violation("compile-reconciliation", "inventory paths are not unique"))
    if len(execution_by_path) != len(execution):
        violations.append(Violation("compile-reconciliation", "execution paths are not unique"))

    for path, row in execution_by_path.items():
        if inventory_by_path.get(path) != row:
            violations.append(
                Violation("compile-reconciliation", f"executed row is absent or changed: {path}")
            )
        fixture = inventory_path.parent.parent.parent / path
        if not fixture.is_file():
            violations.append(Violation("compile-reconciliation", f"missing fixture: {path}"))
        if row["kind"] == "fail" and not fixture.with_suffix(".stderr").is_file():
            violations.append(
                Violation("compile-reconciliation", f"missing compiler diagnostic: {path}")
            )

    replacement_patterns = [
        re.compile(pattern) for pattern in config["compile_structural_replacement_patterns"]
    ]
    removed = [row for row in inventory if row["path"] not in execution_by_path]
    for row in removed:
        if row["kind"] == "fail" and not any(
            pattern.search(row["path"]) for pattern in replacement_patterns
        ):
            violations.append(
                Violation(
                    "compile-reconciliation",
                    f"non-redundant compiler denial was removed: {row['path']}",
                )
            )

    inventory_owners = {row["legacy_harness"] for row in inventory}
    execution_owners = {row["legacy_harness"] for row in execution}
    for owner in sorted(inventory_owners - execution_owners):
        violations.append(
            Violation("compile-reconciliation", f"legacy proof family has no representative: {owner}")
        )
    return violations


def read_compile_rows(path: Path) -> list[dict[str, str]]:
    with path.open(encoding="utf-8", newline="") as source:
        rows = list(csv.DictReader(source))
    required = {"kind", "path", "legacy_harness"}
    if rows and not required.issubset(rows[0]):
        raise ValueError(f"compile reconciliation columns are invalid: {path}")
    for row in rows:
        if row["kind"] not in {"pass", "fail"}:
            raise ValueError(f"compile reconciliation kind is invalid: {row}")
    return rows


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Check Worth UI test execution topology")
    parser.add_argument("--root", type=Path, default=Path.cwd())
    parser.add_argument("--budget", type=Path, default=DEFAULT_BUDGET)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    root = args.root.resolve()
    budget_path = args.budget if args.budget.is_absolute() else root / args.budget
    try:
        config = load_json(budget_path)
        violations, compile_targets, integration_targets = package_violations(root, config)
        violations.extend(lane_violations(config, integration_targets))
        maximum = required_int(config, "max_compile_contract_targets")
        if compile_targets > maximum:
            violations.append(
                Violation("compile-target-budget", f"{compile_targets} exceeds {maximum}")
            )
        violations.extend(source_violations(root, config))
        violations.extend(source_inventory_violations(root, config))
        violations.extend(required_source_violations(root, config))
        violations.extend(ci_contract_violations(root, config))
        violations.extend(compile_reconciliation_violations(root, config))
        violations.extend(ledger_violations(root, config))
        lane_runner = root / required_string(config, "lane_runner")
        if not lane_runner.is_file():
            violations.append(Violation("lane-runner", f"missing {normalized(lane_runner.relative_to(root))}"))
    except (OSError, ValueError, json.JSONDecodeError, tomllib.TOMLDecodeError) as error:
        print(f"[worth-ui-test-topology] invalid configuration: {error}", file=sys.stderr)
        return 2
    if violations:
        print("[worth-ui-test-topology] execution topology violated:", file=sys.stderr)
        for violation in violations:
            print(f"  {violation.rule}: {violation.detail}", file=sys.stderr)
        return 1
    print(
        f"[worth-ui-test-topology] {len(config['packages'])} packages and "
        f"{compile_targets} compile-contract targets satisfy the budget"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
