# Host Facts

## What This Feature Is

This page covers the host facts a form can bind and read, such as focus,
visibility, viewport, online, persistence, credentials, and autofill.

## Why You Use It

- keep host-owned environment facts explicit on the form controller
- inspect which host facts are available before using them
- understand host-dependent readiness and presentation behavior

## Stable Entry Points

- `host: { ... }`
- `form.host()`

## Core Mental Model

Host facts are not form state. They are environment facts the form can depend
on. The controller reads them and keeps their availability explicit.

## How It Executes

1. host bindings are declared
2. the runtime derives host fact reads
3. readiness, accessibility, layout, and later lifecycle reads can consume
   them

## Small Example

```ts
const form = signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  host: {
    online: true,
    visibility: "visible",
  },
});

console.log(form.host().summary);
```

## Real Example

```ts
const host = form.host();

console.log(host.facts.online);
console.log(host.facts.viewport);
console.log(host.facts.persistence);
```

## How It Relates To Other Features

- Read [Offline And Host Blockers](./offline-and-host-blockers.md) for the
  readiness consequences of these facts.
- Read [Entry Bootstrap](../lifecycle/entry-bootstrap.md) when host facts are
  part of entry readiness.

## Inspection And Debugging

- `facts` shows each host fact directly
- `summary.supported` and `summary.unavailable` show how much host support the
  form has
- each fact carries its own `reason` when unavailable

## Anti-Patterns

- treating host facts as if they were persistent form truth
- assuming undeclared host facts are supported

## Current Limits

- route-level host coordination belongs in later docs

## Related Docs

- [Offline And Host Blockers](./offline-and-host-blockers.md)
- [Entry Bootstrap](../lifecycle/entry-bootstrap.md)
