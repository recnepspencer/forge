from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from runner.operator_signals.detectors import signals_for_event
from runner.operator_signals.policies import admit_notification_policy
from runner.authority.run_identity import RuntimePaths


@dataclass(frozen=True)
class SignalFanout:
    payload: dict[str, Any]
    sinks: tuple[str, ...]


def planned_fanout_for_event(config: dict[str, Any], event: dict[str, Any]) -> tuple[SignalFanout, ...]:
    policy = admit_notification_policy(config)
    if policy is None:
        return ()
    plans: list[SignalFanout] = []
    for signal in signals_for_event(event):
        signal_policy = policy.for_kind(signal.signal_kind)
        if signal_policy is not None and signal_policy.enabled:
            plans.append(SignalFanout(enrich_payload(signal.payload(signal_policy.delivery), config, event), signal_policy.sinks))
    return tuple(plans)


def replay_signal_fanout(config: dict[str, Any], events: list[dict[str, Any]]) -> tuple[SignalFanout, ...]:
    """Cold-path derivation only; replay never invokes external sinks."""
    return tuple(plan for event in events for plan in planned_fanout_for_event(config, event))


def enrich_payload(payload: dict[str, Any], config: dict[str, Any], event: dict[str, Any]) -> dict[str, Any]:
    paths = RuntimePaths(payload["run_id"])
    phase_title = next((phase["title"] for phase in config.get("phases", []) if phase.get("id") == payload["phase_id"]), None)
    session = config.get("session_defaults", {})
    details = dict(payload["details"])
    details.update({"event_log_file": str(paths.events), "projection_file": str(paths.projection)})
    enriched = dict(payload)
    enriched.update({
        "project_name": config.get("project", {}).get("name"), "phase_title": phase_title,
        "provider": session.get("provider"), "model": session.get("model"),
        "thread_id": event.get("thread_id"), "occurred_at": event.get("at"),
        "actions": {"next_automatic_step": event.get("payload", {}).get("attempt_action")},
        "details": details,
    })
    return enriched
