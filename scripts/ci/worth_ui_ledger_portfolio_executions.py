from __future__ import annotations

import hashlib
import json
from pathlib import Path
from typing import Any

from worth_ui_ledger_causal_revalidation import validate_causal_reuse
from worth_ui_ledger_durable_receipts import read_durable_envelope
from worth_ui_ledger_execution_cache import (
    artifact_bindings,
    artifact_bindings_match,
    causal_artifact_dependencies,
)
from worth_ui_ledger_runner_authentication import authenticates


def aggregate_executions(
    payloads: list[dict[str, Any]],
    root: Path,
    revision: str,
    state_digest: str,
    requirements: set[str],
) -> list[dict[str, Any]]:
    aggregate: dict[str, dict[str, Any]] = {}
    observed_roles: dict[str, set[str]] = {}
    historical_receipts: set[str] = set()
    for payload in payloads:
        if not authenticated_row_payload(root, payload):
            raise RuntimeError("retained row evidence lacks runner provenance")
        validate_causal_reuse(
            root,
            payload,
            str(payload.get("source_revision", "")),
            str(payload.get("source_state_digest", "")),
        )
        historical_receipts.update(portfolio_receipt_keys(root, payload))
    for payload in payloads:
        requirement = payload.get("requirement")
        receipts = payload.get("execution_receipts", [])
        if not isinstance(requirement, str) or not isinstance(receipts, list):
            raise RuntimeError("retained row omits its execution receipt inventory")
        payload_roles: list[str] = []
        for receipt in receipts:
            if not isinstance(receipt, dict) or not isinstance(receipt.get("key"), str):
                raise RuntimeError("retained execution receipt is malformed")
            key = receipt["key"]
            record_sha256 = validate_execution_receipt(
                root,
                revision,
                state_digest,
                receipt,
                key in historical_receipts,
            )
            role = receipt.get("role")
            if not isinstance(role, str):
                raise RuntimeError("retained execution receipt omits its role")
            payload_roles.append(role)
            expected = expected_command_sha256(payload, role)
            if expected is None or receipt.get("command_sha256") != expected:
                raise RuntimeError("retained execution receipt is bound to the wrong row command")
            observed_roles.setdefault(requirement, set()).add(role)
            observed = {
                "key": key,
                "command_sha256": receipt.get("command_sha256"),
                "duration_ms": receipt.get("duration_ms"),
                "receipt_sha256": record_sha256,
                "requirements": [],
            }
            current = aggregate.setdefault(key, observed)
            if (
                current["command_sha256"] != observed["command_sha256"]
                or current["duration_ms"] != observed["duration_ms"]
                or current["receipt_sha256"] != observed["receipt_sha256"]
            ):
                raise RuntimeError("one execution identity has conflicting retained evidence")
            if requirement not in current["requirements"]:
                current["requirements"].append(requirement)
        expected_roles = ["main-discovery", "ignored-discovery", "main-test"]
        if requirement == "P4-FONT-COLLECTION-01":
            expected_roles.append("public-example")
        if isinstance(payload.get("hostile_control"), dict):
            expected_roles.extend(("control-discovery", "control-test"))
        if sorted(payload_roles) != sorted(expected_roles):
            raise RuntimeError(
                "retained row execution receipts have a missing, duplicate, or unexpected role"
            )
    for execution in aggregate.values():
        execution["requirements"].sort()
    mandatory_roles = {"main-discovery", "ignored-discovery", "main-test"}
    for requirement in requirements:
        if not mandatory_roles.issubset(observed_roles.get(requirement, set())):
            raise RuntimeError(
                f"retained portfolio omits governed execution receipts for {requirement}"
            )
    return sorted(aggregate.values(), key=lambda execution: execution["key"])


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
    if not isinstance(command, list) or not all(isinstance(part, str) for part in command):
        return None
    return digest(json.dumps(command, separators=(",", ":")).encode("utf-8"))


def validate_execution_receipt(
    root: Path,
    revision: str,
    state_digest: str,
    receipt: dict[str, Any],
    historical_allowed: bool = False,
) -> str:
    key = receipt["key"]
    try:
        envelope = read_durable_envelope(root, key)
        record = envelope["record"]
    except (KeyError, RuntimeError, TypeError) as error:
        raise RuntimeError("retained execution receipt is absent") from error
    receipt_sha256 = digest_json(record)
    binding = {
        "schema": record.get("schema"),
        "command": record.get("command"),
        "source_revision": record.get("source_revision"),
        "source_state_digest": record.get("source_state_digest"),
        "artifact_bindings": record.get("artifact_bindings"),
    }
    command_sha256 = digest(
        json.dumps(record.get("command"), separators=(",", ":")).encode("utf-8")
    )
    if envelope.get("receipt_sha256") != receipt_sha256:
        raise RuntimeError("retained execution receipt differs from its exact execution")
    if not authenticates(record, envelope.get("runner_authentication"), root):
        raise RuntimeError("retained execution receipt differs from its exact execution")
    if (
        record.get("key") != key
        or digest_json(binding) != key
        or (
            not historical_allowed
            and (
                record.get("source_revision") != revision
                or record.get("source_state_digest") != state_digest
            )
        )
        or record.get("returncode") != 0
        or command_sha256 != receipt.get("command_sha256")
        or record.get("duration_ms") != receipt.get("duration_ms")
    ):
        raise RuntimeError("retained execution receipt differs from its exact execution")
    return receipt_sha256


def portfolio_receipt_keys(root: Path, payload: dict[str, Any]) -> list[str]:
    receipts = payload.get("execution_receipts")
    requirement = payload.get("requirement")
    if not isinstance(receipts, list) or not isinstance(requirement, str):
        raise RuntimeError("retained row omits its execution receipt inventory")
    keys = []
    for receipt in receipts:
        if not isinstance(receipt, dict) or not isinstance(receipt.get("key"), str):
            raise RuntimeError("retained execution receipt is malformed")
        role = receipt.get("role")
        if not isinstance(role, str):
            raise RuntimeError("retained execution receipt omits its role")
        validate_execution_receipt(root, "", "", receipt, historical_allowed=True)
        record = read_durable_envelope(root, receipt["key"])["record"]
        command = record.get("command")
        if not isinstance(command, list) or not all(
            isinstance(part, str) for part in command
        ):
            raise RuntimeError("retained execution receipt has no exact command")
        expected = artifact_bindings(
            root,
            command,
            causal_artifact_dependencies(command, role),
            requirement,
        )
        if not artifact_bindings_match(
            record.get("artifact_bindings"), expected, command
        ):
            raise RuntimeError(
                "retained execution receipt differs from its causal artifact binding"
            )
        keys.append(receipt["key"])
    return keys


def authenticated_row_payload(root: Path, payload: dict[str, Any]) -> bool:
    unsigned = {
        key: value for key, value in payload.items() if key != "runner_authentication"
    }
    return authenticates(unsigned, payload.get("runner_authentication"), root)

def digest(content: bytes) -> str:
    return hashlib.sha256(content).hexdigest()


def digest_json(value: object) -> str:
    return digest(
        json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")
    )
