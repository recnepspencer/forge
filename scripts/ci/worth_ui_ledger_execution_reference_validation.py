from __future__ import annotations

import hashlib
import os
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from worth_ui_ledger_execution_binding import (
    artifact_bindings,
    artifact_bindings_match,
    causal_artifact_dependencies,
    digest_json,
)
from worth_ui_ledger_execution_identity import (
    AuthenticatedExecution,
    portfolio_execution_identity,
    valid_hex,
)
from worth_ui_ledger_execution_observation import (
    REFERENCE_SCHEMA,
    validate_envelope,
)
from worth_ui_ledger_execution_observation_store import read, read_available
from worth_ui_ledger_artifact_identity import requirement_phase
from worth_ui_predecessor_handoff_currentness import (
    PredecessorVerification,
    expected_identity,
    is_current,
)


@dataclass(frozen=True)
class ExecutionExpectation:
    root: Path
    revision: str
    state_digest: str
    role: str
    requirement: str
    historical_allowed: bool = False


def validate_execution(
    reference: dict[str, Any], expectation: ExecutionExpectation
) -> AuthenticatedExecution:
    return validate_with_reader(reference, expectation, read)


def validate_available_execution(
    reference: dict[str, Any], expectation: ExecutionExpectation
) -> AuthenticatedExecution:
    return validate_with_reader(reference, expectation, read_available)


def validate_with_reader(
    reference: dict[str, Any], expectation: ExecutionExpectation, reader: Any
) -> AuthenticatedExecution:
    observation = reference.get("observation_sha256")
    binding_key = reference.get("execution_binding_key")
    if (
        reference.get("schema") != REFERENCE_SCHEMA
        or not valid_hex(observation, 64)
        or not valid_hex(binding_key, 64)
    ):
        raise RuntimeError("execution reference is malformed")
    envelope = reader(expectation.root, observation)
    record = validate_envelope(expectation.root, envelope)
    if record is None:
        raise RuntimeError("execution observation is absent or unauthenticated")
    binding = record["execution_binding"]
    command = binding.get("command")
    bindings = binding.get("artifact_bindings")
    if not isinstance(command, list) or not isinstance(bindings, dict):
        raise RuntimeError("execution observation has no exact causal binding")
    if not exact_reference(reference, record, observation, binding_key):
        raise RuntimeError("execution reference differs from its observation")
    if not expectation.historical_allowed:
        validate_current_binding(binding, command, bindings, expectation)
    identity = portfolio_execution_identity(expectation.role, command, bindings)
    return AuthenticatedExecution(
        observation, binding_key, expectation.role, record, identity
    )


def exact_reference(
    reference: dict[str, Any],
    record: dict[str, Any],
    observation: str,
    binding_key: str,
) -> bool:
    binding = record["execution_binding"]
    return (
        record.get("execution_binding_key") == binding_key
        and record.get("returncode") == 0
        and reference.get("observation_sha256") == observation
        and reference.get("command_sha256") == digest_json(binding["command"])
        and reference.get("duration_ms") == record.get("duration_ms")
        and reference.get("acquisition") in {"executed", "reused"}
    )


def validate_current_binding(
    binding: dict[str, Any],
    command: list[str],
    observed: dict[str, Any],
    expectation: ExecutionExpectation,
) -> None:
    if (
        binding.get("source_revision") != expectation.revision
        or binding.get("source_state_digest") != expectation.state_digest
    ):
        raise RuntimeError("execution observation has stale source identity")
    expected = artifact_bindings(
        expectation.root,
        command,
        causal_artifact_dependencies(command, expectation.role),
        expectation.requirement,
    )
    if not current_artifact_bindings_match(observed, expected, command, expectation):
        raise RuntimeError("execution observation has stale artifact bindings")


def current_artifact_bindings_match(
    observed: dict[str, Any],
    expected: dict[str, Any],
    command: list[str],
    expectation: ExecutionExpectation,
) -> bool:
    if artifact_bindings_match(observed, expected, command):
        return True
    if not expectation.requirement.endswith("-PREDECESSOR-01"):
        return False
    phase = requirement_phase(expectation.requirement)
    configured = os.environ.get("WORTH_UI_MILESTONE_3141_LEDGER")
    ledger = (
        Path(configured).resolve()
        if configured
        else expectation.root / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
    )
    verification = PredecessorVerification(
        expectation.root, ledger, phase, expectation.revision,
        expectation.state_digest,
    )
    typed, _ = expected_identity(verification)
    if not is_current(typed, verification):
        return False
    temporary = dict(expected)
    temporary["WORTH_UI_PREDECESSOR_ARTIFACT"] = {
        "sha256": hashlib.sha256(typed.destination(expectation.root).read_bytes()).hexdigest()
    }
    return artifact_bindings_match(observed, temporary, command)
