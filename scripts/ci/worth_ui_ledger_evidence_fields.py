from __future__ import annotations

import secrets
from typing import Any

from worth_ui_3141_ledger_contracts import EXPECTED_IGNORED
from worth_ui_ledger_command import execution_budget_ms


def identity_fields(test: Any, snapshot: Any) -> dict[str, Any]:
    return {
        "schema_version": 7,
        "requirement": test.requirement,
        "claim_digest": snapshot.claim_digest,
        "package": test.package,
        "target_kind": test.target_kind,
        "target_name": test.target_name,
        "features": list(test.features),
        "test_name": test.test_name,
    }


def execution_fields(test: Any, main: Any, posture: str) -> dict[str, Any]:
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


def source_fields(test: Any, snapshot: Any) -> dict[str, Any]:
    return {
        "source_revision": snapshot.revision,
        "source_digest": snapshot.source_digest,
        "source_state_digest": snapshot.source_state_digest,
        "run_nonce": secrets.token_hex(16),
        "source_identity": list(test.sources),
    }


def command_output_fields(main: Any) -> dict[str, Any]:
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
    observations: Any,
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
    return {} if evidence is None else {"public_example_command": evidence["command"]}
