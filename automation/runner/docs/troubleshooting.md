# Troubleshooting

## What This Feature Is

Troubleshooting maps common runner failures to the command that explains them.
Use it when the runner looks stuck, Telegram did not behave as expected, or a
run cannot be safely archived yet.

## First Commands

From a source checkout, set `PYTHONPATH='automation/runner/src'` before running
these commands. Installed environments do not need that line.

```powershell
python -m runner.facade.cli report <run_id>
python -m runner.facade.cli doctor <run_id>
python -m runner.facade.cli status <run_id>
```

Use `report` for the short summary, `doctor` for health findings, and `status`
for the full JSON view.

## I Did Not Get A Telegram Message

Check:

```powershell
python -m runner.facade.cli status <run_id>
```

Look for `notification_delivery_failure`. If present, the runner created a
signal but the sink failed.

Also confirm:

- `notification_policy.command_hook` is configured.
- `RUNNER_TELEGRAM_BOT_TOKEN` and `RUNNER_TELEGRAM_CHAT_ID` are available.
- The run emitted a signal kind enabled in `notification_policy.signals`.

## I Replied But It Did Not Route

Check `telegram.latest_inbound_receipt` in status.

Common statuses:

- `rejected_unthreaded`: you did not reply directly to the alert.
- `rejected_unmapped`: the message you replied to was not a recorded runner
  alert.
- `rejected_wrong_chat`: the message came from the wrong Telegram chat.
- `rejected_stale`: the run completed, stopped, or advanced away from that
  alert's cursor.
- `duplicate_ignored`: the same Telegram update was already consumed.

## The Run Looks Active But Nothing Is Happening

Run:

```powershell
python -m runner.facade.cli doctor <run_id>
python -m runner.facade.cli artifacts <run_id>
```

Check for:

- unhealthy Telegram poller
- notification delivery failure
- stopped run with a stop reason
- stale process locks
- no recent provider log growth

`doctor` does not yet prove every time-based no-output condition. If it finds
nothing but the agent is clearly stuck, inspect the provider log listed by
`artifacts`.

## Config Validation Fails

Run:

```powershell
python -m runner.facade.cli validate automation/runner/config/my-run.json
```

Common causes:

- missing `project.cwd`, `spec_file`, or context file
- unknown top-level key
- ambiguous prompt asset or assembly id
- unsupported phase program
- unsupported provider or role
- direct prompt file binding
- unknown notification signal kind

## Prompt Overlay Missing

Generated configs should include recovery overlay assets under
`automation/project_prompts/assets/recovery/`. If a hand-written prompt library
does not include `recovery/operator_injection_overlay.md`, live operator
injection can fail during prompt materialization.

Regenerate or add the missing registered asset.

## Logs Are Too Large

Run:

```powershell
python -m runner.facade.cli artifacts <run_id>
python -m runner.facade.cli archive <run_id>
```

Only use `--prune-derived` after you know the run is done and raw diagnostic
artifacts are no longer needed.

## The Run Is Done

Run:

```powershell
python -m runner.facade.cli report <run_id>
python -m runner.facade.cli archive <run_id>
```

Future agents can also use the `runner-archive` Codex skill to perform this
closeout safely.

## I Do Not Know The Run Id

Run:

```powershell
python -m runner.facade.cli active
```

`active` lists active and stopped local runs. If the run already completed and
was archived, inspect `automation/runner/runtime/archives/`.

## The Provider Never Starts

Check the provider outside the runner first:

```powershell
codex --help
grok --help
```

Cursor installations vary by environment; use the local command or adapter
entrypoint your workspace expects. Then inspect the config model policy. See
[Provider Setup](provider-setup.md) for provider-specific fields.

## Related Docs

- [Runner Reporting](runner-reporting.md)
- [Runtime Artifacts And Retention](runtime-artifacts-and-retention.md)
- [Telegram Operator Bridge](telegram-operator-bridge.md)
- [Provider Setup](provider-setup.md)
