# Runtime Artifacts And Retention

## What This Feature Is

Runtime retention defines what the runner keeps, what it can rebuild, and what
can be archived or pruned after a run is done. Use it when a workspace starts
accumulating old run output, large provider logs, prompt captures, Telegram
receipts, or checkpoint files.

## Why You Use It

- Keep active runs debuggable without letting old logs grow forever.
- Preserve the event ledger and config needed to prove what happened.
- Remove noisy provider output once a completed run has been archived.
- Give future agents a safe rule for cleanup.

## Stable Entry Points

From a source checkout, set `PYTHONPATH='automation/runner/src'` before running
these commands. Installed environments do not need that line.

## Provider Execution Receipts

`runtime/executions/<run_id>/` is the authoritative provider-execution lane.
Each receipt is keyed by the semantic turn id plus a digest of the immutable
delivery prompt and progresses through `claimed`, `launched`, and `finished`.

Graph checkpoints may replay orchestration, but they may not create a second
provider launch for an existing execution receipt. A finished receipt supplies
the captured result to replay. An unfinished receipt becomes an interrupted
execution failure and enters normal recovery. A recovery prompt has a distinct
prompt digest, so it receives new execution authority without changing the
semantic turn id expected in `RUNNER_EVENT`.

```powershell
python -m runner.facade.cli artifacts <run_id>
python -m runner.facade.cli archive <run_id>
python -m runner.facade.cli archive <run_id> --prune-derived
```

## Core Mental Model

Runtime files have retention classes:

- Authority: `runtime/events/<run_id>.jsonl`. This is the run truth.
- Archive: `runtime/archives/<run_id>/`. This is a retained bundle for a run
  that is done or ready for handoff.
- Derived: projections, notifications, Telegram routing state. These explain or
  display authority.
- Continuity: checkpoints. These help resume execution.
- Observation: provider logs and prompt instantiations. These are useful while
  debugging but can become bulky.
- Process control: locks and stop files. These are live control artifacts, not
  history.

Deleting derived, continuity, or observation files must not change run truth.
Deleting the event ledger destroys authority.

## How It Executes

`archive <run_id>` refreshes the projection, reads the bound config, and writes
an archive bundle:

```text
runtime/archives/<run_id>/
  events.jsonl
  config.json
  projection.json
  report.json
  manifest.json
```

By default, archive does not delete anything. With `--prune-derived`, it removes
run-scoped derived, continuity, and observation artifacts after the archive
bundle is written. Pruning is only admitted for completed runs.

Codex agents can also use the local `runner-archive` skill as a guardrailed
workflow around these same commands. The skill does not add a second archive
system; it tells an agent to inspect `report`, inspect `doctor`, write the
archive bundle, and prune only when the run is safely completed.

## Small Example

```powershell
python -m runner.facade.cli artifacts worthyroad-m1
python -m runner.facade.cli archive worthyroad-m1
```

This is the smallest safe cleanup flow: inspect the artifact classes, then
archive without deletion.

## Real Example

```powershell
python -m runner.facade.cli report worthyroad-m1
python -m runner.facade.cli doctor worthyroad-m1
python -m runner.facade.cli archive worthyroad-m1 --prune-derived
```

Use this after a run is completed and no one needs raw logs or prompt captures
for immediate debugging. The archive keeps the event ledger, config, final
projection, and report.

## Inspection And Debugging

Use `artifacts` when storage looks too large or when a future agent needs to
know what is safe to delete. The output includes each path, whether it exists,
its byte size, and its retention class.

Use `report` before archiving to confirm the run state. Use `doctor` first if
the run still appears active or stopped.

## Anti-Patterns

- Do not manually delete `runtime/events/<run_id>.jsonl`.
- Do not prune an active run.
- Do not treat checkpoints as evidence that the run completed.
- Do not keep raw provider logs forever when an archive bundle is enough.

## Current Limits

- Archive pruning is explicit per run. There is no age-based bulk prune command
  yet.
- Archive bundles are local filesystem artifacts. There is no remote storage
  upload or compression policy yet.
- Shared Telegram receipt files are retained outside the per-run prune path.

## Related Docs

- [Run Lifecycle](run-lifecycle.md)
- [Sample Command Output](sample-command-output.md)
- [Runner Reporting](runner-reporting.md)
- [Consumer Runner Rules](consumer-runner-rules.md)
