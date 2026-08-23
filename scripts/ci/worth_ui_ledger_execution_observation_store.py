from __future__ import annotations

import json
import os
import tempfile
from pathlib import Path
from typing import Any

from worth_ui_ledger_artifact_identity import execution_observation
from worth_ui_ledger_execution_observation import validate_envelope


CACHE_ENV = "WORTH_UI_LEDGER_EXECUTION_CACHE"
BINDING_INDEX_SCHEMA = "worth-ui-ledger-execution-binding-index-v1"


def durable_identity(root: Path, observation: str) -> Path:
    return execution_observation(observation).destination(root)


def cached_identity(observation: str) -> Path | None:
    root = os.environ.get(CACHE_ENV)
    return None if root is None else Path(root) / "execution-observations" / observation[:2] / f"{observation}.json"


def binding_index(binding_key: str) -> Path | None:
    root = os.environ.get(CACHE_ENV)
    return None if root is None else Path(root) / "execution-bindings" / binding_key[:2] / f"{binding_key}.json"


def read_for_binding(
    root: Path, binding_key: str, binding: dict[str, Any]
) -> tuple[dict[str, Any], str] | None:
    index = binding_index(binding_key)
    if index is None or not index.is_file():
        return None
    try:
        payload = json.loads(index.read_text(encoding="utf-8"))
        observation = payload["observation_sha256"]
    except (KeyError, OSError, json.JSONDecodeError):
        return None
    if payload.get("schema") != BINDING_INDEX_SCHEMA or not isinstance(observation, str):
        return None
    envelope = read_available(root, observation)
    record = validate_envelope(root, envelope)
    if record is None or record.get("execution_binding") != binding or record.get("returncode") != 0:
        index.unlink(missing_ok=True)
        cached = cached_identity(observation)
        if cached is not None:
            cached.unlink(missing_ok=True)
        return None
    return record, observation


def read(root: Path, observation: str) -> dict[str, Any] | None:
    return read_identities(root, [durable_identity(root, observation)], observation)


def read_available(root: Path, observation: str) -> dict[str, Any] | None:
    return read_identities(
        root,
        [durable_identity(root, observation), cached_identity(observation)],
        observation,
    )


def read_identities(
    root: Path, identities: list[Path | None], observation: str
) -> dict[str, Any] | None:
    for identity in identities:
        if identity is None or not identity.is_file():
            continue
        try:
            envelope = json.loads(identity.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            continue
        if (
            envelope.get("observation_sha256") == observation
            and validate_envelope(root, envelope) is not None
        ):
            return envelope
    return None


def stage(envelope: dict[str, Any]) -> None:
    observation = str(envelope["observation_sha256"])
    identity = cached_identity(observation)
    if identity is None:
        return
    write_immutable(identity, envelope)
    index = binding_index(str(envelope["record"]["execution_binding_key"]))
    if index is not None:
        replace_json(index, {
            "schema": BINDING_INDEX_SCHEMA,
            "observation_sha256": observation,
        })


def retain(root: Path, observation: str) -> None:
    destination = durable_identity(root, observation)
    if destination.is_file():
        return
    source = cached_identity(observation)
    if source is None or not source.is_file():
        raise RuntimeError("execution observation is absent")
    envelope = json.loads(source.read_text(encoding="utf-8"))
    if validate_envelope(root, envelope) is None:
        raise RuntimeError("execution observation is malformed")
    register_transactional_identity(root, destination)
    write_immutable(destination, envelope)


def retain_envelope(root: Path, envelope: dict[str, Any]) -> None:
    if validate_envelope(root, envelope) is None:
        raise RuntimeError("execution observation is malformed")
    observation = str(envelope["observation_sha256"])
    identity = durable_identity(root, observation)
    register_transactional_identity(root, identity)
    write_immutable(identity, envelope)


def invalidate_binding(binding_key: str) -> None:
    identity = binding_index(binding_key)
    if identity is not None:
        identity.unlink(missing_ok=True)


def invalidate_references(
    references: list[dict[str, Any]], roles: set[str]
) -> None:
    for reference in references:
        if reference.get("role") not in roles:
            continue
        binding_key = reference.get("execution_binding_key")
        if isinstance(binding_key, str):
            invalidate_binding(binding_key)


def write_immutable(identity: Path, payload: dict[str, Any]) -> None:
    encoded = json.dumps(payload, sort_keys=True).encode() + b"\n"
    if identity.is_file():
        if identity.read_bytes() != encoded:
            raise RuntimeError("execution observation identity collision")
        return
    replace_bytes(identity, encoded)


def replace_json(identity: Path, payload: dict[str, Any]) -> None:
    replace_bytes(identity, json.dumps(payload, sort_keys=True).encode() + b"\n")


def replace_bytes(identity: Path, content: bytes) -> None:
    identity.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary = tempfile.mkstemp(prefix=f".{identity.name}.", dir=identity.parent)
    try:
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(content)
        os.replace(temporary, identity)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)


def register_transactional_identity(root: Path, identity: Path) -> None:
    from worth_ui_ledger_artifact_transaction import register_active_identity

    register_active_identity(root, identity)
