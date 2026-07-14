from __future__ import annotations

IMPLEMENTER_ROLE_ID = "implementer"
REVIEWER_ROLE_ID = "reviewer"
SUPPORTED_ROLE_IDS = (IMPLEMENTER_ROLE_ID, REVIEWER_ROLE_ID)


def require_supported_role_id(role_id: object, field_name: str) -> str:
    if role_id not in SUPPORTED_ROLE_IDS:
        raise ValueError(f"{field_name} must be one of {list(SUPPORTED_ROLE_IDS)}")
    return str(role_id)
