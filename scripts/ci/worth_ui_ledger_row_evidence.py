from __future__ import annotations

import os
import secrets
import subprocess
import sys
from dataclasses import dataclass
from typing import Any, Callable

from worth_ui_3141_ledger_contracts import (
    COUNTERS,
    EXPECTED_IGNORED,
    construction_cost,
    execution_cost,
)
from worth_ui_3141_case_contracts import positive_cases
from worth_ui_3141_supporting_world import validate_supporting_dependency
from worth_ui_ledger_command import (
    ROOT,
    GovernedTest,
    cargo_command,
    execution_budget_ms,
    execution_counts,
    exact_test_duration_ms,
    ignored_list_command,
    listed_test_names,
)
from worth_ui_ledger_execution_cache import invalidate_receipts, timed_execution
from worth_ui_ledger_observation import (
    boundary_observation,
    governed_case_observation,
    observed_costs,
    p1_counter_observation,
    p2_counter_observation,
    predecessor_costs,
)
from worth_ui_ledger_public_example import execute_if_required
from worth_ui_ledger_governed_snapshot import (
    GovernedSnapshot,
    governed_snapshot,
    governed_sources_changed,
    refresh_handoff_when_required,
)
from worth_ui_ledger_hostile_control_evidence import control_payload


Execution = Callable[[list[str], str], tuple[subprocess.CompletedProcess[str], int]]


@dataclass
class MainExecution:
    list_command: list[str]
    ignored_list_command: list[str]
    test_command: list[str]
    discovery: subprocess.CompletedProcess[str]
    ignored_discovery: subprocess.CompletedProcess[str]
    execution: subprocess.CompletedProcess[str] | None
    list_duration_ms: int
    ignored_list_duration_ms: int
    test_duration_ms: int | None
    matches: int
    ignored_matches: int
    executed: int
    passed: int
    ignored: int
    posture: str


@dataclass(frozen=True)
class RowObservations:
    posture: str
    boundary: dict[str, Any] | None
    governed_cases: list[str] | None
    counter: str | None
    costs: tuple[str, str] | None
    predecessor_costs: tuple[str, str] | None


@dataclass(frozen=True)
class RowEvaluationInput:
    test: GovernedTest
    snapshot: GovernedSnapshot
    main: MainExecution
    control: dict[str, Any] | None
    supporting_world: dict[str, Any] | None
    public_example: dict[str, object] | None


@dataclass(frozen=True)
class PayloadInput:
    evaluation: RowEvaluationInput
    observations: RowObservations
    receipts: list[dict[str, Any]]


class ExecutionRecorder:
    def __init__(self, snapshot: GovernedSnapshot, requirement: str) -> None:
        self._snapshot = snapshot
        self._requirement = requirement
        self.receipts: list[dict[str, Any]] = []

    def execute(
        self, command: list[str], role: str
    ) -> tuple[subprocess.CompletedProcess[str], int]:
        result, duration, receipt = timed_execution(
            command,
            ROOT,
            self._snapshot.revision,
            self._snapshot.source_state_digest,
            role,
            self._requirement,
        )
        self.receipts.append({"role": role, **receipt})
        return result, duration


def result_payload(test: GovernedTest) -> tuple[dict[str, Any], int]:
    os.environ.setdefault("PYTHON", sys.executable)
    refresh_handoff_when_required(test)
    snapshot = governed_snapshot(test)
    recorder = ExecutionRecorder(snapshot, test.requirement)
    supporting_world = validate_supporting_dependency(
        test, snapshot.revision, snapshot.source_state_digest, ROOT
    )
    main = execute_main(test, recorder.execute)
    control = control_payload(test.control, test.requirement, recorder.execute)
    public_example = execute_if_required(test.requirement, recorder.execute)
    evaluation = RowEvaluationInput(
        test, snapshot, main, control, supporting_world, public_example
    )
    observations = evaluate_row(evaluation)
    payload = build_payload(PayloadInput(evaluation, observations, recorder.receipts))
    invalidate_over_budget_receipts(observations.posture, control, recorder.receipts)
    return payload, 0 if observations.posture == "passed" else 1


def execute_main(test: GovernedTest, execute: Execution) -> MainExecution:
    list_command = cargo_command(test, True)
    ignored_command = ignored_list_command(test)
    test_command = cargo_command(test, False)
    discovery, list_duration = execute(list_command, "main-discovery")
    ignored_discovery, ignored_duration = execute(ignored_command, "ignored-discovery")
    matches = listed_test_names(discovery.stdout).count(test.test_name)
    ignored_matches = listed_test_names(ignored_discovery.stdout).count(test.test_name)
    execution = None
    test_duration = None
    executed = passed = ignored = 0
    posture = "list-failed" if discovery.returncode else "match-count-rejected"
    if discovery.returncode == 0 and matches == 1:
        execution, command_duration = execute(test_command, "main-test")
        test_duration = exact_test_duration_ms(execution.stdout, command_duration)
        executed, passed, ignored = execution_counts(execution.stdout)
        posture = execution_posture(execution, executed, passed, ignored)
    return MainExecution(
        list_command, ignored_command, test_command, discovery, ignored_discovery,
        execution, list_duration, ignored_duration, test_duration, matches,
        ignored_matches, executed, passed, ignored, posture,
    )


def execution_posture(
    execution: subprocess.CompletedProcess[str], executed: int, passed: int, ignored: int
) -> str:
    return (
        "passed"
        if execution.returncode == 0
        and executed == 1
        and passed == 1
        and ignored == 0
        else "test-failed"
    )


def evaluate_row(evaluation: RowEvaluationInput) -> RowObservations:
    test = evaluation.test
    snapshot = evaluation.snapshot
    main = evaluation.main
    control = evaluation.control
    posture = main.posture
    posture = validate_declared_ignore(test, main, posture)
    posture = validate_main_budget(test, main, posture)
    if posture == "passed" and governed_sources_changed(
        test,
        snapshot.revision,
        snapshot.source_digest,
        snapshot.source_state_digest,
        snapshot.claim_digest,
    ):
        posture = "source-changed"
    boundary, cases = observe_main(test, main.execution)
    if (
        posture == "passed"
        and positive_cases(test.requirement) is not None
        and cases is None
    ):
        posture = "governed-case-mismatch"
    counter = observed_counter(test, main, control, boundary)
    if posture == "passed" and counter != "{}={}".format(*COUNTERS[test.requirement]):
        posture = "structural-counter-mismatch"
    if (
        posture == "passed"
        and evaluation.public_example is not None
        and evaluation.public_example["exit_code"] != 0
    ):
        posture = "public-example-failed"
    costs = observed_costs(
        test, main.execution, control, boundary, evaluation.supporting_world,
        evaluation.public_example,
    )
    if posture == "passed" and costs != (
        construction_cost(test.requirement), execution_cost(test.requirement)
    ):
        posture = "cost-observation-mismatch"
    if posture == "passed" and control is not None and control["exit_posture"] != "passed":
        posture = "control-failed"
    predecessor = (
        predecessor_costs(test, control)
        if test.requirement
        in {"P3-PREDECESSOR-01", "P4-PREDECESSOR-01", "P5-PREDECESSOR-01"}
        else None
    )
    return RowObservations(posture, boundary, cases, counter, costs, predecessor)


def validate_declared_ignore(test: GovernedTest, main: MainExecution, posture: str) -> str:
    declared_ignored = main.ignored_matches == 1
    if posture == "passed" and (
        main.ignored_discovery.returncode != 0
        or main.ignored_matches not in (0, 1)
        or declared_ignored != EXPECTED_IGNORED[test.requirement]
    ):
        return "declared-ignore-mismatch"
    return posture


def validate_main_budget(test: GovernedTest, main: MainExecution, posture: str) -> str:
    if (
        posture == "passed"
        and main.test_duration_ms is not None
        and main.test_duration_ms > execution_budget_ms(test.requirement)
    ):
        return "execution-budget-exceeded"
    return posture


def observe_main(
    test: GovernedTest, execution: subprocess.CompletedProcess[str] | None
) -> tuple[dict[str, Any] | None, list[str] | None]:
    if execution is None:
        return None, None
    return (
        boundary_observation(execution.stdout),
        governed_case_observation(execution.stdout, test.requirement),
    )


def observed_counter(
    test: GovernedTest,
    main: MainExecution,
    control: dict[str, Any] | None,
    observation: dict[str, Any] | None,
) -> str | None:
    if main.execution is None:
        return None
    if test.requirement == "P1-CONSUMERS-01":
        control_count = 0 if control is None else control.get("executed_test_count", 0)
        return f"consumer={main.executed + control_count}"
    counter = (
        p2_counter_observation(test, observation)
        if test.requirement.startswith("P2-")
        else p1_counter_observation(main.execution.stdout, test.requirement)
    )
    if test.requirement == "P3-HP02-WORLD-01" and counter == "worlds=1":
        return "worlds=2"
    return counter


def build_payload(payload: PayloadInput) -> dict[str, Any]:
    evaluation = payload.evaluation
    return {
        **identity_fields(evaluation.test, evaluation.snapshot),
        **execution_fields(
            evaluation.test, evaluation.main, payload.observations.posture
        ),
        **source_fields(evaluation.test, evaluation.snapshot),
        **command_output_fields(evaluation.main),
        **observation_fields(
            evaluation.control, evaluation.supporting_world, payload.observations
        ),
        **public_example_fields(evaluation.public_example),
        "execution_receipts": payload.receipts,
    }


def identity_fields(test: GovernedTest, snapshot: GovernedSnapshot) -> dict[str, Any]:
    return {
        "schema_version": 5,
        "requirement": test.requirement,
        "claim_digest": snapshot.claim_digest,
        "package": test.package,
        "target_kind": test.target_kind,
        "target_name": test.target_name,
        "features": list(test.features),
        "test_name": test.test_name,
    }


def execution_fields(
    test: GovernedTest, main: MainExecution, posture: str
) -> dict[str, Any]:
    return {
        "matched_test_count": main.matches,
        "declared_ignored_test_count": main.ignored_matches,
        "expected_declared_ignored": EXPECTED_IGNORED[test.requirement],
        "executed_test_count": main.executed,
        "passed_test_count": main.passed,
        "ignored_test_count": main.ignored,
        "exit_posture": posture,
        "list_exit_code": main.discovery.returncode,
        "test_exit_code": None if main.execution is None else main.execution.returncode,
        "list_duration_ms": main.list_duration_ms,
        "ignored_list_duration_ms": main.ignored_list_duration_ms,
        "test_duration_ms": main.test_duration_ms,
        "test_budget_ms": execution_budget_ms(test.requirement),
    }


def source_fields(test: GovernedTest, snapshot: GovernedSnapshot) -> dict[str, Any]:
    return {
        "source_revision": snapshot.revision,
        "source_digest": snapshot.source_digest,
        "source_state_digest": snapshot.source_state_digest,
        "run_nonce": secrets.token_hex(16),
        "source_identity": list(test.sources),
    }


def command_output_fields(main: MainExecution) -> dict[str, Any]:
    return {
        "list_command": main.list_command,
        "ignored_list_command": main.ignored_list_command,
        "test_command": main.test_command,
        "list_stdout": main.discovery.stdout,
        "list_stderr": main.discovery.stderr,
        "ignored_list_stdout": main.ignored_discovery.stdout,
        "ignored_list_stderr": main.ignored_discovery.stderr,
        "test_stdout": "" if main.execution is None else main.execution.stdout,
        "test_stderr": "" if main.execution is None else main.execution.stderr,
    }


def observation_fields(
    control: dict[str, Any] | None,
    supporting_world: dict[str, Any] | None,
    observations: RowObservations,
) -> dict[str, Any]:
    predecessor = observations.predecessor_costs
    return {
        "boundary_observation": observations.boundary,
        "governed_cases": observations.governed_cases,
        "hostile_control": control,
        "supporting_world": supporting_world,
        "structural_counter": observations.counter,
        "construction_cost": None if observations.costs is None else observations.costs[0],
        "execution_cost": None if observations.costs is None else observations.costs[1],
        "operational_predecessor_cost": None if predecessor is None else {
            "construction_cost": predecessor[0],
            "execution_cost": predecessor[1],
        },
    }


def public_example_fields(evidence: dict[str, object] | None) -> dict[str, object]:
    return {} if evidence is None else {
        "public_example_command": evidence["command"],
    }


def invalidate_over_budget_receipts(
    posture: str,
    control: dict[str, Any] | None,
    receipts: list[dict[str, Any]],
) -> None:
    if posture == "execution-budget-exceeded":
        invalidate_receipts(receipts, {"main-test"})
    elif (
        posture == "control-failed"
        and isinstance(control, dict)
        and control.get("exit_posture") == "execution-budget-exceeded"
    ):
        invalidate_receipts(receipts, {"control-test"})
