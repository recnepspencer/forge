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

### The contract token

Every turn template ends with `{contract}`, which renders `templates/_contract.md`
(override with a top-level `contract_template` field in the state file). The
contract is the shared, load-bearing half of every prompt — it carries the rules
that must be identical on every turn:

- the state-mutation protocol (read the state file fresh in the same command
  that writes it; mutate only the current phase row, cursor, `completed_at`, and
  history; preserve everything else)
- the `status` / `qa_status` enums and the transition mapping
- the turn state machine and cursor-advancement rules (the runner sends exactly
  the cursor turn and infers nothing — the model advances `current` itself)
- the rule that acceptance checks are *run* and their evidence recorded, and
  that a status is a claim backed by recorded evidence rather than an assertion

The contract is rendered first, against the same context, so it can resolve
`{status_values}`, `{qa_status_values}`, and `{turns}`. Each turn template adds
only its turn-specific stance on top.
