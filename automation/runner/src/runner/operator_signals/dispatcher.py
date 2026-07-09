from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from runner.phase_programs.policy_bindings import signal_delivery_policy_for_kind


@dataclass(frozen=True)
class SignalDispatchRoute:
    signal_kind: str
    delivery_policy: str


def dispatch_route_for_signal_kind(config: dict[str, Any], signal_kind: str) -> SignalDispatchRoute:
    return SignalDispatchRoute(
        signal_kind=signal_kind,
        delivery_policy=signal_delivery_policy_for_kind(config, signal_kind),
    )
