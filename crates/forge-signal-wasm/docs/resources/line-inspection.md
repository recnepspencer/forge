# Line Inspection

Use this page when the family is already declared and you need to render,
inspect, debug, or reason about a live line.

## Start With `line.summary()`

`line.summary()` is the grouped first read.

```ts
const line = userDetail.line({ userId: "u1" });

console.log(line.summary());
```

Use it first when you want:

- current status
- freshness
- request posture summary
- upload, processing, and download summary
- diagnostics summary

## The Main Reads

- `line.value()`
- `line.summary()`
- `line.request()`
- `line.upload()`
- `line.processing()`
- `line.download()`
- `line.diagnosticsSummary()`
- `line.history()`

## Happy Path

```ts
const line = reportDetail.line({ reportId: "r1" });

console.log(line.summary());
console.log(line.request());
console.log(line.diagnosticsSummary());
console.log(line.history().availability);
```

## When To Go Lower

Drop below `line.summary()` when you need:

- exact request target or admitted body
- exact download descriptors
- detailed diagnostics counters
- replay or restore availability

## Where To Go Next

- task-first examples:
  [Resource Recipes](../learn/recipes.md)
- low-level line reference:
  [Resource Line Reference](../api-reference/resource-line.md)
- diagnostics/history reference:
  [Inspection And History Contract](../resource-contracts/inspection-and-history.md)
