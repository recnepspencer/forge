from __future__ import annotations

import subprocess
from dataclasses import dataclass
from typing import Any, Callable

from worth_ui_ledger_command import (
    ControlTest,
    control_budget_ms,
    control_cargo_command,
    execution_counts,
    exact_test_duration_ms,
    listed_test_names,
)
from worth_ui_ledger_observation import mutation_case_observation, mutation_control_observation
from worth_ui_3141_phase4_case_contracts import hostile_cases


Execution = Callable[[list[str], str], tuple[subprocess.CompletedProcess[str], int]]


@dataclass
class ControlExecution:
    list_command: list[str]
    test_command: list[str]
    discovery: subprocess.CompletedProcess[str]
    discovery_duration_ms: int
    matches: int
    execution: subprocess.CompletedProcess[str] | None = None
    test_duration_ms: int | None = None
    counts: tuple[int, int, int] = (0, 0, 0)
    posture: str = "match-count-rejected"
    mutation_control: str | None = None
    mutation_cases: list[str] | None = None


def control_payload(
    test: ControlTest | None, requirement: str, execute: Execution
) -> dict[str, Any] | None:
    if test is None:
        return None
    list_command = control_cargo_command(test, True)
    test_command = control_cargo_command(test, False)
    discovery, discovery_duration = execute(list_command, "control-discovery")
    matches = listed_test_names(discovery.stdout).count(test.test_name)
    observed = ControlExecution(
        list_command, test_command, discovery, discovery_duration, matches
    )
    execute_control(requirement, observed, execute)
    observed.mutation_control = mutation_control_observation(
        "" if observed.execution is None else observed.execution.stdout, requirement
    )
    if (
        requirement.startswith(("P3-", "P4-", "P5-"))
        and observed.mutation_control is None
    ):
        observed.posture = "mutation-control-mismatch"
    observed.mutation_cases = mutation_case_observation(
        "" if observed.execution is None else observed.execution.stdout, requirement
    )
    if hostile_cases(requirement) is not None and observed.mutation_cases is None:
        observed.posture = "mutation-case-mismatch"
    return control_fields(test, requirement, observed)


def execute_control(
    requirement: str,
    observed: ControlExecution,
    execute: Execution,
) -> None:
    observed.posture = (
        "list-failed" if observed.discovery.returncode else "match-count-rejected"
    )
    if observed.discovery.returncode == 0 and observed.matches == 1:
        observed.execution, command_duration = execute(
            observed.test_command, "control-test"
        )
        observed.test_duration_ms = exact_test_duration_ms(
            observed.execution.stdout, command_duration
        )
        observed.counts = execution_counts(observed.execution.stdout)
        observed.posture = (
            "passed"
            if observed.execution.returncode == 0 and observed.counts == (1, 1, 0)
            else "test-failed"
        )
        if observed.test_duration_ms > control_budget_ms(requirement):
            observed.posture = "execution-budget-exceeded"


def control_fields(
    test: ControlTest,
    requirement: str,
    observed: ControlExecution,
) -> dict[str, Any]:
    executed, passed, ignored = observed.counts
    return {
        "package": test.package,
        "target_kind": test.target_kind,
        "target_name": test.target_name,
        "test_name": test.test_name,
        "features": list(test.features),
        "matched_test_count": observed.matches,
        "executed_test_count": executed,
        "passed_test_count": passed,
        "ignored_test_count": ignored,
        "exit_posture": observed.posture,
        "list_exit_code": observed.discovery.returncode,
        "test_exit_code": (
            None if observed.execution is None else observed.execution.returncode
        ),
        "list_duration_ms": observed.discovery_duration_ms,
        "test_duration_ms": observed.test_duration_ms,
        "test_budget_ms": control_budget_ms(requirement),
        "list_command": observed.list_command,
        "test_command": observed.test_command,
        "list_stdout": observed.discovery.stdout,
        "list_stderr": observed.discovery.stderr,
        "test_stdout": "" if observed.execution is None else observed.execution.stdout,
        "test_stderr": "" if observed.execution is None else observed.execution.stderr,
        "mutation_control": observed.mutation_control,
        "mutation_cases": observed.mutation_cases,
    }
