from __future__ import annotations

from runner.generation.scaffold_types import ScaffoldRequest


def scaffold_config(request: ScaffoldRequest) -> dict:
    program = "single_prompt" if request.kind == "single_prompt" else "standard_loop"
    config = {
        "schema_version": 1,
        "project": {"name": request.name, "cwd": str(request.project_root), "spec_file": request.spec_file, "context_files": [request.spec_file]},
        "prompt_library_policy": {
            "runner_asset_roots": ["automation/project_prompts/assets"],
            "runner_assembly_roots": ["automation/project_prompts/assemblies"],
            "consumer_asset_roots": ["automation/consumer_prompts/assets"],
            "consumer_assembly_roots": ["automation/consumer_prompts/assemblies"],
            "allow_consumer_prompts": True,
            "allow_direct_file_binding": False,
        },
        "turn_templates": {turn: {"assembly_id": "turns/default"} for turn in standard_turns()},
        "contract_template": {"asset_id": "contracts/default"},
        "session_defaults": {"provider": "codex", "model": "gpt-5", "reasoning_effort": "medium", "config": {}},
        "runner_control": {}, "loop_escalation": {"families": {"review_family": {"turns": ["review"], "threshold": 4, "action": "start_fresh_session"}}},
        "escalation_policy": {family: {"attempts": [], "on_exhausted": "notify"} for family in ("provider_crash", "invalid_outcome", "same_phase_loop_exceeded", "no_edit_stall")},
        "outcome_repair_policy": {family: {"max_attempts": 1, "first_attempt": "same_agent_event_repair_prompt", "on_exhausted": "route_to_recovery"} for family in ("missing_runner_event", "malformed_runner_event")},
        "operator_intervention_policy": {"allow_live_injection": True, "allow_immediate_interrupt": False, "record_as_authority_event": True, "default_injection_mode": "next_turn_preface", "default_post_injection_route": "continue_current_phase"},
        "phases": [{"id": 1, "title": request.name, "owner": "consumer", "scope": ["."], "acceptance": ["consumer scaffold"], "instructions": "implement the declared milestone", "qa_focus": "preserve runner authority", "program_id": program, "contract_template": {"asset_id": "contracts/default"}, "role_bindings": {turn: role_binding(turn) for turn in standard_turns()}}],
    }
    if request.kind == "single_prompt":
        phase = config["phases"][0]
        phase["prompt_template"] = {"asset_id": "turns/default"}
        phase["success_event_type"] = "single_prompt_completed"
        phase["role_bindings"] = {"single_prompt": role_binding("single_prompt")}
    if request.telegram:
        config["notification_policy"] = telegram_notification_policy()
    return config


def prompt_readme(request: ScaffoldRequest) -> str:
    return f"# {request.name} prompts\n\nRegister prompt assets and assemblies here; do not bind raw files.\n"


def standard_turns() -> tuple[str, ...]:
    return ("plan", "implement", "review", "repair", "test_review", "test_repair_implement", "code_quality_review", "code_quality_repair")


def role_binding(turn: str) -> dict:
    return {"role_id": "reviewer" if "review" in turn else "implementer", "model_policy": {"provider": "codex", "model": "gpt-5", "reasoning_effort": "medium"}, "session_policy": {"continuity_family": "default"}, "prompt_template": {"assembly_id": "turns/default"}}


def telegram_notification_policy() -> dict:
    signal_kinds = (
        "blocker",
        "crash",
        "invalid_outcome",
        "wall_timeout",
        "idle_timeout",
        "no_edit_stall",
        "same_phase_loop_exceeded",
    )
    return {
        "command_hook": ["python", "-m", "runner.telegram_bridge", "send"],
        "signals": {
            kind: {"enabled": True, "delivery": "immediate", "sinks": ["command_hook"]}
            for kind in signal_kinds
        },
    }
