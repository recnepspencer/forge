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

## Fast Verification

Use the fast Worth lane during implementation and review turns:

```powershell
.\scripts\ci\check_worth_fast_iteration.ps1
```

This lane compiles the `forge-query` and `worth-spatial` test surfaces, runs
their fast unit gates, and runs the `worth-spatial` compile-fail boundary target.
It intentionally does not execute whole-crate suites such as
`cargo test -p forge-query --tests`, `cargo test -p worth-spatial --tests`, or
the `worth-spatial` `public_api_contract` umbrella. Those are closeout lanes and
must be requested explicitly by a phase acceptance item.

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

The touched graph authority gate uses the same runner:

```powershell
python automation/phase_runner/runner.py `
  automation/phase_runner/worth-touched-graph-authority-gate.json `
  --loop `
  --sleep-seconds 30 `
  --log automation/phase_runner/worth-touched-graph-authority-gate.jsonl
```

By default, loop mode is recovery-aware. If validation, prompt rendering, or a
Codex turn fails, the runner sends a recovery prompt into the same persisted
Codex thread, records `runner_recovery_requested`, and then keeps looping after
the recovery turn exits successfully. The runner also writes a `.bak` copy
before each state save, so recovery can still resume the persisted Codex thread
when the live state file is malformed JSON. Use `--no-recover` when you want
local debugging to stop at the first runner failure.

## Boundary

The runner knows generic phase state:

- `not_started`, `in_progress`, `complete`, `regressed`, `blocked`
- `not_started`, `needed`, `in_progress`, `passed`, `failed`
- `plan`, `implement`, `review`, `repair`, and `close` template turns
- phase note buckets: `plan`, `done`, `remaining`, `findings`, and
  `verification`

It does not know what a crate, milestone, proof, or closeout means. Those belong
in the config and templates.

The bundled templates assume the spec owns the high-level phase order. A plan
turn creates an in-chat implementation plan with relevant context, the
adversarial constraint, DX target code block, directory skeleton, implicit
requirements, and a phase-relevant implementation sequence. The JSON state is
not the artifact of record for those details; it only tracks progress.

The required loop is phase done-ness:

- `review` runs the phase-done QA loop and sends genuinely incomplete work to
  `repair`.
- `repair` fixes the done-check findings and returns to `review`.
- in the original prompt set, `close` runs non-looping hardening passes after
  the phase-done loop passes.
- in the split test-hardening prompt set, the post-done sequence is
  `test_review -> test_repair_plan -> test_repair_implement -> code_quality_review`.
  That set separates review, planning, and implementation for test realism and
  structural QA so each turn has one job.

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

## Prompt Sets

The runner can use more than one prompt set.

### Original close-pass set

The bundled default templates are the compact five-turn set:

- `plan`
- `implement`
- `review`
- `repair`
- `close`

This keeps the phase loop small, but the `close` turn compresses test QA,
follow-up planning, implementation, and code-quality review into one prompt.

### Split test-hardening set

For better answer quality on hardening work, use the split post-done set:

- `plan`
- `implement`
- `review`
- `repair`
- `test_review`
- `test_repair_plan`
- `test_repair_implement`
- `code_quality_review`

Recommended template mapping:

```json
{
  "contract_template": "templates/_contract_test_hardening.md",
  "turn_templates": {
    "plan": "templates/plan.md",
    "implement": "templates/implement.md",
    "review": "templates/review_test_hardening.md",
    "repair": "templates/repair.md",
    "test_review": "templates/test_review.md",
    "test_repair_plan": "templates/test_repair_plan.md",
    "test_repair_implement": "templates/test_repair_implement.md",
    "code_quality_review": "templates/code_quality_review.md"
  }
}
```

This set drops the extra closeout/aerospace-grade prompt and ends the phase
after code-quality review records final verification and advances the cursor.

## Templates

Templates use simple `{dot.path}` variables. Lists render as bullets. Missing
variables are errors. There are intentionally no conditionals or embedded code.

Keep prompt content in templates, and keep project semantics in JSON fields.

### The contract token

Every turn template ends with `{contract}`, which renders `templates/_contract.md`
(override with a top-level `contract_template` field in the state file). The
contract is the shared, load-bearing half of every prompt. It carries the rules
that must be identical on every turn:

- the state-mutation protocol: read the state file fresh in the same command
  that writes it; mutate only the current phase row, cursor, `completed_at`, and
  small history entries; preserve everything else
- the rule that JSON is only lightweight progress state, not a place for logs,
  artifacts, command output tails, full plans, long findings, or proof
  transcripts
- the `status` / `qa_status` enums and the transition mapping
- the turn state machine and cursor-advancement rules: the runner sends exactly
  the cursor turn and infers nothing; the model advances `current` itself
- the rule that only phase done-ness loops; test-purity, directory polish, and
  aerospace-grade follow-up are close-pass hardening inputs, not loop conditions

The contract is rendered first, against the same context, so it can resolve
`{status_values}`, `{qa_status_values}`, and `{turns}`. Each turn template adds
only its turn-specific stance on top.
