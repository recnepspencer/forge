from __future__ import annotations

from pathlib import Path
from typing import Any

from runner.authority.events.event_log import EventLogDecodeError, load_events, validate_event_log
from runner.authority.run_identity import RuntimePaths


def load_admitted_run_projection_inputs(run_id: str) -> tuple[Path, dict[str, Any], tuple[dict[str, Any], ...]]:
    paths = RuntimePaths(run_id)
    try:
        events = load_events(paths.events)
    except EventLogDecodeError as error:
        raise ValueError(str(error)) from error
    if not events:
        raise ValueError(f"run {run_id!r} does not exist")
    errors = validate_event_log(events, run_id)
    if errors:
        raise ValueError("; ".join(errors))

    from runner.facade.plan_revision import assert_config_matches_adopted_plan, latest_plan_payload

    plan_payload = latest_plan_payload(events)
    config_path = (
        plan_payload.get("config_path")
        if isinstance(plan_payload, dict)
        else events[0].get("payload", {}).get("config_path")
    )
    if not isinstance(config_path, str) or not config_path:
        raise ValueError(f"run {run_id!r} does not record a config_path")
    resolved_config_path = Path(config_path).resolve()
    from runner.authority.config.loader import load_config
    from runner.authority.config.validator import validate_config

    config = load_config(resolved_config_path)
    config_errors = validate_config(config, resolved_config_path)
    if config_errors:
        raise ValueError("; ".join(config_errors))
    assert_config_matches_adopted_plan(config, events, run_id)
    return resolved_config_path, config, tuple(events)
