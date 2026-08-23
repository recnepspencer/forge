from __future__ import annotations

from pathlib import Path
from typing import Any

from worth_ui_ledger_execution_observation_migration import migrate_payload
from worth_ui_ledger_execution_observation_store import retain


def retain_payload_observations(
    root: Path, state_digest: str, payload: dict[str, Any]
) -> None:
    migrate_payload(root, payload, state_digest, materialize=True)
    references = payload.get("execution_receipts")
    if not isinstance(references, list):
        raise RuntimeError("retained row omits its execution receipts")
    for reference in references:
        observation = reference.get("observation_sha256")
        if not isinstance(observation, str):
            raise RuntimeError("execution reference is malformed")
        retain(root, observation)
