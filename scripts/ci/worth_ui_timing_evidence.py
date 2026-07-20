import statistics
from pathlib import Path
from typing import Any

from worth_ui_test_topology_config import Violation, load_json, required_string


REQUIRED_MEASUREMENTS = {
    "targeted_warm_ordinary": "warm",
    "warm_fast_lane": "warm",
    "warm_application_contracts": "warm",
    "cold_compile_contracts": "cold",
    "warm_compile_contracts": "warm",
}


def timing_evidence_violations(
    root: Path, config: dict[str, Any]
) -> list[Violation]:
    path = root / required_string(config, "timing_evidence")
    if not path.is_file():
        return [Violation("timing-evidence", f"missing {path.relative_to(root).as_posix()}")]
    evidence = load_json(path)
    violations: list[Violation] = []
    if evidence.get("schema_version") != 1:
        violations.append(Violation("timing-evidence", "schema_version must be 1"))
    if evidence.get("milestone") != "3.9":
        violations.append(Violation("timing-evidence", "milestone must be 3.9"))
    opening = evidence.get("opening")
    if not isinstance(opening, dict):
        return [*violations, Violation("timing-evidence", "opening must be an object")]
    violations.extend(run_set_violations("opening", opening))
    closing = evidence.get("closing")
    if closing is not None:
        if not isinstance(closing, dict):
            violations.append(Violation("timing-evidence", "closing must be null or an object"))
        else:
            violations.extend(run_set_violations("closing", closing))
            comparability = comparability_violations(opening, closing)
            violations.extend(comparability)
            if not comparability:
                violations.extend(closing_budget_violations(opening, closing))
    return violations


def run_set_violations(label: str, run_set: dict[str, Any]) -> list[Violation]:
    violations: list[Violation] = []
    for field in ("captured_at", "git_commit", "platform", "cargo", "rustc"):
        if not isinstance(run_set.get(field), str) or not run_set[field]:
            violations.append(Violation("timing-evidence", f"{label}.{field} is missing"))
    if not isinstance(run_set.get("cargo_incremental"), bool):
        violations.append(
            Violation("timing-evidence", f"{label}.cargo_incremental must be boolean")
        )
    cache = run_set.get("compiler_cache")
    if not isinstance(cache, str) or not cache:
        violations.append(Violation("timing-evidence", f"{label}.compiler_cache is missing"))
    environment = run_set.get("environment")
    isolated_target_root = None
    if not isinstance(environment, dict):
        violations.append(Violation("timing-evidence", f"{label}.environment is missing"))
    else:
        for field in ("operating_system", "processor", "worktree_state"):
            if not isinstance(environment.get(field), str) or not environment[field]:
                violations.append(
                    Violation("timing-evidence", f"{label}.environment.{field} is missing")
                )
        memory = environment.get("physical_memory_bytes")
        if not isinstance(memory, int) or memory <= 0:
            violations.append(
                Violation(
                    "timing-evidence",
                    f"{label}.environment.physical_memory_bytes must be positive",
                )
            )
        isolated_target_root = environment.get("isolated_target_root")
        if not isinstance(isolated_target_root, str) or not isolated_target_root:
            violations.append(
                Violation(
                    "timing-evidence",
                    f"{label}.environment.isolated_target_root is missing",
                )
            )
            isolated_target_root = None
    measurements = run_set.get("measurements")
    if not isinstance(measurements, dict):
        return [*violations, Violation("timing-evidence", f"{label}.measurements is missing")]
    for name in sorted(set(measurements) - set(REQUIRED_MEASUREMENTS)):
        violations.append(Violation("timing-evidence", f"{label}: unexpected measurement {name}"))
    for name, classification in REQUIRED_MEASUREMENTS.items():
        measurement = measurements.get(name)
        if not isinstance(measurement, dict):
            violations.append(Violation("timing-evidence", f"{label}: missing measurement {name}"))
            continue
        violations.extend(
            measurement_violations(
                label,
                name,
                classification,
                measurement,
                isolated_target_root,
            )
        )
    return violations


def measurement_violations(
    label, name, classification, measurement, isolated_target_root
) -> list[Violation]:
    prefix = f"{label}.{name}"
    violations: list[Violation] = []
    if not isinstance(measurement.get("command"), str) or not measurement["command"]:
        violations.append(Violation("timing-evidence", f"{prefix}.command is missing"))
    if measurement.get("classification") != classification:
        violations.append(
            Violation(
                "timing-evidence", f"{prefix}.classification must be {classification}"
            )
        )
    samples = measurement.get("samples_seconds")
    targets = measurement.get("target_directories")
    if not valid_samples(samples):
        violations.append(
            Violation("timing-evidence", f"{prefix}.samples_seconds needs 3 positive samples")
        )
    if not isinstance(targets, list) or len(targets) != 3 or not all(
        isinstance(target, str) and target for target in targets
    ):
        violations.append(
            Violation("timing-evidence", f"{prefix}.target_directories needs 3 paths")
        )
    elif classification == "cold" and len(set(targets)) != 3:
        violations.append(
            Violation("timing-evidence", f"{prefix} cold samples need distinct target directories")
        )
    elif classification == "warm" and len(set(targets)) != 1:
        violations.append(
            Violation("timing-evidence", f"{prefix} warm samples need one shared target directory")
        )
    if isinstance(targets, list) and any(
        "workspaces/worth-ui/target" in target.replace("\\", "/") for target in targets
    ):
        violations.append(
            Violation("timing-evidence", f"{prefix} must use an isolated target directory")
        )
    if isinstance(targets, list) and isolated_target_root is not None:
        normalized_root = isolated_target_root.replace("\\", "/").rstrip("/") + "/"
        if any(
            not target.replace("\\", "/").startswith(normalized_root)
            for target in targets
            if isinstance(target, str)
        ):
            violations.append(
                Violation(
                    "timing-evidence",
                    f"{prefix} target directories must remain under isolated_target_root",
                )
            )
    median = measurement.get("median_seconds")
    if valid_samples(samples) and (
        not isinstance(median, (int, float))
        or abs(float(median) - statistics.median(samples)) > 0.001
    ):
        violations.append(Violation("timing-evidence", f"{prefix}.median_seconds is incorrect"))
    comparison = measurement.get("comparison")
    if label == "opening" and comparison != "opening_baseline":
        violations.append(
            Violation("timing-evidence", f"{prefix}.comparison must be opening_baseline")
        )
    elif label == "closing" and comparison not in {
        "within_10_percent",
        "reviewed_budget_amendment",
    }:
        violations.append(
            Violation("timing-evidence", f"{prefix}.comparison is not a closing result")
        )
    return violations


def valid_samples(samples: Any) -> bool:
    return (
        isinstance(samples, list)
        and len(samples) == 3
        and all(isinstance(sample, (int, float)) and sample > 0 for sample in samples)
    )


def comparability_violations(opening, closing) -> list[Violation]:
    violations: list[Violation] = []
    for field in ("platform", "cargo", "rustc", "cargo_incremental", "compiler_cache"):
        if opening.get(field) != closing.get(field):
            violations.append(
                Violation(
                    "timing-evidence-comparability",
                    f"closing.{field} does not match opening.{field}",
                )
            )
    opening_environment = opening.get("environment", {})
    closing_environment = closing.get("environment", {})
    if isinstance(opening_environment, dict) and isinstance(closing_environment, dict):
        for field in ("operating_system", "processor", "physical_memory_bytes"):
            if opening_environment.get(field) != closing_environment.get(field):
                violations.append(
                    Violation(
                        "timing-evidence-comparability",
                        f"closing.environment.{field} does not match opening",
                    )
                )
    opening_measurements = opening.get("measurements", {})
    closing_measurements = closing.get("measurements", {})
    if isinstance(opening_measurements, dict) and isinstance(closing_measurements, dict):
        for name in REQUIRED_MEASUREMENTS:
            opening_measurement = opening_measurements.get(name)
            closing_measurement = closing_measurements.get(name)
            if not isinstance(opening_measurement, dict) or not isinstance(
                closing_measurement, dict
            ):
                continue
            if opening_measurement.get("command") != closing_measurement.get("command"):
                violations.append(
                    Violation(
                        "timing-evidence-comparability",
                        f"closing.{name}.command does not match opening",
                    )
                )
    return violations


def closing_budget_violations(opening, closing) -> list[Violation]:
    violations: list[Violation] = []
    amendments = closing.get("reviewed_budget_amendments", {})
    if not isinstance(amendments, dict):
        return [
            Violation(
                "timing-evidence-budget",
                "closing.reviewed_budget_amendments must be an object",
            )
        ]
    opening_measurements = opening.get("measurements", {})
    closing_measurements = closing.get("measurements", {})
    for name in REQUIRED_MEASUREMENTS:
        opening_measurement = opening_measurements.get(name)
        closing_measurement = closing_measurements.get(name)
        if not isinstance(opening_measurement, dict) or not isinstance(
            closing_measurement, dict
        ):
            continue
        opening_median = opening_measurement.get("median_seconds")
        closing_median = closing_measurement.get("median_seconds")
        if not isinstance(opening_median, (int, float)) or not isinstance(
            closing_median, (int, float)
        ):
            continue
        regressed = closing_median > opening_median * 1.10
        reason = amendments.get(name)
        reviewed = isinstance(reason, str) and bool(reason.strip())
        expected = "reviewed_budget_amendment" if regressed and reviewed else "within_10_percent"
        if regressed and not reviewed:
            violations.append(
                Violation(
                    "timing-evidence-budget",
                    f"{name} regressed more than 10 percent without a reviewed amendment",
                )
            )
        if closing_measurement.get("comparison") != expected:
            violations.append(
                Violation(
                    "timing-evidence",
                    f"closing.{name}.comparison must be {expected}",
                )
            )
    for name in sorted(set(amendments) - set(REQUIRED_MEASUREMENTS)):
        violations.append(
            Violation("timing-evidence-budget", f"unexpected budget amendment {name}")
        )
    return violations
