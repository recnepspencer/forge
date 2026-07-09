from __future__ import annotations

from dataclasses import dataclass

from runner.roles.registry import RoleDefinition
from runner.roles.role_ids import IMPLEMENTER_ROLE_ID, REVIEWER_ROLE_ID


@dataclass(frozen=True)
class RoleHandoffPolicy:
    escalation_role_id: str
    escalation_posture: str
    force_fresh_session_on_escalation: bool


def handoff_policy_for_role(role: RoleDefinition) -> RoleHandoffPolicy:
    if role.role_id == REVIEWER_ROLE_ID:
        return RoleHandoffPolicy(
            escalation_role_id=REVIEWER_ROLE_ID,
            escalation_posture="deep_reviewer_pass",
            force_fresh_session_on_escalation=False,
        )
    if role.role_id != IMPLEMENTER_ROLE_ID:
        raise ValueError(f"unsupported Milestone 1 role {role.role_id!r}")
    return RoleHandoffPolicy(
        escalation_role_id=REVIEWER_ROLE_ID,
        escalation_posture="reviewer_escalation",
        force_fresh_session_on_escalation=False,
    )
