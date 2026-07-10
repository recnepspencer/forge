from __future__ import annotations

STATIC_TOP_LEVEL_KEYS = {
    "schema_version",
    "project",
    "prompt_library_policy",
    "turn_templates",
    "contract_template",
    "session_defaults",
    "runner_control",
    "stall_policy",
    "qualifying_edit_policy",
    "loop_escalation",
    "escalation_policy",
    "outcome_repair_policy",
    "operator_intervention_policy",
    "notification_policy",
    "phases",
}

SUPPORTED_PROVIDERS = {"codex", "cursor", "grok"}

SHARED_PHASE_KEYS = {
    "id",
    "phase_key",
    "title",
    "owner",
    "instructions",
    "qa_focus",
    "scope",
    "acceptance",
    "program_id",
    "contract_template",
    "role_bindings",
}

STANDARD_PHASE_KEYS = SHARED_PHASE_KEYS
SINGLE_PROMPT_PHASE_KEYS = SHARED_PHASE_KEYS | {"prompt_template", "success_event_type"}
