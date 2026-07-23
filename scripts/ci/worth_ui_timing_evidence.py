import hashlib
import re
import statistics
import subprocess
from datetime import datetime
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

RFC3339_CAPTURE = re.compile(
    r"^(?P<base>\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2})"
    r"(?:\.(?P<fraction>\d{1,9}))?(?P<zone>Z|[+-]\d{2}:\d{2})$"
)


def timing_evidence_violations(
    root: Path, config: dict[str, Any]
) -> list[Violation]:
    path = root / required_string(config, "timing_evidence")
    if not path.is_file():
        return [Violation("timing-evidence", f"missing {path.relative_to(root).as_posix()}")]
    evidence = load_json(path)
    violations: list[Violation] = []
    if evidence.get("schema_version") != 2:
        violations.append(Violation("timing-evidence", "schema_version must be 2"))
    if evidence.get("milestone") != "3.9":
        violations.append(Violation("timing-evidence", "milestone must be 3.9"))
    opening = evidence.get("opening")
    if not isinstance(opening, dict):
        return [*violations, Violation("timing-evidence", "opening must be an object")]
    violations.extend(run_set_violations(root, "opening", opening))
    closing = evidence.get("closing")
    if closing is not None:
        if not isinstance(closing, dict):
            violations.append(Violation("timing-evidence", "closing must be null or an object"))
        else:
            violations.extend(run_set_violations(root, "closing", closing))
            violations.extend(capture_order_violations(opening, closing))
            comparability = comparability_violations(opening, closing)
            violations.extend(comparability)
            if not comparability:
                violations.extend(closing_budget_violations(opening, closing))
            violations.extend(source_transition_violations(opening, closing))
    return violations


def run_set_violations(root: Path, label: str, run_set: dict[str, Any]) -> list[Violation]:
    violations: list[Violation] = []
    for field in ("captured_at", "git_commit", "platform", "cargo", "rustc"):
        if not isinstance(run_set.get(field), str) or not run_set[field]:
            violations.append(Violation("timing-evidence", f"{label}.{field} is missing"))
    captured_at = run_set.get("captured_at")
    if isinstance(captured_at, str) and captured_at and parse_rfc3339(captured_at) is None:
        violations.append(
            Violation(
                "timing-evidence-capture",
                f"{label}.captured_at must be a valid RFC3339 timestamp",
            )
        )
    if not isinstance(run_set.get("cargo_incremental"), bool):
        violations.append(
            Violation("timing-evidence", f"{label}.cargo_incremental must be boolean")
        )
    cache = run_set.get("compiler_cache")
    if not isinstance(cache, str) or not cache:
        violations.append(Violation("timing-evidence", f"{label}.compiler_cache is missing"))
    violations.extend(source_snapshot_violations(root, label, run_set))
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


def source_snapshot_violations(
    root: Path, label: str, run_set: dict[str, Any]
) -> list[Violation]:
    snapshot = run_set.get("source_snapshot")
    if not isinstance(snapshot, dict):
        return [Violation("timing-evidence-source", f"{label}.source_snapshot is missing")]
    if snapshot.get("algorithm") != "sha256-path-and-git-blob-v1":
        return [
            Violation(
                "timing-evidence-source",
                f"{label}.source_snapshot.algorithm is unsupported",
            )
        ]
    scope = snapshot.get("scope")
    if not isinstance(scope, list) or not scope or not all(
        isinstance(path, str) and path for path in scope
    ):
        return [
            Violation("timing-evidence-source", f"{label}.source_snapshot.scope is invalid")
        ]
    kind = snapshot.get("kind")
    try:
        if kind == "working_tree":
            digest, file_count = filesystem_source_digest(root, scope)
        elif kind == "git_commit":
            digest, file_count = git_commit_source_digest(
                root, required_string(run_set, "git_commit"), scope
            )
        else:
            return [
                Violation(
                    "timing-evidence-source",
                    f"{label}.source_snapshot.kind must be working_tree or git_commit",
                )
            ]
    except (OSError, subprocess.CalledProcessError, ValueError) as error:
        return [
            Violation(
                "timing-evidence-source",
                f"{label}.source_snapshot could not be verified: {error}",
            )
        ]
    violations = []
    if snapshot.get("digest") != digest:
        violations.append(
            Violation(
                "timing-evidence-source",
                f"{label}.source_snapshot.digest does not match the declared source bytes",
            )
        )
    if snapshot.get("file_count") != file_count:
        violations.append(
            Violation(
                "timing-evidence-source",
                f"{label}.source_snapshot.file_count must be {file_count}",
            )
        )
    return violations


def filesystem_source_digest(root: Path, scope: list[str]) -> tuple[str, int]:
    files: set[Path] = set()
    for raw in scope:
        scoped = root / raw
        if scoped.is_file():
            files.add(scoped)
        elif scoped.is_dir():
            files.update(
                path
                for path in scoped.rglob("*")
                if path.is_file()
                and not any(part in {".git", "target", "__pycache__"} for part in path.parts)
            )
        else:
            raise ValueError(f"source scope does not exist: {raw}")
    entries = []
    for path in sorted(files):
        relative = path.relative_to(root).as_posix()
        entries.append((relative, git_blob_digest(path.read_bytes())))
    return aggregate_source_entries(entries), len(entries)


def git_commit_source_digest(
    root: Path, commit: str, scope: list[str]
) -> tuple[str, int]:
    command = ["git", "ls-tree", "-r", "-z", commit, "--", *scope]
    result = subprocess.run(command, cwd=root, check=True, capture_output=True)
    entries = []
    for raw in result.stdout.split(b"\0"):
        if not raw:
            continue
        metadata, raw_path = raw.split(b"\t", 1)
        _, object_type, object_id = metadata.split(b" ", 2)
        if object_type != b"blob":
            continue
        entries.append((raw_path.decode("utf-8"), object_id.decode("ascii")))
    return aggregate_source_entries(entries), len(entries)


def git_blob_digest(data: bytes) -> str:
    header = f"blob {len(data)}\0".encode("ascii")
    return hashlib.sha1(header + data).hexdigest()


def aggregate_source_entries(entries: list[tuple[str, str]]) -> str:
    digest = hashlib.sha256()
    for path, object_id in sorted(entries):
        digest.update(path.encode("utf-8"))
        digest.update(b"\0")
        digest.update(object_id.encode("ascii"))
        digest.update(b"\n")
    return digest.hexdigest()


def source_transition_digest(opening_digest: str, closing_digest: str) -> str:
    return hashlib.sha256(
        opening_digest.encode("ascii") + b"\0" + closing_digest.encode("ascii")
    ).hexdigest()


def source_transition_violations(opening, closing) -> list[Violation]:
    opening_snapshot = opening.get("source_snapshot", {})
    closing_snapshot = closing.get("source_snapshot", {})
    if not isinstance(opening_snapshot, dict) or not isinstance(closing_snapshot, dict):
        return []
    opening_digest = opening_snapshot.get("digest")
    closing_digest = closing_snapshot.get("digest")
    if not isinstance(opening_digest, str) or not isinstance(closing_digest, str):
        return []
    expected = source_transition_digest(opening_digest, closing_digest)
    if closing.get("source_transition_digest") != expected:
        return [
            Violation(
                "timing-evidence-source",
                "closing.source_transition_digest does not bind opening and closing source trees",
            )
        ]
    return []


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


def parse_rfc3339(value: str) -> tuple[datetime, int] | None:
    matched = RFC3339_CAPTURE.fullmatch(value)
    if matched is None:
        return None
    try:
        zone = "+00:00" if matched.group("zone") == "Z" else matched.group("zone")
        seconds = datetime.fromisoformat(f"{matched.group('base')}{zone}")
        nanoseconds = int((matched.group("fraction") or "0").ljust(9, "0"))
        return seconds, nanoseconds
    except ValueError:
        return None


def capture_order_violations(opening, closing) -> list[Violation]:
    opening_capture = opening.get("captured_at")
    closing_capture = closing.get("captured_at")
    if not isinstance(opening_capture, str) or not isinstance(closing_capture, str):
        return []
    opening_time = parse_rfc3339(opening_capture)
    closing_time = parse_rfc3339(closing_capture)
    if opening_time is None or closing_time is None:
        return []
    if closing_time <= opening_time:
        return [
            Violation(
                "timing-evidence-capture",
                "closing.captured_at must be later than opening.captured_at",
            )
        ]
    return []


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
