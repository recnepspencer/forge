# Recipes

This page is the task-first companion to the feature docs for both resources
and forms.

If you know what you want to build but not which feature page owns it, start
here.

## Quick Answers

- ordinary local forms:
  [Forms Overview](../forms/index.md)
- patching complex edit forms:
  [Patching Complex Edit Forms](../forms/changes/patching-complex-edit-forms.md)
- async validation or server canonicalization:
  [Async Validation](../forms/validation/async-validation.md) and
  [Server Canonicalization](../forms/validation/server-canonicalization.md)
- unchanged submit denial:
  [Unchanged Forms And Submit Readiness](../forms/changes/unchanged-forms-and-submit-readiness.md)
- submit blockers or approval requirements:
  [Readiness Blockers](../forms/availability/readiness-blockers.md) and
  [Approval, Signature, And Review Requirements](../forms/availability/approval-signature-and-review-requirements.md)
- multi-step forms or next/back actions:
  [Controller-Local Steps](../forms/steps/controller-local-steps.md) and
  [Step Actions](../forms/steps/step-actions.md)
- submit execution or retry behavior:
  [Submit Actions](../forms/actions/submit-actions.md) and
  [Repeated Attempts And Idempotency](../forms/actions/repeated-attempts-and-idempotency.md)
- label sizing or accessibility reads:
  [Label Size And Control Sizing](../forms/layout/label-size-and-control-sizing.md) and
  [Accessibility Artifacts](../forms/layout/accessibility-artifacts.md)
- custom dropdowns, search inputs, or imperative controls:
  [Inputs And Controls](../forms/inputs/README.md),
  [Dropdowns, Comboboxes, And Search](../forms/inputs/dropdowns-comboboxes-and-search.md), and
  [Input Capability Matrix](../forms/inputs/input-capability-matrix.md)
- focus state, host facts, or offline blockers:
  [Focus, Touch, And Visited State](../forms/interaction/focus-touch-and-visited-state.md) and
  [Offline And Host Blockers](../forms/interaction/offline-and-host-blockers.md)
- entry bootstrap, exit state, or handoff visibility:
  [Entry Bootstrap](../forms/lifecycle/entry-bootstrap.md),
  [Exit Posture](../forms/lifecycle/exit-posture.md), and
  [Handoffs](../forms/lifecycle/handoffs.md)
- attachment transfers or media preview state:
  [Attachment Transfers](../forms/media/attachment-transfers.md) and
  [Media Visibility](../forms/media/media-visibility.md)
- resource-backed source truth, settlement, or replay/restore:
  [Resource Line Source](../forms/resource-backed/resource-line-source.md),
  [Resource Settlement](../forms/resource-backed/resource-settlement.md), and
  [Replay And Restore](../forms/resource-backed/replay-and-restore.md)
- locks, leases, comments, or reviewer collaboration:
  [Collaboration](../forms/collaboration/README.md),
  [Locks And Leases](../forms/collaboration/locks-and-leases.md), and
  [Comments And Presence](../forms/collaboration/comments-and-presence.md)
- route-coupled draft freeze, discard, or deferred submit:
  [Route-Coupled Forms](../forms/route-coupling/README.md),
  [Freeze, Discard, Defer, And Cleared Authority](../forms/route-coupling/freeze-discard-defer-and-cleared-authority.md), and
  [Route-Coupled Steps And Actions](../forms/route-coupling/route-coupled-steps-and-actions.md)
- form debugging, retained state history, or verification:
  [Diagnostics And History](../forms/diagnostics/README.md),
  [State History](../forms/diagnostics/state-history.md), and
  [Verification Packages](../forms/verification/verification-packages.md)
- fetch one item:
  [Fetch And Write Resources](../resources/fetch-and-write.md)
- create, update, or remove:
  [Fetch And Write Resources](../resources/fetch-and-write.md)
- canonical write-result reconciliation, partial mapping, or identity
  migration:
  [Mutation Response Reconciliation](../resources/mutation-response-reconciliation.md)
- mutation-response support matrix and "is this a happy path or just an honest
  denial?" questions:
  [Mutation Response Closeout Matrix](../resource-contracts/mutation-response-closeout-matrix.md)
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
- mutation-response closeout matrix:
  [Mutation Response Closeout Matrix](../resource-contracts/mutation-response-closeout-matrix.md)
- external push packets or basis refresh:
  [External Delivery And Compatibility](../resources/external-delivery-and-compatibility.md)
- raw family declarations:
  [Raw Escape Hatch](../resources/raw-escape-hatch.md)

## Forms Recipes

## Recipe: Ordinary Local Form

```ts
const source = signals.input({ title: "Ship docs", done: false });

const form = signals.form({
  source,
  fields: ({ field }) => ({
    title: field("title"),
    done: field("done"),
  }),
});

form.fields.title.set("Ship docs today");
console.log(form.effective());
console.log(form.readiness());
```

## Recipe: Complex Edit Form With Nested Patch Truth

```ts
const form = signals.form({
  source: {
    profile: { displayName: "Ship docs" },
    tags: ["regulated"],
  },
  fields: ({ field }) => ({
    displayName: field("profile.displayName"),
    firstTag: field(["tags", 0], { id: "firstTag" }),
  }),
});

form.fields.displayName.set("Published docs");
form.fields.firstTag.set("released");

console.log(form.patchPlan().operations);
```

## Recipe: Submit Lifecycle With Canonical Fulfillment

```ts
const form = signals.form({
  source: { title: "", status: "draft" },
  fields: ({ field }) => ({
    title: field("title"),
    status: field("status"),
  }),
  validation: ({ field }) => ({
    titleRequired: field("title", (value) => value.length > 0 || {
      kind: "invalid",
      message: {
        code: "title.required",
        severity: "error",
        audience: "user",
        visibility: "visible",
      },
    }),
  }),
  actions: ({ submit }) => ({ submit: submit() }),
});

form.fields.title.set("Ship docs");
const pending = form.executeAction("submit");
form.fulfillAction(pending.operationId, {
  canonicalValue: { title: "Ship docs", status: "published" },
});
```

## Recipe: Resource-Backed Form

```ts
const form = signals.form({
  source: signals.form.source.resourceLine(taskLine, { id: "task-resource" }),
  fields: ({ field }) => ({
    title: field("title"),
  }),
  actions: ({ submit }) => ({
    submit: submit({
      resourceEffectProfile: signals.resource.effects.branchNative(),
    }),
  }),
});

console.log(form.resourceSource());
console.log(form.actionPlan("submit").resourceEffectProfile);
```

## Recipe: Async Validation

```ts
const form = signals.form({
  source: { slug: "ship-docs" },
  fields: ({ field }) => ({
    slug: field("slug"),
  }),
  validation: ({ asyncField }) => ({
    slugUnique: asyncField("slug", {
      id: "slugUnique",
      triggers: ["submit"],
    }),
  }),
});

const pending = form.startAsyncValidation("slugUnique");
form.fulfillAsyncValidation(pending.operationId, {
  reason: "slug is unique",
});
```

## Recipe: Host Facts And Generated Layout

```ts
const form = signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title", { row: "hero" }),
  }),
  host: {
    focus: "title",
    online: true,
    viewport: { width: 1280, height: 720 },
  },
  measurement: {
    observe: ["animationFrame"],
  },
});

form.recordLayoutMeasurement([{ row: "hero", controlHeight: 32 }], {
  cause: "animationFrame",
});
console.log(form.host());
console.log(form.layoutMeasurement());
```

## Recipe: Collaboration Posture

```ts
const form = signals.form({
  source: { title: "Ship docs" },
  collaboration: {
    mode: "fieldLease",
    actorId: "me",
    supportsPresence: true,
  },
  fields: ({ field }) => ({
    title: field("title"),
  }),
});

form.reportCollaboration({
  posture: "blocked",
  leasedFields: [{ field: "title", ownerId: "peer-1" }],
  reason: "peer-1 owns the title lease",
});
console.log(form.fieldWritePosture("title"));
```

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
