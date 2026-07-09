# Path, Search, And Hash State

## What This Feature Is

This is the typed route-input lane for params, search fields, and hash fields.

## Why You Use It

- declare route input shape in one place
- avoid stringly query parsing in app code
- build and match routes with typed search and hash values

## Stable Entry Points

- `signals.router.search.optional.*()`
- `signals.router.search.required.*()`
- `signals.router.hash.string()`
- `signals.router.route(..., { search, hash })`
- `routeRef.to(...)`
- `routeRef.match(...)`

## Core Mental Model

Path params come from the route pattern. Search and hash fields are declared on
the route. Once declared, the router owns parsing, canonicalization, and typed
reads for those fields.

## How It Executes

1. declare a route pattern
2. attach `search` and `hash` schemas when needed
3. build or match a route location
4. read typed `params`, `search`, and `hash` from route artifacts

## Small Example

```ts
const routes = signals.router.define({
  projectRoute: signals.router.route("/projects/:projectId", {
    search: {
      tab: signals.router.search.optional.string(),
      page: signals.router.search.required.number(),
    },
    hash: signals.router.hash.string(),
  }),
});
```

## Real Example

```ts
const location = routes.projectRoute.to({
  params: { projectId: "p7" },
  search: { tab: "files", page: 2 },
  hash: "section-a",
});

const matched = routes.projectRoute.match("/projects/p7?tab=files&page=2#section-a");

console.log(location.search.page);
console.log(matched?.hash);
```

The route declaration is authoritative for how these values are interpreted.

## How It Relates To Other Features

- use [Route Schema Authoring](../projection/route_schema_authoring.md) for the
  full route declaration story
- use [Canonical URL Authority](./canonical_url_authority.md) when you need the
  normalized result of these declarations

## Inspection And Debugging

- `routeRef.descriptor().pathParamNames`
- `routeRef.descriptor().searchKeys`
- `canonical.search`
- `canonical.hash`

## Anti-Patterns

- parsing `window.location.search` beside the router
- keeping route state in parallel component-local query parsers
- using hash text as an untyped mini-router beside the declared route

## Current Limits

- current hash support is string-shaped
- search fields are typed by the shipped router scalar families

## Related Docs

- [Route Schema Authoring](../projection/route_schema_authoring.md)
- [Canonical URL Authority](./canonical_url_authority.md)
