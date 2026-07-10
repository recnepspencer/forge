# Run Lifecycle

## What This Feature Is

The run lifecycle explains run identity, states, and legal operator actions.
Use it when you need to know whether to start, resume, stop, inject, or archive
a run.

## Core Mental Model

`--name` names generated files. `run_id` names one execution history.

If you run:

```powershell
python -m runner.facade.cli generate milestone --name store-m1 --spec docs/store-m1.md
```

the config is `automation/runner/config/store-m1.json`. A later `start` may use
any run id:

```powershell
python -m runner.facade.cli start automation/runner/config/store-m1.json --run-id store-m1-run-1
```

If `--run-id` is omitted, the runner creates one. Use `active` to find active
or stopped run ids later:

```powershell
python -m runner.facade.cli active
```

## States

- Active: the projection has a current phase/turn and is not stopped or
  completed.
- Stopped: an operator or policy stop was recorded. Use `resume` or inspect
  `stop_reason`.
- Completed: the run ledger contains `run_completed`.
- Unreadable: the runner could not load the ledger/config/projection inputs.

## Commands

Start a new run:

```powershell
python -m runner.facade.cli start automation/runner/config/store-m1.json --run-id store-m1-run-1 --loop
```

Resume an existing stopped or interrupted run:

```powershell
python -m runner.facade.cli resume store-m1-run-1 --loop
```

If the bound config changed after the run adopted a plan, `resume` refuses the
drift. Use [Plan Revisions](plan-revisions.md) to diff, revise, or fork.

Stop a run cooperatively:

```powershell
python -m runner.facade.cli stop store-m1-run-1 --reason "operator stop"
```

Inject direction into the current cursor:

```powershell
python -m runner.facade.cli inject store-m1-run-1 --message "Focus only on the failing Telegram E2E."
```

Archive after completion or after explicit operator closeout:

```powershell
python -m runner.facade.cli archive store-m1-run-1
```

## `--loop`

With `--loop`, the runner keeps driving graph turns until completion, stop,
pause, or process failure. Without `--loop`, it performs one graph turn and
returns. Use no-loop runs for smoke tests or controlled manual stepping.

## Archive Eligibility

Safe archive without pruning can preserve evidence for active, stopped, or
completed runs. It does not delete anything.

`archive --prune-derived` requires a completed run. It removes run-scoped
derived, continuity, and observation artifacts after writing the archive bundle.

## Current Limits

- `active` lists active and stopped local event ledgers, not every completed
  historical run.
- There is no bulk run index command yet for all completed/archived runs.
- Resume depends on the event ledger and config path still being available.

## Related Docs

- [Runner Reporting](runner-reporting.md)
- [Runtime Artifacts And Retention](runtime-artifacts-and-retention.md)
- [Plan Revisions](plan-revisions.md)
