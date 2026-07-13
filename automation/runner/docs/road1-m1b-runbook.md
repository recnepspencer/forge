# Road 1 Milestone 1B Runbook

M1B runs the Query Constitution Enforcement milestone defined by
`cad/docs/road-1/road-1_milestone-1b.md`. Its generated configuration is
`automation/runner/config/road1-m1b-query-constitution.json`; regenerate it
from `automation/runner/config/build_road1_m1b.py` rather than hand-editing it.

## Normal-mode policy

M1B deliberately uses normal provider turns. `GOAL_MODE_REPAIR` is false, so
the three repair turns and every other configured turn run without Codex's
experimental `goals` feature or Grok's `--check` self-verification loop. Goal
mode remains available only as an explicit operator custom-turn choice using a
`goal`-prefixed Telegram reply.

M1B enables immediate Telegram notifications for crash, blocker, invalid
outcome, timeout, stall, and review-loop signals.

Before starting, verify the generated configuration and policy:

```powershell
$env:PYTHONPATH='automation/runner/src'
python automation/runner/config/build_road1_m1b.py
python -m runner.facade.cli validate automation/runner/config/road1-m1b-query-constitution.json
python -c "import json; c=json.load(open('automation/runner/config/road1-m1b-query-constitution.json')); p=[b['model_policy'] for x in c['phases'] for b in x['role_bindings'].values()]; p.append(c['escalation_policy']['same_phase_loop_exceeded']['stages'][1]['model_policy']); assert not any(x.get('goal_mode', False) for x in p); print('M1B goal mode: disabled')"
```

## Prompt sequence

The standard loop uses these versioned assemblies. Each assembly combines a
runner-owned turn prompt with the listed consumer handoff overlay when present.

| Turn | Assembly | Prompt parts |
| --- | --- | --- |
| Boundary review | `turns/boundary_review_m1b` | `turns/boundary_review` + `handoff/boundary_enforcement` |
| Plan | `turns/plan_m1b` | `turns/plan` + `handoff/write_plan` |
| Implement | `turns/implement_m1b` | `turns/implement` + `handoff/read_plan` |
| Review | `turns/review_m1b` | `turns/review` + `handoff/write_findings` |
| Repair | `turns/repair_m1b` | `turns/repair` + `handoff/read_findings` |
| Test review | `turns/test_review_m1b` | `turns/test_review` |
| Test repair | `turns/test_repair_implement_m1b` | `turns/test_repair_implement` |
| Code-quality review | `turns/code_quality_review_m1b` | `turns/code_quality_review` |
| Code-quality repair | `turns/code_quality_repair_m1b` | `turns/code_quality_repair` |

The first turn is boundary review. It runs `boundary-check` and
`agent-context check` before planning; a constitutional diagnostic blocks the
phase. The plan and review handoffs are written to `.runner-handoff/phase-N/`
so the separate build and oversight sessions receive the full plan or findings.

## Prompt audit

The M1B assemblies are explicit and no direct prompt-file bindings are allowed.
Their shared prompt assets use WORTH platform terminology.

## Start

After the prompt audit is accepted and validation is green:

```powershell
$env:PYTHONPATH='automation/runner/src'
python -m runner.facade.cli start automation/runner/config/road1-m1b-query-constitution.json --loop
```
