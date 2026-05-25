# Inspect A Resource Line

## What This Feature Is

This is the normal debugging front door for one live resource line.

## Why You Use It

Use it when you need a quick read of:

- current status and freshness
- admitted request posture
- upload, processing, and download state
- current diagnostics
- whether retained history features are available

## Stable Entry Points

- `family.line(...)`
- `line.summary()`
- `line.request()`
- `line.processing()`
- `line.upload()`
- `line.download()`
- `line.diagnosticsSummary()`
- `line.history().availability`

## Core Mental Model

`line.summary()` is the grouped first read. It does not hide lower truth, but it
does keep the common path from turning into six separate calls.

The runtime intentionally does not bundle the entire retained history object
into `summary()`. You still reach for `line.history()` when you need replay,
restore, lifecycle entries, or verification proof.

## How It Executes

`line.summary()` groups the same truth you would otherwise read one call at a
time:

- `line.diagnosticsSummary().current`
- `line.request()`
- `line.processing()`
- `line.upload()`
- `line.download()`
- `line.diagnosticsSummary()`
- `line.history().availability`

It is a grouped read shape, not a second source of truth.

## Small Example

```ts
const line = reportDetail.line({ reportId: "q1" });

const summary = line.summary();

console.log(summary.current.status.kind);
console.log(summary.current.freshness.kind);
console.log(summary.request.method);
console.log(summary.download.readyCount);
```

## Real Example

```ts
const line = assetDetail.line({ assetId: "report-q1" });

console.log(line.summary());
console.log(line.request());
console.log(line.diagnosticsSummary());
console.log(line.history().availability);
```

Use the grouped read first. Drop lower only when you need exact descriptors,
history entries, or verification proof.

## How It Relates To Other Features

- Use [Check Status, Freshness, And History](./check-status-settlement-and-history.md)
  when the important question is retained lifecycle, not the grouped read.
- Use [Read Delivery And Compatibility](./read-delivery-and-compatibility.md)
  when the line came from external delivery or basis refresh.
- Use [Restore, Replay, And Recover](./restore-replay-and-recover.md)
  when you need exact recovery behavior.

## Inspection And Debugging

The quickest sequence is:

1. `line.summary()`
2. `line.diagnosticsSummary().latest`
3. `line.history().availability`
4. `line.history().verificationPackage()`

## Anti-Patterns

- Do not treat `line.summary()` as a retained-history snapshot.
- Do not expect `summary()` to include replay artifacts or lifecycle arrays.
- Do not start with `verificationPackage()` unless you actually need proof.

## Current Limits

`line.summary()` is intentionally compact. It does not inline lifecycle entries,
replay payloads, or exact restore results.

## Related Docs

- [Check Status, Freshness, And History](./check-status-settlement-and-history.md)
- [Line Inspection](../line-inspection.md)
