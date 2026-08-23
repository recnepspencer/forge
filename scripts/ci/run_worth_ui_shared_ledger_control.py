from __future__ import annotations

import hashlib
import json
import os
import secrets
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from run_worth_ui_ledger_test import write_artifact
from worth_ui_ledger_dependency import require_proved_artifact
from worth_ui_3141_ledger_contracts import COUNTERS, construction_cost, execution_cost
from worth_ui_ledger_command import (
    GovernedTest,
    ROOT,
    cargo_command,
    claim_digest,
    execution_budget_ms,
    parse_args,
    repository_path,
    source_digest,
    source_revision,
)
from worth_ui_ledger_execution_runner import timed_execution
from worth_ui_ledger_governed_snapshot import governed_sources_changed
from worth_ui_ledger_hostile_control_evidence import control_payload
from worth_ui_ledger_observation import p1_counter_observation, p2_counter_observation
from worth_ui_ledger_portfolio_snapshot import source_state_for_row


P1_WORLD = Path("_docs/worth-ui/milestone-3.14.1-evidence/p1-worlds-01.json")
P2_WORLD = Path("_docs/worth-ui/milestone-3.14.1-evidence/p2-world-01.json")
P3_WORLD = Path("_docs/worth-ui/milestone-3.14.1-evidence/p3-hp02-world-01.json")


@dataclass(frozen=True)
class GovernedSnapshot:
    revision: str
    source_digest: str
    source_state_digest: str
    claim_digest: str


@dataclass(frozen=True)
class SharedEvidence:
    artifact: dict[str, Any]
    artifact_digest: str
    control: dict[str, Any] | None
    observation: dict[str, Any] | None


@dataclass(frozen=True)
class RowVerdict:
    counter: str | None
    costs: tuple[str, str]
    posture: str


@dataclass(frozen=True)
class SharedWorldSpec:
    requirement: str
    path: Path
    package: str
    target_kind: str
    target_name: str
    features: tuple[str, ...]
    test_name: str
    control_required: bool
    shared_cost_name: str
    shared_presentations: int


def shared_world_spec(test: GovernedTest) -> SharedWorldSpec:
    if test.requirement == "P1-HEADLESS-COST-01":
        return SharedWorldSpec(
            "P1-WORLDS-01",
            P1_WORLD,
            "worth-ui-certification",
            "test",
            "application_contracts",
            (),
            "host_platform::maximum_overlap_removals_cross_public_runtime_and_headless_with_exact_work",
            False,
            "shared-mounted-worlds",
            7,
        )
    if test.requirement.startswith("P2-") and test.requirement != "P2-WORLD-01":
        return SharedWorldSpec(
            "P2-WORLD-01",
            P2_WORLD,
            "worth-ui-platform-pulse",
            "test",
            "executable_world",
            ("executable-world",),
            "courtroom::native_phase2::windows_native_boundary_world_presents_quiesces_and_closes_without_residue",
            True,
            "shared-native-worlds",
            1,
        )
    if test.requirement in {
        "P3-BASELINE-REPLAY-01", "P3-DAMAGE-REPLAY-01", "P3-DRAW-LIST-01",
        "P3-PHYSICAL-AMPLIFICATION-01",
        "P3-TRANSACTION-01", "P3-UNCHANGED-01",
    }:
        return SharedWorldSpec(
            "P3-HP02-WORLD-01",
            P3_WORLD,
            "worth-ui-platform-pulse",
            "test",
            "executable_world",
            ("executable-world",),
            "courtroom::native_phase3::maximum_overlap_deltas_cross_public_runtime_native_pixels_and_exact_costs",
            True,
            "shared-native-worlds",
            7,
        )
    if test.requirement in {"P3-HEADLESS-COST-01", "P3-PRODUCER-SLOPE-01"}:
        return SharedWorldSpec(
            "P3-DELTA-SOURCE-01",
            Path("_docs/worth-ui/milestone-3.14.1-evidence/p3-delta-source-01.json"),
            "worth-ui-certification",
            "test",
            "application_contracts",
            (),
            "host_platform::mixed_carrier_successors_are_local_at_the_4096_command_ceiling",
            True,
            "shared-mounted-worlds",
            5,
        )
    raise ValueError("requirement has no governed shared world")


def shared_world_path(test: GovernedTest, spec: SharedWorldSpec) -> Path:
    identity = Path(os.environ.get("WORTH_UI_SHARED_WORLD_ARTIFACT", spec.path.as_posix()))
    if identity.as_posix() not in test.sources:
        raise ValueError("shared native world is not bound as a row source")
    return repository_path(identity.as_posix())


def read_shared_world(
    test: GovernedTest, spec: SharedWorldSpec
) -> tuple[dict[str, Any], str]:
    path = shared_world_path(test, spec)
    raw = path.read_bytes()
    value = json.loads(raw)
    if not isinstance(value, dict):
        raise ValueError("shared native world artifact is not an object")
    validate_shared_world(value, spec)
    digest = require_proved_artifact(
        ROOT, spec.requirement, path.relative_to(ROOT).as_posix(), value
    )
    return value, digest


def validate_shared_world(value: dict[str, Any], spec: SharedWorldSpec) -> None:
    exact = cargo_command(
        GovernedTest(
            spec.requirement,
            spec.package,
            spec.target_kind,
            spec.target_name,
            spec.features,
            spec.test_name,
            (),
            "unused",
            None,
        ),
        False,
    )
    if value.get("schema_version") not in {5, 7}:
        raise ValueError("shared native world has wrong schema_version")
    required = {
        "requirement": spec.requirement,
        "exit_posture": "passed",
        "executed_test_count": 1,
        "passed_test_count": 1,
        "ignored_test_count": 0,
        "test_exit_code": 0,
        "test_command": exact,
    }
    for field, expected in required.items():
        if value.get(field) != expected:
            raise ValueError(f"shared native world has wrong {field}")
    if spec.requirement == "P2-WORLD-01" and not isinstance(value.get("boundary_observation"), dict):
        raise ValueError("shared native world omits its boundary observation")
    if value.get("source_revision") != source_revision():
        raise ValueError("shared native world revision is stale")
    if value.get("source_state_digest") != source_state_for_row(value["source_revision"]):
        raise ValueError("shared native world source state is stale")


def shared_costs(
    control: dict[str, Any] | None, spec: SharedWorldSpec
) -> tuple[str, str]:
    control_tests = 0 if control is None else control.get("executed_test_count")
    if control_tests != int(spec.control_required):
        raise ValueError("shared row has the wrong hostile-control count")
    construction = (
        f"main-tests=0;hostile-controls={control_tests};product-processes=0;compile-sessions=0;"
        f"courtroom-worlds=0;{spec.shared_cost_name}=1"
    )
    execution = (
        f"executed-tests={control_tests};presentations=0;"
        f"shared-presentations={spec.shared_presentations}"
    )
    return construction, execution


def validate_test(test: GovernedTest) -> None:
    spec = shared_world_spec(test)
    if (test.control is not None) != spec.control_required:
        raise ValueError("shared row has the wrong hostile-control posture")


def result_payload(test: GovernedTest) -> tuple[dict[str, Any], int]:
    validate_test(test)
    spec = shared_world_spec(test)
    revision = source_revision()
    snapshot = GovernedSnapshot(
        revision,
        source_digest(test.sources),
        source_state_for_row(revision),
        claim_digest(test.requirement),
    )
    shared, shared_digest = read_shared_world(test, spec)
    receipts = shared_main_receipts(shared)

    def execute(command: list[str], role: str):
        result, duration, receipt = timed_execution(
            command,
            ROOT,
            snapshot.revision,
            snapshot.source_state_digest,
            role,
        )
        receipts.append({"role": role, **receipt})
        return result, duration

    control = (
        control_payload(
            test.control,
            test.requirement,
            execute,
            snapshot.revision,
            snapshot.source_state_digest,
        )
        if test.control is not None
        else None
    )
    observation = shared.get("boundary_observation")
    counter = (
        p2_counter_observation(test, observation)
        if test.requirement.startswith("P2-")
        else p1_counter_observation(shared["test_stdout"], test.requirement)
    )
    costs = shared_costs(control, spec)
    posture = shared_row_posture(test, control, counter, costs)
    if governed_sources_changed(
        test,
        snapshot.revision,
        snapshot.source_digest,
        snapshot.source_state_digest,
        snapshot.claim_digest,
    ):
        posture = "source-changed"
    payload = payload_fields(
        test,
        SharedEvidence(shared, shared_digest, control, observation),
        snapshot,
        RowVerdict(counter, costs, posture),
    )
    payload["execution_receipts"] = receipts
    return payload, 0 if posture == "passed" else 1


def shared_main_receipts(shared: dict[str, Any]) -> list[dict[str, Any]]:
    allowed = {"main-discovery", "ignored-discovery", "main-test"}
    receipts = shared.get("execution_receipts", [])
    if not isinstance(receipts, list):
        raise ValueError("shared evidence has an invalid execution receipt inventory")
    return [
        dict(receipt)
        for receipt in receipts
        if isinstance(receipt, dict) and receipt.get("role") in allowed
    ]


def shared_row_posture(
    test: GovernedTest,
    control: dict[str, Any] | None,
    counter: str | None,
    costs: tuple[str, str],
) -> str:
    if control is not None and control.get("exit_posture") != "passed":
        return "control-failed"
    if counter != "{}={}".format(*COUNTERS[test.requirement]):
        return "structural-counter-mismatch"
    if costs != (construction_cost(test.requirement), execution_cost(test.requirement)):
        return "cost-observation-mismatch"
    return "passed"


def payload_fields(
    test: GovernedTest,
    shared: SharedEvidence,
    snapshot: GovernedSnapshot,
    verdict: RowVerdict,
) -> dict[str, Any]:
    world = shared.artifact
    return {
        "schema_version": 7,
        "requirement": test.requirement,
        "claim_digest": snapshot.claim_digest,
        "package": test.package,
        "target_kind": test.target_kind,
        "target_name": test.target_name,
        "features": list(test.features),
        "test_name": test.test_name,
        "matched_test_count": 1,
        "declared_ignored_test_count": world["declared_ignored_test_count"],
        "expected_declared_ignored": world["expected_declared_ignored"],
        "executed_test_count": 0,
        "passed_test_count": 0,
        "ignored_test_count": 0,
        "exit_posture": verdict.posture,
        "list_exit_code": 0,
        "test_exit_code": None,
        "list_duration_ms": world["list_duration_ms"],
        "ignored_list_duration_ms": world["ignored_list_duration_ms"],
        "test_duration_ms": 0,
        "test_budget_ms": execution_budget_ms(test.requirement),
        "source_revision": snapshot.revision,
        "source_digest": snapshot.source_digest,
        "source_state_digest": snapshot.source_state_digest,
        "run_nonce": secrets.token_hex(16),
        "source_identity": list(test.sources),
        "list_command": world["list_command"],
        "ignored_list_command": world["ignored_list_command"],
        "test_command": world["test_command"],
        "list_stdout": world["list_stdout"],
        "list_stderr": world["list_stderr"],
        "ignored_list_stdout": world["ignored_list_stdout"],
        "ignored_list_stderr": world["ignored_list_stderr"],
        "test_stdout": world["test_stdout"],
        "test_stderr": world["test_stderr"],
        "boundary_observation": shared.observation,
        "hostile_control": shared.control,
        "structural_counter": verdict.counter,
        "construction_cost": verdict.costs[0],
        "execution_cost": verdict.costs[1],
        "shared_main_artifact": shared_world_path(test, shared_world_spec(test)).relative_to(ROOT).as_posix(),
        "shared_main_artifact_digest": shared.artifact_digest,
        "shared_main_requirement": shared_world_spec(test).requirement,
    }


def main() -> int:
    try:
        test = parse_args()
        payload, exit_code = result_payload(test)
        artifact_digest = write_artifact(test.requirement, payload)
    except (OSError, RuntimeError, ValueError) as error:
        print(f"shared ledger control runner: {error}", file=sys.stderr)
        return 2
    print(json.dumps({"artifact_sha256": artifact_digest, **payload}, sort_keys=True))
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())
