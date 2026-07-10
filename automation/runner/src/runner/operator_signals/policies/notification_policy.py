from __future__ import annotations

from dataclasses import dataclass
from typing import Any


@dataclass(frozen=True)
class SignalPolicy:
    enabled: bool
    delivery: str
    sinks: tuple[str, ...]


@dataclass(frozen=True)
class NotificationPolicy:
    signals: dict[str, SignalPolicy]
    command_hook: tuple[str, ...] | None

    def for_kind(self, signal_kind: str) -> SignalPolicy | None:
        return self.signals.get(signal_kind)


def admit_notification_policy(config: dict[str, Any]) -> NotificationPolicy | None:
    raw = config.get("notification_policy")
    if raw is None:
        return None
    signals = {
        kind: SignalPolicy(bool(value["enabled"]), value["delivery"], tuple(value["sinks"]))
        for kind, value in raw["signals"].items()
    }
    hook = raw.get("command_hook")
    return NotificationPolicy(signals, None if hook is None else tuple(hook))
