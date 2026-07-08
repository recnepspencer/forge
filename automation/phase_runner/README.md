# Durable Phase Runner

This runner now uses an append-only event log as authority.

- Static milestone config lives in `automation/phase_runner/config/`
- Authoritative runtime events live in `automation/phase_runner/runtime/events/`
- Operator-readable derived projections live in `automation/phase_runner/runtime/projections/`
- Optional raw Codex JSON stream logs live in `automation/phase_runner/runtime/logs/`

The model no longer mutates runner state files directly. Prompts end with a
single `RUNNER_EVENT: {...}` marker, and the orchestrator appends the
authoritative event after the turn result is interpreted.

## Commands

Validate a static config:

```powershell
python automation/phase_runner/runner.py validate `
  automation/phase_runner/config/worth-ui-milestone-3.3.json
```

Start a new run:

```powershell
python automation/phase_runner/runner.py start `
  automation/phase_runner/config/worth-ui-milestone-3.3.json `
  --run-id worthui33 `
  --loop `
  --sleep-seconds 30
```

Resume an existing run:

```powershell
python automation/phase_runner/runner.py resume worthui33 `
  --loop `
  --sleep-seconds 30
```

Show derived status:

```powershell
python automation/phase_runner/runner.py status worthui33
```

Stop a run without deleting authority:

```powershell
python automation/phase_runner/runner.py stop worthui33 `
  --reason "operator stop"
```

Import a legacy mutable-state runner into the durable format:

```powershell
python automation/phase_runner/runner.py import-legacy `
  automation/phase_runner/worth-ui-milestone-3.3.json `
  automation/phase_runner/config/worth-ui-milestone-3.3.json `
  --run-id worthui33import
```

## Boundary

The durable runner is still a single local process. It is not a workflow
service, replay engine, or distributed scheduler.

Its architectural contract is:

- config is static
- events are authoritative
- projection is derived
- Codex chat is the artifact of record for plans, findings, and explanations
- operator commands append lifecycle events instead of patching progress state

The runner also enforces a few operational guards:

- one active runner process per `run_id`
- serialized event-log appends per `run_id`
- recovery instead of blind rerun when a prior Codex turn finished but its outcome was not recorded
- idle and wall-clock turn timeouts through optional `runner_control.idle_timeout_seconds` and `runner_control.turn_timeout_seconds`
- optional fresh-session recovery through `runner_control.fresh_session_after_qa_repair_cycles`

## Fresh-Session Recovery

Long QA/repair loops can become anchored in one persistent agent session. A
runner config may set:

```json
{
  "runner_control": {
    "fresh_session_after_qa_repair_cycles": 4
  }
}
```

When the same phase records that many completed repair cycles without passing
QA, the runner appends a `session_reset` event, clears the persisted thread id
from the derived projection, and gives the next turn a short fresh-recovery
preface. The reset does not advance the phase and does not mark QA as passed;
it only makes the next agent invocation start from a fresh session with the
existing event log, projection, spec, and phase context as authority.

## Phase Ordering

Phase ids are labels. The configured `phases` array is the authoritative phase
sequence:

```text
phases[0] -> phases[1] -> ... -> phases[last]
```

The runner advances to the next configured phase by array order, not by
calculating `current_phase_id + 1`. This allows milestones to use native phase
ids such as `0`, sparse ids, inserted interruption phases, or other numbering
schemes without lying to the config.

Use `runner_control.phase_id_start` only as a guard for the first configured
phase id. If present, it must match `phases[0].id`.
