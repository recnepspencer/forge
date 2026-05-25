# Resource Line Source

## What This Feature Is

This page covers forms whose source truth comes from a resource line.

## Why You Use It

- bind a form directly to a resource-backed source
- inspect lifecycle, transfer, settlement, and verification reads from the
  controller
- keep resource ownership explicit instead of hiding it inside local state

## Stable Entry Points

- `signals.form.source.resourceLine(...)`
- `form.resourceSource()`

## Core Mental Model

A resource-backed form still owns draft truth, but the source layer comes from
one resource line. The controller exposes that line's status, freshness,
transfer, lifecycle, and verification reads directly.

## How It Executes

1. a resource line is chosen as the form source
2. the runtime binds source truth to that line
3. `resourceSource()` exposes the current resource-backed readback

## Small Example

```ts
console.log(form.resourceSource()?.status);
```

## Real Example

```ts
const resource = form.resourceSource();

console.log(resource?.descriptor);
console.log(resource?.freshness);
console.log(resource?.lifecycle);
console.log(resource?.verification);
```

## How It Relates To Other Features

- Read [Choosing A Form Source](../getting-started/choosing-a-form-source.md)
  for the source-authority choice behind this lane.
- Read [Resource Shape And Visible Selection](./resource-shape-and-visible-selection.md)
  for the most important shape and visibility reads on top of the source.

## Inspection And Debugging

- `status`, `freshness`, and `lifecycle` are the main operational reads
- `transfer` shows upload/download/processing state
- `verification` shows package-level verification digests

## Anti-Patterns

- pretending a resource-backed form is just a local form with fetches around it
- rebuilding resource lifecycle state beside `resourceSource()`

## Current Limits

- deeper line-family semantics live in the resource docs

## Related Docs

- [Resource Shape And Visible Selection](./resource-shape-and-visible-selection.md)
- [Resource Settlement](./resource-settlement.md)
