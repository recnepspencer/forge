from __future__ import annotations

import hashlib
import json
import secrets
from pathlib import Path
from typing import Any

from worth_ui_ledger_execution_observation_migration import migrate_payload
from worth_ui_ledger_execution_reference_validation import (
    ExecutionExpectation,
    validate_available_execution,
    validate_execution,
)
from worth_ui_ledger_row_cache import source_artifact_bindings
from worth_ui_ledger_runner_authentication import authentication_tag, authenticates


SCHEMA = "worth-ui-ledger-causal-reuse-v2"


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
    if row["requirement"].startswith("P6-"):
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
    if (
        row["requirement"].startswith("P6-")
        and (
            retained.get("source_revision") != revision
            or retained.get("source_state_digest") != state_digest
        )
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
    observation_ids = authenticated_receipt_keys(root, payload)
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
        "execution_observation_ids": sorted(set(observation_ids)),
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
        and (
            payload.get("causal_reuse") is not None
            or receipts_match_payload_source(root, payload)
        )
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
    if not isinstance(reuse, dict):
        raise RuntimeError("retained row has malformed causal-reuse evidence")
    migrate_payload(root, payload)
    reuse = payload.get("causal_reuse")
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
    inherited = reuse.get("execution_observation_ids")
    if (
        not isinstance(inherited, list)
        or not inherited
        or not all(valid_hex(key, 64) for key in inherited)
    ):
        raise RuntimeError("retained row causal reuse differs from its causal binding")
    observation_ids = authenticated_receipt_keys(root, payload)
    if (
        payload.get("source_digest") != current_digest
        or reuse.get("source_artifact_bindings") != expected_bindings
        or reuse.get("claim_digest") != payload.get("claim_digest")
        or reuse.get("exact_command") != command
        or not set(inherited).issubset(observation_ids)
        or not valid_hex(reuse.get("predecessor_artifact_sha256"), 64)
        or not valid_hex(reuse.get("predecessor_source_digest"), 64)
        or not valid_hex(reuse.get("predecessor_run_nonce"), 32)
    ):
        raise RuntimeError("retained row causal reuse differs from its causal binding")
    return set(inherited)


def authenticated_receipt_keys(
    root: Path,
    payload: dict[str, Any],
    *,
    fail_closed: bool = True,
) -> list[str] | None:
    try:
        migrate_payload(root, payload)
        receipts = payload.get("execution_receipts")
        if not isinstance(receipts, list) or not receipts:
            return receipt_failure("retained row omits execution receipts", fail_closed)
        identities: list[str] = []
        reuse = payload.get("causal_reuse")
        inherited = set(
            reuse.get("execution_observation_ids", [])
            if isinstance(reuse, dict) else []
        )
        for reference in receipts:
            if not isinstance(reference, dict) or not isinstance(reference.get("role"), str):
                raise RuntimeError("retained row has a malformed execution reference")
            observation = reference.get("observation_sha256")
            expectation = ExecutionExpectation(
                root,
                str(payload.get("source_revision", "")),
                str(payload.get("source_state_digest", "")),
                reference["role"],
                str(payload.get("requirement", "")),
                observation in inherited,
            )
            validated = validate_execution(reference, expectation)
            identities.append(validated.observation_sha256)
    except (OSError, RuntimeError, TypeError, ValueError):
        if fail_closed:
            raise
        return None
    return sorted(identities)


def receipts_match_payload_source(root: Path, payload: dict[str, Any]) -> bool:
    return receipts_match_payload_source_with(root, payload, validate_execution)


def available_receipts_match_payload_source(
    root: Path, payload: dict[str, Any]
) -> bool:
    return receipts_match_payload_source_with(
        root, payload, validate_available_execution
    )


def receipts_match_payload_source_with(
    root: Path, payload: dict[str, Any], validator: Any
) -> bool:
    revision = payload.get("source_revision")
    state_digest = payload.get("source_state_digest")
    receipts = payload.get("execution_receipts")
    if not isinstance(revision, str) or not isinstance(state_digest, str):
        return False
    if not isinstance(receipts, list) or not receipts:
        return False
    try:
        migrate_payload(root, payload)
        receipts = payload["execution_receipts"]
        for reference in receipts:
            if not isinstance(reference, dict) or not isinstance(reference.get("role"), str):
                return False
            execution = validator(
                reference,
                ExecutionExpectation(
                    root, revision, state_digest, reference["role"],
                    str(payload.get("requirement", "")), False,
                ),
            )
            binding = execution.record["execution_binding"]
            if (
                binding.get("source_revision") != revision
                or binding.get("source_state_digest") != state_digest
            ):
                return False
    except (OSError, RuntimeError, TypeError, ValueError, json.JSONDecodeError):
        return False
    return True


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
