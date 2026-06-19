# Phase Runner

This is a reusable JSON-backed Codex phase runner. The runner owns the small
mechanics: state loading, cursor rendering, Codex invocation, thread capture,
and runner history. Project semantics live in the state file and prompt
templates.

## Validate

```powershell
python automation/phase_runner/runner.py `
  automation/phase_runner/worth-query-graph-authority-hardening.json `
  --validate
```

## Dry Run

```powershell
python automation/phase_runner/runner.py `
  automation/phase_runner/worth-query-graph-authority-hardening.json `
  --dry-run
```

Render a specific prompt without mutating state:

```powershell
python automation/phase_runner/runner.py `
  automation/phase_runner/worth-query-graph-authority-hardening.json `
  --dry-run `
  --phase 3 `
  --turn review
```

## Run

```powershell
python automation/phase_runner/runner.py `
  automation/phase_runner/worth-query-graph-authority-hardening.json `
  --log automation/phase_runner/worth-query-graph-authority-hardening.jsonl
```

Loop until completion or block:

```powershell
python automation/phase_runner/runner.py `
  automation/phase_runner/worth-query-graph-authority-hardening.json `
  --loop `
  --sleep-seconds 30 `
  --log automation/phase_runner/worth-query-graph-authority-hardening.jsonl
```

## Boundary

The runner knows generic phase state:

- `not_started`, `in_progress`, `complete`, `regressed`, `blocked`
- `not_started`, `needed`, `in_progress`, `passed`, `failed`
- `plan`, `implement`, `review`, `repair`, and `close` template turns

It does not know what a crate, milestone, proof, or closeout means. Those belong
in the config and templates.

The state file drives execution through an explicit cursor:

```json
{
  "current": {
    "phase": 1,
    "turn": "plan"
  }
}
```

The runner sends exactly that turn. It does not infer the next message from
status fields.

## Templates

Templates use simple `{dot.path}` variables. Lists render as bullets. Missing
variables are errors. There are intentionally no conditionals or embedded code.

Keep prompt content in templates, and keep project semantics in JSON fields.
