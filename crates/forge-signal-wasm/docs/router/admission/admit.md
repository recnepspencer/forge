# Admit

## What This Feature Is

Admission is the router step that resolves a projected route into an admitted
route outcome or a non-admitted terminal outcome.

## Why You Use It

- turn route matches into policy-aware route truth
- run prerequisites and recovery in the router instead of framework-local
  glue
- get one typed outcome surface for success and failure

## Stable Entry Points

- `routes.admit(...)`
- `candidate.admission(...)`
- `plan.resolve()`

## Core Mental Model

Projection says what matched. Admission says what is actually true now after
prerequisites, recovery, and route-local forms authority have had their say.

## How It Executes

1. project the route
2. evaluate prerequisites
3. if needed, run recovery
4. produce an admitted or non-admitted outcome
5. emit diagnostics and provenance

## Small Example

```ts
const outcome = await routes.admit("/projects/p7");

console.log(outcome.kind);
```

## Real Example

```ts
const projected = routes.project("/projects/p7");

if (projected) {
  const plan = projected.admission({
    tenant: "acme",
  });

  console.log(plan.prerequisiteNames());
  console.log(plan.provenance());

  const outcome = await plan.resolve();
  console.log(outcome.kind);
}
```

## How It Relates To Other Features

- use [Projected Candidates](../projection/projected_candidates.md) before
  admission
- use [Route Outcomes](./route_outcomes.md) to interpret the result surface

## Inspection And Debugging

- `plan.provenance()`
- `outcome.diagnostics()`
- `outcome.provenance()`
- `outcome.recovery()`

## Anti-Patterns

- treating `routes.project(...)` as a substitute for `routes.admit(...)`
- rebuilding prerequisite and redirect logic outside the router

## Current Limits

- admission only sees the facts and admission sources you declare explicitly

## Related Docs

- [Route Outcomes](./route_outcomes.md)
- [Prerequisites](./prerequisites.md)
- [Admission Facts](./admission_facts.md)
