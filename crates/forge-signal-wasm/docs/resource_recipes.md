# Resource Recipes

This page is the task-first companion to the feature docs.

If you know what you want to build but not which feature page owns it, start
here.

## Quick Answers

- fetch one item:
  [feature_fetch_and_write_api_resources.md](./feature_fetch_and_write_api_resources.md)
- create, update, or remove:
  [feature_fetch_and_write_api_resources.md](./feature_fetch_and_write_api_resources.md)
- auth, retry policy, request context, or continuation:
  [feature_request_posture_and_policy.md](./feature_request_posture_and_policy.md)
- list or paged collections:
  [feature_fetch_and_write_api_resources.md](./feature_fetch_and_write_api_resources.md)
- item patching, summaries, or delivery:
  [feature_collections_and_delivery.md](./feature_collections_and_delivery.md)
- signed upload or multipart upload:
  [feature_transfers.md](./feature_transfers.md)
- downloads or multipart downloads:
  [feature_downloads.md](./feature_downloads.md)
- line reads and debugging:
  [feature_line_inspection.md](./feature_line_inspection.md)
- exact restore, replay availability, or verification packages:
  [feature_history_and_restore.md](./feature_history_and_restore.md)
- branch-native optimistic effects, response topology proof, JSON effects, or UI
  lifecycle event reads:
  [feature_branch_native_resource_effects.md](./feature_branch_native_resource_effects.md)
- external push packets or basis refresh:
  [feature_external_delivery_and_compatibility.md](./feature_external_delivery_and_compatibility.md)
- raw family declarations:
  [feature_raw_escape_hatch.md](./feature_raw_escape_hatch.md)

## Recipe: Route-First Detail

```ts
const userDetail = api.url("/users/:userId").detail({
  load: ({ userId }) => ({ id: userId, name: `User ${userId}` }),
});

const line = userDetail.line({ userId: "u1" });
console.log(line.summary());
```

## Recipe: Create With The Standard Write Lane

```ts
const createUser = api.url("/users").create({
  load: ({ body }) => ({ id: body.userId, name: body.name }),
});

const line = createUser.line({
  body: { userId: "u1", name: "Ada" },
});
```

## Recipe: Collection With Item Patching

```ts
const tasks = api.url("/tasks")
  .items((item: { id: string; title: string }) => item.id)
  .aspect(
    "title",
    (item) => item.title,
    (item, title: string) => ({ ...item, title }),
  )
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

## Recipe: Branch-Native Effect With Lifecycle Reads

```ts
const branchNativeApi = signals.api({
  effects: signals.resource.effects.branchNative(),
});

const tasks = branchNativeApi.url("/branch-native-tasks")
  .items((task: { id: string }) => task.id)
  .aspect("title", (task) => task.title, (task, title: string) => ({
    ...task,
    title,
  }))
  .list({
    load: () => [{ id: "task:1", title: "First" }],
  });

const line = tasks.line({});
line.patch(tasks.patch.itemAspect({
  itemId: "task:1",
  aspect: "title",
  value: "Draft",
}));

const effect = line.diagnostics().lastEffect;
const events = line.history().lifecycle.map((entry) => entry.lastOutcome);
```

## Recipe: Signed Upload With Deferred Processing

```ts
const receiptUpload = api.url("/receipts/upload")
  .signedUpload({
    method: "POST",
    finalizeRequired: true,
  })
  .processing("poll")
  .create({
    load: ({ body }) => ({ receiptId: body.receiptId }),
  });

const line = receiptUpload.line({ body: { receiptId: "r1" } });
console.log(line.upload());
console.log(line.processing());
```

## Recipe: Builder-Owned Download

```ts
const reportDetail = api.url("/reports/:reportId")
  .downloads(({ reportId }, _value: { id: string }, download) => [
    download.file("report-pdf", {
      fileName: `${reportId}.pdf`,
      mediaType: "application/pdf",
      download: download.ready({
        url: `https://downloads.example/${reportId}.pdf`,
      }),
    }),
  ])
  .detail({
    load: ({ reportId }) => ({ id: reportId }),
  });

console.log(reportDetail.line({ reportId: "r1" }).download());
```

## Recipe: Multipart Download Handoff

```ts
const exportDetail = api.url("/exports/:exportId")
  .downloads(({ exportId }, _value: { id: string }, download) => [
    download.export("export-bundle", {
      fileName: `${exportId}.zip`,
      mediaType: "application/zip",
      download: download.multipart({
        url: `https://downloads.example/${exportId}`,
        fields: { token: exportId },
        objectKey: `exports/${exportId}.zip`,
      }),
    }),
  ])
  .detail({
    load: ({ exportId }) => ({ id: exportId }),
  });
```

## Recipe: Raw Escape Hatch

```ts
const userDetail = signals.resource.detail({
  params: resourceParams(),
  normalizeParams: ({ userId }) =>
    resourceParamIdentity({ userId }, `/users/${userId}`),
  load: ({ userId }) => ({ id: userId, name: `User ${userId}` }),
});
```

## Related Docs

- [start_here.md](./start_here.md)
- [feature_index.md](./feature_index.md)
