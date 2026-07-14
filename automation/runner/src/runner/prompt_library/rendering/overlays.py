from __future__ import annotations

FRESH_RECOVERY_OVERLAY_ASSET_ID = "recovery/fresh_session_overlay"
OPERATOR_INJECTION_OVERLAY_ASSET_ID = "recovery/operator_injection_overlay"


def fresh_recovery_overlay_asset_id(projection: dict[str, object]) -> str | None:
    recovery = projection.get("session", {}).get("fresh_recovery")
    current = projection.get("current")
    if not isinstance(recovery, dict) or not isinstance(current, dict):
        return None
    if recovery.get("phase") != current.get("phase") or recovery.get("turn") != current.get("turn"):
        return None
    return FRESH_RECOVERY_OVERLAY_ASSET_ID


def operator_injection_overlay_asset_id(projection: dict[str, object]) -> str | None:
    intervention = projection.get("operator_intervention")
    current = projection.get("current")
    if not isinstance(intervention, dict) or not isinstance(current, dict):
        return None
    if intervention.get("current") != current:
        return None
    return OPERATOR_INJECTION_OVERLAY_ASSET_ID
