# Canonical URL Authority

## What This Feature Is

Canonical URL authority is the normalized route-shaped URL truth used by the
router once path, search, and hash state have been admitted into one stable
representation.

## Why You Use It

- compare semantically equivalent URLs without string folklore
- hold a normalized route-facing URL artifact in inspection or caching code
- move from raw browser or app input into route-safe truth

## Stable Entry Points

- `signals.router.canonical(...)`
- `routeRef.canonical(...)`
- `routeLocation.canonical()`

## Core Mental Model

A canonical URL artifact is still URL authority, not route admission. It tells
you what the normalized href is and, for route-backed calls, which typed route
state that href resolves to. It does not tell you whether prerequisites or
recovery allow the route.

## How It Executes

1. take a path or href
2. normalize it into one canonical local form
3. compute stable search and hash digests
4. expose verification and equivalence digests for downstream use

## Small Example

```ts
const canonical = signals.router.canonical("/projects/p7?tab=files#section-a");

console.log(canonical.href);
console.log(canonical.pathname);
```

This is the smallest honest example because it shows the direct canonical lane
without adding route matching.

## Real Example

```ts
const routes = signals.router.define({
  projectRoute: signals.router.route("/projects/:projectId", {
    search: {
      tab: signals.router.search.optional.string(),
    },
    hash: signals.router.hash.string(),
  }),
});

const canonical = routes.projectRoute.canonical({
  params: { projectId: "p7" },
  search: { tab: "files" },
  hash: "section-a",
});

console.log(canonical.search.tab);
console.log(canonical.hash);
console.log(canonical.equivalenceDigest);
```

Authoritative truth is the canonical route artifact. Projection, admission, and
history can derive from it later.

## How It Relates To Other Features

- use [Raw Location Authority](./raw_location_authority.md) when you are still
  holding browser-shaped input
- use [Route Identity And Equivalence](./route_identity_and_equivalence.md)
  when you care about sameness, not just normalized shape
- use [Admit](../admission/admit.md) when route policy and recovery matter

## Inspection And Debugging

- `canonical.descriptor()`
- `canonical.verification()`
- `canonical.searchDigest`
- `canonical.hashDigest`
- `canonical.equivalenceDigest`

## Anti-Patterns

- treating canonical URL truth as admitted route truth
- comparing raw href strings when the router already exposes equivalence
- rebuilding query parsing in app code beside the typed search lane

## Current Limits

- canonical truth is about normalized route shape, not policy admission
- route-specific canonical artifacts depend on a declared route reference
- external or ambient browser reads are still separate host-boundary concerns

## Related Docs

- [Raw Location Authority](./raw_location_authority.md)
- [Route Identity And Equivalence](./route_identity_and_equivalence.md)
- [Route Schema Authoring](../projection/route_schema_authoring.md)
