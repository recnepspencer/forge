from __future__ import annotations

from typing import Any


def validate_stall_policy(stall_policy: dict[str, Any], errors: list[str]) -> None:
    signals = stall_policy.get("signals")
    if not isinstance(signals, dict) or not signals:
        errors.append("stall_policy.signals must be a non-empty object")
        return
    for signal_name, signal_policy in signals.items():
        prefix = f"stall_policy.signals.{signal_name}"
        if not isinstance(signal_policy, dict):
            errors.append(f"{prefix} must be an object")
            continue
        enabled = signal_policy.get("enabled")
        if not isinstance(enabled, bool):
            errors.append(f"{prefix}.enabled must be a boolean")
        minutes_without_qualifying_edit = signal_policy.get("minutes_without_qualifying_edit")
        if minutes_without_qualifying_edit is not None and (
            not isinstance(minutes_without_qualifying_edit, int) or minutes_without_qualifying_edit <= 0
        ):
            errors.append(f"{prefix}.minutes_without_qualifying_edit must be a positive integer when present")
        minutes_without_phase_progress = signal_policy.get("minutes_without_phase_progress")
        if minutes_without_phase_progress is not None and (
            not isinstance(minutes_without_phase_progress, int) or minutes_without_phase_progress <= 0
        ):
            errors.append(f"{prefix}.minutes_without_phase_progress must be a positive integer when present")
