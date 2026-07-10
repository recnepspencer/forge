# Consumer Runner Quickstart

## What This Feature Is

The consumer runner flow lets a downstream project start using the canonical
automation runner from its own workspace. Use it when a project wants durable
agent execution without copying runner internals or reviving legacy automation
scripts.

## Why You Use It

- Generate a legal run config and prompt home.
- Validate the config before starting an agent.
- Start a run with durable event history.
- Add Telegram blocker alerts without writing a transport adapter.

## Stable Entry Points

If this is a new workspace, read
[Installation And Prerequisites](installation-and-prerequisites.md) first.

From a source checkout, set `PYTHONPATH` first:

```powershell
$env:PYTHONPATH='automation/runner/src'
```

If the runner package is installed in the environment, skip that line.

```powershell
python -m runner.facade.cli generate milestone --name my-run --spec docs/milestone.md --telegram
python -m runner.facade.cli validate automation/runner/config/my-run.json
python -m runner.facade.cli start automation/runner/config/my-run.json --loop
python -m runner.telegram_bridge poll
```

Use `single_prompt` instead of `milestone` for one-off runs:

```powershell
python -m runner.facade.cli generate single_prompt --name closeout --spec docs/closeout.md
```

## Core Mental Model

The consumer owns config and prompts. The runner owns execution, event logging,
status projection, recovery, and notification routing.

Consumer-owned files:

```text
automation/runner/config/
automation/project_prompts/
```

Generated configs point runner prompt roots at `automation/project_prompts/`
and reserve `automation/consumer_prompts/` for later consumer overlays. Do not
point runner and consumer roots at the same directory; duplicate roots make
prompt ids ambiguous.

Runner runtime files:

```text
automation/runner/runtime/
```

The event ledger inside runtime is authority. Projections, logs, checkpoints,
Telegram state, and prompt instantiations are supporting state.

## How It Executes

1. `generate` writes a config, prompt assets, prompt assemblies, and recovery
   overlays.
2. `validate` checks config shape, prompt references, phase programs, role
   bindings, and notification policy.
3. `start` appends `run_started` and drives the graph.
4. `report`, `doctor`, and `status` inspect the run.
5. `archive` preserves the completed run record.

## Small Example

```powershell
python -m runner.facade.cli generate milestone --name store-m1 --spec docs/store-m1.md
python -m runner.facade.cli validate automation/runner/config/store-m1.json
```

This creates the smallest local runner setup without Telegram.

## Real Example

```powershell
python -m runner.facade.cli generate milestone --name store-m1 --spec docs/store-m1.md --telegram
python -m runner.facade.cli validate automation/runner/config/store-m1.json
python -m runner.facade.cli start automation/runner/config/store-m1.json --loop
python -m runner.telegram_bridge poll
```

Use this when the run should page the operator on blockers, crashes, stalls, or
invalid outcomes. Keep the Telegram poller running beside the runner.

## How It Relates To Other Features

- Use [Runner Reporting](runner-reporting.md) to summarize a run.
- Use [Telegram Operator Bridge](telegram-operator-bridge.md) for blocker
  replies.
- Use [Runtime Artifacts And Retention](runtime-artifacts-and-retention.md)
  after completion.

## Inspection And Debugging

```powershell
python -m runner.facade.cli report <run_id>
python -m runner.facade.cli doctor <run_id>
python -m runner.facade.cli artifacts <run_id>
```

If validation fails, fix the config or prompt reference before starting the run.
Do not patch runtime files to force the runner forward.

## Anti-Patterns

- Do not call legacy `automation/phase_runner` from new consumers.
- Do not copy runner internals into the consumer project.
- Do not bind raw prompt file paths. Use registered prompt assets and
  assemblies.
- Do not run multiple active writers in one worktree.

## Current Limits

- Parallel writers in one worktree are not supported without future lease
  authority.
- Consumers configure JSON directly today; there is no interactive config UI.
- Archive and prune are local filesystem operations.

## Related Docs

- [Installation And Prerequisites](installation-and-prerequisites.md)
- [First Run From Zero](first-run-from-zero.md)
- [Run Lifecycle](run-lifecycle.md)
- [Provider Setup](provider-setup.md)
- [Consumer Runner Rules](consumer-runner-rules.md)
- [Config Reference](config-reference.md)
- [Troubleshooting](troubleshooting.md)
