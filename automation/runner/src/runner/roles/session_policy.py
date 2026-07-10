from __future__ import annotations

from dataclasses import dataclass
from typing import Any


@dataclass(frozen=True)
class RoleSessionPolicySeed:
    continuity_family: str
    reuse_session: bool = True
    fresh_session_after_qa_repair_cycles: int | None = None

    @classmethod
    def from_mapping(
        cls,
        session_policy_seed: dict[str, Any],
        field_name: str,
        require_continuity_family: bool = True,
    ) -> "RoleSessionPolicySeed":
        reuse_session = session_policy_seed.get("reuse_session", True)
        if not isinstance(reuse_session, bool):
            raise ValueError(f"{field_name}.reuse_session must be a boolean when present")
        continuity_family = session_policy_seed.get("continuity_family")
        if not require_continuity_family and continuity_family is None:
            continuity_family = ""
        if not isinstance(continuity_family, str) or (require_continuity_family and not continuity_family):
            raise ValueError(f"{field_name}.continuity_family is required")
        fresh_session_cycles = session_policy_seed.get("fresh_session_after_qa_repair_cycles")
        if fresh_session_cycles is not None and (not isinstance(fresh_session_cycles, int) or fresh_session_cycles <= 0):
            raise ValueError(f"{field_name}.fresh_session_after_qa_repair_cycles must be a positive integer when present")
        return cls(
            continuity_family=continuity_family,
            reuse_session=reuse_session,
            fresh_session_after_qa_repair_cycles=fresh_session_cycles,
        )


@dataclass(frozen=True)
class RoleSessionPolicy:
    reuse_session: bool
    continuity_family: str
    fresh_session_after_qa_repair_cycles: int | None


def role_session_policy_from_seed(session_policy_seed: RoleSessionPolicySeed) -> RoleSessionPolicy:
    return RoleSessionPolicy(
        reuse_session=session_policy_seed.reuse_session,
        continuity_family=session_policy_seed.continuity_family,
        fresh_session_after_qa_repair_cycles=session_policy_seed.fresh_session_after_qa_repair_cycles,
    )


def validate_session_policy_seed(
    session_policy_seed: dict[str, Any],
    errors: list[str],
    field_name: str,
    require_continuity_family: bool = True,
) -> None:
    try:
        RoleSessionPolicySeed.from_mapping(session_policy_seed, field_name, require_continuity_family)
    except ValueError as error:
        errors.append(str(error))


def session_state_for_execution(
    projection_session: dict[str, Any],
    session_policy: RoleSessionPolicy,
    force_fresh_session: bool,
) -> dict[str, Any]:
    execution_session = dict(projection_session)
    execution_session["reuse_session"] = session_policy.reuse_session and not force_fresh_session
    if not execution_session["reuse_session"]:
        execution_session["thread_id"] = None
    return execution_session
