# Common Resource Recipes

## What This Feature Is

This is the quick pattern page for the first few resource jobs people usually
have.

## Recipe: One Detail

```ts
const userDetail = api.url("/users/:userId").detail({
  load: ({ userId }) => ({ id: userId, name: `User ${userId}` }),
});
```

## Recipe: One Collection

```ts
const tasks = api.url("/workspaces/:workspaceId/tasks")
  .items((item: { id: string; title: string }) => item.id)
  .list({
    load: ({ workspaceId }) => [{ id: `${workspaceId}:1`, title: "First" }],
  });
```

## Recipe: One Paged List

```ts
const taskPages = api.url("/tasks/search")
  .items((item: { id: string }) => item.id)
  .paged({
    accumulatePage: (existing, next) => [...existing, ...next],
    load: ({ params }) => [{ id: `page:${params.query}:1` }],
  });
```

## Recipe: Shared Auth And Context

```ts
const workspaceApi = signals.api({
  auth: resourceAuth.workspace(),
}).scope({
  requestContext: ({ workspaceId }) =>
    resourceRequestContext({
      headers: { "x-workspace-id": workspaceId },
      correlationId: `workspace:${workspaceId}`,
    }),
});
```

## Recipe: Standard Create

```ts
const createUser = api.url("/users").create({
  load: ({ body }) => ({ id: body.userId, name: body.name }),
});
```

## Recipe: Branch-Native Patch

```ts
const tasks = signals.api({
  effects: signals.resource.effects.branchNative(),
}).url("/tasks")
  .items((item: { id: string; title: string }) => item.id)
  .aspect("title", (item) => item.title, (item, title: string) => ({
    ...item,
    title,
  }))
  .list({
    load: () => [{ id: "t1", title: "First" }],
  });

const line = tasks.line({});
line.patch(
  tasks.patch.itemAspect({
    itemId: "t1",
    aspect: "title",
    value: "Updated",
  }),
);
```

## Related Docs

- [Fetching Data](../fetching/README.md)
- [Updating Data](../updating/README.md)
- [Resource Recipes](../recipes.md)
