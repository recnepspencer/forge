# Runner Operator Guide

## What This Feature Is

The operator guide is the day-to-day playbook for supervising runner agents.
Use it when you need to start, inspect, redirect, stop, resume, or archive a
run without opening internal runtime files.

## Why You Use It

- See which runs need attention.
- Reply to blockers through Telegram.
- Inject direction into an active run.
- Tell whether a run is healthy or merely looks alive.
- Archive completed runs before logs become clutter.

## Stable Entry Points

From a source checkout, set `PYTHONPATH='automation/runner/src'` before running
these commands. Installed environments do not need that line.

```powershell
python -m runner.facade.cli active
python -m runner.facade.cli stability-canary
python -m runner.facade.cli report <run_id>
python -m runner.facade.cli doctor <run_id>
python -m runner.facade.cli status <run_id>
python -m runner.facade.cli inject <run_id> --message "operator instruction"
python -m runner.facade.cli stop <run_id> --reason "operator stop"
python -m runner.facade.cli resume <run_id> --loop
python -m runner.facade.cli archive <run_id>
```

## Core Mental Model

The runner has an active cursor: the phase and turn currently being worked.
Operator instructions may only target that cursor. This prevents a Telegram
reply from accidentally steering the wrong agent or an old phase.

Telegram replies become `operator_override` events in the ledger. The next
prompt receives the instruction as a preface. Telegram itself is not authority;
the event ledger is.

## How It Executes

Typical operator loop:

1. Run `active` to see active or stopped runs.
2. Run `report <run_id>` for a human summary.
3. Run `doctor <run_id>` if anything looks wrong.
4. Reply to Telegram alerts or use `inject` for manual direction.
5. Run `archive <run_id>` after completion.

`doctor` exits nonzero when it finds an error-level health issue. That is
intentional so agents and scripts can stop instead of summarizing a bad run as
healthy.

Run `stability-canary` after changing provider execution, checkpoint recovery,
or process ownership. It proves that completed results are reused, interrupted
executions are not relaunched, and recovery prompts receive distinct execution
authority even when they retain the semantic turn id.

Run ids are execution histories, not config names. If you do not know the run
id, start with `active`. See [Run Lifecycle](run-lifecycle.md) for start,
resume, stop, no-loop, and archive rules.

## Small Example

```powershell
python -m runner.facade.cli active
python -m runner.facade.cli report worthyroad-m1
```

This answers the first operator question: what is still alive and what does it
need next?

## Real Example

```powershell
python -m runner.facade.cli doctor worthyroad-m1
python -m runner.facade.cli inject worthyroad-m1 --message "Stop broad refactors. Finish only the Telegram E2E proof."
python -m runner.facade.cli resume worthyroad-m1 --loop
```

Use this when an agent is drifting but the run cursor is still valid. The
instruction is recorded as run authority and consumed by the next prompt.

## How It Relates To Other Features

- Telegram is the remote operator channel.
- `doctor` is the health check.
- `archive` is the safe closeout path.
- `status` is the full JSON view for deeper inspection.

## Inspection And Debugging

If no Telegram message arrived:

```powershell
python -m runner.facade.cli status <run_id>
python -m runner.facade.cli doctor <run_id>
```

If a reply did not route, inspect `telegram.latest_inbound_receipt` in status.
It will say whether the reply was wrong-chat, unmapped, stale, duplicate, or
injected.

## Anti-Patterns

- Do not infer health from a visible terminal window.
- Do not manually edit projections or checkpoints.
- Do not reply to old Telegram messages and expect them to steer a new cursor.
- Do not archive an active run to hide it.

## Current Limits

- The runner does not yet provide a hosted dashboard.
- `doctor` does not yet calculate all time-based "no output" conditions.
- Telegram replies must be direct replies to recorded alert messages.

## Related Docs

- [Runner Reporting](runner-reporting.md)
- [Run Lifecycle](run-lifecycle.md)
- [Multi-Agent Worktrees](multi-agent-worktrees.md)
- [Telegram Operator Bridge](telegram-operator-bridge.md)
- [Runtime Artifacts And Retention](runtime-artifacts-and-retention.md)
