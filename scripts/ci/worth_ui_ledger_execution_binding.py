from __future__ import annotations

import hashlib
import json
import os
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from worth_ui_ledger_candidate_basis import execution_input_digest, from_path
from worth_ui_ledger_artifact_identity import requirement_phase


SCHEMA = "worth-ui-ledger-execution-binding-v3"
COMPILE_ARTIFACT_ENV = "WORTH_UI_COMPILE_ARTIFACT"
PREDECESSOR_ARTIFACT_ENV = "WORTH_UI_PREDECESSOR_ARTIFACT"
CANDIDATE_LEDGER_ENV = "WORTH_UI_MILESTONE_3141_LEDGER"
COMPILE_ARTIFACT = "_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json"
LEDGER = "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
P3_PREDECESSOR = "_docs/worth-ui/milestone-3.14.1-evidence/p3-predecessor-handoff.json"
P4_PREDECESSOR = "_docs/worth-ui/milestone-3.14.1-evidence/p4-predecessor-handoff.json"
P5_PREDECESSOR = "_docs/worth-ui/milestone-3.14.1-evidence/p5-predecessor-handoff.json"
P6_PREDECESSOR = "_docs/worth-ui/milestone-3.14.1-evidence/p6-predecessor-handoff.json"


@dataclass(frozen=True)
class GovernedExecutionSnapshot:
    revision: str
    state_digest: str


def execution_binding(
    command: list[str],
    root: Path,
    snapshot: GovernedExecutionSnapshot,
    artifact_dependencies: tuple[str, ...] = (),
    requirement: str | None = None,
) -> dict[str, Any]:
    return {
        "schema": SCHEMA,
        "command": command,
        "source_revision": snapshot.revision,
        "source_state_digest": snapshot.state_digest,
        "artifact_bindings": artifact_bindings(
            root, command, artifact_dependencies, requirement
        ),
    }


def artifact_bindings(
    root: Path,
    command: list[str],
    dependencies: tuple[str, ...],
    requirement: str | None = None,
) -> dict[str, dict[str, str]]:
    result = {}
    for name in sorted(set(dependencies)):
        value = os.environ.get(name, default_artifact(name, command))
        if value is None:
            continue
        identity = Path(value)
        if not identity.is_absolute():
            identity = root / identity
        result[name] = {"sha256": artifact_digest(identity, name, requirement)}
    return result


def artifact_digest(identity: Path, name: str, requirement: str | None) -> str:
    if not identity.is_file():
        return "missing"
    if name != CANDIDATE_LEDGER_ENV:
        return hashlib.sha256(identity.read_bytes()).hexdigest()
    if requirement is None:
        raise ValueError("candidate ledger dependency requires its governed row")
    return ledger_claim_prefix_digest(identity, requirement)


def ledger_claim_prefix_digest(identity: Path, requirement: str) -> str:
    phase = requirement_phase(requirement)
    if requirement.endswith("-PREDECESSOR-01"):
        return from_path(identity, phase - 1).candidate_prefix_digest
    through_phase = phase
    return execution_input_digest(identity, through_phase, requirement)


def default_artifact(name: str, command: list[str]) -> str:
    if name == COMPILE_ARTIFACT_ENV:
        return COMPILE_ARTIFACT
    if name == CANDIDATE_LEDGER_ENV:
        return LEDGER
    if name == PREDECESSOR_ARTIFACT_ENV:
        joined = " ".join(command)
        if "phase_six" in joined:
            return P6_PREDECESSOR
        if "phase_five" in joined:
            return P5_PREDECESSOR
        return P4_PREDECESSOR if "phase_four" in joined else P3_PREDECESSOR
    raise ValueError(f"unknown ledger execution artifact dependency: {name}")


def causal_artifact_dependencies(command: list[str], role: str) -> tuple[str, ...]:
    if role.endswith("discovery"):
        return ()
    joined = " ".join(command)
    dependencies = []
    if "compile_contract_artifact" in joined:
        dependencies.append(COMPILE_ARTIFACT_ENV)
    if command_reads_candidate_ledger(command):
        dependencies.append(CANDIDATE_LEDGER_ENV)
    if "predecessor_handoff" in joined or "predecessor_artifact" in joined:
        dependencies.append(PREDECESSOR_ARTIFACT_ENV)
    return tuple(dependencies)


def command_reads_candidate_ledger(command: list[str]) -> bool:
    joined = " ".join(command)
    excluded = (
        "result_artifact::mutation_tests::"
        "phase_two_boundary_observation_rejects_each_causal_mutation"
    )
    return "milestone_3141_phase1_ledger" in joined and excluded not in joined


def artifact_bindings_match(
    observed: object,
    expected: dict[str, dict[str, str]],
    command: list[str],
) -> bool:
    if not isinstance(observed, dict):
        return False
    retained = dict(observed)
    current = dict(expected)
    if not command_reads_candidate_ledger(command):
        retained.pop(CANDIDATE_LEDGER_ENV, None)
        current.pop(CANDIDATE_LEDGER_ENV, None)
    return retained == current


def digest_json(value: object) -> str:
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(encoded).hexdigest()
