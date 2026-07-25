from pathlib import Path
from typing import Any

from worth_ui_test_topology_config import Violation, load_json, required_string


REQUIRED_COST_LAYERS = {
    "compilation",
    "linking",
    "immutable_world_construction",
    "isolated_delta_construction",
    "execution",
    "external_startup",
    "retained_artifacts",
    "retries",
}

MIRRORED_OPENING_METRICS = {
    ("compilation", "workspace_cargo_targets"): ("topology", "workspace_cargo_targets"),
    ("compilation", "compile_contract_cargo_sessions"): (
        "topology",
        "compile_contract_cargo_sessions",
    ),
    ("linking", "integration_test_targets"): ("topology", "integration_test_targets"),
    ("execution", "application_contract_cases"): (
        "measurements",
        "warm_application_contracts",
        "executed_tests_per_sample",
    ),
    ("external_startup", "filesystem_cases"): (
        "measurements",
        "warm_filesystem_external_boundary",
        "executed_tests_per_sample",
    ),
    ("retained_artifacts", "files"): ("retained_artifacts", "valid_measurement_target_files"),
    ("retained_artifacts", "bytes"): ("retained_artifacts", "valid_measurement_target_bytes"),
    ("retries", "budget"): ("topology", "flake_retry_budget"),
    ("retries", "used"): ("topology", "test_retries_used"),
}


def test_cost_evidence_violations(
    root: Path, config: dict[str, Any]
) -> list[Violation]:
    path = root / required_string(config, "test_cost_evidence")
    if not path.is_file():
        return [
            Violation(
                "test-cost-evidence",
                f"missing {path.relative_to(root).as_posix()}",
            )
        ]
    evidence = load_json(path)
    violations: list[Violation] = []
    if evidence.get("schema_version") != 2:
        violations.append(Violation("test-cost-evidence", "schema_version must be 2"))
    if evidence.get("milestone") != "3.10":
        violations.append(Violation("test-cost-evidence", "milestone must be 3.10"))
    opening = evidence.get("opening")
    if not isinstance(opening, dict):
        return [
            *violations,
            Violation("test-cost-evidence", "opening must be an object"),
        ]
    violations.extend(run_set_violations("opening", opening))
    violations.extend(mirrored_opening_metric_violations(opening))
    closing = evidence.get("closing")
    if closing is not None:
        if not isinstance(closing, dict):
            violations.append(
                Violation("test-cost-evidence", "closing must be null or an object")
            )
        else:
            violations.extend(run_set_violations("closing", closing))
            violations.extend(comparability_violations(opening, closing))
    return violations


def run_set_violations(label: str, run_set: dict[str, Any]) -> list[Violation]:
    layers = run_set.get("proof_cost_layers")
    if not isinstance(layers, dict):
        return [
            Violation(
                "test-cost-evidence",
                f"{label}.proof_cost_layers must be an object",
            )
        ]
    violations: list[Violation] = []
    for name in sorted(REQUIRED_COST_LAYERS - set(layers)):
        violations.append(
            Violation("test-cost-evidence", f"{label}: missing cost layer {name}")
        )
    for name in sorted(set(layers) - REQUIRED_COST_LAYERS):
        violations.append(
            Violation("test-cost-evidence", f"{label}: unexpected cost layer {name}")
        )
    for name in sorted(REQUIRED_COST_LAYERS & set(layers)):
        layer = layers[name]
        if not isinstance(layer, dict):
            violations.append(
                Violation("test-cost-evidence", f"{label}.{name} must be an object")
            )
            continue
        violations.extend(layer_violations(label, name, layer))
    return violations


def layer_violations(
    label: str, name: str, layer: dict[str, Any]
) -> list[Violation]:
    prefix = f"{label}.proof_cost_layers.{name}"
    violations: list[Violation] = []
    for field in ("owner", "posture"):
        if not isinstance(layer.get(field), str) or not layer[field].strip():
            violations.append(
                Violation("test-cost-evidence", f"{prefix}.{field} is missing")
            )
    evidence = layer.get("evidence")
    if not isinstance(evidence, list) or not evidence or not all(
        isinstance(item, str) and item.strip() for item in evidence
    ):
        violations.append(
            Violation(
                "test-cost-evidence",
                f"{prefix}.evidence must name at least one source",
            )
        )
    metrics = layer.get("metrics")
    if not isinstance(metrics, dict) or not metrics:
        violations.append(
            Violation("test-cost-evidence", f"{prefix}.metrics must be non-empty")
        )
    elif any(
        not isinstance(metric, str)
        or not metric
        or not isinstance(value, int)
        or isinstance(value, bool)
        or value < 0
        for metric, value in metrics.items()
    ):
        violations.append(
            Violation(
                "test-cost-evidence",
                f"{prefix}.metrics must contain non-negative integer values",
            )
        )
    return violations


def mirrored_opening_metric_violations(opening: dict[str, Any]) -> list[Violation]:
    violations: list[Violation] = []
    layers = opening.get("proof_cost_layers", {})
    for (layer, metric), source_path in MIRRORED_OPENING_METRICS.items():
        declared = nested_value(layers, layer, "metrics", metric)
        source = nested_value(opening, *source_path)
        if declared != source:
            violations.append(
                Violation(
                    "test-cost-evidence-consistency",
                    f"opening.{layer}.{metric} must mirror "
                    f"opening.{'.'.join(source_path)}",
                )
            )
    return violations


def comparability_violations(
    opening: dict[str, Any], closing: dict[str, Any]
) -> list[Violation]:
    opening_layers = opening.get("proof_cost_layers", {})
    closing_layers = closing.get("proof_cost_layers", {})
    if not isinstance(opening_layers, dict) or not isinstance(closing_layers, dict):
        return []
    violations: list[Violation] = []
    for name in sorted(REQUIRED_COST_LAYERS):
        opening_metrics = nested_value(opening_layers, name, "metrics")
        closing_metrics = nested_value(closing_layers, name, "metrics")
        if isinstance(opening_metrics, dict) and isinstance(closing_metrics, dict):
            if set(opening_metrics) != set(closing_metrics):
                violations.append(
                    Violation(
                        "test-cost-evidence-comparability",
                        f"closing.{name} metric names do not match opening",
                    )
                )
    return violations


def nested_value(value: Any, *path: str) -> Any:
    current = value
    for part in path:
        if not isinstance(current, dict):
            return None
        current = current.get(part)
    return current
