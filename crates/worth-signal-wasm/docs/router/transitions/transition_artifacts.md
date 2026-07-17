# Transition Artifacts

`routes.transition(...)` changes route truth from one **admitted** outcome to a
target. It returns an explanation artifact; it does not render a page or mutate
browser history.

```ts
const current = await routes.admit("/");

if (current.kind === "admitted") {
  const transition = await routes.transition(current, "/projects/p7", {
    facts: admissionFacts,
  });

  console.log(transition.target().kind);
  console.log(transition.diagnostics().visiblePolicy);
}
```

## Why The Artifact Matters

The target may be admitted, redirected, rejected, or temporarily backed by
pending resources. `transition.diagnostics()` tells you:

- the requested source (`directNavigation`, `speculativeCommit`, `redirect`,
  or `prefetchAdmission`)
- why visible truth changed
- the continuity policy used
- which resources were pending

That is more useful than a boolean “navigation finished.”

## Prefetched Targets

A projected prefetch artifact can be the target. Narrow the nullable result and
dispose it when its lifetime ends.

```ts
const prefetched = routes.warmup("/projects/p7", "intent");

if (current.kind === "admitted" && prefetched) {
  try {
    const transition = await routes.transition(current, prefetched);
    console.log(transition.diagnostics().requestedSource);
  } finally {
    prefetched.free();
  }
}
```

The router's transition artifact and the host's browser writeback are separate
steps. Keeping them separate lets an application inspect or reject target truth
before performing a browser side effect.

Next: [Pending Visibility](./pending_visibility.md),
[Continuity Preservation](./continuity_preservation.md), and
[Browser History Writeback](../history/browser_history_writeback.md).
