# Installation And Prerequisites

## What This Feature Is

This page explains what must exist before a downstream project can run the
automation runner. Use it before the quickstart if you are setting up a new
workspace or moving the runner into a project that has never used it.

## Why You Use It

- Confirm Python and package access.
- Decide whether you are using a source checkout or an installed package.
- Make provider CLIs available before a long run starts.
- Set Telegram environment variables only when Telegram is enabled.

## Stable Entry Points

Source checkout:

```powershell
$env:PYTHONPATH='automation/runner/src'
python -m runner.facade.cli --help
```

Installed package:

```powershell
python -m runner.facade.cli --help
```

## Core Mental Model

The runner is a Python package. A workspace needs to make that package
importable, then call the public module entry points. The docs use source
checkout commands because this repository currently carries the runner under
`automation/runner/src`.

Commands should be run from the workspace root. In these docs, the workspace
root is the directory containing `automation/runner/`, `automation/project_prompts/`,
and the spec files referenced by config.

## Requirements

- Python 3.11 or newer.
- Access to the runner package, either by source checkout `PYTHONPATH` or by
  installing the package into the active environment.
- A project spec file, usually under `docs/`.
- At least one working provider adapter: Codex, Cursor, or Grok.
- Optional Telegram bot token and chat id when using `--telegram`.

## Provider Smoke Checks

Before starting a long run, confirm the provider command works outside the
runner:

```powershell
codex --help
grok --help
```

Cursor installations vary by environment; use the local command or adapter
entrypoint your workspace expects. See [Provider Setup](provider-setup.md) for
model-policy config examples.

## Telegram Prerequisites

Telegram is optional. When enabled, set:

```powershell
$env:RUNNER_TELEGRAM_BOT_TOKEN='...'
$env:RUNNER_TELEGRAM_CHAT_ID='...'
```

Do not commit these values. The bridge can also read `automation/runner/.env`
in a local workspace.

## Anti-Patterns

- Do not assume `python -m runner...` will work before the package is
  importable.
- Do not start a long run before checking provider credentials or CLI access.
- Do not copy tokens into docs, config committed to git, or test fixtures.

## Related Docs

- [First Run From Zero](first-run-from-zero.md)
- [Provider Setup](provider-setup.md)
- [Telegram Operator Bridge](telegram-operator-bridge.md)
