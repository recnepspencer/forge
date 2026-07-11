# Configurable Escalation Engine

## Intent

Escalation is **general, configurable runner behavior**, not a milestone-bespoke
ladder. Any run can declare, per escalation family, an ordered ladder of typed
**stage actions** plus an operator-driven custom turn. The M1B "3 review fails
-> fresh session -> escalate repair model -> blocker" ladder is one *config* of
this engine, not a hardcoded path. Future runs may mix approaches freely.

## Model

An escalation **family** (e.g. `same_phase_loop_exceeded`, `provider_crash`,
`invalid_outcome`, `no_edit_stall`) owns:

- an ordered list of **stages**, each a typed action with action-specific params
- an **on_exhausted** stage, applied once the ladder is spent

The ladder is walked by `attempt_index` (count of `recovery_requested` events
for the phase/turn). Loop families additionally carry a **trigger**
(`loop_escalation.families.<f>` = turns + threshold). Fault families are
triggered by their fault event.

### Config surface (generalized, backward-compatible)

```json
"escalation_policy": {
  "same_phase_loop_exceeded": {
    "stages": [
      { "action": "start_fresh_session", "prompt": "recovery/escalated_fresh" },
      { "action": "override_model",
        "turns": ["repair", "test_repair_implement", "code_quality_repair"],
        "model_policy": { "provider": "codex", "model": "gpt-5.6-sol", "reasoning_effort": "high" },
        "scope": "phase" }
    ],
    "on_exhausted": { "action": "notify_and_pause", "signal": "blocker" }
  }
}
```

Backward compatibility is a hard requirement: the existing form
`"attempts": ["start_fresh_session", ...]` and `"on_exhausted": "notify"`
(strings) still parses. A bare string `s` lifts to `{ "action": s }` with empty
params. So every existing config keeps working unchanged; `stages` is the new
richer spelling.

### Action registry (extensible)

Each action kind declares required params, a validation rule, and a planner
effect. v1 kinds:

| action | params | planner effect |
|---|---|---|
| `same_session_recovery` | — | re-run current turn, same session |
| `start_fresh_session` | `prompt?` | re-run current turn, fresh session (session_reset re-arms the loop window) |
| `deep_reviewer_pass` | — | re-run via reviewer role, fresh |
| `override_model` | `turns`, `model_policy`, `scope` (`phase`\|`run`) | record a scoped model override for named turns; fresh session to re-arm |
| `notify` | `signal?`, `message?` | page operator, continue |
| `notify_and_pause` | `signal?`, `message?` | page operator, pause the run |

New kinds are added by registering a name + validator + effect — nothing else in
the ladder changes. This is the extensibility contract.

### Operator custom turn (model + instructions)

Triggered by an operator Telegram reply, not by a ladder position. Config:

```json
"operator_custom_turn": {
  "aliases": {
    "codex": { "provider": "codex", "command": "...", "model": "gpt-5.6-sol", "reasoning_effort": "high" },
    "grok":  { "provider": "grok",  "command": "...", "model": "grok-4.5" }
  },
  "default_alias": "grok",
  "reset_ladder": true,
  "max_ladders_per_phase": 2
}
```

Reply grammar: `<alias> <instructions...>` (or `<alias>: <instructions...>`).
The first token, if it matches a configured alias, selects the model; the
remainder is the **turn instructions**, injected as the custom turn's prompt
body. If the first token is not an alias, `default_alias` is used and the whole
reply is instructions. Instructions are required (a bare alias with no
instructions is rejected with a help reply) so the operator always says what to
do, not just which model.

The engine runs one turn with the chosen provider + instructions at the current
cursor, records its outcome, then returns to the standard program. It resets the
family's ladder window, subject to `max_ladders_per_phase`: once a phase has
consumed that many full ladders, the engine stops resetting, pages a blocker,
and stays paused rather than looping unattended.

## The `required_attempt_action` landmine (must fix first)

`maybe_handle_same_phase_loop` passes `required_attempt_action=loop_family.action`
into `plan_recovery_attempt`, and `loop_escalation` actions are constrained to
`{start_fresh_session}`. `plan_recovery_attempt` then does
`attempt_action = required_attempt_action or policy.attempts[prior_attempts]`, so
today the ladder is never walked — every loop recovery is forced to the one loop
action. Distinct stages require decoupling this: loop families walk
`stages[attempt_index]`; the loop `action` becomes the trigger's re-arm hint, not
an override. Preserve single-action behavior for configs that still want it.
This is shared recovery semantics and must land with the graph suite green.

## Hooks (located)

- config model + validation + admit: `phase_programs/policy_bindings.py`
  (`EscalationStage`, `EscalationFamilyPolicy.stages`, `SUPPORTED_ESCALATION_ACTIONS`).
- schema allowlist: `authority/config/schema.py` (`operator_custom_turn`,
  `model_escalation` -> folded into stage params, so no separate top-level key).
- ladder walk + action dispatch: `graph_runtime/continuation/recovery_planning.py`.
- side-effect events (`model_escalation_activated`): `recovery_events.py`,
  `authority/events/event_types.py`.
- scoped model override fold: `authority/projections/projector.py`
  (`projection["model_overrides"]`).
- model override applied at resolution: `roles/role_policy.py` +
  `graph_runtime/nodes/authority_nodes.select_role_session`.
- exhausted notify/pause + blocker: `recovery_disposition.py`,
  `notification_policy`.
- operator custom turn: `telegram_bridge` reply parse -> new operator continuation
  in `graph_runtime` -> resume standard.

## Goal mode (implemented)

`goal_mode` is a model-policy flag: when set, the adapter appends the provider's
self-verification loop so the turn drives itself to completion before handing
back to review. For grok this is `--check`; codex has no equivalent flag, so
goal mode is a no-op there. It is set two ways:

- **Config default**: any turn's `model_policy` may carry `"goal_mode": true`.
  The M1B repair turns default to goal mode (`GOAL_MODE_REPAIR`).
- **Operator toggle**: a Telegram reply prefixed with `goal` (e.g.
  `goal finish the repair`, `goal codex land the cutover`) runs that one custom
  turn in goal mode with the named/default model.

## Resumable operator pause (implemented)

When operator custom turns are configured, an exhausted ladder emits
`operator_pause` instead of `run_stopped`: the loop idles (paging once) without
exiting, and an operator reply resumes it automatically — the override consumes
the open fault so the custom turn runs rather than re-recovering. Without
operator custom turns configured, `notify_and_pause` still hard-stops.

## Staged, tested build

1. **Config model** (this increment): `EscalationStage` + `stages` +
   `SUPPORTED_ESCALATION_ACTIONS` + backward-compatible validation/admit, with
   `.attempts`/`.on_exhausted` compat properties so the planner is untouched.
   Unit tests: string form lifts to stages; object form parses; bad action /
   bad params rejected.
2. **Ladder walk + `override_model`:** decouple `required_attempt_action`; walk
   `stages[attempt_index]`; dispatch actions; emit + fold scoped model overrides;
   apply at `select_role_session`. Unit + graph tests for the 3-stage cadence and
   the override.
3. **Exhausted blocker + Telegram custom turn:** notify_and_pause message,
   alias+instructions parse, custom-turn continuation, reset + cap. Unit + graph
   tests.

Each stage lands on the `runner-escalation-engine` branch with the full runner
suite green before the next begins.
