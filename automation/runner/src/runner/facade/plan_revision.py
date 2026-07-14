from __future__ import annotations

import hashlib
import json
from pathlib import Path
from typing import Any

from runner.authority.config import load_config
from runner.authority.config.validator import validate_config
from runner.authority.events import load_events
from runner.authority.events.event_types import PHASE_PROGRESS_EVENTS
from runner.authority.run_identity import RuntimePaths


PLAN_EVENTS = {"plan_adopted", "plan_revised"}


def adopt_plan_payload(config_path: Path, config: dict[str, Any], plan_version: int = 1) -> dict[str, Any]:
    return {
        "plan_version": plan_version,
        "config_path": str(config_path.resolve()),
        "config_hash": config_hash(config),
        "prompt_manifest_hash": hash_json(prompt_manifest_for_hash(config)),
        "provider_manifest_hash": hash_json(provider_manifest_for_hash(config)),
        "phase_fingerprints": phase_fingerprints(config),
    }


def config_hash(config: dict[str, Any]) -> str:
    canonical = {key: value for key, value in config.items() if key != "_config_path"}
    encoded = json.dumps(canonical, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def phase_fingerprints(config: dict[str, Any]) -> list[dict[str, Any]]:
    fingerprints: list[dict[str, Any]] = []
    for ordinal, phase in enumerate(config.get("phases", []), start=1):
        phase_key = stable_phase_key(phase)
        fingerprints.append(
            {
                "phase_key": phase_key,
                "phase_id": phase["id"],
                "ordinal": ordinal,
                "phase_hash": hash_json(phase_content_for_hash(phase)),
                "prompt_binding_hash": hash_json(prompt_binding_for_hash(phase)),
                "provider_policy_hash": hash_json(provider_policy_for_hash(phase)),
            }
        )
    return fingerprints


def stable_phase_key(phase: dict[str, Any]) -> str:
    phase_key = phase.get("phase_key")
    if isinstance(phase_key, str) and phase_key:
        return phase_key
    return f"phase_{phase['id']}"


def hash_json(value: Any) -> str:
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def phase_content_for_hash(phase: dict[str, Any]) -> dict[str, Any]:
    excluded = {"role_bindings", "contract_template", "prompt_template"}
    return {key: value for key, value in phase.items() if key not in excluded}


def prompt_binding_for_hash(phase: dict[str, Any]) -> dict[str, Any]:
    role_prompt_bindings: dict[str, Any] = {}
    for turn, binding in sorted(phase.get("role_bindings", {}).items()):
        role_prompt_bindings[turn] = binding.get("prompt_template") if isinstance(binding, dict) else None
    return {
        "contract_template": phase.get("contract_template"),
        "prompt_template": phase.get("prompt_template"),
        "role_prompt_bindings": role_prompt_bindings,
    }


def provider_policy_for_hash(phase: dict[str, Any]) -> dict[str, Any]:
    providers: dict[str, Any] = {}
    for turn, binding in sorted(phase.get("role_bindings", {}).items()):
        if isinstance(binding, dict):
            providers[turn] = {
                "role_id": binding.get("role_id"),
                "model_policy": binding.get("model_policy"),
                "session_policy": binding.get("session_policy"),
            }
    return providers


def prompt_manifest_for_hash(config: dict[str, Any]) -> dict[str, Any]:
    return {
        "prompt_library_policy": config.get("prompt_library_policy"),
        "turn_templates": config.get("turn_templates"),
        "contract_template": config.get("contract_template"),
    }


def provider_manifest_for_hash(config: dict[str, Any]) -> dict[str, Any]:
    return {"session_defaults": config.get("session_defaults")}


def latest_plan_payload(events: list[dict[str, Any]]) -> dict[str, Any] | None:
    for event in reversed(events):
        if event.get("event_type") in PLAN_EVENTS and isinstance(event.get("payload"), dict):
            return event["payload"]
    return None


def latest_plan_payload_for_run(run_id: str) -> dict[str, Any] | None:
    return latest_plan_payload(load_events(RuntimePaths(run_id).events))


def load_validated_config(config_path: Path) -> dict[str, Any]:
    config = load_config(config_path.resolve())
    errors = validate_config(config, config_path.resolve())
    if errors:
        raise ValueError("; ".join(errors))
    return config


def assert_config_matches_adopted_plan(config: dict[str, Any], events: list[dict[str, Any]], run_id: str) -> None:
    plan = latest_plan_payload(events)
    if plan is None:
        return
    actual_hash = config_hash(config)
    if actual_hash == plan.get("config_hash"):
        return
    raise ValueError(
        f"run {run_id!r} config changed since plan_version={plan.get('plan_version')}; "
        "use plan diff and plan revise/fork instead of resume"
    )


def diff_plan(run_id: str, revised_config_path: Path) -> dict[str, Any]:
    events = load_events(RuntimePaths(run_id).events)
    if not events:
        raise ValueError(f"run {run_id!r} does not exist")
    current_plan = latest_plan_payload(events)
    if current_plan is None:
        current_config_path = Path(events[0]["payload"]["config_path"]).resolve()
        current_config = load_validated_config(current_config_path)
        current_plan = adopt_plan_payload(current_config_path, current_config)
    revised_config = load_validated_config(revised_config_path)
    revised_plan = adopt_plan_payload(revised_config_path, revised_config, current_plan["plan_version"] + 1)
    completed = completed_phase_keys_from_events(events, current_plan["phase_fingerprints"])
    current_key = current_phase_key_from_events(current_plan["phase_fingerprints"], completed)
    changes = classify_changes(
        current_plan,
        revised_plan,
        completed,
        current_key,
    )
    if current_plan["config_hash"] != revised_plan["config_hash"] and not changes:
        changes.append(change("run_configuration", current_key or "run", "current_restart_required"))
    return {
        "run_id": run_id,
        "from_plan_version": current_plan["plan_version"],
        "to_plan_version": revised_plan["plan_version"],
        "revision_class": revision_class(changes),
        "completed_phase_keys": sorted(completed),
        "current_phase_key": current_key,
        "changes": changes,
        "new_config_path": revised_plan["config_path"],
        "new_config_hash": revised_plan["config_hash"],
    }


def completed_phase_keys(projection: dict[str, Any], fingerprints: list[dict[str, Any]]) -> set[str]:
    keys_by_id = {item["phase_id"]: item["phase_key"] for item in fingerprints}
    completed: set[str] = set()
    for phase in projection.get("phases", []):
        if phase.get("status") == "complete" and phase.get("qa_status") == "passed":
            key = keys_by_id.get(phase.get("id"))
            if key:
                completed.add(key)
    return completed


def completed_phase_keys_from_events(
    events: list[dict[str, Any]],
    fingerprints: list[dict[str, Any]],
) -> set[str]:
    keys_by_id = {item["phase_id"]: item["phase_key"] for item in fingerprints}
    completed: set[str] = set()
    for event in events:
        if event.get("event_type") in PHASE_PROGRESS_EVENTS:
            phase_key = keys_by_id.get(event.get("phase_id"))
            if phase_key and progress_event_closes_phase(event["event_type"]):
                completed.add(phase_key)
        if event.get("event_type") == "external_phase_completed":
            phase_key = event.get("payload", {}).get("phase_key")
            if isinstance(phase_key, str):
                completed.add(phase_key)
    return completed


def progress_event_closes_phase(event_type: str) -> bool:
    return event_type in {
        "single_prompt_completed",
        "review_passed",
        "test_review_passed",
        "code_quality_review_passed",
    }


def current_phase_key(projection: dict[str, Any], fingerprints: list[dict[str, Any]]) -> str | None:
    current = projection.get("current")
    if not isinstance(current, dict):
        return None
    keys_by_id = {item["phase_id"]: item["phase_key"] for item in fingerprints}
    return keys_by_id.get(current.get("phase"))


def current_phase_key_from_events(fingerprints: list[dict[str, Any]], completed: set[str]) -> str | None:
    for fingerprint in fingerprints:
        if fingerprint["phase_key"] not in completed:
            return fingerprint["phase_key"]
    return None


def classify_changes(
    old_plan: dict[str, Any],
    new_plan: dict[str, Any],
    completed_keys: set[str],
    current_key: str | None,
) -> list[dict[str, Any]]:
    old_fingerprints = old_plan["phase_fingerprints"]
    new_fingerprints = new_plan["phase_fingerprints"]
    old_by_key = {item["phase_key"]: item for item in old_fingerprints}
    new_by_key = {item["phase_key"]: item for item in new_fingerprints}
    new_order = [item["phase_key"] for item in new_fingerprints]
    changes: list[dict[str, Any]] = []
    global_disposition = "current_restart_required" if current_key is not None else "future_only"
    if old_plan.get("prompt_manifest_hash") != new_plan.get("prompt_manifest_hash"):
        changes.append(change("modify_global_prompt_set", "*", global_disposition))
    if old_plan.get("provider_manifest_hash") != new_plan.get("provider_manifest_hash"):
        changes.append(change("modify_global_provider_policy", "*", global_disposition))
    last_completed_index = last_index(new_order, completed_keys)
    current_index = new_order.index(current_key) if current_key in new_order else None
    for key, old in old_by_key.items():
        new = new_by_key.get(key)
        if new is None:
            changes.append(change("delete_phase", key, "fork_required" if key in completed_keys else "future_only"))
            continue
        if old.get("ordinal") != new.get("ordinal"):
            changes.append(change("move_phase", key, phase_change_disposition(key, completed_keys, current_key)))
        for field in ("phase_hash", "prompt_binding_hash", "provider_policy_hash"):
            if old.get(field) == new.get(field):
                continue
            changes.append(change(f"modify_{field.removesuffix('_hash')}", key, phase_change_disposition(key, completed_keys, current_key)))
    for key in new_order:
        if key in old_by_key:
            continue
        new_index = new_order.index(key)
        if last_completed_index is not None and new_index <= last_completed_index:
            disposition = "fork_required"
        elif current_index is not None and new_index <= current_index:
            disposition = "current_restart_required"
        else:
            disposition = "future_only"
        changes.append(change("add_phase", key, disposition))
    return changes


def last_index(order: list[str], keys: set[str]) -> int | None:
    indexes = [order.index(key) for key in keys if key in order]
    return max(indexes) if indexes else None


def phase_change_disposition(phase_key: str, completed_keys: set[str], current_key: str | None) -> str:
    if phase_key in completed_keys:
        return "fork_required"
    if phase_key == current_key:
        return "current_restart_required"
    return "future_only"


def change(kind: str, phase_key: str, disposition: str) -> dict[str, str]:
    return {"kind": kind, "phase_key": phase_key, "disposition": disposition}


def revision_class(changes: list[dict[str, Any]]) -> str:
    dispositions = {item["disposition"] for item in changes}
    if "fork_required" in dispositions:
        return "fork_required"
    if "current_restart_required" in dispositions:
        return "current_restart_required"
    if changes:
        return "future_only"
    return "no_change"


def revise_plan(run_id: str, revised_config_path: Path, *, allow_current_restart: bool, reason: str) -> dict[str, Any]:
    from runner.facade.runtime_state import append_runtime_event_if_plan_version, refresh_projection_for_run

    diff = diff_plan(run_id, revised_config_path)
    if diff["revision_class"] == "fork_required":
        raise ValueError("plan revision touches completed history; use plan fork")
    if diff["revision_class"] == "current_restart_required" and not allow_current_restart:
        raise ValueError("plan revision changes the active cursor; pass --allow-current-restart or use plan fork")
    if diff["revision_class"] == "no_change":
        return diff
    revised_config = load_validated_config(revised_config_path)
    payload = adopt_plan_payload(revised_config_path, revised_config, diff["to_plan_version"])
    payload.update(
        {
            "from_plan_version": diff["from_plan_version"],
            "revision_class": diff["revision_class"],
            "changes": diff["changes"],
            "reason": reason,
        }
    )
    append_runtime_event_if_plan_version(
        RuntimePaths(run_id),
        "plan_revised",
        payload,
        diff["from_plan_version"],
    )
    refresh_projection_for_run(run_id)
    return diff


def fork_plan(
    parent_run_id: str,
    revised_config_path: Path,
    new_run_id: str,
    reason: str,
    *,
    resume_phase_key: str | None = None,
    resume_turn: str | None = None,
) -> dict[str, Any]:
    from runner.facade.runtime_state import initialize_runtime_events, refresh_projection_for_run
    from runner.authority.projections import project_run
    from runner.phase_programs import lower_phase_program

    parent_events = load_events(RuntimePaths(parent_run_id).events)
    if not parent_events:
        raise ValueError(f"parent run {parent_run_id!r} does not exist")
    revised_config = load_validated_config(revised_config_path)
    if (resume_phase_key is None) != (resume_turn is None):
        raise ValueError("plan fork resume requires both --resume-phase-key and --resume-turn")
    fork_payload: dict[str, Any] = {"parent_run_id": parent_run_id, "fork_reason": reason}
    if resume_phase_key is not None and resume_turn is not None:
        phase = next(
            (
                item
                for item in revised_config["phases"]
                if (item.get("phase_key") or f"phase_{item['id']}") == resume_phase_key
            ),
            None,
        )
        if phase is None:
            raise ValueError(f"resume phase key {resume_phase_key!r} does not exist in revised config")
        if not lower_phase_program(revised_config, phase).supports_turn(resume_turn):
            raise ValueError(f"resume turn {resume_turn!r} is not supported by phase {resume_phase_key!r}")
        # Forking is the authorized response to a changed config at the same
        # path. Do not reload the parent through config-hash admission here;
        # project its immutable event history onto the already validated,
        # phase-compatible revised plan solely to copy phase state.
        parent_projection = project_run(revised_config, parent_events, parent_run_id)
        parent_phase = next(
            (item for item in parent_projection["phases"] if item.get("phase_key") == resume_phase_key),
            None,
        )
        if parent_phase is None:
            raise ValueError(f"parent run has no phase state for {resume_phase_key!r}")
        fork_payload["resume_cursor"] = {"phase_key": resume_phase_key, "turn": resume_turn}
        fork_payload["phase_states"] = [
            {
                "phase_key": item["phase_key"],
                "status": item["status"],
                "qa_status": item["qa_status"],
                "notes": item["notes"],
            }
            for item in parent_projection["phases"]
            if item["id"] <= parent_phase["id"]
        ]
    paths = RuntimePaths(new_run_id)
    if paths.events.exists():
        raise ValueError(f"run {new_run_id!r} already exists")
    initialize_runtime_events(
        paths,
        [
            ("run_started", {"config_path": str(revised_config_path.resolve())}),
            ("plan_adopted", adopt_plan_payload(revised_config_path, revised_config)),
            ("run_forked", fork_payload),
        ],
    )
    refresh_projection_for_run(new_run_id)
    return {
        "parent_run_id": parent_run_id,
        "run_id": new_run_id,
        "config_path": str(revised_config_path.resolve()),
        "resume_cursor": fork_payload.get("resume_cursor"),
    }

