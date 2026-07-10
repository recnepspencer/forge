from __future__ import annotations

from typing import Any

from runner.operator_signals.signal_types import SIGNAL_KINDS

_DELIVERIES = {"immediate", "queued", "final"}
_SINKS = {"stdout", "file", "command_hook"}


def validate_notification_policy(policy: dict[str, Any], errors: list[str]) -> None:
    signals = policy.get("signals")
    if not isinstance(signals, dict):
        errors.append("notification_policy.signals must be an object")
        return
    for kind, value in signals.items():
        if kind not in SIGNAL_KINDS:
            errors.append(f"notification_policy.signals has unknown signal kind {kind!r}")
            continue
        if not isinstance(value, dict) or not isinstance(value.get("enabled"), bool):
            errors.append(f"notification_policy.signals.{kind} must declare boolean enabled")
            continue
        if value.get("delivery") not in _DELIVERIES:
            errors.append(f"notification_policy.signals.{kind}.delivery is invalid")
        sinks = value.get("sinks")
        if not isinstance(sinks, list) or any(sink not in _SINKS for sink in sinks):
            errors.append(f"notification_policy.signals.{kind}.sinks is invalid")
    hook = policy.get("command_hook")
    if hook is not None and (not isinstance(hook, list) or not hook or any(not isinstance(part, str) or not part for part in hook)):
        errors.append("notification_policy.command_hook must be a non-empty command list when present")
