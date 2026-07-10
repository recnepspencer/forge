# Runner Reporting

## What This Feature Is

Runner reporting gives operators and agents a small set of commands for asking
what a run is doing, whether it is healthy, and what to do next. Use it instead
of reading runtime files by hand.

## Why You Use It

- Tell whether a run is active, stopped, completed, or unreadable.
- Detect common "looks running but is not healthy" states.
- Summarize a run for the human operator.
- Find active-looking runs before starting another writer in the same worktree.
- Archive completed work without losing the authoritative run story.

## Stable Entry Points

From a source checkout, set `PYTHONPATH='automation/runner/src'` before running
these commands. Installed environments do not need that line.

```powershell
python -m runner.facade.cli status <run_id>
python -m runner.facade.cli report <run_id>
python -m runner.facade.cli report <run_id> --json
python -m runner.facade.cli doctor <run_id>
python -m runner.facade.cli artifacts <run_id>
python -m runner.facade.cli active
python -m runner.facade.cli archive <run_id>
```

## Core Mental Model

`status` is the full machine-readable current view.

`report` is the operator summary. It answers: what state is this run in, what
was the latest event, and what should the operator do next?

`doctor` is the health check. It looks for notification failures, unhealthy
Telegram polling, stopped runs, and contradictory active state.
It exits nonzero when it finds an error-level issue.

`artifacts` explains retention. It names authority, derived state, checkpoints,
logs, locks, and archive bundles.

`active` lists runs that still look active or stopped from their event ledgers.

## How It Executes

Reporting reads authority first: config path plus event ledger. It refreshes the
derived projection, then adds diagnostic state such as Telegram poller health
and notification delivery failures.

This means a report can explain diagnostic failures without making diagnostics
the source of truth.

## Small Example

```powershell
python -m runner.facade.cli report worthyroad-m1
```

Example output:

```text
Run worthyroad-m1: active
Events: 42
Current: phase 6 review
Latest: provider crash during review
Next: continue monitoring current run
```

## Real Example

```powershell
python -m runner.facade.cli active
python -m runner.facade.cli doctor worthyroad-m1
python -m runner.facade.cli report worthyroad-m1 --json
```

Use this when multiple agents may be running. `active` tells you what runs still
need attention. `doctor` tells you whether a specific run has an obvious health
problem. JSON report output is intended for agents that need to summarize runner
state back to the human.

## Inspection And Debugging

If Telegram is not working, start with:

```powershell
python -m runner.facade.cli doctor <run_id>
python -m runner.facade.cli status <run_id>
```

`status` includes `telegram.poller_health` and
`telegram.latest_inbound_receipt`. `doctor` raises an error finding when the
poller reports unhealthy.

If logs are growing:

```powershell
python -m runner.facade.cli artifacts <run_id>
python -m runner.facade.cli archive <run_id>
```

## Anti-Patterns

- Do not infer run health from a process window alone.
- Do not inspect only checkpoints. They are continuity, not truth.
- Do not ask agents to summarize a run by opening random runtime files.
- Do not archive an active run just to make status quieter.

## Current Limits

- `doctor` reports current structural findings; it does not yet compute elapsed
  time since the last provider output.
- `active` scans local event ledgers only.
- `report` is local JSON/text output; there is no hosted dashboard yet.

## Related Docs

- [Sample Command Output](sample-command-output.md)
- [Runtime Artifacts And Retention](runtime-artifacts-and-retention.md)
- [Telegram Operator Bridge](telegram-operator-bridge.md)
