# Prerequisites

## What This Feature Is

Prerequisites are declared route admission checks created with
`signals.router.prerequisite(...)`.

## Why You Use It

- keep route gating in typed router declarations
- consume explicit host, resource, or graph admission sources
- produce structured admission artifacts instead of boolean guard folklore

## Stable Entry Points

- `signals.router.prerequisite(...)`
- `signals.router.host.*(...)`
- `signals.router.resource.*(...)`
- `signals.router.graph.*(...)`
- `route(..., { admission: [...] })`

## Core Mental Model

A prerequisite is a named evaluation step. It returns router-understood truth
that can become admission artifacts such as redirect, forbidden, unavailable,
or denied.

## How It Executes

1. declare the prerequisite
2. attach it to a route
3. admission evaluates it with route context and consumed sources
4. the result becomes part of route diagnostics and provenance

## Small Example

```ts
const onlineSource = signals.router.host.boolean("online");

const requiresOnline = signals.router.prerequisite("requiresOnline", {
  consumes: [onlineSource],
  evaluate: ({ consume, unavailable }) =>
    consume(onlineSource) ? true : unavailable({ reason: "offline" }),
});
```

## Real Example

```ts
const tenantSource = signals.router.host.string("tenantId");

const requiresTenant = signals.router.prerequisite("requiresTenant", {
  consumes: [tenantSource],
  evaluate: ({ consume, href, redirect }) =>
    consume(tenantSource)
      ? true
      : redirect({
          href: `/select-tenant?returnTo=${encodeURIComponent(href)}`,
        }),
});

const routes = signals.router.define({
  project: signals.router.route("/projects/:projectId", {
    admission: [requiresTenant],
  }),
});
```

## How It Relates To Other Features

- use [Access Policy](./access_policy.md) for the broader mental model of
  admission policy
- use [Admission Facts](./admission_facts.md) when prerequisites need caller
  supplied facts

## Inspection And Debugging

- `plan.prerequisiteNames()`
- `outcome.diagnostics().prerequisiteDecisions`
- `outcome.provenance().prerequisiteDecisions`

## Anti-Patterns

- placing route prerequisites in component render code
- returning raw booleans where a structured artifact is the real meaning

## Current Limits

- prerequisites only consume sources the router explicitly admits

## Related Docs

- [Access Policy](./access_policy.md)
- [Admission Facts](./admission_facts.md)
- [Admit](./admit.md)
