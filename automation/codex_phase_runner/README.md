# Codex Phase Runner

This runner drives a long Forge phase plan through a persistent Codex CLI
session. The JSON file is the state authority. The runner only decides which
prompt to send next.

## Run One Turn

```powershell
python automation/codex_phase_runner/codex_phase_runner.py `
  automation/codex_phase_runner/worth-query-hardening.json `
  --log automation/codex_phase_runner/worth-query-hardening.jsonl
```

## Run Until Done

```powershell
python automation/codex_phase_runner/codex_phase_runner.py `
  automation/codex_phase_runner/worth-query-hardening.json `
  --loop `
  --sleep-seconds 30 `
  --log automation/codex_phase_runner/worth-query-hardening.jsonl
```

With `--loop` and no `--max-turns`, the runner keeps sending prompts until the
JSON says all phases are complete or the current phase is blocked.

## Optional Safety Cap

```powershell
python automation/codex_phase_runner/codex_phase_runner.py `
  automation/codex_phase_runner/worth-query-hardening.json `
  --loop `
  --max-turns 3 `
  --sleep-seconds 30 `
  --log automation/codex_phase_runner/worth-query-hardening.jsonl
```

## Dry Run

```powershell
python automation/codex_phase_runner/codex_phase_runner.py `
  automation/codex_phase_runner/worth-query-hardening.json `
  --dry-run
```

## State Rules

- `session.thread_id` starts as `null`.
- The first Codex turn creates a persistent session and the runner stores the
  returned `thread_id`.
- Later turns use `codex exec resume <thread_id>`.
- Each phase must set `status` to one of:
  - `not_started`
  - `in_progress`
  - `complete`
  - `blocked`
  - `regressed`
- Each phase must set `qa_status` to one of:
  - `not_started`
  - `needed`
  - `in_progress`
  - `passed`
  - `failed`
- The runner advances only when the current phase has:
  - `status: complete`
  - `qa_status: passed`
- When the last phase reaches `status: complete` and `qa_status: passed`, the
  runner sets `current_phase` to `null` and writes top-level `completed_at`.
- Large `.jsonl` transcripts, runner stdout/stderr logs, and Python cache files
  are local artifacts. They are intentionally ignored and are not part of the
  state authority.

The agent may undo completion by setting a phase back to `regressed` or
`in_progress` if implementation or QA discovers a real issue.

## QA Posture

QA turns inject a short Sam Harris-level skepticism guard: the spec is
authority, passing tests are weak evidence, and QA should find fake proof,
authority leaks, and blockers to the current phase.

## Model

The Worth hardening state file is configured for:

```json
{
  "model": "gpt-5.5",
  "reasoning_effort": "medium"
}
```

The runner passes those values to Codex CLI on every turn.
