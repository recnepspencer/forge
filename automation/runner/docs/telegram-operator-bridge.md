# Telegram Operator Bridge

## What This Feature Is

The Telegram bridge sends runner blocker alerts to a Telegram chat and turns
direct replies into operator instructions for the active run. Use it when you
want agents to page you for blockers while keeping the runner event ledger as
the source of truth.

## Why You Use It

- Receive blocker, crash, stall, timeout, or invalid-outcome alerts away from
  the terminal.
- Reply to the exact alert instead of remembering run ids by hand.
- Route replies to the right run when several agents are active.
- Keep every accepted instruction in the runner event ledger.

## Stable Entry Points

From a source checkout, set `PYTHONPATH='automation/runner/src'` before running
the module commands. Installed environments do not need that line.

First-time Telegram setup:

1. Create a bot with BotFather and copy the bot token.
2. Send any message to the bot from the Telegram chat you want to use.
3. Set `RUNNER_TELEGRAM_BOT_TOKEN` and `RUNNER_TELEGRAM_CHAT_ID` in your local
   environment or ignored runner `.env` file.
4. Keep the bot token out of committed docs, config, tests, and logs.

Configure a run with:

```json
"notification_policy": {
  "command_hook": ["python", "-m", "runner.telegram_bridge", "send"],
  "signals": {
    "crash": {"enabled": true, "delivery": "immediate", "sinks": ["command_hook"]},
    "invalid_outcome": {"enabled": true, "delivery": "immediate", "sinks": ["command_hook"]},
    "wall_timeout": {"enabled": true, "delivery": "immediate", "sinks": ["command_hook"]},
    "idle_timeout": {"enabled": true, "delivery": "immediate", "sinks": ["command_hook"]},
    "no_edit_stall": {"enabled": true, "delivery": "immediate", "sinks": ["command_hook"]},
    "same_phase_loop_exceeded": {"enabled": true, "delivery": "immediate", "sinks": ["command_hook"]}
  }
}
```

Run the poller:

```powershell
python -m runner.telegram_bridge poll
```

For one polling pass:

```powershell
python -m runner.telegram_bridge poll-once
```

`poll` and `poll-once` share a process lock. Only one may consume the bot's
update stream at a time; `poll-once` refuses to run while the long-lived poller
is active.

## Core Mental Model

Telegram is a transport, not authority. The bridge records which Telegram
message belongs to which runner alert. When you reply directly to that message,
the bridge injects your text into the recorded run and active cursor.

Accepted replies become `operator_override` events. Duplicate Telegram updates
are ignored by idempotency key. Stale replies are rejected if the run has
completed, stopped, or advanced away from the recorded cursor.

## How It Executes

1. Runner classifies an operator-visible signal.
2. `command_hook` calls `python -m runner.telegram_bridge send`.
3. The bridge sends a Telegram message and records the Telegram message id.
4. The poller reads Telegram updates.
5. A direct reply is matched to the recorded alert.
6. The bridge writes an `operator_override` event.
7. The next prompt includes the instruction.

## Small Example

```powershell
$env:RUNNER_TELEGRAM_BOT_TOKEN="..."
$env:RUNNER_TELEGRAM_CHAT_ID="..."
python -m runner.telegram_bridge poll
```

This starts only the reply poller. A run still needs `notification_policy` to
send alerts.

One poller can serve multiple local runs because replies are matched to the
recorded Telegram alert message id.

## Real Example

```powershell
python -m runner.facade.cli generate milestone --name store-m1 --spec docs/store-m1.md --telegram
python -m runner.facade.cli validate automation/runner/config/store-m1.json
python -m runner.facade.cli start automation/runner/config/store-m1.json --loop
python -m runner.telegram_bridge poll
```

When the runner sends a blocker, reply directly to that Telegram message. Do
not start a new Telegram message with the instruction.

## How It Relates To Other Features

- `notification_policy.command_hook` chooses the bridge as a sink.
- `operator_intervention_policy` decides whether live injection is allowed.
- `status` and `doctor` expose poller health and inbound receipts.
- `archive` preserves the run ledger that contains accepted replies.

## Inspection And Debugging

```powershell
python -m runner.facade.cli status <run_id>
python -m runner.facade.cli doctor <run_id>
```

Look at:

- `telegram.poller_health`
- `telegram.latest_inbound_receipt`
- `notification_delivery_failure`

Receipt statuses:

- `injected`: reply became an operator override.
- `duplicate_ignored`: Telegram update was already consumed.
- `rejected_stale`: alert no longer matches the active run cursor.
- `rejected_unmapped`: reply target is not a recorded runner alert.
- `rejected_unthreaded`: message was not a direct reply.
- `rejected_wrong_chat`: message came from another chat.

## Anti-Patterns

- Do not treat Telegram as a command console.
- Do not paste run ids into free-form messages and expect routing.
- Do not reply to old alerts after the run has advanced.
- Do not store bot tokens in docs, commits, or test fixtures.

## Current Limits

- Telegram is the first project-local hook adapter; other transports should use
  the same command-hook pattern.
- Replies are text-only today.
- The poller is a local process and should be supervised by the operator.
- Exactly one poller may consume a bot update stream for this runtime root.

## Related Docs

- [Runner Operator Guide](runner-operator-guide.md)
- [First Run From Zero](first-run-from-zero.md)
- [Runner Reporting](runner-reporting.md)
- [Troubleshooting](troubleshooting.md)
