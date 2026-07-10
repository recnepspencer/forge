# First Run From Zero

## What This Feature Is

This is the shortest complete path from a fresh workspace to a validated runner
run. Use it when you want to prove the runner is wired correctly before giving
an agent a real milestone.

## Before You Start

Run these from the workspace root:

```powershell
$env:PYTHONPATH='automation/runner/src'
python -m runner.facade.cli --help
```

Create a small spec:

```powershell
New-Item -ItemType Directory -Force docs
Set-Content docs/runner-smoke.md "Smoke run: prove the runner can start and report status."
```

## Generate

```powershell
python -m runner.facade.cli generate single_prompt --name runner-smoke --spec docs/runner-smoke.md
```

Expected result: the command prints the config path, usually:

```text
<workspace>/automation/runner/config/runner-smoke.json
```

## Validate

```powershell
python -m runner.facade.cli validate automation/runner/config/runner-smoke.json
```

Expected result: exit code `0`. Validation errors are printed when config,
prompt roots, roles, providers, or phase declarations are invalid.

## Start

```powershell
python -m runner.facade.cli start automation/runner/config/runner-smoke.json --run-id runner-smoke-test
```

Use `--run-id` in smoke tests so later commands have a known id. If omitted,
the runner creates one.

Use `--loop` for long unattended runs. Without `--loop`, the runner executes
one graph turn and returns.

## Inspect

```powershell
python -m runner.facade.cli report runner-smoke-test
python -m runner.facade.cli doctor runner-smoke-test
python -m runner.facade.cli status runner-smoke-test
```

`report` is the human summary. `doctor` is the health check. `status` is the
full JSON view.

## Archive

After the run completes:

```powershell
python -m runner.facade.cli archive runner-smoke-test
```

Use `--prune-derived` only after completion and only when raw diagnostics are
no longer needed:

```powershell
python -m runner.facade.cli archive runner-smoke-test --prune-derived
```

## Telegram Variant

```powershell
$env:RUNNER_TELEGRAM_BOT_TOKEN='...'
$env:RUNNER_TELEGRAM_CHAT_ID='...'
python -m runner.facade.cli generate milestone --name runner-smoke --spec docs/runner-smoke.md --telegram --force
python -m runner.facade.cli validate automation/runner/config/runner-smoke.json
python -m runner.facade.cli start automation/runner/config/runner-smoke.json --run-id runner-smoke-telegram --loop
python -m runner.telegram_bridge poll
```

Run the poller in a separate terminal. One poller can route replies for all
runs in the workspace because replies are matched to recorded Telegram alert
messages.

## Related Docs

- [Run Lifecycle](run-lifecycle.md)
- [Runner Reporting](runner-reporting.md)
- [Troubleshooting](troubleshooting.md)
