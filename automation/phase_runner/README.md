# Phase Runner

This runner is intentionally simple:

- the model owns semantic phase truth
- the runner owns mechanical state integrity
- the model may not hand-edit runner JSON

The only legal state mutation surface is:

```powershell
python automation/phase_runner/state_tool.py apply <state-file> -
```

The model sends a small phase outcome payload on stdin. The state tool reads the
live state file, validates the requested transition against the current cursor,
commits the phase row change atomically, appends one completion history event,
and saves once.

## Validate

```powershell
python automation/phase_runner/runner.py `
  automation/phase_runner/worth-query-graph-authority-hardening.json `
  --validate
```

## Show Current Cursor

```powershell
python automation/phase_runner/state_tool.py show-current `
  automation/phase_runner/worth-query-graph-authority-hardening.json
```

## Apply A Phase Outcome

```powershell
@'
{
  "phase": 1,
  "completed_turn": "plan",
  "status": "in_progress",
  "qa_status": "not_started",
  "next_turn": "implement",
  "detail": "phase 1 plan committed",
  "notes": {
    "plan": ["phase plan posted in chat"]
  }
}
'@ | python automation/phase_runner/state_tool.py apply `
  automation/phase_runner/worth-query-graph-authority-hardening.json `
  -
```

## Dry Run

```powershell
python automation/phase_runner/runner.py `
  automation/phase_runner/worth-query-graph-authority-hardening.json `
  --dry-run
```

## Run

```powershell
python automation/phase_runner/runner.py `
  automation/phase_runner/worth-query-graph-authority-hardening.json `
  --loop `
  --sleep-seconds 30 `
  --log automation/phase_runner/worth-query-graph-authority-hardening.jsonl
```

The runner no longer performs Codex-authored recovery edits. If the model fails
to commit a legal outcome through `state_tool.py apply`, validation will fail
loudly instead of improvising state surgery.

## Boundaries

- `state_tool.py`: the only legal mutation surface
- `phase_update.py`: typed turn-commit rules
- `validation.py`: structural validation only
- `runner_runtime.py`: prompt/render/invoke loop
- `codex_cli.py`: Codex subprocess boundary

This keeps semantic authority with the model while removing ad hoc shell edits,
duplicate history writes, and recovery loops from the orchestration path.
