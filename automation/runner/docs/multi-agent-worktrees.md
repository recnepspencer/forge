# Multi-Agent Worktrees

## What This Feature Is

This page explains the safe current rules for running more than one agent. Use
it before starting parallel agents, judges, or independent milestone runs.

## Current Rule

Use one active writer per worktree.

A writer is any agent run that may edit files in that worktree. Until lease
authority exists, the runner does not prove that two writers are changing
disjoint files safely.

## Safe Checklist

Before starting a writer:

```powershell
python -m runner.facade.cli active
```

If another run is active in the same worktree, do one of these:

- wait for it to complete
- stop it intentionally
- resume/finish it
- create a separate worktree for the second writer

## Parallel Judges

Parallel judges may inspect a frozen revision or read-only artifact set. They
should not write into the same worktree as an active implementation agent.

If a judge needs to produce files, run it in a separate worktree or make its
output path explicitly separate from the writer's project scope.

## Telegram With Multiple Runs

One Telegram poller can serve multiple runs in the same workspace. Replies
route by Telegram message id to the alert that was recorded for a specific
run/cursor. Operators should still reply directly to the alert message, not send
free-form instructions.

## Anti-Patterns

- Do not start two implementation agents in one worktree because their scopes
  "probably do not overlap."
- Do not let a judge write into the same source tree while an implementer is
  active.
- Do not use Telegram as a manual run-id router. The bridge routes direct
  replies automatically.

## Future Boundary

Lease authority may later allow planned parallel writers in one worktree. Until
that exists, separate worktrees are the safe path.

## Related Docs

- [Runner Operator Guide](runner-operator-guide.md)
- [Telegram Operator Bridge](telegram-operator-bridge.md)
