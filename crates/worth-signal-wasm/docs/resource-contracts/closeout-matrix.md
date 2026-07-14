# Effect Closeout Matrix

The effect closeout matrix is a profile checklist. Use it to see what a
resource effect profile claims about local patches, delivery, rollback, merge,
history, and performance.

## What This Feature Is

`signals.resource.effects.closeoutMatrix(profile)` returns a structured matrix
for a `ResourceEffectProfile`. It is useful when you are documenting or
reviewing profile behavior and want one place to compare the supported lanes.

## Quick Use

```ts
const matrix = signals.resource.effects.closeoutMatrix(
  signals.resource.effects.branchNative(),
);

for (const row of matrix.rows) {
  console.log(row.effectFamily, row.capability);
}
```

If a row says `unsupported` or `unavailable`, do not describe that profile as if
the capability exists.

## Why You Use It

- Compare built-in effect profiles.
- Confirm whether a profile claims optimistic writes, rollback, merge/rebase,
  confirmation, broad replacement, or diagnostics/history support.
- See which proof categories back each claim.
- Keep custom profile docs aligned with what the profile actually says.

## Stable Entry Points

- `signals.resource.effects.closeoutMatrix(profile)`
- `signals.resource.effects.branchNative()`
- `signals.resource.effects.serverCanonical()`
- `signals.resource.effects.pessimistic()`
- `signals.resource.effects.deliveryAuthoritative()`
- `signals.resource.effects.nonReversible()`
- `signals.resource.effects.sensitive()`
- `signals.resource.effects.custom(...)`
- `ResourceEffectCloseoutMatrix`

## Core Mental Model

An effect profile is a bundle of behavior choices. The closeout matrix lists the
behavior families and the proof categories attached to those choices.

Rows are grouped by effect family:

- local patch
- delivery patch
- optimistic write
- confirmation
- failure rollback
- branch restore
- merge/rebase
- broad replacement
- diagnostics/history

Each row carries evidence in these categories:

- runtime
- type surface
- diagnostics/history
- branch merge
- performance

The API exposes those categories as `matrix.proofLanes`; those proof lanes are
the checklist columns behind each profile capability.

## How It Executes

The matrix is generated from the selected profile. It does not execute a
resource line and it does not certify your endpoint-specific hooks. It tells you
what the profile surface claims, then points at the evidence categories that
support those claims.

Use line diagnostics and history to inspect a concrete effect. Use the matrix to
understand profile-level behavior.

## Small Example

```ts
const profile = signals.resource.effects.branchNative();
const matrix = signals.resource.effects.closeoutMatrix(profile);

console.log(matrix.profileName);
console.log(matrix.proofLanes);
console.log(matrix.rows.map((row) => row.effectFamily));
```

## Real Example

```ts
const matrix = signals.resource.effects.closeoutMatrix(
  signals.resource.effects.sensitive(),
);

const rollbackRow = matrix.rows.find((row) =>
  row.effectFamily === "failureRollback");

console.log(rollbackRow?.capability);
console.log(rollbackRow?.evidence.diagnosticsHistory);
```

This lets a product feature explain that a sensitive profile avoids retained
preimages while still showing the evidence categories attached to that decision.

## How It Relates To Other Features

- [Branch-Native Resource Effects](../resources/branch-native-effects.md)
  explains how profiles affect real writes.
- [Effect Envelope Contract](./effect-envelope.md) explains the concrete effect
  record for one admitted operation.
- [Effect Merge And Rebase](../resources/merge-and-rebase.md) explains the
  merge/rebase row in practice.
- [History And Restore](./history-and-restore.md) explains rollback behavior.

## Inspection And Debugging

When a user-facing doc names a profile capability, check the matching matrix
row. If the row says `unsupported`, `unavailable`, or something narrower than
the prose, revise the prose.

For a specific line, prefer `line.diagnostics().lastEffect.profile` and
`line.diagnostics().lastEffect.optimistic.rollback`. The matrix is profile-level
evidence; the envelope is operation-level evidence.

## Anti-Patterns

- Do not use the matrix as a substitute for testing endpoint hooks.
- Do not cite closeout evidence as if it were the result of a particular line
  operation.
- Do not treat `branchNative()` proof as applying to a custom profile with
  different rollback, rebase, or preimage settings.
- Do not hide unsupported rows. They are part of the contract.

## Current Limits

- The matrix summarizes profile capability. It does not prove a particular
  response topology hook is correct.
- Custom profiles can be inspected, but the caller is responsible for explaining
  why the custom behavior is appropriate.

## Related Docs

- [Branch-Native Resource Effects](../resources/branch-native-effects.md)
- [Effect Envelope Contract](./effect-envelope.md)
- [Effect Merge And Rebase](../resources/merge-and-rebase.md)
- [History And Restore](./history-and-restore.md)
