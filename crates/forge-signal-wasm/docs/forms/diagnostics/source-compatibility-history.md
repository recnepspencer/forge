# Source Compatibility History

## What This Feature Is

This page covers the retained history of source schema compatibility and draft
migration.

## Why You Use It

- inspect when the draft stayed compatible, migrated, or became unavailable
- explain draft replacement after schema version changes
- connect source compatibility to verification and diagnostics

## Stable Entry Points

- `form.sourceCompatibility()`
- `form.sourceCompatibilityHistory()`
- `form.diagnostics().sourceCompatibilityHistory`

## Core Mental Model

Source compatibility is about whether the current draft still matches the
source schema shape the form expects.

## How It Executes

1. source schema truth changes
2. the runtime derives compatibility posture
3. compatibility artifacts are retained when migration or incompatibility
   happens

## Small Example

```ts
console.log(form.sourceCompatibility().posture);
console.log(form.sourceCompatibilityHistory());
```

## Real Example

```ts
for (const artifact of form.sourceCompatibilityHistory()) {
  console.log({
    posture: artifact.posture,
    previousSchemaVersion: artifact.previousSchemaVersion,
    currentSchemaVersion: artifact.currentSchemaVersion,
    reason: artifact.reason,
  });
}
```

## How It Relates To Other Features

- Read [Source Compatibility And Draft Migration](../validation/source-compatibility-and-draft-migration.md)
  for the behavior itself.
- Read [Diagnostics History](./diagnostics-history.md) for the summary-shaped
  retained view.

## Inspection And Debugging

- `compatible` means the draft still fits the current schema
- `migrated` means the runtime produced a new draft shape
- `unavailable` means the drift could not be resolved honestly

## Anti-Patterns

- treating schema compatibility as if it were just another validation error
- skipping the retained history when you need to explain a migrated draft

## Current Limits

- source compatibility history is about schema shape drift, not ordinary field
  edits

## Related Docs

- [Source Compatibility And Draft Migration](../validation/source-compatibility-and-draft-migration.md)
- [Diagnostics History](./diagnostics-history.md)
