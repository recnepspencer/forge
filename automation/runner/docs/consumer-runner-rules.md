# Consumer runner rules

Consumers configure and call the canonical runner from their own workspace.
They do not copy, patch, or invoke legacy automation loops.

- Keep the consumer config and prompt library in the consumer workspace.
- Treat `runtime/events` as authority; projections, checkpoints, logs, and
  Telegram state are derived and must never be edited to change a run.
- Use one active writer per worktree. Parallel judges may inspect a frozen
  revision only; parallel writers require separate worktrees until lease
  authority is implemented.
- Run `python -m runner.facade.cli active` before starting another writer in a
  worktree.
- Generate new configs with `python -m runner.facade.cli generate ...`; use
  `--telegram` when the workspace should page the operator through Telegram.
- Configure Telegram only through `notification_policy.command_hook`; run
  `python -m runner.telegram_bridge poll` beside the runner.
- Reply directly to a Telegram alert. Unthreaded, stale, duplicate, and
  wrong-chat replies are recorded and rejected.
