from __future__ import annotations

from pathlib import Path
from typing import Any

from runner.authority.config.phase_validation import validate_phases
from runner.authority.config.prompt_library_policy_validation import validate_prompt_library_policy
from runner.authority.config.project_validation import validate_project_section
from runner.authority.config.qualifying_edit_policy_validation import validate_qualifying_edit_policy
from runner.authority.config.runner_control_validation import validate_runner_control
from runner.authority.config.schema import STATIC_TOP_LEVEL_KEYS
from runner.authority.config.stall_policy_validation import validate_stall_policy
from runner.authority.config.session_defaults_validation import validate_session_defaults
from runner.authority.config.template_validation import validate_contract_template, validate_turn_templates
from runner.phase_programs.policy_bindings import (
    validate_escalation_policy,
    validate_loop_escalation,
    validate_operator_custom_turn,
    validate_operator_intervention_policy,
    validate_outcome_repair_policy,
)
from runner.operator_signals.policies.validation import validate_notification_policy


def validate_config(config: dict[str, Any], config_path: Path) -> list[str]:
    errors: list[str] = []
    unknown = set(config.keys()) - STATIC_TOP_LEVEL_KEYS - {"_config_path"}
    if unknown:
        errors.append(f"unknown top-level keys: {sorted(unknown)}")
    if not isinstance(config.get("schema_version"), int):
        errors.append("schema_version must be a number")

    require_mapping(config, "project", errors)
    require_mapping(config, "prompt_library_policy", errors)
    require_mapping(config, "turn_templates", errors)
    require_mapping(config, "session_defaults", errors)
    require_mapping(config, "loop_escalation", errors)
    require_mapping(config, "escalation_policy", errors)
    require_mapping(config, "outcome_repair_policy", errors)
    require_mapping(config, "operator_intervention_policy", errors)

    project = config.get("project", {})
    if isinstance(project, dict):
        validate_project_section(project, errors)

    prompt_library_policy = config.get("prompt_library_policy", {})
    if isinstance(prompt_library_policy, dict):
        validate_prompt_library_policy(prompt_library_policy, errors)

    templates = config.get("turn_templates", {})
    if isinstance(templates, dict):
        validate_turn_templates(config, templates, errors)
    validate_contract_template(config, errors)

    session_defaults = config.get("session_defaults", {})
    if isinstance(session_defaults, dict):
        validate_session_defaults(session_defaults, errors)

    stall_policy = config.get("stall_policy")
    if stall_policy is not None and not isinstance(stall_policy, dict):
        errors.append("stall_policy must be an object when present")
    elif isinstance(stall_policy, dict):
        validate_stall_policy(stall_policy, errors)

    qualifying_edit_policy = config.get("qualifying_edit_policy")
    if qualifying_edit_policy is not None and not isinstance(qualifying_edit_policy, dict):
        errors.append("qualifying_edit_policy must be an object when present")
    elif isinstance(qualifying_edit_policy, dict):
        validate_qualifying_edit_policy(qualifying_edit_policy, errors)

    loop_escalation = config.get("loop_escalation", {})
    if isinstance(loop_escalation, dict):
        validate_loop_escalation(loop_escalation, errors)

    escalation_policy = config.get("escalation_policy", {})
    if isinstance(escalation_policy, dict):
        validate_escalation_policy(escalation_policy, errors)

    outcome_repair_policy = config.get("outcome_repair_policy", {})
    if isinstance(outcome_repair_policy, dict):
        validate_outcome_repair_policy(outcome_repair_policy, errors)

    operator_intervention_policy = config.get("operator_intervention_policy", {})
    if isinstance(operator_intervention_policy, dict):
        validate_operator_intervention_policy(operator_intervention_policy, errors)
    operator_custom_turn = config.get("operator_custom_turn")
    if operator_custom_turn is not None:
        if not isinstance(operator_custom_turn, dict):
            errors.append("operator_custom_turn must be an object when present")
        else:
            validate_operator_custom_turn(operator_custom_turn, errors)
    notification_policy = config.get("notification_policy")
    if notification_policy is not None:
        if not isinstance(notification_policy, dict):
            errors.append("notification_policy must be an object when present")
        else:
            validate_notification_policy(notification_policy, errors)

    runner_control = config.get("runner_control", {})
    if runner_control is not None and not isinstance(runner_control, dict):
        errors.append("runner_control must be an object when present")
    elif isinstance(runner_control, dict):
        validate_runner_control(runner_control, errors)

    phases = config.get("phases")
    if not isinstance(phases, list) or not phases:
        errors.append("phases must be a non-empty list")
    else:
        validate_phases(config, phases, runner_control, errors)
    return errors


def require_mapping(config: dict[str, Any], key: str, errors: list[str]) -> None:
    if not isinstance(config.get(key), dict):
        errors.append(f"{key} must be an object")
