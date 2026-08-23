from __future__ import annotations

import json
import os
from pathlib import Path
from typing import Any

from worth_ui_ledger_execution_binding import SCHEMA as BINDING_SCHEMA, digest_json
from worth_ui_ledger_execution_observation import (
    OBSERVATION_SCHEMA,
    REFERENCE_SCHEMA,
    create_observation,
    validate_envelope,
)
from worth_ui_ledger_execution_observation_store import (
    durable_identity,
    read,
    register_transactional_identity,
    replace_bytes,
    retain_envelope,
)
from worth_ui_ledger_artifact_transaction import require_active_transaction
from worth_ui_ledger_runner_authentication import authenticates


LEGACY_SCHEMA = "worth-ui-ledger-execution-receipt-v2"
LEGACY_ROOT = Path("_docs/worth-ui/milestone-3.14.1-evidence/executions")
MIGRATION_SCHEMA = "worth-ui-ledger-execution-observation-migration-v1"


def migrate_payload(
    root: Path,
    payload: dict[str, Any],
    state_digest: str | None = None,
    *,
    materialize: bool = False,
) -> dict[str, Any]:
    receipts = payload.get("execution_receipts")
    if not isinstance(receipts, list):
        return payload
    state = state_digest or payload.get("source_state_digest")
    migrated = [
        migrate_reference(root, receipt, state, materialize=materialize)
        for receipt in receipts
    ]
    payload["execution_receipts"] = migrated
    reuse = payload.get("causal_reuse")
    if isinstance(reuse, dict) and "execution_receipt_keys" in reuse:
        inherited = set(reuse.pop("execution_receipt_keys"))
        reuse["schema"] = "worth-ui-ledger-causal-reuse-v2"
        reuse["execution_observation_ids"] = [
            receipt["observation_sha256"]
            for old, receipt in zip(receipts, migrated)
            if old.get("key") in inherited
        ]
    return payload


def migrate_reference(
    root: Path,
    reference: object,
    state_digest: object = None,
    *,
    materialize: bool = False,
) -> dict[str, object]:
    if not isinstance(reference, dict):
        raise RuntimeError("execution reference is malformed")
    if reference.get("schema") == REFERENCE_SCHEMA:
        return reference
    key = reference.get("key")
    if not isinstance(key, str):
        raise RuntimeError("legacy execution reference is malformed")
    migrated = migrated_observation(
        root, key, reference, state_digest, materialize=materialize
    )
    if migrated is not None:
        record, observation = migrated
        return reference_from_record(reference, record, observation)
    if not materialize:
        raise RuntimeError(
            "legacy execution observation requires governed migration publication"
        )
    envelope = read_legacy(root, key, state_digest)
    record = authenticated_legacy_record(root, key, reference, envelope)
    binding = observation_binding(record)
    observation, migrated = create_observation(
        root,
        binding,
        int(record["returncode"]),
        str(record["stdout"]),
        str(record["stderr"]),
        int(record["duration_ms"]),
    )
    require_active_transaction(root)
    register_transactional_identity(
        root, durable_identity(root, str(migrated.observation_sha256))
    )
    register_transactional_identity(root, migration_identity(root, key))
    retain_envelope(root, observation)
    persist_migration(root, key, envelope, migrated_payload := migrated.payload())
    return reference_from_payload(reference, migrated_payload)


def migrated_observation(
    root: Path,
    legacy_key: str,
    reference: dict[str, object],
    state_digest: object,
    *,
    materialize: bool,
) -> tuple[dict[str, Any], str] | None:
    identity = migration_identity(root, legacy_key)
    if not identity.is_file():
        return None
    try:
        migration = json.loads(identity.read_text(encoding="utf-8"))
        observation = migration["observation_sha256"]
    except (KeyError, OSError, json.JSONDecodeError):
        raise RuntimeError("execution observation migration provenance is invalid")
    legacy_envelope = migration.get("legacy_envelope")
    needs_backfill = not isinstance(legacy_envelope, dict)
    if needs_backfill:
        if not materialize:
            raise RuntimeError(
                "legacy execution observation requires governed migration publication"
            )
        legacy_envelope = read_legacy(root, legacy_key, state_digest)
    legacy_record = authenticated_legacy_record(
        root, legacy_key, reference, legacy_envelope
    )
    envelope = read(root, observation)
    record = validate_envelope(root, envelope)
    expected = expected_observation_record(legacy_record)
    if (
        migration.get("schema") != MIGRATION_SCHEMA
        or migration.get("legacy_execution_key") != legacy_key
        or migration.get("legacy_record_digest")
        != digest_json(legacy_envelope["record"])
        or record is None
        or migration.get("execution_binding_key")
        != record.get("execution_binding_key")
        or record != expected
    ):
        raise RuntimeError("execution observation migration provenance is invalid")
    if needs_backfill:
        require_active_transaction(root)
        persist_migration(
            root,
            legacy_key,
            legacy_envelope,
            {
                "execution_binding_key": record["execution_binding_key"],
                "observation_sha256": observation,
            },
        )
    return record, observation


def authenticated_legacy_record(
    root: Path,
    key: str,
    reference: dict[str, object],
    envelope: dict[str, Any],
) -> dict[str, Any]:
    record = envelope.get("record")
    if not isinstance(record, dict) or not authenticates(
        record, envelope.get("runner_authentication"), root
    ):
        raise RuntimeError("legacy execution observation is unauthenticated")
    require_exact_legacy_reference(reference, key, envelope, record)
    return record


def validate_embedded_migration(
    root: Path, legacy_key: str, state_digest: str
) -> None:
    identity = migration_identity(root, legacy_key)
    try:
        migration = json.loads(identity.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise RuntimeError("execution observation migration provenance is invalid") from error
    envelope = migration.get("legacy_envelope")
    record = envelope.get("record") if isinstance(envelope, dict) else None
    if not isinstance(record, dict):
        raise RuntimeError("execution observation migration has no embedded legacy envelope")
    reference = {
        "key": legacy_key,
        "command_sha256": digest_json(record.get("command")),
        "duration_ms": record.get("duration_ms"),
    }
    if migrated_observation(
        root,
        legacy_key,
        reference,
        state_digest,
        materialize=False,
    ) is None:
        raise RuntimeError("execution observation migration provenance is invalid")


def observation_binding(record: dict[str, Any]) -> dict[str, Any]:
    return {
        "schema": BINDING_SCHEMA,
        "command": record["command"],
        "source_revision": record["source_revision"],
        "source_state_digest": record["source_state_digest"],
        "artifact_bindings": record["artifact_bindings"],
    }


def expected_observation_record(record: dict[str, Any]) -> dict[str, Any]:
    binding = observation_binding(record)
    return {
        "schema": OBSERVATION_SCHEMA,
        "execution_binding": binding,
        "execution_binding_key": digest_json(binding),
        "returncode": record["returncode"],
        "stdout": record["stdout"],
        "stderr": record["stderr"],
        "duration_ms": record["duration_ms"],
    }


def reference_from_record(
    legacy: dict[str, object], record: dict[str, Any], observation: str
) -> dict[str, object]:
    binding = record["execution_binding"]
    return reference_from_payload(legacy, {
        "schema": REFERENCE_SCHEMA,
        "execution_binding_key": record["execution_binding_key"],
        "observation_sha256": observation,
        "command_sha256": digest_json(binding["command"]),
        "duration_ms": record["duration_ms"],
        "acquisition": "executed",
    })


def reference_from_payload(
    legacy: dict[str, object], migrated: dict[str, object]
) -> dict[str, object]:
    result = dict(migrated)
    result["acquisition"] = "reused" if legacy.get("reused") is True else "executed"
    if isinstance(legacy.get("role"), str):
        result["role"] = legacy["role"]
    return result


def persist_migration(
    root: Path,
    legacy_key: str,
    legacy_envelope: dict[str, Any],
    reference: dict[str, object],
) -> None:
    payload = {
        "schema": MIGRATION_SCHEMA,
        "legacy_execution_key": legacy_key,
        "legacy_record_digest": digest_json(legacy_envelope["record"]),
        "legacy_envelope": legacy_envelope,
        "execution_binding_key": reference["execution_binding_key"],
        "observation_sha256": reference["observation_sha256"],
    }
    identity = migration_identity(root, legacy_key)
    encoded = json.dumps(payload, sort_keys=True).encode() + b"\n"
    if identity.is_file():
        try:
            existing = json.loads(identity.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as error:
            raise RuntimeError(
                "execution observation migration identity collision"
            ) from error
        legacy_free = dict(payload)
        legacy_free.pop("legacy_envelope")
        if existing not in (payload, legacy_free):
            raise RuntimeError("execution observation migration identity collision")
        if existing == payload:
            return
    register_transactional_identity(root, identity)
    replace_bytes(identity, encoded)


def migration_identity(root: Path, legacy_key: str) -> Path:
    return (
        root / "_docs/worth-ui/milestone-3.14.1-evidence/execution-observation-migrations"
        / legacy_key[:2] / f"{legacy_key}.json"
    )


def read_legacy(root: Path, key: str, state_digest: object) -> dict[str, Any]:
    identities = [root / LEGACY_ROOT / key[:2] / f"{key}.json"]
    if isinstance(state_digest, str):
        identities.append(
            root / "workspaces/worth-ui/target/milestone-3141-execution-cache"
            / state_digest / "executions" / key[:2] / f"{key}.json"
        )
    configured = os.environ.get("WORTH_UI_LEDGER_EXECUTION_CACHE")
    if configured:
        identities.append(Path(configured) / "executions" / key[:2] / f"{key}.json")
    cache_root = root / "workspaces/worth-ui/target/milestone-3141-execution-cache"
    identities.extend(cache_root.glob(f"*/executions/{key[:2]}/{key}.json"))
    for identity in identities:
        try:
            return json.loads(identity.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            continue
    raise RuntimeError("legacy execution observation is absent")


def require_exact_legacy_reference(
    reference: dict[str, object],
    key: str,
    envelope: dict[str, Any],
    record: dict[str, Any],
) -> None:
    binding = {
        field: record.get(field)
        for field in (
            "schema", "command", "source_revision", "source_state_digest",
            "artifact_bindings",
        )
    }
    if (
        record.get("schema") != LEGACY_SCHEMA
        or digest_json(binding) != key
        or record.get("key") != key
        or envelope.get("receipt_sha256") != digest_json(record)
        or reference.get("command_sha256") != digest_json(record.get("command"))
        or reference.get("duration_ms") != record.get("duration_ms")
    ):
        raise RuntimeError("legacy execution reference differs from its observation")
