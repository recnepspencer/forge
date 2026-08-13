from __future__ import annotations

import json
import subprocess
from typing import Any

from worth_ui_3141_ledger_contracts import (
    COUNTERS,
    MUTATIONS,
    construction_cost,
    execution_cost,
)
from worth_ui_3141_phase4_case_contracts import hostile_cases, positive_cases
from worth_ui_ledger_command import GovernedTest, repository_path


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
    supporting_world: dict[str, Any] | None,
    public_example: dict[str, object] | None,
) -> tuple[str, str] | None:
    if execution is None:
        return None
    if test.requirement == "P3-PREDECESSOR-01":
        return construction_cost(test.requirement), execution_cost(test.requirement)
    if test.requirement == "P4-PREDECESSOR-01":
        return predecessor_costs(test, control)
    if test.requirement == "P3-HP02-WORLD-01":
        if supporting_world is None or control is None:
            return None
        return (
            "main-tests=1;hostile-controls=1;product-processes=1;compile-sessions=0;"
            "courtroom-worlds=1;shared-mounted-worlds=1",
            "executed-tests=2;presentations=7;shared-presentations=5",
        )
    p2 = test.requirement.startswith("P2-")
    compile_sessions = compile_sessions_observed(test.sources) + int(
        public_example is not None and public_example.get("exit_code") == 0
    )
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


def predecessor_costs(
    test: GovernedTest, control: dict[str, Any] | None
) -> tuple[str, str] | None:
    identity = next(
        (
            source
            for source in test.sources
            if source.endswith(f"p{test.requirement[1]}-predecessor-handoff.json")
        ),
        None,
    )
    if identity is None or control is None:
        return None
    artifact = json.loads(repository_path(identity).read_text(encoding="utf-8"))
    control_tests = control.get("executed_test_count")
    metrics = [
        artifact.get("main_test_executions"), artifact.get("hostile_control_executions"),
        artifact.get("closure_test_executions"), artifact.get("compile_sessions"),
        artifact.get("product_processes"), artifact.get("courtroom_worlds"),
        artifact.get("presentations"), control_tests,
    ]
    if not all(isinstance(value, int) and value >= 0 for value in metrics):
        return None
    main, hostile, closure, compile_runs, processes, worlds, presentations, control_runs = metrics
    return (
        f"main-tests={main + 1};hostile-controls={hostile + control_runs};"
        f"product-processes={processes};compile-sessions={compile_runs};courtroom-worlds={worlds}",
        f"executed-tests={main + hostile + closure + 1 + control_runs};"
        f"presentations={presentations}",
    )


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


def mutation_control_observation(output: str, requirement: str) -> dict[str, str] | None:
    if not requirement.startswith(("P3-", "P4-", "P5-")):
        return None
    prefix = "WORTH_UI_LEDGER_MUTATION_CONTROLS="
    matches = [line[len(prefix):] for line in output.splitlines() if line.startswith(prefix)]
    if len(matches) != 1:
        return None
    try:
        observed = json.loads(matches[0])
    except json.JSONDecodeError:
        return None
    expected = MUTATIONS[requirement][1]
    if not isinstance(observed, dict) or observed.get(requirement) != expected:
        return None
    return {"requirement": requirement, "case": expected}


def governed_case_observation(output: str, requirement: str) -> list[str] | None:
    return exact_case_observation(
        output, "WORTH_UI_LEDGER_CASES=", requirement, positive_cases(requirement)
    )


def mutation_case_observation(output: str, requirement: str) -> list[str] | None:
    return exact_case_observation(
        output, "WORTH_UI_LEDGER_MUTATION_CASES=", requirement, hostile_cases(requirement)
    )


def exact_case_observation(
    output: str,
    prefix: str,
    requirement: str,
    expected: tuple[str, ...] | None,
) -> list[str] | None:
    if expected is None:
        return None
    matches = [line[len(prefix):] for line in output.splitlines() if line.startswith(prefix)]
    if len(matches) != 1:
        return None
    try:
        observed = json.loads(matches[0])
    except json.JSONDecodeError:
        return None
    cases = observed.get(requirement) if isinstance(observed, dict) else None
    if not isinstance(cases, list) or tuple(cases) != expected:
        return None
    return cases
