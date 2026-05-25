# Route Identity And Equivalence

## What This Feature Is

Route identity and equivalence are the router surfaces that let you talk about
one route as a stable declared thing, and talk about multiple hrefs as the same
route truth when their canonical meaning matches.

## Why You Use It

- distinguish route declaration identity from raw href strings
- compare navigations by route meaning
- use stable route ids in history, diagnostics, and host-boundary envelopes

## Stable Entry Points

- `routeRef.descriptor()`
- `routeLocation.descriptor()`
- `canonicalArtifact.descriptor()`
- `canonicalArtifact.equivalenceDigest`

## Core Mental Model

`routeId` names the declared route. `equivalenceDigest` names canonical
sameness for one built route artifact. They are related but not identical:

- `routeId` says which route family this is
- `equivalenceDigest` says whether two concrete route instances are the same
  normalized route truth

## How It Executes

1. a route declaration becomes a stable `routeId`
2. built route artifacts expose a descriptor for structural identity
3. canonical artifacts compute equivalence digests for same-meaning comparison

## Small Example

```ts
const routes = signals.router.define({
  projectRoute: signals.router.route("/projects/:projectId"),
});

const location = routes.projectRoute.to({ params: { projectId: "p7" } });

console.log(location.routeId);
console.log(location.descriptor().declarationPath);
```

## Real Example

```ts
const routes = signals.router.define({
  projectRoute: signals.router.route("/projects/:projectId", {
    search: {
      tab: signals.router.search.optional.string(),
    },
  }),
});

const left = routes.projectRoute.canonical({
  params: { projectId: "p7" },
  search: { tab: "files" },
});

const right = routes.projectRoute.match("/projects/p7?tab=files")?.canonical();

console.log(left.routeId === right?.routeId);
console.log(left.equivalenceDigest === right?.equivalenceDigest);
```

This is the right comparison lane when you care about route truth equality, not
string formatting.

## How It Relates To Other Features

- use [Path, Search, And Hash State](./path_search_hash_state.md) when you need
  typed route input
- use [Route Outcomes](../admission/route_outcomes.md) when identity alone is
  not enough and admission semantics matter

## Inspection And Debugging

- `descriptor().routeId`
- `descriptor().scopeId`
- `descriptor().declarationPath`
- `equivalenceDigest`

## Anti-Patterns

- using `routeIdentity` host-envelope fields as if they were a replacement for
  canonical equivalence
- comparing raw search-string order instead of canonical route artifacts

## Current Limits

- route identity does not imply admission
- equivalence is defined only for canonical route truth, not arbitrary raw
  href text

## Related Docs

- [Canonical URL Authority](./canonical_url_authority.md)
- [Path, Search, And Hash State](./path_search_hash_state.md)
- [Browser History Writeback](../history/browser_history_writeback.md)
