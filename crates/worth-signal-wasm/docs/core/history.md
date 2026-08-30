# History, Replay, And Runtime Branches

The history surface records runtime execution state, snapshots, lineage, and
runtime branches. Use it to inspect or restore the runtime's own derived world.

Do not confuse that with durable application history. Runtime history can be
extremely exact and still belong to a process-local execution engine.

## Stable Entry Points

- `signals.history()`
- `history.replay_for(id)`
- `history.lineage_for(id)`
- `history.snapshot()`
- `history.restore_exact_snapshot(snapshot)`
- `history.current_branch()`
- `history.create_branch(name)`
- `history.switch_branch(branchId)`
- `history.plan_merge_branches(...)`
- `history.merge_branches(...)`
- `graph.inspectHistory()`

## Inspect Replay And Lineage

```ts
const replay = await signals.history().replay_for(total.id);
const lineage = await signals.history().lineage_for(total.id);

console.log(replay);
console.log(lineage);
```

Replay describes retained execution for a runtime node. Lineage explains how
that node relates to its execution ancestry. At a published boundary,
`graph.inspectHistory()` is usually easier to read because it uses public names.

## Capture And Restore An Exact Snapshot

```ts
const history = signals.history();
const snapshot = await history.snapshot();

await count.set(9);
await history.restore_exact_snapshot(snapshot);
```

The exact snapshot carries a same-runtime restore token. Treat that token as an
authority artifact, not a generic JSON backup format.

Exact wire restore tokens are single-use transfer authorities. Restoring
consumes the token; call the raw-module `discardRestoreToken(token)` export
when abandoning one. Each JavaScript/worker realm retains at most 64 pending
exact restore artifacts and returns `restoreTokenCapacityExhausted` before
creating another. Portable wire artifacts do not occupy this exact-token
registry.

## Runtime Branches

```ts
const history = signals.history();
const main = await history.current_branch();
const experiment = await history.create_branch("experiment");

await history.switch_branch(experiment.id);
await count.set(12);

const plan = await history.plan_merge_branches(experiment.id, main.id);
console.log(plan);
```

A runtime branch stages an alternative runtime state and retains branch
ancestry. It is useful for execution experiments, replay, and exact runtime
restore.

## Runtime Merge Versus Application Merge

Runtime merge operates on native Signal branch state. It does not understand
that `teeth`, `thickness`, or `approval` are application aspects requiring a
human decision.

Use [Local Truth](../local-truth/README.md) when application values need declared
aspects, stale-basis denial, conflict alternatives, and manual resolution.
Signal then consumes the committed result as derivation.

## Worker-First Parity

Worker-first history owns branch lifecycle and exact snapshot restore in the
worker runtime. Calls may be asynchronous, so awaiting history operations is the
portable application posture.

## Anti-Patterns

- Do not store business records only in runtime snapshots.
- Do not use a runtime branch ID as an application-value merge decision.
- Do not edit restore tokens.
- Do not retain unused exact restore tokens; explicitly discard them.
- Do not describe retained process history as durable cross-process audit
  history.

## Related Docs

- [Diagnostics And Explanation](./diagnostics.md)
- [Graphs And Controllers](./graphs-and-controllers.md)
- [Branch Merge And Manual Resolution](../local-truth/branch-merge.md)
