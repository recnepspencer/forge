# Closeout Surfaces

## What This Feature Is

This page covers the public surfaces people usually reach for when they need to
finish debugging a form behavior:

- diagnostics summary
- diagnostics history
- verification package

## Why You Use It

- choose the right final inspection surface for current state, retained change
  state, or proof state
- avoid reading low-level histories when a higher-level surface already answers
  the question

## Stable Entry Points

- `form.diagnosticsSummary()`
- `form.diagnosticsHistory()`
- `form.verification()`

## Core Mental Model

Use:

- `diagnosticsSummary()` for "what is true now?"
- `diagnosticsHistory()` for "when did that summary truth change?"
- `verification()` for "does the shipped package prove the same thing?"

## How It Executes

1. current feature reports are derived
2. summary truth is materialized
3. changed summary truth is retained in diagnostics history
4. verification collects the digest set that ties the public surfaces together

## Small Example

```ts
console.log(form.diagnosticsSummary());
console.log(form.diagnosticsHistory().length);
console.log(form.verification().packageDigest);
```

## Real Example

```ts
const summary = form.diagnosticsSummary();
const history = form.diagnosticsHistory();
const verification = form.verification();

console.log({
  summaryDigest: summary.digest,
  latestHistoryDigest: history.at(-1)?.summaryDigest,
  verificationSummaryDigest: verification.digests.diagnosticsSummaryDigest,
});
```

## How It Relates To Other Features

- Read [Diagnostics Summary](../diagnostics/diagnostics-summary.md) for the
  current-state surface.
- Read [Verification Packages](./verification-packages.md) for the proof
  package itself.

## Inspection And Debugging

- if the question is visual or current-state, start with diagnostics summary
- if the question is "what changed?", move to diagnostics history
- if the question is "do all the public surfaces agree?", read verification

## Anti-Patterns

- using verification first for every routine UI check
- using summary reads when the bug is obviously about retained history

## Current Limits

- these closeout surfaces summarize public behavior; they do not replace every
  specialized feature report

## Related Docs

- [Diagnostics Summary](../diagnostics/diagnostics-summary.md)
- [Verification Packages](./verification-packages.md)
