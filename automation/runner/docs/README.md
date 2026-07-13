# Automation Runner

The automation runner drives long local implementation runs while keeping a
durable record of what happened. Use it when an agent needs to work through a
milestone, recover from crashes or loops, and ask the operator for instructions
without losing the run story.

The runner is not a chat bot, CI service, or general scheduler. It is a local
orchestration tool with an event ledger as its source of truth. Everything else
is either execution continuity, diagnostics, or derived operator state.

## Start Here

- Starting from zero: read [Installation And Prerequisites](installation-and-prerequisites.md),
  then [First Run From Zero](first-run-from-zero.md).
- New downstream project: read
  [Consumer Runner Quickstart](consumer-runner-quickstart.md), then
  [Consumer Runner Rules](consumer-runner-rules.md).
- Running or supervising agents: read
  [Runner Operator Guide](runner-operator-guide.md), then
  [Runner Reporting](runner-reporting.md).
- Understanding run ids, `--loop`, start/resume/stop, and archive eligibility:
  read [Run Lifecycle](run-lifecycle.md).
- Changing phases, prompts, providers, or external completions after a run has
  started: read [Plan Revisions](plan-revisions.md).
- Configuring Codex, Cursor, or Grok: read [Provider Setup](provider-setup.md).
- Running multiple agents safely: read [Multi-Agent Worktrees](multi-agent-worktrees.md).
- Managing runtime bloat: read
  [Runtime Artifacts And Retention](runtime-artifacts-and-retention.md).
- Telegram setup and reply routing: read
  [Telegram Operator Bridge](telegram-operator-bridge.md).
- Config fields: read [Config Reference](config-reference.md).
- Running CAD Road 1 Milestone 1B: read
  [Road 1 M1B Runbook](road1-m1b-runbook.md).
- Incident response: read [Troubleshooting](troubleshooting.md).

## Stable Commands

From a source checkout, set `PYTHONPATH` before running module commands:

```powershell
$env:PYTHONPATH='automation/runner/src'
```

If the runner package is installed in the environment, the `PYTHONPATH` line is
not needed.

```powershell
python -m runner.facade.cli generate milestone --name my-run --spec docs/milestone.md --telegram
python -m runner.facade.cli validate automation/runner/config/my-run.json
python -m runner.facade.cli start automation/runner/config/my-run.json --loop
python -m runner.facade.cli resume <run_id> --loop
python -m runner.facade.cli status <run_id>
python -m runner.facade.cli report <run_id>
python -m runner.facade.cli doctor <run_id>
python -m runner.facade.cli artifacts <run_id>
python -m runner.facade.cli archive <run_id>
python -m runner.facade.cli active
python -m runner.facade.cli stop <run_id> --reason "operator stop"
python -m runner.facade.cli inject <run_id> --message "change direction"
```

Telegram polling is a separate long-running operator process:

```powershell
python -m runner.telegram_bridge poll
```

The docs assume commands are run from the workspace root: the directory that
contains `automation/runner/` and the project `docs/` paths used in examples.

## Core Mental Model

An event ledger is the authoritative record for a run. It lives at
`runtime/events/<run_id>.jsonl`. If the ledger says a run completed, stopped,
crashed, or received an operator instruction, that is the run truth.

A projection is a derived status file. It is useful for humans and tools, but it
can be rebuilt from the ledger and config.

A checkpoint is execution continuity. It helps the graph resume, but it is not
the public run record.

Logs, prompt instantiations, Telegram offsets, and notification receipts are
diagnostics or derived operator state. They explain what happened, but they do
not decide what happened.

## Anti-Patterns

- Do not edit projections, checkpoints, Telegram files, or logs to change run
  truth.
- Do not use legacy `automation/phase_runner` for new consumers.
- Do not copy runner internals into a downstream project.
- Do not treat Telegram as the authority for a run. Telegram is only a transport
  for alerts and operator replies.
- Do not run multiple active writers in one worktree unless the runner has a
  future lease authority that explicitly permits it.
