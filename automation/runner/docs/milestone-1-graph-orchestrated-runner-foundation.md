# Milestone 1: Graph-Orchestrated Runner Foundation

## Goal

Replace the current script-accumulated runner shape with a graph-orchestrated
runner substrate that can drive long implementation programs durably, recover
cleanly from crashes and review loops, separate prompt content from prompt use,
and notify the operator when progress has become suspicious.

## Why This Milestone Exists

The current runner already has the right instincts:

- static config
- append-only event authority
- derived projection
- resumable execution
- optional fresh-session recovery

What it does not yet have is a structure that scales cleanly when we add:

- custom prompt cycles for special phases
- multiple agent roles with different model policies
- cleaner handoffs between runs
- loop escalation rules stronger than ad hoc retries
- operator notifications for blockers, crashes, and stall conditions
- future judges, parallel specialists, and dependent runs

Without this milestone, every one of those additions lands in the same script
cluster and turns the runner into a coordination swamp.

## Governing Summaries

- `MENTALITY.md`
  - Protects adversarial-first thinking.
  - Strongest implication here: this milestone must solve the failure shape of
    long-running orchestration first, not bolt nicer prompts onto a fragile
    control loop.
- `arch_laws.md`
  - Protects subsystem autonomy, proof-bearing boundaries, and facade honesty.
  - Strongest implication here: prompt content, role policy, graph execution,
    operator signals, and persistence cannot remain one imperative blob.
- `composition_laws.md`
  - Protects semantic predictability of files and functions.
  - Strongest implication here: the new runner must be born as named
    responsibilities, not another big orchestrator file with extracted helpers.
- `domain_structure_laws.md`
  - Protects responsibility-shaped topology and authority-shaped boundaries.
  - Strongest implication here: prompt assets must live in a visible prompt
    authority boundary, and runtime instantiations must not share that space.
- `perf_laws.md`
  - Protects hot/cold path honesty, early rejection, and explicit measurement.
  - Strongest implication here: the ordinary runner loop must not repeatedly
    rediscover prompt selection, recovery policy, or notification policy on the
    hot path, and expensive rich-path artifacts must degrade by policy.
- `automation/phase_runner/README.md`
  - Protects the current durable-runner constitution: config static, events
    authoritative, projections derived, operator commands append lifecycle
    events, and fresh-session recovery is already a real need.
  - Strongest implication here: LangGraph must be incorporated without erasing
    the event-ledger authority boundary the current runner already got right.
- `automation/runner/docs/ROADMAP.md`
  - Protects the milestone sequence for the runner as its own subsystem.
  - Strongest implication here: Milestone 1 must freeze the constitutional
    split strongly enough that later judge, parallel, and importable-product
    milestones land into prepared seams instead of forcing another rewrite.

## Adversarial Constraint

The runner must survive a 100+ turn implementation program in which phases can
loop through review and repair repeatedly, providers can crash or exhaust
session continuity, prompts can vary by phase program, completion can hand off
to another run, and the operator may be absent for hours, while still
preserving one authoritative run story and surfacing a notification within a
configured bound whenever execution blocks, crashes, or appears stalled with no
meaningful code-edit progress.

For this milestone, "appears stalled" is not a vibe. It means the configured
stall detector can prove at least one of:

- no qualifying repository edit has occurred for `N` minutes while the run
  remains active
- the same review-family loop has repeated `K` times without advancing the
  phase
- the current turn exceeded a configured idle or wall-clock budget
- the agent/provider crashed or returned without a valid runner outcome

This milestone is correct only if those states become mechanically detectable,
recoverable, and operator-visible.

## Product Decision Lock

- The new runner is born under `automation/runner/`, not by further widening
  `automation/phase_runner/`.
- LangGraph Python Graph API is required for orchestration. Do not use the
  Functional API as the primary runner architecture because the runner needs
  explicit named nodes, subgraphs, conditional routing, and inspectable graph
  topology.
- Use `StateGraph` as the graph construction surface.
- Use `.add_node(...)`, `.add_edge(...)`, and `.add_conditional_edges(...)` to
  define named orchestration topology.
- Use `.compile(checkpointer=..., store=...)` to attach persistence surfaces.
- Use `Command` for state update plus routing decisions at graph boundaries.
- Use `interrupt()` only for real operator or approval pauses, not for ordinary
  phase progression.
- Use `stream_events(..., version="v3")` for runtime event consumption where
  stream visibility is needed.
- Preserve the current authority split:
  - static runner config is authoritative
  - append-only runner event ledger is authoritative
  - derived operator projection is derived
  - LangGraph checkpoint state is execution-continuity state, not public run
    authority
- Prompt text must live in prompt assets, not inline in Python, not inside
  per-phase config rows, and not inside runtime state.
- Prompt assemblies, prompt bindings, and prompt instantiations are distinct
  boundaries and must remain distinct.
- Consumers may supply their own prompt assets and prompt assemblies, but only
  through the same registry and binding path as runner-bundled prompts.
- Role policy is separate from prompt content.
- Notification policy is separate from recovery policy.
- Completion handoff remains a first-class path in this milestone; it is not a
  future nice-to-have.
- Missing or malformed outcome events must route through an explicit
  outcome-repair lane before broader recovery escalation. The runner must be
  able to prompt the acting agent to emit the missing or corrected event when
  the work itself may already be complete.
- Notification transport in runner core is `command_hook`. Telegram is handled
  through a project-local command-hook sink, not a Telegram-specific core sink.
- Stall detection is mandatory and configurable. The default configuration
  shape must support:
  - qualifying scoped file-edit stall threshold
  - phase-progress stall threshold
  - enable/disable per signal family
- Qualifying edit detection must default to project scope and must exclude
  runner-owned runtime artifacts such as logs, projections, checkpoints, and
  prompt instantiations.
- Loop escalation policy is configuration-owned, not turn-name hardcoded in
  runner core. The configuration may assign different thresholds and actions to
  different turn families.
- Crash visibility is mandatory. If a provider or runner crash occurs, the
  operator must be notified immediately even when automatic recovery is still
  enabled.
- Recovery-success notifications are off by default. Ordinary successful
  recovery should not page the operator.
- SQLite is the required persistence substrate for LangGraph checkpoints in
  this milestone.
- Prompt assets are authored as Markdown, with light metadata if needed.
- Operator escalation behavior is configuration-owned. A run may attempt
  deeper reviewer escalation before notifying and pausing.
- Crash-vs-pause behavior is configuration-owned per signal family. A crash may
  notify immediately and still continue through configured recovery attempts.
- Operator intervention without stopping the run is mandatory. The runner must
  support injection into the active run so the operator can redirect a bad
  agent without tearing down and restarting the run.

## Configuration Lock

The runner must freeze the first canonical configuration vocabulary in this
milestone. These names are not advisory.

Top-level operational policy sections must include:

- `stall_policy`
- `qualifying_edit_policy`
- `loop_escalation`
- `escalation_policy`
- `notification_policy`
- `outcome_repair_policy`
- `operator_intervention_policy`
- `prompt_library_policy`

The canonical shape for milestone 1 should be equivalent to:

```json
{
  "stall_policy": {
    "signals": {
      "no_edit_stall": {
        "enabled": true,
        "minutes_without_qualifying_edit": 20
      },
      "phase_progress_stall": {
        "enabled": true,
        "minutes_without_phase_progress": 45
      }
    }
  },
  "qualifying_edit_policy": {
    "include": [
      "crates/worth-*",
      "cad/docs/**",
      "automation/runner/**"
    ],
    "exclude": [
      "automation/runner/runtime/**",
      "automation/phase_runner/runtime/**",
      "**/*.runner.out.log",
      "**/*.runner.err.log"
    ],
    "proof_source": "git_scoped_diff",
    "early_detector": "filesystem_mtime"
  },
  "loop_escalation": {
    "families": {
      "review_family": {
        "turns": ["review", "code_quality_review"],
        "threshold": 4,
        "action": "start_fresh_session"
      }
    }
  },
  "escalation_policy": {
    "provider_crash": {
      "attempts": [
        "same_session_recovery",
        "deep_reviewer_pass"
      ],
      "on_exhausted": "notify_and_pause"
    },
    "same_phase_loop_exceeded": {
      "attempts": [
        "start_fresh_session",
        "deep_reviewer_pass"
      ],
      "on_exhausted": "notify_and_pause"
    },
    "no_edit_stall": {
      "attempts": [],
      "on_exhausted": "notify"
    }
  },
  "outcome_repair_policy": {
    "missing_runner_event": {
      "max_attempts": 1,
      "first_attempt": "same_agent_event_repair_prompt",
      "on_exhausted": "route_to_recovery"
    },
    "malformed_runner_event": {
      "max_attempts": 1,
      "first_attempt": "same_agent_event_repair_prompt",
      "on_exhausted": "route_to_recovery"
    }
  },
  "operator_intervention_policy": {
    "allow_live_injection": true,
    "default_injection_mode": "next_turn_preface",
    "allow_immediate_interrupt": true,
    "default_post_injection_route": "continue_current_phase",
    "record_as_authority_event": true
  },
  "prompt_library_policy": {
    "runner_asset_roots": [
      "automation/runner/prompts/assets",
      "automation/runner/prompts/assemblies"
    ],
    "consumer_asset_roots": [
      "automation/project_prompts/assets",
      "automation/project_prompts/assemblies"
    ],
    "allow_consumer_prompts": true,
    "allow_direct_file_binding": false
  },
  "notification_policy": {
    "signals": {
      "crash": {
        "enabled": true,
        "delivery": "immediate"
      },
      "no_edit_stall": {
        "enabled": true,
        "delivery": "immediate"
      },
      "phase_progress_stall": {
        "enabled": true,
        "delivery": "immediate"
      },
      "recovery_succeeded": {
        "enabled": false
      },
      "completion_handoff_failed": {
        "enabled": false
      }
    }
  }
}
```

This milestone must reject unknown top-level policy sections and unknown
signal-family keys unless the owning registry explicitly declares them.

## Phase Plan

### Phase 1: Canonical Runner Authority

This phase freezes the authoritative objects and the runner directory skeleton.
It makes the wrong storage shape structurally visible before any graph rebuild
begins.

**Relevant subsystems**
- runner facade
- run authority
- projection publication
- LangGraph persistence boundary

**Relevant APIs**
- current `automation/phase_runner/README.md` authority contract
- LangGraph `StateGraph.compile(checkpointer=..., store=...)`
- current `runner.py` start/resume/stop/status lifecycle

**Warnings**
- Do not let LangGraph checkpoint state become the only source of truth for run
  progress. That would collapse execution continuity into public authority.
- Do not keep authored prompt text or role policy inside `config/*.json`.
- Do not let runtime artifacts, authored prompts, and operator projections
  share the same directory space.
- Do not treat SQLite checkpoint state as a replacement for the event ledger.

**Test requirements**
- Adversarial equivalence test: replaying the same run ledger into the derived
  projection must produce the same phase cursor, status, and notification
  candidates even when checkpoints are absent.
- Adversarial denial test: deleting LangGraph checkpoint state must not destroy
  the authoritative run story or make status reconstruction impossible.

**Engineering decisions**
- Birth this skeleton:

```text
automation/runner/
  docs/
    ROADMAP.md
    milestone-1-graph-orchestrated-runner-foundation.md
  pyproject.toml
  src/runner/
    facade/
      commands/
      status.py
    authority/
      config/
      events/
      projections/
      run_identity/
    graph_runtime/
      state/
      nodes/
      edges/
      subgraphs/
      checkpoints/
    prompt_library/
    roles/
    phase_programs/
    recovery/
    operator_signals/
    generation/
    adapters/
      codex/
      cursor/
      grok/
  runtime/
    events/
    projections/
    checkpoints/
    instantiations/
    notifications/
```

- `facade/` owns public CLI and package entry points.
- `authority/events/` owns the canonical runner event envelope and append
  surfaces.
- `authority/projections/` owns derived operator-readable status publication.
- `graph_runtime/checkpoints/` owns LangGraph checkpointer integration only.
- `graph_runtime/checkpoints/` uses SQLite in this milestone.

**Open questions**
- None. The authority split must be frozen now.

### Phase 2: Prompt Authority Library

This phase freezes prompt authorship as its own subsystem. It separates authored
prompt text from reusable assemblies, from phase bindings, and from runtime
instantiations.

**Relevant subsystems**
- prompt library
- prompt binding
- prompt rendering
- runtime instantiation store

**Relevant APIs**
- prompt asset registry
- prompt assembly registry
- prompt binding resolver
- prompt renderer

**Warnings**
- Do not keep prompts in Python string literals except for tiny invariant
  wrappers that point at authored assets.
- Do not let phase config choose arbitrary files from disk. It must bind to
  registered prompt assets or assemblies by id.
- Do not let runtime prompt instantiations mutate authored assets.

**Test requirements**
- Adversarial equivalence test: two phase bindings that resolve to the same
  assembly plus the same inputs must render byte-identical prompt
  instantiations.
- Adversarial denial test: a phase config that references an unknown prompt
  asset, unknown assembly, or missing required input must fail before any agent
  turn begins.

**Engineering decisions**
- Freeze this prompt topology:

```text
src/runner/prompt_library/
  assets/
    shared/
    implementer/
    reviewer/
    operator/
  assemblies/
    standard_loop/
    boundary_review/
    code_quality_loop/
    single_prompt/
    completion_handoff/
  bindings/
    phases/
    programs/
    roles/
  rendering/
    context_schema.py
    renderer.py
    overlays.py
  instantiations/
    recorder.py
    loader.py
  registry.py
```

- `assets/` owns authored prompt text and local metadata only.
- Prompt-library registration must distinguish:
  - runner-bundled prompt assets and assemblies
  - consumer-local prompt assets and assemblies
  while resolving both through the same registry contract.
- `assemblies/` own ordered prompt compositions such as standard review/repair
  cycles.
- `bindings/` connect a phase program or role to registered assets or
  assemblies.
- `rendering/` owns interpolation and policy overlays.
- `instantiations/` writes rendered prompt artifacts under
  `runtime/instantiations/<run_id>/...`.
- A phase may bind either:
  - one prompt assembly id
  - one prompt asset id for a single-prompt program
  - one role-specific override binding on top of its phase-program binding
- Prompt assets are Markdown files. If metadata is needed, it must be light
  frontmatter rather than a separate heavyweight authored schema.
- Consumer-supplied prompt assets are allowed in Milestone 1, but phase config
  must reference them by registered ids. Direct raw file-path binding is not
  allowed.
- The runner should reserve a consumer prompt home such as:

```text
automation/project_prompts/
  assets/
  assemblies/
```

  or an equivalent configured root, but the consumer path still resolves
  through registry-owned admission rather than ad hoc filesystem lookup.

**Open questions**
- None. The separation between assets, assemblies, bindings, and
  instantiations is mandatory.

### Phase 3: Minimal Role Registry And Session Policy

This phase freezes the minimal role surface for Milestone 1, how each role is
configured, and where model/session policy lives.

**Relevant subsystems**
- role registry
- provider adapters
- model policy
- session policy

**Relevant APIs**
- role registry facade
- provider session factory
- role model policy resolver
- run/session identity mapping

**Warnings**
- Do not infer role behavior from prompt names.
- Do not bury provider/model policy inside generic session defaults once roles
  exist.
- Do not force every role to reuse the same session continuity policy.

**Test requirements**
- Adversarial equivalence test: the same run phase bound to different roles
  must resolve to the same phase program while receiving role-specific prompt
  and session policy.
- Adversarial denial test: a role binding that names an unsupported provider,
  missing model policy, or invalid session reuse policy must be rejected before
  graph execution starts.

**Engineering decisions**
- The first-class role registry must include:
  - `implementer`
  - `reviewer`
- Milestone 1 must not birth additional roles. Test-review, quality-review,
  judge, operator-proxy, and parallel specialist roles belong to later runner
  milestones if they remain necessary.
- Freeze this topology:

```text
src/runner/roles/
  registry.py
  role_ids.py
  role_policy.py
  model_policy.py
  session_policy.py
  handoff_policy.py
```

- `model_policy.py` chooses provider/model/reasoning profile per role.
- `session_policy.py` chooses reuse, reset, and fresh-session thresholds per
  role and per phase-program family.
- `handoff_policy.py` governs whether a later node may inherit the prior role's
  persistent session, must force a new session, or must switch provider.
- Current `session_defaults` becomes a seed input only. It is no longer the
  final authority once role policy exists.
- The default stance is session continuity first: preserve the same session
  whenever it remains healthy and policy allows it.
- Deeper root-cause escalation in Milestone 1 must still route through the
  `reviewer` role, using a different prompt assembly or escalation posture
  rather than birthing a third role.

**Open questions**
- None. Milestone 1 only needs the minimal role surface.

### Phase 4: Phase Programs And Prompt Bindings

This phase freezes execution programs as named orchestration products rather
than scattered conditionals.

**Relevant subsystems**
- phase programs
- phase binding resolver
- current config schema replacement

**Relevant APIs**
- program registry
- program input schema
- phase-to-program binder
- outcome transition contract
- outcome-repair policy registry

**Warnings**
- Do not keep execution behavior encoded as ad hoc `if execution_mode == ...`
  branches inside a central orchestrator.
- Do not let a custom phase bypass the standard outcome contract.
- Do not let prompt specialization imply program specialization when the
  lifecycle is the same.

**Test requirements**
- Adversarial equivalence test: standard-loop phases with different prompt
  assemblies must produce the same state-transition envelope when their outcome
  event is the same.
- Adversarial denial test: an invalid phase-program declaration with missing
  required turns, unknown outcome edges, or illegal role assignment must fail
  schema validation before run start.

**Engineering decisions**
- Freeze these first-class phase programs:
  - `standard_loop`
  - `single_prompt`
  - `boundary_review_loop`
  - `code_quality_loop`
  - `completion_handoff`
  - `recovery_replay`
- Freeze this topology:

```text
src/runner/phase_programs/
  registry.py
  standard_loop/
  single_prompt/
  boundary_review_loop/
  code_quality_loop/
  completion_handoff/
  recovery_replay/
```

- Each program owns:
  - legal turns
  - legal outcomes
  - default role bindings
  - allowed prompt-binding forms
  - loop and reset policy hooks
- This milestone must support configuration-owned loop escalation such as:

```json
{
  "loop_escalation": {
    "families": {
      "review_family": {
        "turns": ["review", "code_quality_review"],
        "threshold": 4,
        "action": "start_fresh_session"
      }
    }
  }
}
```

- This milestone must also support configuration-owned escalation routing such
  as:

```json
{
  "escalation_policy": {
    "provider_crash": {
      "attempts": [
        "same_session_recovery",
        "deep_reviewer_pass"
      ],
      "on_exhausted": "notify_and_pause"
    },
    "same_phase_loop_exceeded": {
      "attempts": [
        "start_fresh_session",
        "deep_reviewer_pass"
      ],
      "on_exhausted": "notify_and_pause"
    },
    "no_edit_stall": {
      "attempts": [],
      "on_exhausted": "notify"
    }
  }
}
```

- This milestone must support explicit outcome-repair policy such as:

```json
{
  "outcome_repair_policy": {
    "missing_runner_event": {
      "max_attempts": 1,
      "first_attempt": "same_agent_event_repair_prompt",
      "on_exhausted": "route_to_recovery"
    }
  }
}
```

- The runner must support special phases that use a different prompt cycle
  without pretending they are standard-loop phases.

**Open questions**
- None. Phase-program authority must exist before graph lowering begins.

### Phase 5: LangGraph Orchestration Graph

This phase lowers the runner into named graph topology.

**Relevant subsystems**
- graph runtime
- phase-program subgraphs
- provider execution nodes
- transition routing
- operator injection routing

**Relevant APIs**
- LangGraph `StateGraph`
- `add_node`
- `add_edge`
- `add_conditional_edges`
- `Command`
- `.compile(checkpointer=..., store=...)`
- `SqliteSaver` for durable local development checkpoints

**Warnings**
- Do not rebuild the graph around one giant "run turn" node. That would port
  the current blob into a new library.
- Do not put provider-specific execution logic directly into routing nodes.
- Do not let execution nodes rediscover prompt bindings, role policy, or loop
  policy that earlier nodes already resolved.
- Do not model operator intervention as a stop-and-restart-only workflow when
  live injection is allowed by policy.

**Test requirements**
- Adversarial equivalence test: resuming the same `thread_id` after an
  interrupt or crash must continue from the same graph boundary rather than
  replaying earlier successful lowering decisions.
- Adversarial denial test: a malformed or incomplete lowered phase program must
  be rejected before the provider-execution node is reached.
- Adversarial intervention test: an operator injection recorded during an
  active run must be consumed as the next routing input without losing run
  authority or restarting the run.

**Engineering decisions**
- Use the Python Graph API, not the Functional API.
- Freeze a top-level graph with named nodes equivalent to:

```text
load_run_authority
-> lower_phase_program
-> select_role_session
-> materialize_prompt_instantiation
-> execute_role_turn
-> classify_turn_outcome
-> route_outcome_repair_or_recovery
-> append_runner_event
-> publish_projection
-> evaluate_escalation
-> route_next_step
```

- `lower_phase_program` returns a proof-bearing lowered program packet.
- `execute_role_turn` receives only lowered execution input, never raw config.
- `route_outcome_repair_or_recovery` must distinguish:
  - valid outcome event
  - missing outcome event
  - malformed outcome event
  - provider/runtime failure before outcome extraction
- Program-specific subgraphs may be compiled separately and attached as nodes
  when they share state keys; otherwise use adapter nodes with explicit input
  and output mapping.
- Use `Command` when a node must both update state and route to a new node.
- Compile with `SqliteSaver` for persistent local development checkpoints and a
  store boundary reserved for longer-lived cross-run artifacts.

**Open questions**
- None. The graph topology is the heart of this milestone.

### Phase 6: Recovery, Loop Reset, And Session Cutover

This phase freezes how the runner detects bad continuity and what it does next.

**Relevant subsystems**
- recovery
- loop detection
- session reset
- provider failure classification
- outcome repair
- operator intervention

**Relevant APIs**
- current `fresh_session_after_qa_repair_cycles`
- LangGraph checkpoint continuation via `thread_id`
- LangGraph `interrupt()`
- LangGraph `Command`
- LangGraph `TimeoutPolicy`
- LangGraph node `error_handler`
- LangGraph `RunControl.request_drain()`
- provider adapter failure envelope
- escalation policy registry
- outcome-repair policy registry
- operator intervention registry

**Warnings**
- Do not conflate crash recovery with review-loop escalation. They have
  different causes and different operator meaning.
- Do not solve repetition by silently skipping a turn family.
- Do not restart into a fresh session without preserving the authoritative run
  story and the reason for reset.
- Do not hardcode escalation behavior by turn name in runner core once
  configuration-owned escalation policy exists.
- Do not treat missing-event repair as the same thing as root-cause recovery.
  The first asks for a corrected outcome artifact; the second changes the
  actual problem-solving path.

**Test requirements**
- Adversarial equivalence test: after four same-phase review-family loops, the
  runner must start a fresh session and continue the same phase honestly
  without losing event authority or duplicating prior completed work.
- Adversarial denial test: if a provider returns without a valid outcome event,
  the runner must classify the failure, record it, and route into recovery
  rather than pretending the turn completed.
- Adversarial escalation test: when a configured deeper-reviewer escalation pass
  is enabled before notify-and-pause, the runner must honor that ordering and
  notify only after configured escalation attempts are exhausted.
- Adversarial outcome-repair test: when the acting agent completed work but
  failed to emit a valid `RUNNER_EVENT`, the runner must prompt the same agent
  to repair the missing or malformed event before escalating to broader
  recovery.
- Adversarial operator-injection test: when the operator injects a correction
  such as "stop doing X and build Y instead", the runner must record the
  injection, preface the next turn with it, and continue the active run without
  stop/restart churn.

**Engineering decisions**
- Freeze the recovery families:
  - `provider_crash`
  - `invalid_outcome`
  - `missing_outcome_event`
  - `malformed_outcome_event`
  - `wall_timeout`
  - `idle_timeout`
  - `same_phase_loop_exceeded`
  - `no_edit_stall`
  - `operator_blocker_pause`
  - `usage_exhaustion`
- `same_phase_loop_exceeded` must support thresholds by turn family, including
  configurable review-family thresholds such as 4.
- Provider adapters must classify explicit usage exhaustion as its own recovery
  family rather than flattening it into generic provider crash.
- `qualifying_edit_policy` must use filesystem mtime as the cheap early
  detector and git-scoped diff as the proof-bearing confirmation surface.
- Outcome-repair policy must give the acting agent one explicit chance to emit
  the missing or corrected outcome event before routing to broader recovery,
  unless configuration disables that lane.
- A fresh-session reset must:
  - append a canonical session-reset event
  - clear the persistent session binding for the affected role or run
  - preserve the same run id and authoritative event ledger
  - emit a recovery preface into the next prompt instantiation
- Use checkpoint continuation for ordinary crash/restart recovery.
- Use `TimeoutPolicy` to encode node-level idle and wall-clock budgets where
  the runtime can enforce them honestly.
- Use node `error_handler` hooks to lower exhausted retries into typed recovery
  events rather than raw process failure.
- Use `RunControl.request_drain()` for cooperative operator stop and graceful
  suspension.
- Use a new persistent agent session only when policy says the old one is no
  longer trustworthy or useful.
- The default escalation order should preserve same-session recovery first when
  the session is still healthy.
- The second escalation surface, when configured, should route through the
  `reviewer` role in a deeper escalation posture rather than through an untyped
  "smarter model" escape hatch.
- Operator injection must be represented as an authoritative run event, not as
  an ephemeral console-side hack.
- The default live-injection mode should preface the next turn of the active
  phase rather than forcing a run restart.
- Crash recovery attempts do not suppress immediate crash notification; the
  operator should know a crash occurred even if later recovery succeeds.
- Successful recovery after a crash, timeout, or invalid outcome does not emit
  a second operator notification by default.

**Open questions**
- None. This milestone must make session reset a designed policy, not folklore.

### Phase 7: Operator Signals And Notification Policy

This phase freezes how the runner tells the human that something needs
attention.

**Relevant subsystems**
- operator signals
- notifier sinks
- runtime health detectors
- notification policy

**Relevant APIs**
- notification policy registry
- signal detector registry
- notifier sink interface
- current stop/status lifecycle
- command-hook sink contract

**Warnings**
- Do not make the notification transport the policy authority.
- Do not fire notifications from deep provider adapters.
- Do not notify on every loop by default; notify on structurally significant
  events and configured thresholds.
- Do not emit "recovered successfully" notifications by default.

**Test requirements**
- Adversarial equivalence test: the same canonical crash event must yield the
  same signal classification and sink fanout regardless of whether it was
  observed through live streaming or replayed from the event ledger.
- Adversarial denial test: a run that remains healthy and actively editing code
  must not emit false-positive stall notifications.
- Adversarial signal-suppression test: successful automatic recovery after a
  crash must not emit a second recovery-success notification when policy says
  crash and stall are the only operator-visible signal families.

**Engineering decisions**
- Freeze first-class signal kinds:
  - `blocker`
  - `crash`
  - `invalid_outcome`
  - `wall_timeout`
  - `idle_timeout`
  - `no_edit_stall`
  - `same_phase_loop_exceeded`
  - `run_completed`
  - `completion_handoff_failed`
- Freeze a notifier contract shape:

```text
signal -> policy resolution -> sink fanout
```

- Freeze this topology:

```text
src/runner/operator_signals/
  signal_types.py
  detectors/
  policies/
  sinks/
    stdout_sink.py
    file_sink.py
    command_hook_sink.py
  dispatcher.py
```

- `command_hook_sink` is the mandatory extensibility seam for Telegram,
  texting, webhooks, or other project-local notification transports without
  hardcoding a provider into runner core.
- The hook input format must be structured JSON, not plain-text interpolation.
- The hook is responsible for final Telegram message formatting.
- The canonical command-hook payload must be equivalent to:

```json
{
  "signal_id": "sig_01",
  "signal_kind": "crash",
  "delivery": "immediate",
  "run_id": "worthyroad1m1",
  "project_name": "worthy",
  "phase_id": 6,
  "phase_title": "Recovery, Loop Reset, And Session Cutover",
  "turn": "review",
  "role": "implementer",
  "provider": "codex",
  "model": "gpt-5.4-medium",
  "thread_id": "thread_abc123",
  "summary": "Provider crash during review turn; recovery will be attempted.",
  "details": {
    "failure_family": "provider_crash",
    "attempt_index": 1,
    "attempts_remaining": 1,
    "minutes_without_qualifying_edit": null,
    "minutes_without_phase_progress": null,
    "turn_instance_id": "turn_123",
    "event_log_file": "C:/.../automation/runner/runtime/events/run.jsonl",
    "projection_file": "C:/.../automation/runner/runtime/projections/run.json"
  },
  "actions": {
    "operator_policy": "notify_then_continue_recovery",
    "next_automatic_step": "same_session_recovery"
  },
  "occurred_at": "2026-07-08T12:34:56Z"
}
```

- The Telegram hook contract should be:
  - executable path configured in runner config or environment
  - receives the JSON payload on stdin
  - exits nonzero on delivery failure
  - must not become the authority of whether a signal exists
- The runner must support configuration such as:
  - notify on crash immediately
  - notify when no qualifying code edit is observed for `X` minutes while a run
    remains active
  - optionally notify when a completion handoff could not start
  - suppress recovery-success notifications by default
- The first project-local notification adapter should target Telegram through
  the command-hook sink.

**Open questions**
- None. Operator visibility is a core outcome of this milestone.

### Phase 8: Consumer Configuration And Runner Generation

This phase freezes how projects declare runs without touching runner internals.

**Relevant subsystems**
- config schema
- scaffold generation
- runtime-root policy
- consumer API

**Relevant APIs**
- config loader and validator
- scaffold generator
- runtime root resolver
- public CLI facade

**Warnings**
- Do not let every worktree improvise its own runner layout.
- Do not make consumers author raw LangGraph wiring.
- Do not expose internal directories as required consumer knowledge.

**Test requirements**
- Adversarial equivalence test: generating a run scaffold and then validating it
  without edits must produce a legal, runnable configuration with the expected
  directory skeleton.
- Adversarial denial test: a consumer config that attempts to bind an
  unregistered role, unregistered program, or illegal prompt binding must fail
  fast with typed errors.
- Adversarial consumer-prompt test: a consumer-supplied prompt asset admitted
  through a configured consumer prompt root must resolve the same way as a
  runner-bundled prompt asset once both are registered, while a direct raw file
  path must be rejected.

**Engineering decisions**
- Freeze a consumer-facing split:
  - authored runner package under `automation/runner/`
  - consumer prompt roots under a project-local prompt home such as
    `automation/project_prompts/`
  - project-local run configs under `automation/runner/config/`
  - runtime artifacts under `automation/runner/runtime/`
- Freeze scaffold generation outputs for:
  - new roadmap/milestone implementation run
  - special single-prompt closeout phase
  - chained completion handoff into a follow-on run
- Public CLI commands must remain simple:
  - `validate`
  - `start`
  - `resume`
  - `status`
  - `stop`
  - `generate`
- The config validator must reject:
  - unknown policy section names
  - unknown signal family names
  - unknown escalation attempt names
  - unknown command-hook delivery kinds
  - illegal include/exclude path roots
  - direct prompt file bindings that bypass prompt registration
- Consumers configure:
  - project scope and context files
  - phase programs
  - prompt bindings
  - role policies
  - loop/reset thresholds
  - qualifying-edit include/exclude rules
  - stall thresholds
  - escalation ordering
  - notification policies
  - completion handoff policy

**Open questions**
- None. The runner must be consumable without source archaeology.

## Must Ship

- `automation/runner/` as the new canonical runner home
- a runner roadmap and this milestone spec
- a responsibility-shaped directory skeleton
- event authority preserved with projection derivation preserved
- LangGraph graph runtime integrated through named nodes and subgraphs
- prompt library with assets, assemblies, bindings, and instantiations
- role registry with model and session policy
- minimal two-role surface only: `implementer` and `reviewer`
- first-class phase programs
- loop escalation with fresh-session reset after configured repetition,
  including a review-family threshold of 4
- operator signals and notifier sinks, including no-edit stall detection
- completion handoff as a first-class path

## Must Preserve

- static config as authority
- append-only event ledger as authority
- derived status projection as derived
- simple CLI entry commands
- current durable runner's recovery honesty
- ability to run special phases with custom prompt logic
- ability to resume from an existing run id without lying about prior work

## Acceptance Evidence

- a legal runner roadmap at `automation/runner/docs/ROADMAP.md`
- this milestone spec with phase-dominant design content
- a generated runner scaffold that validates cleanly
- proof tests for:
  - prompt binding denial
  - role policy denial
  - same-phase review-loop session reset at threshold 4
  - provider crash recovery
  - no-edit stall detection
  - completion handoff
  - projection equivalence from event replay

## Sequencing Notes

- This milestone belongs first in the runner roadmap because every later runner
  capability depends on these boundaries.
- Multi-agent parallelism, judges, and cross-run dependency graphs should not
  be built before prompt authority, role policy, graph topology, and operator
  signals are real.
