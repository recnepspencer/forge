from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any

from worth_ui_ledger_execution_binding import digest_json
from worth_ui_ledger_runner_authentication import authentication_tag, authenticates


OBSERVATION_SCHEMA = "worth-ui-ledger-execution-observation-v1"
REFERENCE_SCHEMA = "worth-ui-ledger-execution-reference-v1"


@dataclass(frozen=True)
class ExecutionReference:
    execution_binding_key: str
    observation_sha256: str
    command_sha256: str
    duration_ms: int
    acquisition: str

    def payload(self) -> dict[str, object]:
        return {"schema": REFERENCE_SCHEMA, **self.__dict__}


def create_observation(
    root: Path,
    binding: dict[str, Any],
    returncode: int,
    stdout: str,
    stderr: str,
    duration_ms: int,
) -> tuple[dict[str, Any], ExecutionReference]:
    binding_key = digest_json(binding)
    record = {
        "schema": OBSERVATION_SCHEMA,
        "execution_binding": binding,
        "execution_binding_key": binding_key,
        "returncode": returncode,
        "stdout": stdout,
        "stderr": stderr,
        "duration_ms": duration_ms,
    }
    tag = authentication_tag(record, root)
    identity = digest_json({"record": record, "runner_authentication": tag})
    envelope = {
        "observation_sha256": identity,
        "record": record,
        "runner_authentication": tag,
    }
    reference = ExecutionReference(
        binding_key,
        identity,
        digest_json(binding["command"]),
        duration_ms,
        "executed",
    )
    return envelope, reference


def validate_envelope(root: Path, envelope: object) -> dict[str, Any] | None:
    if not isinstance(envelope, dict) or not isinstance(envelope.get("record"), dict):
        return None
    record = envelope["record"]
    tag = envelope.get("runner_authentication")
    identity = digest_json({"record": record, "runner_authentication": tag})
    binding = record.get("execution_binding")
    if (
        envelope.get("observation_sha256") != identity
        or record.get("schema") != OBSERVATION_SCHEMA
        or not isinstance(binding, dict)
        or record.get("execution_binding_key") != digest_json(binding)
        or not authenticates(record, tag, root)
    ):
        return None
    return record


def reused_reference(reference: ExecutionReference) -> ExecutionReference:
    return ExecutionReference(
        reference.execution_binding_key,
        reference.observation_sha256,
        reference.command_sha256,
        reference.duration_ms,
        "reused",
    )
