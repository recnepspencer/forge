# Reading And Caching

Use this guide when the question is not “how do I fetch JSON?” but “which value
is visible, why was this line reused, and should it load again?”

## The Mental Model

A family turns params into canonical identity. A line retains browser-local
resource state for one identity. That state includes:

- the latest confirmed value observed from the server;
- an explicit pending, fulfilled, rejected, or timed-out status;
- fresh or stale posture;
- the admitted request descriptor;
- diagnostics and, where supported, retained history;
- a projected value when open optimistic effects are present.

The cache key is not a string you manage beside the family. It follows from the
family's canonical param identity. Two calls with canonically equivalent params
refer to the same logical member; unstable normalization creates accidental
duplicates.

## Read A Line

```ts
const project = projectDetail.line({ projectId: "project-42" });

console.log(project.value());
console.log(project.status());
console.log(project.freshness());
console.log(project.summary());
```

`value()` is the currently projected value. It may be `null` during an initial
asynchronous load. `summary()` is the best grouped read for UI and debugging;
it does not duplicate the underlying truth or inline the whole history.

When code cannot continue without settlement:

```ts
const result = await project.awaitSettlement({ timeoutMs: 5_000 });

switch (result.resultKind) {
  case "fulfilled":
  case "partial":
    useProject(result.value);
    break;
  case "rejected":
  case "timedOut":
    reportProjectFailure(result.summary);
    break;
}
```

`partial` is a successful settlement with explicitly incomplete canonical
truth. Do not collapse it into either full success or failure when the
distinction matters to the application.

## Invalidate, Refresh, And Revalidate

```ts
project.invalidate();
project.refresh();
await project.awaitSettlement();
```

- `invalidate()` keeps the visible value and marks the line stale.
- `refresh()` starts a new load using the line's admitted request.
- `revalidate()` starts the family's revalidation behavior.

A refresh can preserve the previous visible value while the new request is
pending. Read status and freshness separately: “I have something to show” and
“that value is current” are different facts.

If a newer refresh supersedes an older load, the runtime keeps the line's
lifecycle coherent. Application code should not race two component caches and
choose a winner itself.

## Canonical And Projected Value

Confirmed server observations are canonical for the line. An admitted
optimistic effect can contribute to the projected value returned by `value()`
while its request is unresolved. The runtime retains the distinction so one
failed effect can retire without erasing confirmed siblings.

If the UI needs to explain that distinction, inspect `line.effects().projection()`
on a family using the branch-native effect profile. Do not infer canonical
truth by subtracting optimistic changes from the current object.

## Identity Rules

- Detail params must identify one logical record.
- Collections must declare stable item identity for item-level behavior.
- Paged families also declare how pages accumulate.
- Raw families use `normalizeParams(...)` and `resourceParamIdentity(...)` when
  route-derived identity is not enough.
- Display labels, array positions, timestamps generated during render, and
  object serialization order are poor identity sources.

## Common Mistakes

- Building a second React or store cache beside a resource line.
- Recreating a family to force a reload instead of refreshing its line.
- Treating stale as empty or pending as rejected.
- Assuming a previous visible value proves the latest request succeeded.
- Using `value()` alone when the UI or workflow needs lifecycle truth.

## Go Deeper

- [How Resource Caching Works](./how-resource-caching-works.md)
- [Cache Keys And Resource Identity](./cache-keys-and-resource-identity.md)
- [Stale, Pending, And Settled State](./stale-pending-and-settled-state.md)
- [Invalidation And Refresh](./invalidation-and-refresh.md)
- [Authoritative Vs Derived Resource Truth](./authoritative-vs-derived-resource-truth.md)
- [Resource Line Reference](../../api-reference/resource-line.md)
