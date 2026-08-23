from __future__ import annotations

import json
import hashlib
import subprocess
from typing import Any

from worth_ui_3141_ledger_contracts import (
    COUNTERS,
    MUTATIONS,
    construction_cost,
    execution_cost,
)
from worth_ui_3141_case_contracts import hostile_cases, positive_cases
from worth_ui_ledger_command import GovernedTest, repository_path
from worth_ui_ledger_artifact_identity import requirement_phase


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
    if test.requirement in {
        "P4-PREDECESSOR-01", "P5-PREDECESSOR-01", "P6-PREDECESSOR-01"
    }:
        return predecessor_costs(test, control)
    if test.requirement == "P3-HP02-WORLD-01":
        if supporting_world is None or control is None:
            return None
        return (
            "main-tests=1;hostile-controls=1;product-processes=1;compile-sessions=0;"
            "courtroom-worlds=1;shared-mounted-worlds=1",
            "executed-tests=2;presentations=7;shared-presentations=5",
        )
    if test.requirement == "P5-ATLAS-PINNING-01":
        control_tests = 0 if control is None else control.get("executed_test_count")
        presentations = None if observation is None else observation.get("presentations")
        atlas_transactions = None if observation is None else observation.get("atlas_transactions")
        if (
            supporting_world is None
            or supporting_world.get("requirement") != "P5-ATLAS-01"
            or observation is None
            or observation.get("schema") != "worth-ui-native-gate-d-pin-world-v3"
            or not isinstance(control_tests, int)
            or not isinstance(presentations, int)
            or presentations < 0
            or not isinstance(atlas_transactions, int)
            or atlas_transactions < 0
        ):
            return None
        return (
            f"main-tests=1;hostile-controls={control_tests};product-processes=1;"
            "compile-sessions=0;courtroom-worlds=1",
            f"executed-tests={1 + control_tests};presentations={presentations};"
            f"atlas-transactions={atlas_transactions}",
        )
    if test.requirement == "P6-WINDOWS-WORLD-01":
        if (
            observation is None
            or control is None
            or observation.get("schema")
            != "worth-ui-native-phase6-boundary-observation-v1"
            or observation.get("terminal_zero") is not True
            or observation.get("product_processes") != 1
            or not isinstance(observation.get("input", {}).get("retained_events"), int)
            or observation["input"]["retained_events"] <= 0
        ):
            return None
        control_tests = control.get("executed_test_count")
        if control_tests != 1:
            return None
        return (
            "main-tests=1;hostile-controls=1;product-processes=1;"
            "compile-sessions=0;courtroom-worlds=1",
            "executed-tests=2;presentations=1",
        )
    p2 = test.requirement.startswith("P2-")
    compile_sessions = compile_sessions_observed(test.sources) + int(
        public_example is not None and public_example.get("exit_code") == 0
    )
    world_count = stdout_numeric(execution.stdout, "WORTH_UI_LEDGER_WORLD=", int(p2))
    product_processes = (
        observation.get("product_processes")
        if p2 and observation
        else world_count
        if test.requirement
        in {
            "P5-TEXT-PIXELS-01",
            "P5-TEXT-RECONSTRUCTION-01",
            "P5-TEXT-COST-01",
            "P5-TEXT-ASYNC-PRESENTATION-01",
        }
        else 0
    )
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
            if source.endswith(
                f"p{requirement_phase(test.requirement)}-predecessor-handoff.json"
            )
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
    if not requirement.startswith(("P3-", "P4-", "P5-", "P6-")):
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


def mutation_receipt_observation(
    output: str,
    requirement: str,
    source_revision: str | None = None,
    source_state_digest: str | None = None,
) -> dict[str, Any] | None:
    if not requirement.startswith("P6-"):
        return None
    prefix = "WORTH_UI_LEDGER_MUTATION_RECEIPTS="
    matches = [line[len(prefix):] for line in output.splitlines() if line.startswith(prefix)]
    if len(matches) != 1:
        return None
    try:
        observed = json.loads(matches[0])
    except json.JSONDecodeError:
        return None
    receipt = observed.get(requirement) if isinstance(observed, dict) else None
    expected = MUTATIONS[requirement][1]
    if (
        not isinstance(receipt, dict)
        or set(observed) != {requirement}
        or receipt.get("schema") != "worth-ui-native-mutation-receipt-v2"
        or receipt.get("requirement") != requirement
        or receipt.get("case") != expected
        or receipt.get("schedule_id") != expected
        or receipt.get("mutation_identity") != f"{requirement}:{expected}"
        or receipt.get("observed_failure") is not True
    ):
        return None
    source_identity = receipt.get("source_identity")
    if not (
        isinstance(source_identity, dict)
        and isinstance(source_identity.get("revision"), str)
        and source_identity["revision"]
        and isinstance(source_identity.get("state_digest"), str)
        and source_identity["state_digest"]
    ):
        return None
    if (
        source_revision is not None
        and source_identity["revision"] != source_revision
    ):
        return None
    if (
        source_state_digest is not None
        and source_identity["state_digest"] != source_state_digest
    ):
        return None
    baseline = receipt.get("baseline")
    mutant = receipt.get("mutant")
    if not all(
        isinstance(value, dict)
        and isinstance(value.get("posture"), str)
        and value["posture"]
        and isinstance(value.get("terminal_state"), str)
        and value["terminal_state"]
        and isinstance(value.get("trace"), list)
        and value["trace"] == [value["posture"], value["terminal_state"]]
        and isinstance(value.get("trace_sha256"), str)
        and len(value["trace_sha256"]) == 64
        and all(character in "0123456789abcdef" for character in value["trace_sha256"])
        for value in (baseline, mutant)
    ):
        return None
    for value in (baseline, mutant):
        expected_digest = hashlib.sha256(
            f"{value['posture']}\0{value['terminal_state']}".encode()
        ).hexdigest()
        if value["trace_sha256"] != expected_digest:
            return None
    divergence = receipt.get("first_divergence")
    if not (
        isinstance(divergence, dict)
        and isinstance(divergence.get("index"), int)
        and divergence["index"] >= 0
        and isinstance(divergence.get("description"), str)
        and divergence["description"]
    ):
        return None
    if baseline["trace_sha256"] == mutant["trace_sha256"]:
        return None
    if not isinstance(receipt.get("denial"), str) or not receipt["denial"]:
        return None
    return receipt


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
