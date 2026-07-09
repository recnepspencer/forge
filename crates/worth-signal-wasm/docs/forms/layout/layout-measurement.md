# Layout Measurement

## What This Feature Is

This page covers the measured geometry layer retained by the form controller.

## Why You Use It

- record actual row heights after layout
- keep measurement history on the controller instead of in ad hoc UI state
- inspect coalesced layout updates without mutating semantic form truth

## Stable Entry Points

- `measurement: { observe, maxRetainedSnapshots }`
- `form.recordLayoutMeasurement(rows, options?)`
- `form.layoutMeasurement()`

## Core Mental Model

Layout measurement is imperative event input, but the controller still owns the
retained report and history.

## How It Executes

1. declare which measurement causes the form observes
2. record row measurements as events arrive
3. the runtime coalesces them by animation frame
4. snapshots are retained without changing semantic form truth

## Small Example

```ts
form.recordLayoutMeasurement([{ row: "hero", controlHeight: 32 }], {
  cause: "animationFrame",
});

console.log(form.layoutMeasurement().latestSnapshot);
```

## Real Example

```ts
const report = form.layoutMeasurement();

console.log(report.policy);
console.log(report.latestSnapshot);
console.log(report.snapshots);
```

## How It Relates To Other Features

- Read [Control Sizing](./control-sizing.md) for the common sizing questions
  above this layer.
- Read [Layout Configuration Reference](./layout-configuration-reference.md)
  for the full declared configuration surface.
- Read [Layout Overview](./layout-overview.md) for the declaration-side layout
  report.

## Inspection And Debugging

- `latestSnapshot` is the first read
- `snapshots` shows retained history
- `policy.observe` tells you which causes were declared

## Anti-Patterns

- treating measured layout as semantic source truth
- storing a second measurement history outside the controller

## Current Limits

- presentation settling and entry bootstrap around measurement belong in the
  lifecycle section, not the measurement section

## Related Docs

- [Layout Overview](./layout-overview.md)
- [Layout Configuration Reference](./layout-configuration-reference.md)
- [Control Sizing](./control-sizing.md)
