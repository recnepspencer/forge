from __future__ import annotations

import hashlib
import json
import secrets
from pathlib import Path
from typing import Any

from worth_ui_ledger_durable_receipts import (
    cache_receipt_identity,
    read_durable_envelope,
)
from worth_ui_ledger_execution_cache import (
    artifact_bindings,
    artifact_bindings_match,
    causal_artifact_dependencies,
    digest_json,
)
from worth_ui_ledger_row_cache import source_artifact_bindings
from worth_ui_ledger_runner_authentication import authentication_tag, authenticates


SCHEMA = "worth-ui-ledger-causal-reuse-v1"


def revalidate_row_payload(
    root: Path,
    row: dict[str, str],
    payload: dict[str, Any],
    artifact_sha256: str,
    claim_digest: str,
    revision: str,
    state_digest: str,
) -> dict[str, Any] | None:
    retained = dict(payload)
    retained.pop("artifact_sha256", None)
    sources = row["source_identity"].split(";")
    current_source_digest = source_digest_at(root, tuple(sources))
    if not reusable_payload(
        root, row, retained, claim_digest, sources, current_source_digest
    ):
        return None
    if (
        retained.get("source_revision") == revision
        and retained.get("source_state_digest") == state_digest
    ):
        current = dict(retained)
        current["artifact_sha256"] = artifact_sha256
        return current
    return reissue_payload(
        root,
        row,
        retained,
        artifact_sha256,
        claim_digest,
        revision,
        state_digest,
        sources,
        current_source_digest,
    )


def revalidate_joined_predecessor_payload(
    root: Path,
    row: dict[str, str],
    retained: dict[str, Any],
    claim_evidence: dict[str, Any],
    artifact_sha256: str,
    claim_digest: str,
    revision: str,
    state_digest: str,
    execution_mapping_matches: bool,
) -> dict[str, Any] | None:
    sources = row["source_identity"].split(";")
    current_source_digest = source_digest_at(root, tuple(sources))
    retained_unsigned = {
        key: value for key, value in retained.items() if key != "runner_authentication"
    }
    claim_unsigned = {
        key: value
        for key, value in claim_evidence.items()
        if key != "runner_authentication"
    }
    if not (
        authenticates(
            retained_unsigned, retained.get("runner_authentication"), root
        )
        and authenticates(
            claim_unsigned, claim_evidence.get("runner_authentication"), root
        )
        and execution_mapping_matches
        and retained.get("requirement") == row["requirement"]
        and claim_evidence.get("requirement") == row["requirement"]
        and retained.get("exit_posture") == "passed"
        and claim_evidence.get("exit_posture") == "passed"
        and claim_evidence.get("claim_digest") == claim_digest
        and retained.get("production_entry") == row["production_entry"]
        and retained.get("independent_oracle") == row["independent_oracle"]
        and retained.get("source_identity") == sources
        and retained.get("mapping_source_identity") == sources
        and retained.get("source_digest") == current_source_digest
        and retained.get("execution_receipts")
        == claim_evidence.get("execution_receipts")
        and retained.get("artifact_sha256") == artifact_sha256
        and retained.get("executed_exact_command") == row["exact_command"]
        and authenticated_receipt_keys(root, retained, fail_closed=False) is not None
    ):
        return None
    return reissue_payload(
        root,
        row,
        retained,
        artifact_sha256,
        claim_digest,
        revision,
        state_digest,
        sources,
        current_source_digest,
    )


def reissue_payload(
    root: Path,
    row: dict[str, str],
    payload: dict[str, Any],
    artifact_sha256: str,
    claim_digest: str,
    revision: str,
    state_digest: str,
    sources: list[str],
    current_source_digest: str,
) -> dict[str, Any]:
    receipt_keys = authenticated_receipt_keys(root, payload)
    reused = {
        "schema": SCHEMA,
        "predecessor_artifact_sha256": artifact_sha256,
        "predecessor_source_revision": payload["source_revision"],
        "predecessor_source_state_digest": payload["source_state_digest"],
        "predecessor_source_digest": payload["source_digest"],
        "predecessor_run_nonce": payload["run_nonce"],
        "claim_digest": claim_digest,
        "exact_command": row["exact_command"],
        "source_artifact_bindings": source_artifact_bindings(
            root, row["exact_command"], row["requirement"]
        ),
        "execution_receipt_keys": receipt_keys,
    }
    current = dict(payload)
    current.update(
        {
            "source_revision": revision,
            "source_state_digest": state_digest,
            "source_digest": current_source_digest,
            "source_identity": sources,
            "mapping_source_identity": sources,
            "source_rebindings": [],
            "run_nonce": secrets.token_hex(16),
            "claim_digest": claim_digest,
            "production_entry": row["production_entry"],
            "independent_oracle": row["independent_oracle"],
            "executed_exact_command": row["exact_command"],
            "causal_reuse": reused,
        }
    )
    current.pop("artifact_sha256", None)
    current.pop("runner_authentication", None)
    current["runner_authentication"] = authentication_tag(current, root)
    return current


def reusable_payload(
    root: Path,
    row: dict[str, str],
    payload: dict[str, Any],
    claim_digest: str,
    sources: list[str],
    current_source_digest: str,
) -> bool:
    unsigned = {
        key: value for key, value in payload.items() if key != "runner_authentication"
    }
    receipts = payload.get("execution_receipts")
    return (
        authenticates(unsigned, payload.get("runner_authentication"), root)
        and payload.get("requirement") == row["requirement"]
        and payload.get("exit_posture") == "passed"
        and payload.get("claim_digest") == claim_digest
        and payload.get("production_entry") == row["production_entry"]
        and payload.get("independent_oracle") == row["independent_oracle"]
        and payload.get("source_identity") == sources
        and payload.get("mapping_source_identity") == sources
        and payload.get("source_digest") == current_source_digest
        and payload.get("executed_exact_command") == row["exact_command"]
        and isinstance(receipts, list)
        and receipts
        and authenticated_receipt_keys(root, payload, fail_closed=False) is not None
    )


def validate_causal_reuse(
    root: Path,
    payload: dict[str, Any],
    revision: str,
    state_digest: str,
) -> set[str] | None:
    reuse = payload.get("causal_reuse")
    if reuse is None:
        return None
    if not isinstance(reuse, dict) or reuse.get("schema") != SCHEMA:
        raise RuntimeError("retained row has malformed causal-reuse evidence")
    sources = payload.get("mapping_source_identity")
    command = payload.get("executed_exact_command")
    requirement = payload.get("requirement")
    receipts = payload.get("execution_receipts")
    if (
        payload.get("source_revision") != revision
        or payload.get("source_state_digest") != state_digest
        or not isinstance(sources, list)
        or not all(isinstance(source, str) for source in sources)
        or not isinstance(command, str)
        or not isinstance(requirement, str)
        or not isinstance(receipts, list)
    ):
        raise RuntimeError("retained row causal reuse is not current-source bound")
    current_digest = source_digest_at(root, tuple(sources))
    expected_bindings = source_artifact_bindings(root, command, requirement)
    receipt_keys = authenticated_receipt_keys(root, payload)
    if (
        payload.get("source_digest") != current_digest
        or reuse.get("source_artifact_bindings") != expected_bindings
        or reuse.get("claim_digest") != payload.get("claim_digest")
        or reuse.get("exact_command") != command
        or reuse.get("execution_receipt_keys") != receipt_keys
        or not valid_hex(reuse.get("predecessor_artifact_sha256"), 64)
        or not valid_hex(reuse.get("predecessor_source_digest"), 64)
        or not valid_hex(reuse.get("predecessor_run_nonce"), 32)
    ):
        raise RuntimeError("retained row causal reuse differs from its causal binding")
    return set(receipt_keys)


def authenticated_receipt_keys(
    root: Path,
    payload: dict[str, Any],
    *,
    fail_closed: bool = True,
) -> list[str] | None:
    receipts = payload.get("execution_receipts")
    if not isinstance(receipts, list) or not receipts:
        return receipt_failure("retained row omits execution receipts", fail_closed)
    keys: list[str] = []
    try:
        for receipt in receipts:
            if not isinstance(receipt, dict):
                raise RuntimeError("retained row has a malformed execution receipt")
            key = receipt.get("key")
            role = receipt.get("role")
            if not valid_hex(key, 64) or not isinstance(role, str):
                raise RuntimeError("retained row has a malformed execution receipt")
            envelope = receipt_envelope(root, payload, key)
            record = envelope.get("record")
            if not isinstance(record, dict):
                raise RuntimeError("retained execution receipt is absent")
            command = record.get("command")
            if not isinstance(command, list) or not all(
                isinstance(part, str) for part in command
            ):
                raise RuntimeError("retained execution receipt has no exact command")
            binding = {
                field: record.get(field)
                for field in (
                    "schema",
                    "command",
                    "source_revision",
                    "source_state_digest",
                    "artifact_bindings",
                )
            }
            expected_dependencies = artifact_bindings(
                root,
                command,
                causal_artifact_dependencies(command, role),
                str(payload.get("requirement", "")),
            )
            unsigned = record
            if (
                envelope.get("receipt_sha256") != digest_json(record)
                or not authenticates(
                    unsigned, envelope.get("runner_authentication"), root
                )
                or record.get("key") != key
                or digest_json(binding) != key
                or not artifact_bindings_match(
                    record.get("artifact_bindings"), expected_dependencies, command
                )
                or record.get("returncode") != 0
                or record.get("duration_ms") != receipt.get("duration_ms")
                or digest_json(command) != receipt.get("command_sha256")
            ):
                raise RuntimeError(
                    "retained execution receipt differs from its authenticated dependency binding"
                )
            keys.append(key)
    except (OSError, RuntimeError, TypeError, ValueError):
        if fail_closed:
            raise
        return None
    return sorted(keys)


def receipt_envelope(
    root: Path, payload: dict[str, Any], key: str
) -> dict[str, Any]:
    try:
        return read_durable_envelope(root, key)
    except RuntimeError:
        reuse = payload.get("causal_reuse")
        states = [payload.get("source_state_digest")]
        if isinstance(reuse, dict):
            states.append(reuse.get("predecessor_source_state_digest"))
        for state in states:
            if not isinstance(state, str):
                continue
            identity = cache_receipt_identity(root, state, key)
            try:
                return json.loads(identity.read_text(encoding="utf-8"))
            except (OSError, json.JSONDecodeError):
                continue
        raise RuntimeError("retained execution receipt is absent")


def receipt_failure(message: str, fail_closed: bool) -> None:
    if fail_closed:
        raise RuntimeError(message)
    return None


def encoded_payload(payload: dict[str, Any]) -> bytes:
    return (json.dumps(payload, indent=2) + "\n").encode("utf-8")


def digest(content: bytes) -> str:
    return hashlib.sha256(content).hexdigest()


def source_digest_at(root: Path, sources: tuple[str, ...]) -> str:
    if len(sources) != len(set(sources)):
        raise ValueError("source identities must be unique")
    result = hashlib.sha256()
    for identity in sorted(sources):
        source = (root / identity).resolve()
        source.relative_to(root.resolve())
        if not source.is_file():
            raise ValueError(f"source does not exist: {identity}")
        result.update(identity.encode("utf-8"))
        result.update(b"\0")
        result.update(source.read_bytes())
        result.update(b"\0")
    return result.hexdigest()


def valid_hex(value: object, length: int) -> bool:
    return (
        isinstance(value, str)
        and len(value) == length
        and all(character in "0123456789abcdef" for character in value)
    )
