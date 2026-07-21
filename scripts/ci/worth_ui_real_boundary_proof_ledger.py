import csv
import re
from pathlib import Path
from typing import Any

from worth_ui_test_topology_config import Violation, required_string


REQUIRED_PHASES = {
    "regional_plan_algorithms": "4",
    "filesystem_source_acquisition": "2",
    "filesystem_watcher_settlement": "2",
    "public_candidate_application_lifecycle": "1",
    "public_filesystem_lowering_authority": "3",
    "public_initial_plan_lifecycle": "5",
    "public_semantic_noop_lifecycle": "13",
    "public_atomic_plan_cutover": "15",
    "query_lowering_authority": "3",
    "query_virtualized_execution": "8",
    "query_candidate_preparation_rollback": "8",
    "canvas_spatial_execution": "9",
    "realtime_overlay_execution": "10",
    "query_semantic_noop_lifetime": "13",
    "query_bounded_replacement_lifetime": "14",
    "query_failed_publication_lifetime": "15",
    "egui_host_execution": "11",
    "headless_minimal_plan_execution": "5",
    "headless_complete_ordinary_execution": "7",
    "headless_cross_lane_parity": "11",
    "executor_allocator_observation": "5",
    "cross_lane_allocator_reconciliation": "16",
    "egui_allocator_attribution": "16",
    "same_session_mixed_real_lifecycle": "18",
    "public_multi_removal_successor": "18",
}
REQUIRED_CLAIMS = set(REQUIRED_PHASES)
EXTERNAL_CLAIMS = REQUIRED_CLAIMS - {"regional_plan_algorithms"}
REQUIRED_COLUMNS = {
    "claim",
    "phase",
    "proof_class",
    "compiled_owner",
    "lane",
    "module_path",
    "production_entry_point",
    "independent_observation",
    "fake_implementation_rejected",
    "status",
}
TEST_ATTRIBUTE = re.compile(r"(?m)^\s*#\[test\]\s*$")


def real_boundary_ledger_violations(
    root: Path, config: dict[str, Any]
) -> list[Violation]:
    path = root / required_string(config, "real_boundary_proof_ledger")
    if not path.is_file():
        return [Violation("real-boundary-ledger", f"missing {path.relative_to(root).as_posix()}")]
    with path.open(encoding="utf-8", newline="") as source:
        rows = list(csv.DictReader(source))
    if not rows:
        return [Violation("real-boundary-ledger", "ledger has no claims")]
    missing_columns = REQUIRED_COLUMNS - set(rows[0])
    if missing_columns:
        return [
            Violation(
                "real-boundary-ledger",
                f"missing columns: {', '.join(sorted(missing_columns))}",
            )
        ]

    violations: list[Violation] = []
    application_modules = included_application_contract_modules(root, config)
    claims = [row["claim"] for row in rows]
    for duplicate in sorted({claim for claim in claims if claims.count(claim) > 1}):
        violations.append(Violation("real-boundary-ledger", f"duplicate claim: {duplicate}"))
    for claim in sorted(REQUIRED_CLAIMS - set(claims)):
        violations.append(Violation("real-boundary-ledger", f"missing claim: {claim}"))
    for claim in sorted(set(claims) - REQUIRED_CLAIMS):
        violations.append(Violation("real-boundary-ledger", f"unexpected claim: {claim}"))
    for row in rows:
        violations.extend(row_violations(root, row, application_modules))
    return violations


def row_violations(
    root: Path, row: dict[str, str], application_modules: set[Path]
) -> list[Violation]:
    claim = row["claim"]
    violations: list[Violation] = []
    expected_phase = REQUIRED_PHASES.get(claim)
    if expected_phase is not None and row["phase"] != expected_phase:
        violations.append(
            Violation(
                "real-boundary-ledger",
                f"{claim}: phase {row['phase']} does not match owning phase {expected_phase}",
            )
        )
    if row["status"] not in {"assigned", "proven"}:
        violations.append(
            Violation("real-boundary-ledger", f"{claim}: invalid status {row['status']}")
        )
    if claim in EXTERNAL_CLAIMS:
        violations.extend(external_placement_violations(row))
    elif claim == "regional_plan_algorithms":
        if row["proof_class"] != "local_algorithm" or row["lane"] != "fast":
            violations.append(
                Violation(
                    "real-boundary-ledger",
                    f"{claim}: local algorithms must remain local_algorithm proof in fast",
                )
            )
    module = root / row["module_path"]
    if not module.is_file():
        violations.append(
            Violation("real-boundary-ledger", f"{claim}: missing module {row['module_path']}")
        )
    elif row["status"] == "proven" and not TEST_ATTRIBUTE.search(
        module.read_text(encoding="utf-8")
    ):
        violations.append(
            Violation("real-boundary-ledger", f"{claim}: proven module contains no test")
        )
    if claim in EXTERNAL_CLAIMS and module.resolve() not in application_modules:
        violations.append(
            Violation(
                "real-boundary-ledger",
                f"{claim}: module is not included by application_contracts",
            )
        )
    for field in (
        "production_entry_point",
        "independent_observation",
        "fake_implementation_rejected",
    ):
        if not row[field].strip():
            violations.append(
                Violation("real-boundary-ledger", f"{claim}: {field} is empty")
            )
    observation = row["independent_observation"].lower()
    if "receipt only" in observation or "self reported" in observation:
        violations.append(
            Violation(
                "real-boundary-ledger",
                f"{claim}: independent observation cannot be {row['independent_observation']}",
            )
        )
    return violations


def included_application_contract_modules(
    root: Path, config: dict[str, Any]
) -> set[Path]:
    suite = root / required_string(config, "application_contracts_suite")
    if not suite.is_file():
        raise ValueError("application_contracts_suite must name an existing file")
    path_directive = re.compile(r'#\[path = "([^"]+)"\]')
    included: set[Path] = set()
    pending = [suite]
    while pending:
        owner = pending.pop()
        for relative in path_directive.findall(owner.read_text(encoding="utf-8")):
            module = (owner.parent / relative).resolve()
            if module not in included:
                included.add(module)
                pending.append(module)
    return included


def external_placement_violations(row: dict[str, str]) -> list[Violation]:
    claim = row["claim"]
    violations: list[Violation] = []
    if row["proof_class"] != "real_boundary":
        violations.append(
            Violation(
                "real-boundary-ledger", f"{claim}: external claim requires real_boundary proof"
            )
        )
    if row["compiled_owner"] != "worth-ui-certification:application_contracts":
        violations.append(
            Violation(
                "real-boundary-ledger",
                f"{claim}: must compile in worth-ui-certification:application_contracts",
            )
        )
    if row["lane"] != "hostile-certification":
        violations.append(
            Violation(
                "real-boundary-ledger", f"{claim}: real boundary cannot run in {row['lane']}"
            )
        )
    expected_prefix = (
        "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/"
    )
    if not row["module_path"].startswith(expected_prefix):
        violations.append(
            Violation(
                "real-boundary-ledger",
                f"{claim}: module is outside the application_contracts responsibility tree",
            )
        )
    return violations
