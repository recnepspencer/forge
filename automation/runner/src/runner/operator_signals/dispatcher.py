from __future__ import annotations

from dataclasses import dataclass
import json
from datetime import datetime, timezone
from typing import Any

from runner.authority.run_identity import RuntimePaths
from runner.operator_signals.policies import admit_notification_policy
from runner.operator_signals.replay import planned_fanout_for_event
from runner.operator_signals.sinks import deliver_command_hook, deliver_file, deliver_stdout


@dataclass(frozen=True)
class SignalDispatchRoute:
    signal_kind: str
    delivery_policy: str


def dispatch_route_for_signal_kind(config: dict[str, Any], signal_kind: str) -> SignalDispatchRoute:
    policy = admit_notification_policy(config)
    signal_policy = None if policy is None else policy.for_kind(signal_kind)
    if signal_policy is None or not signal_policy.enabled:
        raise ValueError(f"signal kind {signal_kind!r} is not enabled by notification policy")
    return SignalDispatchRoute(
        signal_kind=signal_kind,
        delivery_policy=signal_policy.delivery,
    )


def dispatch_authority_event(paths: RuntimePaths, config: dict[str, Any], event: dict[str, Any]) -> tuple[dict[str, Any], ...]:
    deliveries: list[dict[str, Any]] = []
    policy = admit_notification_policy(config)
    if policy is None:
        return ()
    for plan in planned_fanout_for_event(config, event):
        for sink in plan.sinks:
            try:
                deliveries.append(deliver(paths, policy.command_hook, sink, plan.payload))
            # Signals are projections of authority, never a second execution
            # authority.  A sink may be an external bridge (for example,
            # Telegram) and can fail in ways we do not control.  It must not
            # turn a durable runner event into a runner crash.
            except Exception:
                deliveries.append({"sink": sink, "delivered": False})
    result = tuple(deliveries)
    if any(not delivery.get("delivered") for delivery in result):
        paths.notification_delivery.parent.mkdir(parents=True, exist_ok=True)
        with paths.notification_delivery.open("a", encoding="utf-8") as output:
            output.write(json.dumps({"at": datetime.now(timezone.utc).isoformat(), "event_sequence": event["sequence"], "event_type": event["event_type"], "deliveries": result}) + "\n")
    return result


def deliver(paths: RuntimePaths, command_hook: tuple[str, ...] | None, sink: str, payload: dict[str, Any]) -> dict[str, Any]:
    if sink == "stdout":
        return deliver_stdout(payload)
    if sink == "file":
        return deliver_file(paths, payload)
    if sink == "command_hook":
        if command_hook is None:
            raise ValueError("command_hook sink requires notification_policy.command_hook")
        return deliver_command_hook(command_hook, payload)
    raise ValueError(f"unsupported notification sink {sink!r}")
