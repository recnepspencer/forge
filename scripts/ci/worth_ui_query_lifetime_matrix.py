import csv
import re
from pathlib import Path
from typing import Any

from worth_ui_test_topology_config import Violation, required_string


ARTIFACT_COLUMNS = {
    "installed_view",
    "consumed_projection_authority",
    "binding_owned_handle",
    "query_live_resource",
    "compact_plan_reference",
    "inspection_reference",
}
REQUIRED_TRANSITIONS = {
    "installation": "3",
    "launch_lowering": "3",
    "projection_handoff": "3",
    "lowering_denial": "3",
    "candidate_discard": "3",
    "activation": "8",
    "candidate_preparation_denial": "8",
    "semantic_noop": "13",
    "rebind": "8",
    "bounded_replacement": "14",
    "removal": "8",
    "failed_publication": "15",
}
REQUIRED_STATUSES = {
    "installation": "proven",
    "launch_lowering": "proven",
    "projection_handoff": "proven",
    "lowering_denial": "proven",
    "candidate_discard": "proven",
    "activation": "proven",
    "candidate_preparation_denial": "proven",
    "semantic_noop": "proven",
    "rebind": "proven",
    "bounded_replacement": "proven",
    "removal": "proven",
    "failed_publication": "proven",
}
ALLOWED_ACTIONS = {
    "moved",
    "borrowed",
    "retained",
    "observed",
    "released_once",
    "absent",
    "succeeded_once",
    "rollback_once",
}
REQUIRED_ACTION_SETS = {
    "candidate_preparation_denial": {
        "installed_view": {"retained", "released_once"},
        "consumed_projection_authority": {"retained", "rollback_once"},
        "binding_owned_handle": {"retained", "released_once"},
        "query_live_resource": {"retained", "rollback_once"},
        "compact_plan_reference": {"retained", "rollback_once"},
        "inspection_reference": {"retained", "released_once"},
    },
    "rebind": {
        "installed_view": {"released_once", "succeeded_once"},
        "consumed_projection_authority": {"released_once", "succeeded_once"},
        "binding_owned_handle": {"released_once", "succeeded_once"},
        "query_live_resource": {"released_once", "succeeded_once"},
        "compact_plan_reference": {"released_once", "succeeded_once"},
        "inspection_reference": {"released_once", "observed"},
    },
    "failed_publication": {
        "installed_view": {"retained", "released_once"},
        "consumed_projection_authority": {"retained", "rollback_once"},
        "binding_owned_handle": {"retained", "released_once"},
        "query_live_resource": {"retained", "rollback_once"},
        "compact_plan_reference": {"retained", "rollback_once"},
        "inspection_reference": {"retained", "released_once"},
    },
}
TEST_ATTRIBUTE = re.compile(r"(?m)^\s*#\[test\]\s*$")


def query_lifetime_matrix_violations(
    root: Path, config: dict[str, Any]
) -> list[Violation]:
    path = root / required_string(config, "query_lifetime_matrix")
    if not path.is_file():
        return [Violation("query-lifetime-matrix", f"missing {path.relative_to(root)}")]
    with path.open(encoding="utf-8", newline="") as source:
        rows = list(csv.DictReader(source))
    required_columns = {
        "transition",
        "phase",
        "proof_path",
        "status",
        *ARTIFACT_COLUMNS,
    }
    if not rows or required_columns - set(rows[0]):
        return [Violation("query-lifetime-matrix", "matrix columns are incomplete")]

    violations: list[Violation] = []
    transitions = [row["transition"] for row in rows]
    for transition in sorted(set(transitions) - set(REQUIRED_TRANSITIONS)):
        violations.append(Violation("query-lifetime-matrix", f"unexpected {transition}"))
    for transition in sorted(set(REQUIRED_TRANSITIONS) - set(transitions)):
        violations.append(Violation("query-lifetime-matrix", f"missing {transition}"))
    for transition in sorted({item for item in transitions if transitions.count(item) > 1}):
        violations.append(Violation("query-lifetime-matrix", f"duplicate {transition}"))
    for row in rows:
        violations.extend(_row_violations(root, row))
    return violations


def _row_violations(root: Path, row: dict[str, str]) -> list[Violation]:
    transition = row["transition"]
    expected_phase = REQUIRED_TRANSITIONS.get(transition)
    violations: list[Violation] = []
    if expected_phase is not None and row["phase"] != expected_phase:
        violations.append(
            Violation(
                "query-lifetime-matrix",
                f"{transition}: phase {row['phase']} must be {expected_phase}",
            )
        )
    expected_status = REQUIRED_STATUSES.get(transition)
    if row["status"] != expected_status:
        violations.append(
            Violation(
                "query-lifetime-matrix",
                f"{transition}: status must be {expected_status}",
            )
        )
    for artifact in ARTIFACT_COLUMNS:
        clauses = row[artifact].split("+")
        actions = set()
        invalid = False
        for clause in clauses:
            action, separator, owner = clause.partition(":")
            if not separator or action not in ALLOWED_ACTIONS or not owner.strip():
                invalid = True
                break
            actions.add(action)
        if invalid or len(actions) != len(clauses):
            violations.append(
                Violation(
                    "query-lifetime-matrix",
                    f"{transition}/{artifact}: invalid ownership action {row[artifact]!r}",
                )
            )
            continue
        required_actions = REQUIRED_ACTION_SETS.get(transition, {}).get(artifact)
        if required_actions is not None and actions != required_actions:
            violations.append(
                Violation(
                    "query-lifetime-matrix",
                    f"{transition}/{artifact}: actions {sorted(actions)} must be "
                    f"{sorted(required_actions)}",
                )
            )
    proof = root / row["proof_path"]
    if not proof.is_file():
        violations.append(
            Violation("query-lifetime-matrix", f"{transition}: missing proof {row['proof_path']}")
        )
    elif row["status"] == "proven" and not TEST_ATTRIBUTE.search(
        proof.read_text(encoding="utf-8")
    ):
        violations.append(
            Violation("query-lifetime-matrix", f"{transition}: proof contains no test")
        )
    return violations
