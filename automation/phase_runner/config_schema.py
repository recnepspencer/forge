from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from event_types import EVENT_TYPES
from phase_execution import (
    SINGLE_PROMPT_MODE,
    SINGLE_PROMPT_SUCCESS_EVENT_TYPES,
    STANDARD_LOOP_MODE,
    STANDARD_OPTIONAL_TURNS,
    STANDARD_REQUIRED_TURNS,
    SUPPORTED_EXECUTION_MODES,
)

STATIC_TOP_LEVEL_KEYS = {
    "schema_version",
    "project",
    "turn_templates",
    "contract_template",
    "session_defaults",
    "runner_control",
    "phases",
}

SUPPORTED_PROVIDERS = {"codex", "cursor", "grok"}
SHARED_PHASE_KEYS = {
    "id",
    "title",
    "owner",
    "instructions",
    "qa_focus",
    "scope",
    "acceptance",
    "execution_mode",
    "contract_template",
}
STANDARD_PHASE_KEYS = SHARED_PHASE_KEYS
SINGLE_PROMPT_PHASE_KEYS = SHARED_PHASE_KEYS | {"prompt_template", "success_event_type"}


def load_config(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8-sig") as config_file:
        config = json.load(config_file)
    config["_config_path"] = str(path.resolve())
    return config


def validate_config(config: dict[str, Any], config_path: Path) -> list[str]:
    errors: list[str] = []
    unknown = set(config.keys()) - STATIC_TOP_LEVEL_KEYS - {"_config_path"}
    if unknown:
        errors.append(f"unknown top-level keys: {sorted(unknown)}")

    if not isinstance(config.get("schema_version"), int):
        errors.append("schema_version must be a number")
    require_mapping(config, "project", errors)
    require_mapping(config, "turn_templates", errors)
    require_mapping(config, "session_defaults", errors)

    project = config.get("project", {})
    if isinstance(project, dict):
        cwd = project.get("cwd")
        if not isinstance(cwd, str) or not cwd:
            errors.append("project.cwd is required")
        elif not Path(cwd).exists():
            errors.append(f"project.cwd does not exist: {cwd}")
        name = project.get("name")
        if not isinstance(name, str) or not name:
            errors.append("project.name is required")
        spec_file = project.get("spec_file")
        if not isinstance(spec_file, str) or not spec_file:
            errors.append("project.spec_file is required")
        else:
            spec_path = resolve_project_path(project, spec_file)
            if not spec_path.exists():
                errors.append(f"project.spec_file does not exist: {spec_path}")
        context_files = project.get("context_files")
        if not isinstance(context_files, list) or not context_files:
            errors.append("project.context_files must be a non-empty list")
        else:
            for context_file in context_files:
                if not isinstance(context_file, str) or not context_file:
                    errors.append("project.context_files entries must be non-empty strings")
                    continue
                context_path = resolve_project_path(project, context_file)
                if not context_path.exists():
                    errors.append(f"project.context_files entry does not exist: {context_path}")

    templates = config.get("turn_templates", {})
    if isinstance(templates, dict):
        for turn in STANDARD_REQUIRED_TURNS:
            template_name = templates.get(turn)
            if not isinstance(template_name, str) or not template_name:
                errors.append(f"turn_templates.{turn} must name a template file")
                continue
            template_path = resolve_config_path(config_path, template_name)
            if not template_path.exists():
                errors.append(f"template not found for {turn}: {template_path}")
        for turn in STANDARD_OPTIONAL_TURNS:
            template_name = templates.get(turn)
            if template_name is None:
                continue
            if not isinstance(template_name, str) or not template_name:
                errors.append(f"turn_templates.{turn} must name a template file when present")
                continue
            template_path = resolve_config_path(config_path, template_name)
            if not template_path.exists():
                errors.append(f"template not found for {turn}: {template_path}")

    contract_template = config.get("contract_template")
    if not isinstance(contract_template, str) or not contract_template:
        errors.append("contract_template is required")
    else:
        contract_path = resolve_config_path(config_path, contract_template)
        if not contract_path.exists():
            errors.append(f"contract template not found: {contract_path}")

    session_defaults = config.get("session_defaults", {})
    if isinstance(session_defaults, dict):
        provider = session_defaults.get("provider", "codex")
        if not isinstance(provider, str) or provider not in SUPPORTED_PROVIDERS:
            errors.append(f"session_defaults.provider must be one of {sorted(SUPPORTED_PROVIDERS)}")
        model = session_defaults.get("model")
        if not isinstance(model, str) or not model:
            errors.append("session_defaults.model is required")
        command = session_defaults.get("command")
        if command is not None and (not isinstance(command, str) or not command):
            errors.append("session_defaults.command must be a non-empty string when present")
        command_args = session_defaults.get("command_args")
        if command_args is not None:
            if not isinstance(command_args, list) or not all(
                isinstance(item, str) and item for item in command_args
            ):
                errors.append("session_defaults.command_args must be an array of non-empty strings")
        reuse_session = session_defaults.get("reuse_session")
        if reuse_session is not None and not isinstance(reuse_session, bool):
            errors.append("session_defaults.reuse_session must be a boolean when present")
        effort = session_defaults.get("reasoning_effort")
        if provider == "codex" and (not isinstance(effort, str) or not effort):
            errors.append("session_defaults.reasoning_effort is required for codex provider")
        if effort is not None and (not isinstance(effort, str) or not effort):
            errors.append("session_defaults.reasoning_effort must be a non-empty string when present")
        config_map = session_defaults.get("config")
        if config_map is None:
            session_defaults["config"] = {}
        elif not isinstance(config_map, dict):
            errors.append("session_defaults.config must be an object")
        env_map = session_defaults.get("env")
        if env_map is not None:
            if not isinstance(env_map, dict) or not all(
                isinstance(key, str) and isinstance(value, str)
                for key, value in env_map.items()
            ):
                errors.append("session_defaults.env must be an object with string keys and values")

    runner_control = config.get("runner_control", {})
    if runner_control is not None and not isinstance(runner_control, dict):
        errors.append("runner_control must be an object when present")
    elif isinstance(runner_control, dict):
        validate_optional_nonnegative_int(runner_control, "phase_id_start", errors)
        validate_optional_positive_int(runner_control, "stop_before_phase", errors)
        validate_optional_nonnegative_int(runner_control, "boundary_review_start_phase", errors)
        validate_optional_positive_int(runner_control, "turn_timeout_seconds", errors)
        validate_optional_positive_int(runner_control, "idle_timeout_seconds", errors)
        validate_optional_positive_int(runner_control, "fresh_session_after_qa_repair_cycles", errors)
        validate_optional_nonnegative_int(runner_control, "repair_plan_start_sequence", errors)
        stop_reason = runner_control.get("stop_reason")
        if stop_reason is not None and (not isinstance(stop_reason, str) or not stop_reason):
            errors.append("runner_control.stop_reason must be a non-empty string when present")
        validate_completion_handoff(runner_control.get("completion_handoff"), errors)

    phases = config.get("phases")
    if not isinstance(phases, list) or not phases:
        errors.append("phases must be a non-empty list")
    else:
        phase_ids: set[int] = set()
        for index, phase in enumerate(phases):
            prefix = f"phases[{index}]"
            if not isinstance(phase, dict):
                errors.append(f"{prefix} must be an object")
                continue
            phase_id = phase.get("id")
            if not isinstance(phase_id, int) or phase_id < 0:
                errors.append(f"{prefix}.id must be a non-negative integer")
            elif phase_id in phase_ids:
                errors.append(f"{prefix}.id duplicates phase id {phase_id}")
            else:
                phase_ids.add(phase_id)
            for key in ("title", "owner", "instructions", "qa_focus"):
                if not isinstance(phase.get(key), str) or not phase.get(key):
                    errors.append(f"{prefix}.{key} is required")
            for key in ("scope", "acceptance"):
                if not isinstance(phase.get(key), list) or not phase.get(key):
                    errors.append(f"{prefix}.{key} must be a non-empty list")
            mode = phase.get("execution_mode", STANDARD_LOOP_MODE)
            if not isinstance(mode, str) or mode not in SUPPORTED_EXECUTION_MODES:
                errors.append(
                    f"{prefix}.execution_mode must be one of {sorted(SUPPORTED_EXECUTION_MODES)}"
                )
                continue
            validate_phase_keys(prefix, phase, mode, errors)
            phase_contract = phase.get("contract_template")
            if phase_contract is not None:
                if not isinstance(phase_contract, str) or not phase_contract:
                    errors.append(f"{prefix}.contract_template must be a non-empty string when present")
                else:
                    template_path = resolve_config_path(config_path, phase_contract)
                    if not template_path.exists():
                        errors.append(f"template not found for {prefix}.contract_template: {template_path}")
            if mode == SINGLE_PROMPT_MODE:
                prompt_template = phase.get("prompt_template")
                if not isinstance(prompt_template, str) or not prompt_template:
                    errors.append(f"{prefix}.prompt_template is required for single_prompt phases")
                else:
                    template_path = resolve_config_path(config_path, prompt_template)
                    if not template_path.exists():
                        errors.append(f"template not found for {prefix}.prompt_template: {template_path}")
                success_event_type = phase.get("success_event_type")
                if not isinstance(success_event_type, str) or not success_event_type:
                    errors.append(f"{prefix}.success_event_type is required for single_prompt phases")
                elif success_event_type not in EVENT_TYPES:
                    errors.append(f"{prefix}.success_event_type must be a known runner event type")
                elif success_event_type not in SINGLE_PROMPT_SUCCESS_EVENT_TYPES:
                    errors.append(
                        f"{prefix}.success_event_type must be one of {sorted(SINGLE_PROMPT_SUCCESS_EVENT_TYPES)}"
                    )
        configured_start = runner_control.get("phase_id_start") if isinstance(runner_control, dict) else None
        if isinstance(configured_start, int) and phases:
            first_phase = phases[0]
            if isinstance(first_phase, dict) and first_phase.get("id") != configured_start:
                errors.append(
                    "runner_control.phase_id_start must match the first configured phase id"
                )

    return errors


def require_mapping(config: dict[str, Any], key: str, errors: list[str]) -> None:
    if not isinstance(config.get(key), dict):
        errors.append(f"{key} must be an object")


def validate_optional_positive_int(config: dict[str, Any], key: str, errors: list[str]) -> None:
    value = config.get(key)
    if value is not None and (not isinstance(value, int) or value <= 0):
        errors.append(f"runner_control.{key} must be a positive integer when present")


def validate_optional_nonnegative_int(config: dict[str, Any], key: str, errors: list[str]) -> None:
    value = config.get(key)
    if value is not None and (not isinstance(value, int) or value < 0):
        errors.append(f"runner_control.{key} must be a non-negative integer when present")


def validate_phase_keys(prefix: str, phase: dict[str, Any], mode: str, errors: list[str]) -> None:
    allowed = SINGLE_PROMPT_PHASE_KEYS if mode == SINGLE_PROMPT_MODE else STANDARD_PHASE_KEYS
    unknown = set(phase.keys()) - allowed
    if unknown:
        errors.append(f"{prefix} has unknown keys for {mode}: {sorted(unknown)}")


def validate_completion_handoff(value: Any, errors: list[str]) -> None:
    if value is None:
        return
    if not isinstance(value, dict):
        errors.append("runner_control.completion_handoff must be an object when present")
        return
    next_run_id = value.get("next_run_id")
    if not isinstance(next_run_id, str) or not next_run_id:
        errors.append("runner_control.completion_handoff.next_run_id is required")
    loop = value.get("loop")
    if loop is not None and not isinstance(loop, bool):
        errors.append("runner_control.completion_handoff.loop must be a boolean when present")
    sleep_seconds = value.get("sleep_seconds")
    if sleep_seconds is not None and (not isinstance(sleep_seconds, int) or sleep_seconds <= 0):
        errors.append("runner_control.completion_handoff.sleep_seconds must be a positive integer when present")
    log = value.get("log")
    if log is not None and (not isinstance(log, str) or not log):
        errors.append("runner_control.completion_handoff.log must be a non-empty string when present")
    reason = value.get("reason")
    if reason is not None and (not isinstance(reason, str) or not reason):
        errors.append("runner_control.completion_handoff.reason must be a non-empty string when present")


def resolve_config_path(config_path: Path, value: str) -> Path:
    path = Path(value)
    if path.is_absolute():
        return path
    return config_path.parent.parent / path


def resolve_project_path(project: dict[str, Any], value: str) -> Path:
    path = Path(value)
    if path.is_absolute():
        return path
    return Path(project["cwd"]) / path
