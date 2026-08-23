from __future__ import annotations

import hashlib
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from worth_ui_ledger_causal_revalidation import validate_causal_reuse
from worth_ui_ledger_execution_binding import digest_json
from worth_ui_ledger_execution_observation_migration import migrate_payload
from worth_ui_ledger_execution_identity import AuthenticatedExecution
from worth_ui_ledger_execution_reference_validation import (
    ExecutionExpectation,
    validate_execution,
)
from worth_ui_ledger_runner_authentication import authenticates


@dataclass(frozen=True)
class AggregationContext:
    root: Path
    revision: str
    state_digest: str
    requirements: set[str]
    logical: bool


def aggregate_executions(
    payloads: list[dict[str, Any]],
    root: Path,
    revision: str,
    state_digest: str,
    requirements: set[str],
) -> list[dict[str, Any]]:
    return aggregate(
        payloads,
        AggregationContext(root, revision, state_digest, requirements, True),
    )


def aggregate_historical_executions(
    payloads: list[dict[str, Any]],
    root: Path,
    revision: str,
    state_digest: str,
    requirements: set[str],
) -> list[dict[str, Any]]:
    return aggregate(
        payloads,
        AggregationContext(root, revision, state_digest, requirements, False),
    )


def aggregate(
    payloads: list[dict[str, Any]],
    context: AggregationContext,
) -> list[dict[str, Any]]:
    groups: dict[str, dict[str, Any]] = {}
    observed_roles: dict[str, set[str]] = {}
    historical: dict[int, set[str]] = {}
    for payload in payloads:
        if not authenticated_row_payload(context.root, payload):
            raise RuntimeError("retained row evidence lacks runner provenance")
        historical_observations(context.root, payload)
        migrate_payload(context.root, payload)
        reuse = payload.get("causal_reuse")
        identities = (
            reuse.get("execution_observation_ids", [])
            if isinstance(reuse, dict) else []
        )
        historical[id(payload)] = set(identities)
    for payload in payloads:
        aggregate_payload(
            payload,
            context,
            historical[id(payload)],
            groups,
            observed_roles,
        )
    require_governed_roles(context.requirements, observed_roles)
    for group in groups.values():
        group["requirements"].sort()
        group["observations"].sort(key=lambda item: item["observation_sha256"])
    key = (
        "portfolio_execution_identity"
        if context.logical
        else "execution_binding_key"
    )
    return sorted(groups.values(), key=lambda group: group[key])


def aggregate_payload(
    payload: dict[str, Any],
    context: AggregationContext,
    historical: set[str],
    groups: dict[str, dict[str, Any]],
    observed_roles: dict[str, set[str]],
) -> None:
    requirement = payload.get("requirement")
    receipts = payload.get("execution_receipts")
    if not isinstance(requirement, str) or not isinstance(receipts, list):
        raise RuntimeError("retained row omits its execution reference inventory")
    roles = []
    for reference in receipts:
        if not isinstance(reference, dict) or not isinstance(reference.get("role"), str):
            raise RuntimeError("execution reference omits its role")
        role = reference["role"]
        expectation = ExecutionExpectation(
            context.root,
            context.revision,
            context.state_digest,
            role,
            requirement,
            reference.get("observation_sha256") in historical,
        )
        try:
            validated = validate_execution(reference, expectation)
        except RuntimeError as error:
            raise RuntimeError(
                f"{requirement} {role} execution reference is invalid: {error}"
            ) from error
        require_row_command(payload, role, reference)
        roles.append(role)
        observed_roles.setdefault(requirement, set()).add(role)
        identity = (
            validated.portfolio_execution_identity
            if context.logical else validated.execution_binding_key
        )
        observed = execution_group(validated, context.logical)
        merge_execution(groups.setdefault(identity, observed), observed, requirement)
    require_exact_roles(payload, requirement, roles)


def execution_group(
    execution: AuthenticatedExecution, logical: bool
) -> dict[str, Any]:
    binding = execution.record["execution_binding"]
    group = {
        "exact_command": binding["command"],
        "normalized_causal_artifact_bindings": binding["artifact_bindings"],
        "observations": [observation(execution)],
        "requirements": [],
    }
    if logical:
        group.update({
            "portfolio_execution_identity": execution.portfolio_execution_identity,
            "role": execution.role,
        })
    else:
        group["execution_binding_key"] = execution.execution_binding_key
    return group


def observation(execution: AuthenticatedExecution) -> dict[str, object]:
    return {
        "execution_binding_key": execution.execution_binding_key,
        "observation_sha256": execution.observation_sha256,
        "duration_ms": execution.record["duration_ms"],
    }


def merge_execution(
    current: dict[str, Any], observed: dict[str, Any], requirement: str
) -> None:
    for field in (
        "exact_command", "normalized_causal_artifact_bindings", "role",
        "portfolio_execution_identity", "execution_binding_key",
    ):
        if field in observed and current.get(field) != observed[field]:
            raise RuntimeError("one execution identity has conflicting evidence")
    if requirement not in current["requirements"]:
        current["requirements"].append(requirement)
    known = {item["observation_sha256"] for item in current["observations"]}
    for item in observed["observations"]:
        if item["observation_sha256"] not in known:
            current["observations"].append(item)


def historical_observations(root: Path, payload: dict[str, Any]) -> set[str]:
    reuse = payload.get("causal_reuse")
    if isinstance(reuse, dict) and reuse.get("schema") == "worth-ui-ledger-causal-reuse-v2":
        identities = reuse.get("execution_observation_ids")
        if not isinstance(identities, list):
            raise RuntimeError("causal reuse omits observation identities")
        return set(identities)
    legacy = validate_causal_reuse(
        root, payload, str(payload.get("source_revision", "")),
        str(payload.get("source_state_digest", "")),
    )
    return set() if legacy is None else set(legacy)


def require_row_command(
    payload: dict[str, Any], role: str, reference: dict[str, Any]
) -> None:
    expected = expected_command_sha256(payload, role)
    if expected is None or reference.get("command_sha256") != expected:
        raise RuntimeError("execution reference is bound to the wrong row command")


def expected_command_sha256(payload: dict[str, Any], role: str) -> str | None:
    field = {
        "main-discovery": "list_command",
        "ignored-discovery": "ignored_list_command",
        "main-test": "test_command",
        "public-example": "public_example_command",
    }.get(role)
    owner: object = payload
    if role in {"control-discovery", "control-test"}:
        owner = payload.get("hostile_control")
        field = "list_command" if role == "control-discovery" else "test_command"
    if not isinstance(owner, dict) or not isinstance(field, str):
        return None
    command = owner.get(field)
    return digest_json(command) if isinstance(command, list) else None


def require_exact_roles(
    payload: dict[str, Any], requirement: str, roles: list[str]
) -> None:
    expected = ["main-discovery", "ignored-discovery", "main-test"]
    if requirement == "P4-FONT-COLLECTION-01":
        expected.append("public-example")
    if isinstance(payload.get("hostile_control"), dict):
        expected.extend(("control-discovery", "control-test"))
    if sorted(roles) != sorted(expected):
        raise RuntimeError("row has a missing, duplicate, or unexpected execution role")


def require_governed_roles(
    requirements: set[str], observed: dict[str, set[str]]
) -> None:
    mandatory = {"main-discovery", "ignored-discovery", "main-test"}
    for requirement in requirements:
        if not mandatory.issubset(observed.get(requirement, set())):
            raise RuntimeError(f"portfolio omits execution references for {requirement}")


def authenticated_row_payload(root: Path, payload: dict[str, Any]) -> bool:
    unsigned = {key: value for key, value in payload.items() if key != "runner_authentication"}
    if authenticates(unsigned, payload.get("runner_authentication"), root):
        return True
    artifact_sha256 = payload.get("artifact_sha256")
    command = payload.get("executed_exact_command")
    if not isinstance(artifact_sha256, str) or not isinstance(command, str):
        return False
    words = command.split()
    try:
        artifact = (root / words[words.index("--artifact") + 1]).resolve()
        artifact.relative_to(root.resolve())
    except (ValueError, IndexError):
        return False
    if not artifact.is_file() or hashlib.sha256(artifact.read_bytes()).hexdigest() != artifact_sha256:
        return False
    unsigned.pop("artifact_sha256", None)
    return authenticates(unsigned, payload.get("runner_authentication"), root)
