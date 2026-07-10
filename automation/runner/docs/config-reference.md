# Config Reference

## What This Feature Is

The runner config declares the project, prompt library, phase programs, role
bindings, recovery policy, notification policy, and operator intervention rules
for one run. Use this reference when editing generated config by hand.

## Stable Entry Points

From a source checkout, set `PYTHONPATH='automation/runner/src'` before running
these commands. Installed environments do not need that line.

```powershell
python -m runner.facade.cli validate automation/runner/config/my-run.json
python -m runner.facade.cli start automation/runner/config/my-run.json --loop
```

## Top-Level Sections

Required sections:

- `schema_version`
- `project`
- `prompt_library_policy`
- `turn_templates`
- `contract_template`
- `session_defaults`
- `loop_escalation`
- `escalation_policy`
- `outcome_repair_policy`
- `operator_intervention_policy`
- `phases`

Optional sections:

- `runner_control`
- `stall_policy`
- `qualifying_edit_policy`
- `notification_policy`

Unknown top-level keys are rejected.

## Project

```json
"project": {
  "name": "store-m1",
  "cwd": "C:/path/to/project",
  "spec_file": "docs/store-m1.md",
  "context_files": ["docs/store-m1.md"]
}
```

`cwd`, `spec_file`, and every `context_files` entry must exist.

## Prompt Library Policy

```json
"prompt_library_policy": {
  "runner_asset_roots": ["automation/project_prompts/assets"],
  "runner_assembly_roots": ["automation/project_prompts/assemblies"],
  "consumer_asset_roots": ["automation/consumer_prompts/assets"],
  "consumer_assembly_roots": ["automation/consumer_prompts/assemblies"],
  "allow_consumer_prompts": true,
  "allow_direct_file_binding": false
}
```

Prompt bindings must reference registered asset ids or assembly ids. Direct raw
file binding is rejected.

Generated configs use `automation/project_prompts/` for the local prompt library
and reserve `automation/consumer_prompts/` for later overlays. Do not configure
two prompt roots to the same path unless you want validation to reject ambiguous
prompt ids.

## Programs And Providers

Supported phase programs:

- `standard_loop`
- `implement_review_loop`
- `standard_single_pass_followups`
- `single_prompt`

Supported providers:

- `codex`
- `cursor`
- `grok`

`codex` model policies require `reasoning_effort`. `command`, `command_args`,
`config`, and `env` are available for provider adapters that need local process
customization.

Supported roles:

- `implementer`
- `reviewer`

## Notification Policy

Supported signal kinds:

- `blocker`
- `crash`
- `no_edit_stall`
- `same_phase_loop_exceeded`
- `run_completed`
- `invalid_outcome`
- `wall_timeout`
- `idle_timeout`
- `completion_handoff_failed`

Supported deliveries:

- `immediate`
- `queued`
- `final`

Supported sinks:

- `stdout`
- `file`
- `command_hook`

## Recovery And Intervention

Loop escalation actions:

- `start_fresh_session`

Escalation attempts:

- `same_session_recovery`
- `deep_reviewer_pass`
- `start_fresh_session`

Escalation exhausted actions:

- `notify`
- `notify_and_pause`

Outcome repair first attempts:

- `same_agent_event_repair_prompt`

Operator injection modes:

- `next_turn_preface`

Operator post-injection routes:

- `continue_current_phase`

## Phase Shape

Every phase needs:

- `id`
- `phase_key` is optional but recommended for plans that may be revised
- `title`
- `owner`
- `instructions`
- `qa_focus`
- `scope`
- `acceptance`
- `program_id`
- `contract_template`
- `role_bindings`

`single_prompt` phases also need:

- `prompt_template`
- `success_event_type`

Role bindings require `role_id`, `model_policy`, `session_policy`, and usually
`prompt_template`. `session_policy.continuity_family` is required on role
bindings.

## Small Example

Generate instead of writing this by hand:

```powershell
python -m runner.facade.cli generate single_prompt --name closeout --spec docs/closeout.md
python -m runner.facade.cli validate automation/runner/config/closeout.json
```

For a full generated flow, use [First Run From Zero](first-run-from-zero.md).
For provider-specific policy examples, use [Provider Setup](provider-setup.md).

## Minimal Generated Config Shape

This is the shape a generated single-prompt config is expected to have. Prefer
generating it, then editing the provider policy or phase text.

```json
{
  "schema_version": 1,
  "project": {
    "name": "runner-smoke-test",
    "cwd": "C:/path/to/workspace",
    "spec_file": "docs/runner-smoke.md",
    "context_files": ["docs/runner-smoke.md"]
  },
  "prompt_library_policy": {
    "runner_asset_roots": ["automation/project_prompts/assets"],
    "runner_assembly_roots": ["automation/project_prompts/assemblies"],
    "consumer_asset_roots": ["automation/consumer_prompts/assets"],
    "consumer_assembly_roots": ["automation/consumer_prompts/assemblies"],
    "allow_consumer_prompts": true,
    "allow_direct_file_binding": false
  },
  "turn_templates": {},
  "contract_template": {
    "success_event_type": "task_completed"
  },
  "session_defaults": {
    "provider": "codex",
    "model": "gpt-5",
    "reasoning_effort": "medium",
    "config": {}
  },
  "loop_escalation": {
    "max_same_phase_loops": 3,
    "action": "start_fresh_session"
  },
  "escalation_policy": {
    "attempts": ["same_session_recovery", "deep_reviewer_pass"],
    "exhausted_action": "notify_and_pause"
  },
  "outcome_repair_policy": {
    "first_attempt": "same_agent_event_repair_prompt",
    "max_attempts": 1
  },
  "operator_intervention_policy": {
    "allow_live_injection": true,
    "injection_mode": "next_turn_preface",
    "post_injection_route": "continue_current_phase"
  },
  "phases": [
    {
      "id": "phase_1",
      "title": "Complete requested work",
      "owner": "implementer",
      "instructions": "Read the spec and complete the requested work.",
      "qa_focus": "Confirm the requested output exists and is correct.",
      "scope": ["docs/runner-smoke.md"],
      "acceptance": ["The requested work is complete."],
      "program_id": "single_prompt",
      "contract_template": "default",
      "prompt_template": "single_prompt",
      "success_event_type": "task_completed",
      "role_bindings": {
        "implementer": {
          "role_id": "implementer",
          "model_policy": {
            "provider": "codex",
            "model": "gpt-5",
            "reasoning_effort": "medium"
          },
          "session_policy": {
            "continuity_family": "runner-smoke-test"
          },
          "prompt_template": "single_prompt"
        }
      }
    }
  ]
}
```

## Anti-Patterns

- Do not add unknown top-level sections.
- Do not bind direct prompt file paths.
- Do not reuse the same prompt root in runner and consumer lists; duplicate
  roots can make prompt ids ambiguous.
- Do not set `allow_direct_file_binding` to true.

## Related Docs

- [Consumer Runner Quickstart](consumer-runner-quickstart.md)
- [Provider Setup](provider-setup.md)
- [Run Lifecycle](run-lifecycle.md)
- [Telegram Operator Bridge](telegram-operator-bridge.md)
