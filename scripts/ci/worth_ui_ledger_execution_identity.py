from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from worth_ui_ledger_execution_binding import digest_json


PORTFOLIO_EXECUTION_SCHEMA = "worth-ui-ledger-portfolio-execution-v2"


@dataclass(frozen=True)
class AuthenticatedExecution:
    observation_sha256: str
    execution_binding_key: str
    role: str
    record: dict[str, Any]
    portfolio_execution_identity: str


def portfolio_execution_identity(
    role: str, command: list[str], bindings: dict[str, Any]
) -> str:
    if not role:
        raise RuntimeError("portfolio execution identity omits its role")
    return digest_json({
        "schema": PORTFOLIO_EXECUTION_SCHEMA,
        "role": role,
        "exact_command": command,
        "normalized_causal_artifact_bindings": bindings,
    })


def valid_hex(value: object, length: int) -> bool:
    return (
        isinstance(value, str)
        and len(value) == length
        and all(character in "0123456789abcdef" for character in value)
    )
