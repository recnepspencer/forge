from __future__ import annotations

from typing import Any

from runner.roles.model_policy import validate_model_policy_seed
from runner.roles.session_policy import validate_session_policy_seed


def validate_session_defaults(session_defaults: dict[str, Any], errors: list[str]) -> None:
    validate_model_policy_seed(session_defaults, errors, "session_defaults")
    validate_session_policy_seed(session_defaults, errors, "session_defaults", require_continuity_family=False)
