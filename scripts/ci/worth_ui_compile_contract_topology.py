import ast
import csv
import os
import re
from pathlib import Path
from typing import Any

from worth_ui_test_topology_config import (
    Violation,
    load_toml,
    required_int,
    required_string,
)


def compile_reconciliation_violations(
    root: Path, config: dict[str, Any]
) -> list[Violation]:
    sessions = config.get("compile_contract_sessions")
    if not isinstance(sessions, dict) or not sessions:
        raise ValueError("compile_contract_sessions must be a non-empty object")
    violations: list[Violation] = []
    for session, session_config in sessions.items():
        if not isinstance(session, str) or not isinstance(session_config, dict):
            raise ValueError("compile_contract_sessions entries must be named objects")
        violations.extend(session_violations(root, session, session_config))
    violations.extend(compile_fixture_owner_violations(root, config, sessions))
    return violations


def compile_fixture_owner_violations(
    root: Path, config: dict[str, Any], sessions: dict[str, Any]
) -> list[Violation]:
    if "compile_contract_fixture_manifest" not in config:
        return []
    manifest_path = root / required_string(config, "compile_contract_fixture_manifest")
    runner_path = root / required_string(config, "compile_contract_runner")
    manifest = load_toml(manifest_path)
    targets = manifest.get("bin", [])
    if not isinstance(targets, list):
        raise ValueError("compile fixture [[bin]] entries must be a list")
    actual_sources: set[Path] = set()
    violations: list[Violation] = []
    for target in targets:
        source = (manifest_path.parent / required_string(target, "path")).resolve()
        if source in actual_sources:
            violations.append(
                Violation("compile-fixture-owner", f"duplicate fixture target: {source}")
            )
        actual_sources.add(source)

    expected_sources: set[Path] = set()
    for session_config in sessions.values():
        execution_path = root / required_string(session_config, "execution")
        crate_root = execution_path.parent.parent.parent
        expected_sources.update(
            (crate_root / row["path"]).resolve()
            for row in read_compile_rows(execution_path)
        )
    for source in sorted(expected_sources - actual_sources):
        violations.append(
            Violation("compile-fixture-owner", f"missing Cargo target: {source}")
        )
    for source in sorted(actual_sources - expected_sources):
        violations.append(
            Violation("compile-fixture-owner", f"uninventoried Cargo target: {source}")
        )
    violations.extend(compile_runner_session_violations(runner_path, config))
    return violations


def compile_runner_session_violations(
    runner_path: Path, config: dict[str, Any]
) -> list[Violation]:
    if not runner_path.is_file():
        return [Violation("compile-fixture-owner", f"missing runner: {runner_path}")]
    tree = ast.parse(runner_path.read_text(encoding="utf-8"))
    calls = sum(
        1
        for node in ast.walk(tree)
        if isinstance(node, ast.Call)
        and isinstance(node.func, ast.Name)
        and node.func.id == "cargo_check"
    )
    maximum = required_int(config, "max_compile_cargo_sessions")
    if calls != maximum:
        return [
            Violation(
                "compile-cargo-session-budget",
                f"runner has {calls} Cargo check calls; expected {maximum}",
            )
        ]
    return []


def session_violations(
    root: Path, session: str, config: dict[str, Any]
) -> list[Violation]:
    inventory_path = root / required_string(config, "inventory")
    execution_path = root / required_string(config, "execution")
    inventory = read_compile_rows(inventory_path)
    execution = read_compile_rows(execution_path)
    violations = count_violations(session, config, inventory, execution)
    inventory_by_path = unique_rows(session, "inventory", inventory, violations)
    execution_by_path = unique_rows(session, "execution", execution, violations)
    fixture_root = inventory_path.parent.parent.parent
    covered_by = included_compile_coverage(
        session,
        fixture_root,
        inventory_by_path,
        execution_by_path,
        config,
        violations,
    )
    violations.extend(
        physical_fixture_violations(session, inventory_path, inventory, covered_by)
    )

    for path, row in execution_by_path.items():
        if inventory_by_path.get(path) != row:
            violations.append(
                Violation(
                    "compile-reconciliation",
                    f"{session}: executed row is absent or changed: {path}",
                )
            )
        fixture = fixture_root / path
        if not fixture.is_file():
            violations.append(
                Violation("compile-reconciliation", f"{session}: missing fixture: {path}")
            )
        if row["kind"] == "fail" and not fixture.with_suffix(".stderr").is_file():
            violations.append(
                Violation(
                    "compile-reconciliation",
                    f"{session}: missing compiler diagnostic: {path}",
                )
            )

    patterns = required_patterns(config)
    removed = [row for row in inventory if row["path"] not in execution_by_path]
    for row in removed:
        if row["path"] in covered_by:
            continue
        if row["kind"] == "pass":
            violations.append(
                Violation(
                    "compile-reconciliation",
                    f"{session}: inventoried compile-pass is not executed: {row['path']}",
                )
            )
            continue
        if row["kind"] == "fail" and not any(
            pattern.search(row["path"]) for pattern in patterns
        ):
            violations.append(
                Violation(
                    "compile-reconciliation",
                    f"{session}: non-redundant compiler denial was removed: {row['path']}",
                )
            )

    inventory_owners = {row["legacy_harness"] for row in inventory}
    execution_owners = {row["legacy_harness"] for row in execution}
    execution_owners.update(
        inventory_by_path[path]["legacy_harness"] for path in covered_by
    )
    for owner in sorted(inventory_owners - execution_owners):
        violations.append(
            Violation(
                "compile-reconciliation",
                f"{session}: proof family has no representative: {owner}",
            )
        )
    return violations


def included_compile_coverage(
    session: str,
    crate_root: Path,
    inventory: dict[str, dict[str, str]],
    execution: dict[str, dict[str, str]],
    config: dict[str, Any],
    violations: list[Violation],
) -> dict[str, str]:
    """Map compiler fixtures compiled inside an executed aggregate case.

    `include!` preserves compiler enforcement while avoiding one rustc process
    per assertion in the same configured architectural family. Only
    inventoried compile-fail fixtures may be aggregated; pass cases remain
    independent executable proof.
    """
    include_pattern = re.compile(r'include!\(\s*"([^"]+)"\s*\)')
    covered_by: dict[str, str] = {}
    for aggregate_path, aggregate_row in execution.items():
        aggregate = crate_root / aggregate_path
        if not aggregate.is_file():
            continue
        for included in include_pattern.findall(aggregate.read_text(encoding="utf-8")):
            included_path = (aggregate.parent / included).resolve()
            try:
                normalized_path = included_path.relative_to(crate_root.resolve()).as_posix()
            except ValueError:
                violations.append(
                    Violation(
                        "compile-reconciliation",
                        f"{session}: aggregate include escapes crate: {included}",
                    )
                )
                continue
            row = inventory.get(normalized_path)
            if row is None:
                violations.append(
                    Violation(
                        "compile-reconciliation",
                        f"{session}: aggregate covers uninventoried fixture: {normalized_path}",
                    )
                )
                continue
            if aggregate_row["kind"] != "fail" or row["kind"] != "fail":
                violations.append(
                    Violation(
                        "compile-reconciliation",
                        f"{session}: only compile-fail fixtures may aggregate: {normalized_path}",
                    )
                )
                continue
            aggregate_group = aggregation_group(aggregate_path, config)
            covered_group = aggregation_group(normalized_path, config)
            same_owner = aggregate_row["legacy_harness"] == row["legacy_harness"]
            same_group = aggregate_group is not None and aggregate_group == covered_group
            if not same_owner and not same_group:
                violations.append(
                    Violation(
                        "compile-reconciliation",
                        f"{session}: aggregate crosses semantic owners: {normalized_path}",
                    )
                )
                continue
            prior = covered_by.get(normalized_path)
            if prior is not None or normalized_path in execution:
                violations.append(
                    Violation(
                        "compile-reconciliation",
                        f"{session}: fixture has duplicate compiler coverage: {normalized_path}",
                    )
                )
                continue
            covered_by[normalized_path] = aggregate_path
    return covered_by


def aggregation_group(path: str, config: dict[str, Any]) -> str | None:
    matches: list[str] = []
    for group in config.get("aggregation_groups", []):
        name = group.get("name")
        patterns = group.get("patterns")
        if not isinstance(name, str) or not isinstance(patterns, list):
            raise ValueError("compile aggregation groups require names and pattern lists")
        if any(re.search(pattern, path) for pattern in patterns):
            matches.append(name)
    if len(matches) > 1:
        raise ValueError(f"compile fixture belongs to multiple aggregation groups: {path}")
    return matches[0] if matches else None


def physical_fixture_violations(
    session: str,
    inventory_path: Path,
    inventory: list[dict[str, str]],
    covered_by: dict[str, str],
) -> list[Violation]:
    crate_root = inventory_path.parent.parent.parent
    fixture_root = crate_root / "tests/ui"
    expected_sources = {Path(row["path"]).as_posix() for row in inventory}
    allowed_diagnostics = {
        Path(row["path"]).with_suffix(".stderr").as_posix()
        for row in inventory
        if row["kind"] == "fail"
    }
    required_diagnostics = {
        Path(row["path"]).with_suffix(".stderr").as_posix()
        for row in inventory
        if row["kind"] == "fail" and row["path"] not in covered_by
    }
    actual_sources = physical_paths(crate_root, fixture_root, "*.rs")
    actual_diagnostics = physical_paths(crate_root, fixture_root, "*.stderr")
    violations: list[Violation] = []
    for path in sorted(actual_sources - expected_sources):
        violations.append(
            Violation(
                "compile-physical-fixture",
                f"{session}: unlisted Rust compile fixture: {path}",
            )
        )
    for path in sorted(expected_sources - actual_sources):
        violations.append(
            Violation(
                "compile-physical-fixture",
                f"{session}: inventoried Rust compile fixture is missing: {path}",
            )
        )
    for path in sorted(actual_diagnostics - allowed_diagnostics):
        violations.append(
            Violation(
                "compile-physical-fixture",
                f"{session}: unlisted compile diagnostic: {path}",
            )
        )
    for path in sorted(required_diagnostics - actual_diagnostics):
        violations.append(
            Violation(
                "compile-physical-fixture",
                f"{session}: inventoried compile diagnostic is missing: {path}",
            )
        )
    for manifest in physical_paths(crate_root, fixture_root, "Cargo.toml"):
        violations.append(
            Violation(
                "generated-compilation",
                f"{session}: embedded compile workspace manifest: {manifest}",
            )
        )
    return violations


def physical_paths(crate_root: Path, fixture_root: Path, pattern: str) -> set[str]:
    matches: set[str] = set()
    for directory, child_directories, filenames in os.walk(fixture_root):
        child_directories[:] = [name for name in child_directories if name != "target"]
        for filename in filenames:
            if Path(filename).match(pattern):
                matches.add((Path(directory) / filename).relative_to(crate_root).as_posix())
    return matches


def count_violations(session, config, inventory, execution) -> list[Violation]:
    violations: list[Violation] = []
    expected_inventory = required_int(config, "inventory_count")
    expected_execution = required_int(config, "execution_count")
    if len(inventory) != expected_inventory:
        violations.append(
            Violation(
                "compile-reconciliation",
                f"{session}: inventory has {len(inventory)} rows; expected {expected_inventory}",
            )
        )
    if len(execution) != expected_execution:
        violations.append(
            Violation(
                "compile-reconciliation",
                f"{session}: execution has {len(execution)} rows; expected {expected_execution}",
            )
        )
    return violations


def unique_rows(session, label, rows, violations) -> dict[str, dict[str, str]]:
    rows_by_path = {row["path"]: row for row in rows}
    if len(rows_by_path) != len(rows):
        violations.append(
            Violation(
                "compile-reconciliation", f"{session}: {label} paths are not unique"
            )
        )
    return rows_by_path


def required_patterns(config: dict[str, Any]) -> list[re.Pattern[str]]:
    values = config.get("structural_replacement_patterns")
    if not isinstance(values, list) or not all(isinstance(value, str) for value in values):
        raise ValueError("structural_replacement_patterns must be a list of strings")
    return [re.compile(value) for value in values]


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
