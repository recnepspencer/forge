# Diagnostics, History, And Verification

## What This Feature Is

This feature is the inspection and proof surface for forms. It gives you a
summary view, a full diagnostics view, retained histories, replay/restore
artifacts, and a verification package with digests and performance counters.

## Why You Use It

- explain why a form is blocked, unavailable, or admitted
- reconstruct what happened without mutating operational truth
- certify current form truth and adjacent resource/collaboration digests
- prove that a UI-only update changed presentation state without changing dirty,
  validation, readiness, or source truth

## Stable Entry Points

- `diagnosticsSummary()`
- `diagnostics()`
- `actionHistory()`
- `actionExecutionHistory()`
- `asyncValidationHistory()`
- `canonicalizationHistory()`
- `diagnosticsHistory()`
- `presentationHistory()`
- `stateHistory()`
- `resetHistory()`
- `replayRestoreHistory()`
- `sourceCompatibilityHistory()`
- `verification()`

## Core Mental Model

Diagnostics and histories are derived from canonical form artifacts. They do
not own operational truth. The verification package is the digest-bearing
boundary that ties current state, retained history, resource proof, and
performance envelope together.

## How It Executes

The runtime derives current diagnostics from canonical artifacts, retains
history only when truth changes materially, records replay/restore/reset and
compatibility artifacts explicitly, then composes those digests into one
verification package.

## Small Example

```ts
const summary = form.diagnosticsSummary();
const verification = form.verification();

console.log(summary);
console.log(verification.packageDigest);
```

This is the smallest honest example because it shows the short explanation lane
and the proof-bearing package lane together.

## Real Example

```ts
const diagnostics = form.diagnostics();
const stateHistory = form.stateHistory();
const replayRestoreHistory = form.replayRestoreHistory();
const verification = form.verification();

console.log(diagnostics.readiness);
console.log(stateHistory);
console.log(replayRestoreHistory);
console.log(verification.digests.presentationDigest);
console.log(verification.digests.semanticEqualityDigest);
console.log(verification.performanceEnvelope);
```

The runtime keeps current state, retained history, and performance accounting
separate but digest-linked.

## How It Relates To Other Features

- Pair it with [Resource-Line Forms](./resource-line-forms.md) when the form
  consumes resource verification and replay/restore proof.
- Pair it with [Collaboration](./collaboration.md) when collaboration posture
  and event digests matter.
- Pair it with [Presentation And External Lanes](./presentation-and-external-lanes.md)
  for retained presentation history and visible settlement inspection.

## Inspection And Debugging

- `diagnosticsSummary()` gives the short current-state explanation surface.
- `diagnostics()` gives the full current-state report.
- `actionHistory()`, `actionExecutionHistory()`, `asyncValidationHistory()`,
  `canonicalizationHistory()`, `diagnosticsHistory()`, `presentationHistory()`,
  `stateHistory()`, `resetHistory()`, `replayRestoreHistory()`, and
  `sourceCompatibilityHistory()` show retained events from different lanes.
- `verification()` is the fastest way to answer two debugging questions:
  "what changed?" and "what stayed semantically the same?" Compare the lane
  digest you expect to move, then compare `semanticEqualityDigest`,
  `validationDigest`, `readinessDigest`, or other adjacent digests that should
  stay stable.
- `verification().performanceEnvelope` tells you how much retained history and
  derived scanning work the current report includes.

## Anti-Patterns

- treating diagnostics as if they were mutable control inputs
- using summary diagnostics as a substitute for retained history
- assuming replay/restore unavailability means the runtime silently did its
  best anyway

## Current Limits

- replay/restore stays typed unavailable when runtime support or retained
  history is absent
- diagnostics summary is intentionally summary-shaped and not a full history
  materialization
- verification exposes the current boundary counters; it is not a replacement
  for subsystem-specific closeout matrices

## Related Docs

- [Resource-Line Forms](./resource-line-forms.md)
- [Collaboration](./collaboration.md)
- [Presentation And External Lanes](./presentation-and-external-lanes.md)
