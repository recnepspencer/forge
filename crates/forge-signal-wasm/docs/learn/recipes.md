# Resource Recipes

This page is the task-first companion to the feature docs.

If you know what you want to build but not which feature page owns it, start
here.

## Quick Answers

- fetch one item:
  [Fetch And Write Resources](../resources/fetch-and-write.md)
- create, update, or remove:
  [Fetch And Write Resources](../resources/fetch-and-write.md)
- auth, retry policy, request context, or continuation:
  [Request Posture And Policy](../resources/request-posture-and-policy.md)
- list or paged collections:
  [Fetch And Write Resources](../resources/fetch-and-write.md)
- item patching, summaries, or delivery:
  [Collections And Delivery](../resources/collections-and-delivery.md)
- signed upload or multipart upload:
  [Transfers](../resources/transfers.md)
- downloads or multipart downloads:
  [Downloads](../resources/downloads.md)
- line reads and debugging:
  [Line Inspection](../resources/line-inspection.md)
- exact restore, replay availability, or verification packages:
  [History And Restore](../resource-contracts/history-and-restore.md)
- branch-native optimistic effects, response topology proof, JSON effects, or UI
  lifecycle event reads:
  [Branch-Native Resource Effects](../resources/branch-native-effects.md)
- effect envelope fields:
  [Effect Envelope Contract](../resource-contracts/effect-envelope.md)
- effect merge or rebase:
  [Effect Merge And Rebase](../resources/merge-and-rebase.md)
- JSON path effects:
  [JSON Path Effects](../resources/json-effects.md)
- response topology proof:
  [Response Topology Proof](../resource-contracts/response-topology-proof.md)
- closeout matrix reads:
  [Effect Closeout Matrix](../resource-contracts/closeout-matrix.md)
- external push packets or basis refresh:
  [External Delivery And Compatibility](../resources/external-delivery-and-compatibility.md)
- raw family declarations:
  [Raw Escape Hatch](../resources/raw-escape-hatch.md)

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

## Recipe: Inspect An Effect Envelope

```ts
const effect = line.diagnostics().lastEffect;

console.log(effect?.effectId);
console.log(effect?.provenance);
console.log(effect?.profile?.name);
console.log(effect?.optimistic.rollback.kind);
console.log(effect?.locus.kind);
```

## Recipe: Plan A Resource Effect Merge

```ts
const effect = line.diagnostics().lastEffect;

const plan = signals.resource.branch.planEffectMerge({
  merge: {
    source_branch_id: effect.optimistic.branchId,
    target_branch_id: 0,
    conflict_isolation_policy_name: "signal.conflict-isolation.per-node",
  },
  effect,
});

console.log(plan.kind);
console.log(plan.resourceEffect?.rebaseArtifact.kind);
```

## Recipe: Roll Back The Last Resource Effect

```ts
const beforeRollback = line.diagnostics().lastEffect;
const rollback = line.history().rollbackLastEffect();

console.log(beforeRollback?.optimistic.rollback.kind);
console.log(rollback.kind);
```

## Recipe: JSON Path Item Aspect

```ts
const response = signals.resource.response.objectItems<{
  tasks: Array<{ id: string; metadata: { priority: number } }>;
}>()({
  field: "tasks",
  itemId: (task) => task.id,
  aspects: signals.resource.response.jsonPathAspects<{
    id: string;
    metadata: { priority: number };
  }>()({
    priority: { field: "metadata", path: ["priority"] },
  }),
});
```

## Recipe: Read Response Topology Proof

```ts
const response = signals.resource.response.map<{
  tasks: ReadonlyMap<string, { id: string; title: string }>;
}>()({
  itemId: (task) => task.id,
  entries: (value) => value.tasks,
  replaceEntries: (value, tasks) => ({ ...value, tasks }),
  replaceEntry: (value, itemId, nextItem) => {
    const tasks = new Map(value.tasks);
    tasks.set(itemId, nextItem);
    return { ...value, tasks };
  },
});

console.log(line.diagnostics().lastEffect?.locusProof?.topology);
```

## Recipe: Compare Effect Profile Closeout

```ts
const matrix = signals.resource.effects.closeoutMatrix(
  signals.resource.effects.branchNative(),
);

console.log(matrix.profileName);
console.log(matrix.rows.map((row) => row.effectFamily));
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

- [start_here.md](../start_here.md)
- [Feature Index](./feature-index.md)
