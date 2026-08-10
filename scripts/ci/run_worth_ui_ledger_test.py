from __future__ import annotations

import argparse
import csv
import hashlib
import json
import os
import re
import secrets
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from worth_ui_ledger_source_state import source_state_digest
from worth_ui_3141_ledger_contracts import (
    COUNTERS,
    EXPECTED_IGNORED,
    construction_cost,
    execution_cost,
)


ROOT = Path(__file__).resolve().parents[2]
MANIFEST = Path("workspaces/worth-ui/Cargo.toml")
LEDGER = Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv")
EVIDENCE_ROOT = Path("_docs/worth-ui/milestone-3.14.1-evidence")
PACKAGES = {
    "worth-ui-certification",
    "worth-ui-host-contract",
    "worth-ui-host-egui",
    "worth-ui-host-headless",
    "worth-ui-host-native",
    "worth-ui-native-platform",
    "worth-ui-platform-pulse",
    "worth-ui-runtime",
}
CLAIM_FIELDS = (
    "phase", "requirement", "owner", "production_boundary", "world_identity",
    "world_version", "proof_kind", "evidence_schema", "baseline_digest",
    "scenario_delta", "generated_seed", "authority_provenance", "production_entry",
    "independent_oracle", "mutation_control", "fault_injection_boundary",
    "retained_failure_artifact", "teardown_result", "construction_cost",
    "execution_cost", "source_identity", "font_profile_identity", "font_profile_digest",
    "native_profile_identity", "native_profile_digest", "platform_versions",
    "structural_counters", "presented_source_readback", "client_area_observation",
)


@dataclass(frozen=True)
class ControlTest:
    package: str
    target_kind: str
    target_name: str
    test_name: str
    features: tuple[str, ...]


@dataclass(frozen=True)
class GovernedTest:
    requirement: str
    package: str
    target_kind: str
    target_name: str
    features: tuple[str, ...]
    test_name: str
    sources: tuple[str, ...]
    artifact: str
    control: ControlTest | None


def parse_args() -> GovernedTest:
    parser = argparse.ArgumentParser(description="Run one ledger-bound Worth UI test")
    parser.add_argument("--manifest-path", required=True)
    parser.add_argument("--package", required=True, choices=sorted(PACKAGES))
    target = parser.add_mutually_exclusive_group(required=True)
    target.add_argument("--lib", action="store_true")
    target.add_argument("--test")
    parser.add_argument("--features", action="append", default=[])
    parser.add_argument("--test-name", required=True)
    parser.add_argument("--control-package", choices=sorted(PACKAGES))
    control_target = parser.add_mutually_exclusive_group()
    control_target.add_argument("--control-lib", action="store_true")
    control_target.add_argument("--control-test")
    parser.add_argument("--control-features", action="append", default=[])
    parser.add_argument("--control-test-name")
    parser.add_argument("--requirement", required=True)
    parser.add_argument("--source", action="append", required=True)
    parser.add_argument("--artifact", required=True)
    arguments = parser.parse_args()
    if Path(arguments.manifest_path).as_posix() != MANIFEST.as_posix():
        parser.error(f"manifest must be {MANIFEST.as_posix()}")
    if "::" not in arguments.test_name:
        parser.error("test name must be fully qualified")
    target_kind = "lib" if arguments.lib else "test"
    target_name = "lib" if arguments.lib else arguments.test
    features = tuple(arguments.features)
    if features and (arguments.package, features) != (
        "worth-ui-platform-pulse", ("executable-world",)
    ):
        parser.error("only the governed Platform Pulse executable-world feature is allowed")
    control_values = (
        arguments.control_package,
        arguments.control_lib or bool(arguments.control_test),
        arguments.control_test_name,
    )
    if any(control_values) != all(control_values):
        parser.error("control package, target, and exact test name must be supplied together")
    control = None
    if all(control_values):
        control_features = tuple(arguments.control_features)
        if control_features and (arguments.control_package, control_features) != (
            "worth-ui-platform-pulse", ("executable-world",)
        ):
            parser.error("only the governed control executable-world feature is allowed")
        control = ControlTest(
            arguments.control_package,
            "lib" if arguments.control_lib else "test",
            "lib" if arguments.control_lib else arguments.control_test,
            arguments.control_test_name,
            control_features,
        )
    requires_control = arguments.requirement.startswith("P2-") or arguments.requirement == "P1-CONSUMERS-01"
    if requires_control != (control is not None):
        parser.error("this requirement has the wrong hostile-control posture")
    return GovernedTest(
        arguments.requirement,
        arguments.package,
        target_kind,
        target_name,
        features,
        arguments.test_name,
        tuple(arguments.source),
        arguments.artifact,
        control,
    )


def repository_path(value: str) -> Path:
    candidate = (ROOT / value).resolve()
    try:
        candidate.relative_to(ROOT)
    except ValueError as error:
        raise ValueError(f"path escapes repository: {value}") from error
    return candidate


def source_digest(sources: tuple[str, ...]) -> str:
    if len(sources) != len(set(sources)):
        raise ValueError("source identities must be unique")
    digest = hashlib.sha256()
    for identity in sorted(sources):
        source = repository_path(identity)
        if not source.is_file():
            raise ValueError(f"source does not exist: {identity}")
        digest.update(identity.encode("utf-8"))
        digest.update(b"\0")
        digest.update(source.read_bytes())
        digest.update(b"\0")
    return digest.hexdigest()


def source_revision() -> str:
    result = subprocess.run(
        ["git", "rev-parse", "--verify", "HEAD"],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    revision = result.stdout.strip()
    if result.returncode != 0 or len(revision) != 40:
        raise RuntimeError("cannot resolve the governed source revision")
    return revision


def claim_digest(requirement: str) -> str:
    with repository_path(LEDGER.as_posix()).open(encoding="utf-8", newline="") as source:
        matches = [row for row in csv.DictReader(source) if row["requirement"] == requirement]
    if len(matches) != 1:
        raise ValueError(f"ledger requirement must occur exactly once: {requirement}")
    digest = hashlib.sha256()
    for field in CLAIM_FIELDS:
        digest.update(field.encode("utf-8"))
        digest.update(b"\0")
        digest.update(matches[0][field].encode("utf-8"))
        digest.update(b"\0")
    return digest.hexdigest()


def cargo_command(test: GovernedTest, list_only: bool) -> list[str]:
    command = [
        "cargo",
        "test",
        "--manifest-path",
        MANIFEST.as_posix(),
        "-p",
        test.package,
    ]
    command.extend(["--lib"] if test.target_kind == "lib" else ["--test", test.target_name])
    for feature in test.features:
        command.extend(["--features", feature])
    if list_only:
        command.extend(["--", "--list", "--format", "terse"])
    else:
        command.extend(
            [test.test_name, "--", "--exact", "--include-ignored", "--nocapture"]
        )
    return command


def ignored_list_command(test: GovernedTest) -> list[str]:
    command = cargo_command(test, True)
    command[-4:] = ["--", "--ignored", "--list", "--format", "terse"]
    return command


def control_cargo_command(test: ControlTest, list_only: bool) -> list[str]:
    command = [
        "cargo", "test", "--manifest-path", MANIFEST.as_posix(), "-p", test.package,
    ]
    command.extend(["--lib"] if test.target_kind == "lib" else ["--test", test.target_name])
    for feature in test.features:
        command.extend(["--features", feature])
    if list_only:
        command.extend(["--", "--list", "--format", "terse"])
    else:
        command.extend([test.test_name, "--", "--exact", "--nocapture"])
    return command


def listed_test_names(output: str) -> list[str]:
    names = []
    for line in output.splitlines():
        name, separator, kind = line.rpartition(": ")
        if separator and kind == "test":
            names.append(name)
    return names


def run(command: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        command,
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
        timeout=300,
    )


def timed_run(command: list[str]) -> tuple[subprocess.CompletedProcess[str], int]:
    started = time.perf_counter_ns()
    result = run(command)
    elapsed = time.perf_counter_ns() - started
    return result, max(1, (elapsed + 999_999) // 1_000_000)


def execution_budget_ms(requirement: str) -> int:
    if requirement.startswith("P2-"):
        return 30_000
    return 60_000


def execution_counts(output: str) -> tuple[int, int, int]:
    summaries = re.findall(
        r"test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;",
        output,
    )
    if len(summaries) != 1:
        return (0, 0, 0)
    passed, failed, ignored = (int(value) for value in summaries[0])
    return (passed + failed, passed, ignored)


def boundary_observation(output: str) -> dict[str, Any] | None:
    prefix = "WORTH_UI_LEDGER_OBSERVATION="
    matches = [line[len(prefix):] for line in output.splitlines() if line.startswith(prefix)]
    if not matches:
        return None
    if len(matches) != 1:
        raise ValueError("governed test emitted multiple boundary observations")
    value = json.loads(matches[0])
    if not isinstance(value, dict):
        raise ValueError("governed boundary observation is not an object")
    return value


def p1_counter_observation(output: str, requirement: str) -> str | None:
    prefix = "WORTH_UI_LEDGER_COUNTERS="
    matches = [line[len(prefix):] for line in output.splitlines() if line.startswith(prefix)]
    if len(matches) != 1:
        return None
    values = json.loads(matches[0])
    value = values.get(requirement) if isinstance(values, dict) else None
    family = COUNTERS[requirement][0]
    return f"{family}={value}" if isinstance(value, int) and value >= 0 else None


def p2_counter_observation(
    test: GovernedTest, observation: dict[str, Any] | None
) -> str | None:
    if observation is None:
        return None
    paths = {
        "P2-APPLICATION-01": ("peak", "application_drivers"),
        "P2-EVENT-LOOP-01": ("graphics", "event_loop_thread_matches_launch"),
        "P2-GRAPHICS-01": ("peak", "devices"),
        "P2-PRESENT-01": ("counters", "presents"),
        "P2-READINESS-01": ("counters", "readiness_signals"),
        "P2-WINDOW-01": ("peak", "windows"),
    }
    if test.requirement == "P2-CLOSE-01":
        census = observation.get("terminal_census", {})
        value = (
            sum(census.values())
            if census and all(isinstance(item, int) for item in census.values())
            else None
        )
    elif test.requirement == "P2-PIXELS-01":
        value = len(observation.get("client_control_points", []))
    elif test.requirement == "P2-PORTS-01":
        value = native_port_crossings(observation)
    elif test.requirement == "P2-WORLD-01":
        value = int(bool(observation.get("terminal_zero")))
    else:
        value: Any = observation
        for field in paths.get(test.requirement, ()):
            value = value.get(field) if isinstance(value, dict) else None
        if isinstance(value, bool):
            value = int(value)
    family = COUNTERS[test.requirement][0]
    return f"{family}={value}" if isinstance(value, int) and value >= 0 else None


def native_port_crossings(observation: dict[str, Any]) -> int:
    value = observation.get("counters", {}).get("port_crossings")
    return value if isinstance(value, int) and value >= 0 else -1


def observed_costs(
    test: GovernedTest,
    execution: subprocess.CompletedProcess[str] | None,
    control: dict[str, Any] | None,
    observation: dict[str, Any] | None,
) -> tuple[str, str] | None:
    if execution is None:
        return None
    p2 = test.requirement.startswith("P2-")
    compile_sessions = compile_sessions_observed(test.sources)
    world_count = int(p2 or "WORTH_UI_LEDGER_WORLD=1" in execution.stdout)
    product_processes = observation.get("product_processes") if p2 and observation else 0
    control_tests = 0 if control is None else control.get("executed_test_count")
    presentations = (
        observation.get("counters", {}).get("presents")
        if p2 and observation
        else stdout_numeric(execution.stdout, "WORTH_UI_LEDGER_PRESENTATIONS=", 0)
    )
    if not all(isinstance(value, int) and value >= 0 for value in [
        compile_sessions, product_processes, control_tests, presentations,
    ]):
        return None
    construction = (
        f"main-tests=1;hostile-controls={control_tests};product-processes={product_processes};"
        f"compile-sessions={compile_sessions};courtroom-worlds={world_count}"
    )
    return construction, f"executed-tests={1 + control_tests};presentations={presentations}"


def compile_sessions_observed(sources: tuple[str, ...]) -> int:
    identity = next((source for source in sources if source.endswith("compile-contracts.json")), None)
    if identity is None:
        return 0
    value = json.loads(repository_path(identity).read_text(encoding="utf-8"))
    sessions = value.get("cargo_sessions")
    return sessions if isinstance(sessions, int) and sessions >= 0 else -1


def stdout_numeric(output: str, prefix: str, default: int) -> int:
    values = [line[len(prefix):] for line in output.splitlines() if line.startswith(prefix)]
    if not values:
        return default
    return int(values[0]) if len(values) == 1 and values[0].isdigit() else -1


def result_payload(test: GovernedTest) -> tuple[dict[str, Any], int]:
    list_command = cargo_command(test, True)
    ignored_command = ignored_list_command(test)
    test_command = cargo_command(test, False)
    revision = source_revision()
    digest = source_digest(test.sources)
    state_digest = source_state_digest(revision)
    governed_claim_digest = claim_digest(test.requirement)
    run_nonce = secrets.token_hex(16)
    discovery, discovery_duration_ms = timed_run(list_command)
    ignored_discovery, ignored_discovery_duration_ms = timed_run(ignored_command)
    matches = listed_test_names(discovery.stdout).count(test.test_name)
    ignored_matches = listed_test_names(ignored_discovery.stdout).count(test.test_name)
    execution = None
    executed = passed = ignored = 0
    posture = "list-failed" if discovery.returncode else "match-count-rejected"
    if discovery.returncode == 0 and matches == 1:
        execution, test_duration_ms = timed_run(test_command)
        executed, passed, ignored = execution_counts(execution.stdout)
        posture = (
            "passed"
            if execution.returncode == 0
            and executed == 1
            and passed == 1
            and ignored == 0
            else "test-failed"
        )
    declared_ignored = ignored_matches == 1
    if (
        ignored_discovery.returncode != 0
        or ignored_matches not in (0, 1)
        or declared_ignored != EXPECTED_IGNORED[test.requirement]
    ):
        posture = "declared-ignore-mismatch"
    control = control_payload(test.control)
    if execution is not None and test_duration_ms > execution_budget_ms(test.requirement):
        posture = "execution-budget-exceeded"
    if governed_sources_changed(
        test, revision, digest, state_digest, governed_claim_digest
    ):
        posture = "source-changed"
    observation = None if execution is None else boundary_observation(execution.stdout)
    observed_counter = None
    if execution is not None:
        if test.requirement == "P1-CONSUMERS-01":
            control_count = 0 if control is None else control.get("executed_test_count", 0)
            observed_counter = f"consumer={executed + control_count}"
        else:
            observed_counter = (
                p2_counter_observation(test, observation)
                if test.requirement.startswith("P2-")
                else p1_counter_observation(execution.stdout, test.requirement)
            )
    expected_counter = "{}={}".format(*COUNTERS[test.requirement])
    if observed_counter != expected_counter:
        posture = "structural-counter-mismatch"
    costs = observed_costs(test, execution, control, observation)
    if costs != (construction_cost(test.requirement), execution_cost(test.requirement)):
        posture = "cost-observation-mismatch"
    if control is not None and control["exit_posture"] != "passed":
        posture = "control-failed"
    payload = {
        "schema_version": 5,
        "requirement": test.requirement,
        "claim_digest": governed_claim_digest,
        "package": test.package,
        "target_kind": test.target_kind,
        "target_name": test.target_name,
        "features": list(test.features),
        "test_name": test.test_name,
        "matched_test_count": matches,
        "declared_ignored_test_count": ignored_matches,
        "expected_declared_ignored": EXPECTED_IGNORED[test.requirement],
        "executed_test_count": executed,
        "passed_test_count": passed,
        "ignored_test_count": ignored,
        "exit_posture": posture,
        "list_exit_code": discovery.returncode,
        "test_exit_code": None if execution is None else execution.returncode,
        "list_duration_ms": discovery_duration_ms,
        "ignored_list_duration_ms": ignored_discovery_duration_ms,
        "test_duration_ms": None if execution is None else test_duration_ms,
        "test_budget_ms": execution_budget_ms(test.requirement),
        "source_revision": revision,
        "source_digest": digest,
        "source_state_digest": state_digest,
        "run_nonce": run_nonce,
        "source_identity": list(test.sources),
        "list_command": list_command,
        "ignored_list_command": ignored_command,
        "test_command": test_command,
        "list_stdout": discovery.stdout,
        "list_stderr": discovery.stderr,
        "ignored_list_stdout": ignored_discovery.stdout,
        "ignored_list_stderr": ignored_discovery.stderr,
        "test_stdout": "" if execution is None else execution.stdout,
        "test_stderr": "" if execution is None else execution.stderr,
        "boundary_observation": observation,
        "hostile_control": control,
        "structural_counter": observed_counter,
        "construction_cost": None if costs is None else costs[0],
        "execution_cost": None if costs is None else costs[1],
    }
    return payload, 0 if posture == "passed" else 1


def governed_sources_changed(
    test: GovernedTest,
    revision: str,
    digest: str,
    state_digest: str,
    governed_claim_digest: str,
) -> bool:
    before = (revision, digest, state_digest, governed_claim_digest)
    after = (
        source_revision(),
        source_digest(test.sources),
        source_state_digest(revision),
        claim_digest(test.requirement),
    )
    return governed_snapshot_changed(before, after)


def governed_snapshot_changed(before: tuple[str, ...], after: tuple[str, ...]) -> bool:
    return before != after


def control_payload(test: ControlTest | None) -> dict[str, Any] | None:
    if test is None:
        return None
    list_command = control_cargo_command(test, True)
    test_command = control_cargo_command(test, False)
    discovery, discovery_duration_ms = timed_run(list_command)
    matches = listed_test_names(discovery.stdout).count(test.test_name)
    execution = None
    executed = passed = ignored = 0
    posture = "list-failed" if discovery.returncode else "match-count-rejected"
    if discovery.returncode == 0 and matches == 1:
        execution, test_duration_ms = timed_run(test_command)
        executed, passed, ignored = execution_counts(execution.stdout)
        if execution.returncode == 0 and (executed, passed, ignored) == (1, 1, 0):
            posture = "passed"
        else:
            posture = "test-failed"
        if test_duration_ms > 10_000:
            posture = "execution-budget-exceeded"
    return {
        "package": test.package,
        "target_kind": test.target_kind,
        "target_name": test.target_name,
        "test_name": test.test_name,
        "features": list(test.features),
        "matched_test_count": matches,
        "executed_test_count": executed,
        "passed_test_count": passed,
        "ignored_test_count": ignored,
        "exit_posture": posture,
        "list_exit_code": discovery.returncode,
        "test_exit_code": None if execution is None else execution.returncode,
        "list_duration_ms": discovery_duration_ms,
        "test_duration_ms": None if execution is None else test_duration_ms,
        "test_budget_ms": 10_000,
        "list_command": list_command,
        "test_command": test_command,
        "list_stdout": discovery.stdout,
        "list_stderr": discovery.stderr,
        "test_stdout": "" if execution is None else execution.stdout,
        "test_stderr": "" if execution is None else execution.stderr,
    }


def write_artifact(identity: str, payload: dict[str, Any]) -> str:
    destination = repository_path(identity)
    destination.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary = tempfile.mkstemp(
        prefix=f".{destination.name}.", dir=destination.parent
    )
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8") as output:
            json.dump(payload, output, indent=2)
            output.write("\n")
        os.replace(temporary, destination)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)
    return hashlib.sha256(destination.read_bytes()).hexdigest()


def main() -> int:
    try:
        governed_test = parse_args()
        payload, exit_code = result_payload(governed_test)
        artifact_digest = write_artifact(governed_test.artifact, payload)
    except (OSError, RuntimeError, ValueError, subprocess.TimeoutExpired) as error:
        print(f"ledger evidence runner: {error}", file=sys.stderr)
        return 2
    print(json.dumps({"artifact_sha256": artifact_digest, **payload}, sort_keys=True))
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())
