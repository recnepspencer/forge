# React Adapter Reference

## What This Feature Is

The React adapter is the thin React-facing consumer for a `forge-signal-wasm`
signals instance.

It does not create a second state engine. It subscribes to signal truth that
already exists in the shared runtime.

## Why You Use It

- subscribe React components to signal handles
- reuse one shared signals instance across a React tree
- observe diagnostics from React without inventing a second observer layer
- keep React integration lightweight while the runtime stays authoritative

## Stable Entry Points

- `createReactSignalsStore(signals)`
- `ReactSignalsStoreProvider`
- `useReactSignalsStore()`
- `createResourceCatalog({ id, build })`
- `createResourceCatalog({ id, scope, domains })`
- `getResourceCatalog(signals, catalog)`
- `useResourceCatalog(store, catalog)`
- `useResourceCatalog(catalog, store?)`
- `useSignalsForm(declaration, store?, options?)`
- `useFormField(form, fieldId, store, options?)`
- `useFormAction(form, actionId, store)`
- `useSignalValue(signal, store)`
- `useOutputValue(output, store)`
- `useSignalsHistory(history)`
- `useBrowserHistoryStory(story)`
- `useRouterSession(routes, { history: "browser", ...options })`
- `useOptionalSignalValue(signal | null | undefined, store, options?)`
- `useOptionalResourceLine(line | null | undefined, store, options?)`
- `useOptionalResourceLineValue(line | null | undefined, store?, options?)`
- `optionalResourceLine(family, selection)`
- `useResourceLine(family, selection, store?, options?)`
- `useResourceView(line | null | undefined, store, options?)`
- `useResourceOperation(execution, store?)`
- `executeManagedResourceWrite(line, options?)`
- `managedResourceWriteRecovery`
- `useManagedResourceWrite({ line, ...options })`
- `useSignalsDiagnostics(store)`
- `useSignalsDiagnosticsValue(selector, store?)`
- `signals.featureStore(...)`
- `signals.local.dialogState(...)`
- `signals.local.listState(...)`
- `signals.local.formSource(...)`

## Core Mental Model

The React adapter wraps an existing `signals` instance in a small subscription
store:

- the runtime still owns state and mutation semantics
- the React store owns subscription fanout and cached snapshots
- hooks read from the store, not from a separate React-only model

If your app already has a `signals` instance, React should consume it. Do not
mirror signal state into another store just to render components.

## How It Executes

1. create a shared `signals` instance
2. create a React store from that instance
3. pass handles and the store into hooks
4. React subscribes through runtime `watch(...)`
5. React receives snapshot updates when the underlying signal truth changes

Diagnostics follow the same pattern through the runtime diagnostics surface.

History and router stories now have the same kind of first-class React lane:

- `useSignalsHistory(...)` for runtime branch truth
- `useBrowserHistoryStory(...)` for browser-history story truth
- `useRouterSession(...)` for retained browser-session routing truth

React should not mirror branch/story state into local `useState` just to render
ordinary session UI.

## Runtime-Scoped Resource Catalogs

Serious apps usually need more than one or two ad hoc families. They need one
runtime-scoped place to group domains, share API scope, and reuse the same
family construction across the app.

Do not rebuild that layer from scratch with app-local `WeakMap` folklore.

Use the catalog helpers instead.

```tsx
const adminCatalog = createResourceCatalog({
  id: "workplace-admin",
  scope(signals) {
    return signals.apiScope("workplace-admin", {
      baseUrl: "/api",
      headers: {
        "x-app-surface": "admin",
      },
    });
  },
  domains: {
    users(api, signals) {
      return buildUserFamilies(api, signals);
    },
    projects(api, signals) {
      return buildProjectFamilies(api, signals);
    },
    auditLogs(api, signals) {
      return buildAuditLogFamilies(api, signals);
    },
  },
});

const catalog = useResourceCatalog(adminCatalog);
```

This gives you:

- one catalog instance per signals runtime
- stable reuse across React rerenders
- a first-class grouping convention for domain families
- one shared `catalog.scope`
- both `catalog.projects` and `catalog.domains.projects`

If you need the catalog outside React, use:

- `getResourceCatalog(signals, adminCatalog)`

## Reactive History Consumption

History and browser-history stories are rich imperative surfaces, but React
should consume them through a reactive lane instead of manually re-reading and
mirroring their state after every command.

### `useSignalsHistory(...)`

```tsx
const historyView = useSignalsHistory(signals.history());

historyView.currentBranch;
historyView.branches;
historyView.canUndo;
historyView.canRedo;
```

This is the intended lane for branch-oriented UI such as:

- current branch chips
- branch registries
- branch pickers
- branch-aware status bars

### `useBrowserHistoryStory(...)`

```tsx
const story = signals.router.browserHistory.story();
const storyView = useBrowserHistoryStory(story);

storyView.current;
storyView.entries;
storyView.breadcrumbTrail;
storyView.backProvenance;
```

This is the intended lane for UI that renders:

- current routed session truth
- accumulated admitted entries
- current breadcrumb trail
- back provenance state

Do not hand-roll:

- `useState` mirrors of `history.current_branch()` / `history.branches()`
- `useState` mirrors of `story.current()` / `story.admittedEntries()`
- custom "command then refresh all history reads" glue

### `useRouterSession(...)`

When the app needs one retained browser-routing session instead of separate
manual ingress/admit/story orchestration, use `useRouterSession(...)`.

```tsx
const session = useRouterSession(routes, {
  history: "browser",
  initialLocation: "/admin/products",
});

await session.navigate(routes.admin.products.to());

session.currentRoute;
session.story.entries;
session.breadcrumbs;
```

`useRouterSession(...)` retains that session per runtime + route tree instead of
replaying the initial browser-load choreography on every remount.

This is the intended React lane for ordinary routed app shells where the UI
needs:

- current admitted route truth
- retained browser-history story entries
- breadcrumb trail
- typed session navigation

Do not rebuild this with app-local:

- browser ingress envelope creation
- `routes.admitBrowserHistoryIngress(...)`
- `story.record(...)`
- `useState` mirrors of current route or breadcrumb state

The hook owns that retained session authority and exposes one subscribed session
surface instead.

When the app needs a stable API identity without jumping straight to a catalog,
prefer `signals.apiScope(...)` over app-local `WeakMap` cache helpers:

```ts
const adminApi = signals.apiScope("workplace-admin", {
  baseUrl: "/api",
  headers: {
    "x-app-surface": "admin",
  },
});
```

## React Form Binding

The form runtime already owns:

- field writes
- blur/focus/touch reporting
- visible messages
- action planning
- pending execution history

But React still needs a thin, repeatable way to consume that truth without each
app rebuilding its own form facade.

Use `useSignalsForm(...)` as the primary React lane.

```tsx
const localFormSource = signals.local.formSource({
  identity: "edit-user-dialog",
  initial: initialValue,
});

const form = useSignalsForm({
  source: localFormSource.source,
  fields: ({ field, repeated }) => ({
    email: field("email"),
    role: field("role"),
    appIds: repeated("appIds", {
      itemIdentity: (appId) => appId,
    }),
  }),
  validate: ({ field, form }) => ({
    email:
      form.fields.role.value() === "admin" && !String(field.value() ?? "").includes("@")
        ? ["Admin users need a valid email address."]
        : [],
  }),
  actions: {
    submit: updateUserLine,
  },
});

return (
  <form>
    <input {...form.field("email")} />
    <select {...form.select("role", roleOptions)} />
    <MultiSelectField {...form.multiSelect("appIds", appOptions)} />

    {form.fieldState("email").messages.map((message, index) => (
      <p key={index}>{String(message.target)}</p>
    ))}

    <button type="button" onClick={() => form.reset()}>
      Cancel
    </button>

    <button
      disabled={form.actions.submit.disabled}
      onClick={() => form.actions.submit.execute()}
    >
      {form.actions.submit.pending ? "Saving..." : "Save"}
    </button>
  </form>
);
```

This keeps the stronger form model intact while giving React a first-class lane
for:

- text field props through `form.field(...)`
- checkbox props through `form.checkbox(...)`
- select props through `form.select(...)`
- multiselect props through `form.multiSelect(...)`
- field state through `form.fieldState(...)`
- source/draft/effective values through `form.source`, `form.draft`, and `form.effective`
- patch visibility through `form.patchPlan`
- action disabled and pending state through `form.actions.*`
- discard/reset through `form.reset()`

This is the intended authoring lane for ordinary CRUD dialogs and settings
forms.

If the form controller already exists and React only needs one low-level field
or action binding, the thinner hooks still exist as escape hatches:

- `useFormField(form, fieldId, store, options?)`
- `useFormAction(form, actionId, store)`

Those hooks are no longer the primary recommended form surface.

## Local Runtime Shapes

When React needs small local runtime state, do not rebuild raw
`signals.scope(identity)` plus `scope.spec.input(...)` scaffolding in every
feature.

Use the local helpers instead:

```ts
const dialog = signals.local.dialogState({
  identity: "project-dialog",
  modes: ["create", "edit", "delete"] as const,
  initial: {
    isOpen: false,
    mode: null,
    data: null as Project | DeleteTarget | null,
    context: null as DialogContext | null,
    loading: false,
  },
  collaboration: {
    mode: "singleWriterLock",
    actorId: "alex",
  },
  actions: ({ custom }) => ({
    saveDraft: custom({
      writes: true,
      execute: ({ state }) => persistDraft(state.data),
    }),
  }),
});

const projectForm = useSignalsForm({
  source: dialog.data,
  fields: ({ field }) => ({
    title: field("title"),
    status: field("status"),
  }),
  actions: {
    submit: updateProjectLine,
  },
});

dialog.bindForm(projectForm.controller, {
  confirmActionId: "submit",
  blockCloseWhenDirty: true,
  closeOnSuccess: true,
  stayOpenOnError: true,
});

const candidateUsers = signals.local.listState({
  identity: "candidate-users",
  initial: [],
});

const editUserSource = signals.local.formSource({
  identity: "edit-user-dialog",
  initial: initialValue,
});
```

These helpers give named local runtime shapes for common app patterns without:

- manual `signals.scope(identity)`
- manual `scope.spec.input(...)`
- repeated debug-name ceremony
- app-authored dialog controllers for mode, payload, context, loading,
  readiness, custom actions, and collaboration posture
- app-authored source binding wrappers for ordinary dialog form state

`signals.local.dialogState(...)` is no longer just a boolean visibility helper.
It is the local modal workflow surface. The returned controller exposes:

- state layers through `source()`, `draft()`, and `effective()`
- dialog-local dirty truth through `dirty()` and `patchPlan()`
- close and action posture through `readiness()`
- visible modal blockers through `visibleMessages()`
- custom and built-in actions through `action(...)` and `actions()`
- native dialog collaboration through `collaboration()`,
  `reportCollaboration(...)`, and `clearCollaboration(...)`
- real form composition through `bindForm(...)`

That is the intended lane for ordinary CRUD dialogs, multistep modal flows, and
collaborative modal workflows. Do not rebuild a parallel dialog controller with
extra `useState` for:

- mode / payload / context
- loading
- close blockers
- discard confirmation
- custom modal actions
- collaboration or reviewer posture

Each helper also owns the runtime handles it authors:

- `dialog.free()` / `dialog[Symbol.dispose]()`
- `candidateUsers.free()` / `candidateUsers[Symbol.dispose]()`
- `editUserSource.free()` / `editUserSource[Symbol.dispose]()`

## Scoped Feature Stores

When an app needs a retained feature-level store with authored actions, do not
invent a second store framework on top of raw `signals.scope(...)`.

Use `signals.featureStore(...)` instead.

```ts
type AuditQueryValues = {
  search: string;
  severities: readonly string[];
  actorIds: readonly string[];
};

const userGroupsStore = signals.featureStore({
  id: "workplace-user-groups-admin",
  state: {
    selectedGroupId: null as string | null,
    selectedCandidateId: "",
    view: "users" as "users" | "groups",
    queryValues: {
      search: "",
      severities: [],
      actorIds: [],
    } satisfies AuditQueryValues,
    layoutConfig: {
      density: "comfortable" as "comfortable" | "compact",
      visibleColumns: ["actor", "action", "timestamp"] as const,
    },
  },
  actions: ({ set, read }) => ({
    setSelectedGroupId: (next) => set("selectedGroupId", next),
    setSelectedCandidateId: (next) => set("selectedCandidateId", next),
    showGroups: () => set("view", "groups"),
    setQueryValues: (next) => set("queryValues", next),
    resetFilters: () =>
      set("queryValues", {
        ...read().queryValues,
        search: "",
        severities: [],
        actorIds: [],
      }),
  }),
});
```

This gives you:

- one scoped runtime per named store id
- named state handles such as `store.state.selectedGroupId`
- a store snapshot signal through `store.snapshot`
- object-shaped state without `as SignalValue` / `unknown as SignalValue`
  casts at the call site
- authored actions without rebuilding custom store plumbing

If the store belongs under an existing scope, use the scoped lane:

```ts
const catalogStore = signals.scope("admin").featureStore({
  id: "catalog",
  state: {
    selectedProductId: null as string | null,
  },
  actions: ({ set }) => ({
    setSelectedProductId: (next) => set("selectedProductId", next),
  }),
});
```

Feature stores also own the state handles they author:

- `userGroupsStore.free()`
- `userGroupsStore[Symbol.dispose]()`

## Optional Resource Selection

React components often need to render before a detail resource line exists.
Examples:

- no row is selected yet
- the current route has no detail target
- a parent flow has not chosen a resource identity yet

Do not solve this with:

- conditional hook calls
- fake sentinel params
- synthetic fallback signals authored in app code

Use the optional adapter hooks instead.

### `useOptionalSignalValue(...)`

```tsx
const result = useOptionalSignalValue(selectedSignal, store, {
  inactiveValue: "nothing selected",
});

if (result.kind === "inactive") {
  return <EmptyState title={result.value} />;
}

return <span>{result.value}</span>;
```

What this guarantees:

- hook order stays stable
- no runtime subscription is created while inactive
- inactive posture is explicit instead of pretending to be loading or error

### `useOptionalResourceLine(...)`

```tsx
const detail = selectedProductId
  ? productFamily.line({ productId: selectedProductId })
  : null;

const result = useOptionalResourceLine(detail, store, {
  inactiveValue: "Select a product",
});

if (result.kind === "inactive") {
  return <EmptyState title={result.value} />;
}

return (
  <section>
    <h2>{result.value.name}</h2>
    <small>
      status: {result.summary.status}
      {" | "}
      freshness: {result.freshness.kind}
    </small>
  </section>
);
```

When inactive, the hook returns:

- `kind: "inactive"`
- `reason: "authorInactive"`
- `line: null`
- `summary: null`
- `status: null`
- `freshness: null`
- `diagnosticsSummary: null`

When active, the hook returns the real line plus current:

- value
- summary
- status
- freshness
- diagnostics summary

This is the intended React lane for conditionally absent resident resource
truth.

### Family-first optional selection

When the component already owns a family and a maybe-selected identity, use the
family-first lane instead of manually building `null` lines in UI code.

```tsx
<ReactSignalsStoreProvider store={store}>
  <ProjectPermissionsPanel />
</ReactSignalsStoreProvider>
```

```tsx
function ProjectPermissionsPanel() {
  const projectPermissions = useResourceLine(
    catalog.projects.projectPermissions,
    selectedProjectId
      ? { projectId: selectedProjectId }
      : { enabled: false },
    undefined,
    {
      inactiveValue: "Select a project",
    },
  );
}
```

If a helper or non-React layer needs the same posture, use:

```ts
const projectPermissionsLine = optionalResourceLine(
  catalog.projects.projectPermissions,
  selectedProjectId ? { projectId: selectedProjectId } : { enabled: false },
);
```

If the line already exists and React only needs its maybe-active value, use:

```tsx
function ProjectPermissionsValue() {
  const projectPermissionsLine = catalog.projects.projectPermissions.optionalLine(
    selectedProjectId ? { projectId: selectedProjectId } : { enabled: false },
  );
  const permissions = useOptionalResourceLineValue(projectPermissionsLine, undefined, {
    inactiveValue: "Select a project",
  });
}
```

This keeps the "nothing selected yet" posture honest without fake params,
conditional hooks, or app-authored fallback signals.

Under the hood, the hook now consumes the line's grouped summary truth through
`line.summarySignal()` rather than polling `line.summary()`, `line.status()`,
and `line.diagnosticsSummary()` opportunistically after only a value update.

## Standard Resource View Projection

Application UI often needs a simpler content-state view over line truth:

- loading
- refreshing
- ready
- empty
- error

Do not hand-roll that mapping in every feature area.

Use `useResourceView(...)` when the UI needs a standard app-facing projection
without losing access to the real line/status/freshness/diagnostics surfaces.

```tsx
const products = catalog.products.line({});

const view = useResourceView(products, store, {
  emptyWhen(value) {
    return value.rows.length === 0;
  },
  errorMessage: "Unable to load products.",
});

switch (view.kind === "inactive" ? "inactive" : view.contentState) {
  case "inactive":
    return <EmptyState title="No selection" />;
  case "loading":
    return <ProductsSkeleton />;
  case "refreshing":
    return <ProductsTable rows={view.value.rows} refreshing />;
  case "empty":
    return <EmptyState title="No products yet" />;
  case "error":
    return <ErrorBanner message={view.message} />;
  default:
    return <ProductsTable rows={view.value.rows} />;
}
```

The active result keeps the real runtime truth attached:

- `contentState`
- `value`
- `summary`
- `status`
- `freshness`
- `diagnosticsSummary`
- `message`
- `hasVisibleValue`
- `isRefreshing`
- `isEmpty`

So the bridge standardizes app-facing content-state policy without pretending
the richer line truth disappeared.

## Managed Transient Resource Writes

React applications often need the same transient write lifecycle:

- create a write line
- expose pending UI state
- wait for settlement
- classify exact vs partial vs failure
- run follow-up behavior such as resident revalidation
- free the transient line

Do not hand-roll that sequence in every feature hook.

Use the managed write lane instead.

### `executeManagedResourceWrite(...)`

```tsx
const line = catalog.projects.createProject.line({
  body: {
    name: draft.name,
  },
});

const result = await executeManagedResourceWrite(line, {
  onPartial() {
    catalog.projects.list.line({}).revalidate();
  },
});

if (result.resultKind === "partial") {
  console.log(result.confirmationKind);
}
```

This helper owns:

- settlement waiting
- confirmation classification
- optional partial follow-up
- transient line cleanup

Under the hood, the helper now waits on the line's own lifecycle truth through
`line.execute().settled(...)` instead of polling `line.status()` in a loop.

If a feature hook needs the lower-level primitive directly, it can use the
resource line lane itself:

```tsx
const settlement = await line.execute().settled({
  timeoutMs: 15_000,
});

if (settlement.resultKind === "partial") {
  console.log(settlement.confirmationKind);
}
```

Or directly from the family:

```tsx
const settlement = await catalog.projects.createProject.execute({
  body: draft,
}).settled({
  timeoutMs: 15_000,
});
```

### Declarative Recovery Policy

When fallback settlement should trigger standard follow-up work, declare it
instead of rebuilding the same `if (partial) line.revalidate()` ritual in every
feature hook.

```tsx
const result = await executeManagedResourceWrite(line, {
  recovery: {
    partial: [
      managedResourceWriteRecovery.revalidate(
        () => catalog.projects.list.line({}),
        "refresh the project list",
      ),
    ],
  },
});

console.log(result.recovery);
```

If the recovery policy depends on the specific execution context, use
`recoveryPolicy(...)` instead of rebuilding bespoke `if (partial)` choreography:

```tsx
const createProject = useManagedResourceWrite({
  line(body) {
    return catalog.projects.createProject.line({ body });
  },
  recoveryPolicy({ args, result }) {
    if (result.resultKind !== "partial") {
      return;
    }
    return {
      partial: [
        managedResourceWriteRecovery.revalidate(
          () => catalog.projects.list.line({ workspaceId: args.workspaceId }),
          "refresh the affected workspace list",
        ),
      ],
    };
  },
});
```

This lane is intentionally narrow. It standardizes common resource follow-up
policy such as:

- `managedResourceWriteRecovery.revalidate(...)`
- `managedResourceWriteRecovery.refresh(...)`

It does not pretend the runtime can infer app-specific selection or dialog
policy automatically.

If one declared recovery step cannot run, that does not erase the primary write
settlement. The returned `result.recovery.executions` entries keep failed
follow-up steps as structured artifacts with:

- `status: null`
- `error`

so the caller can keep the write truth and still inspect the broken recovery
policy honestly.

If you already have a settled result and want to apply standard recovery after
the fact, use:

```tsx
const settled = await executeManagedResourceWrite(line, {
  freeOnSettle: false,
});
const recovered = await settled.recovery.apply({
  partial: [
    managedResourceWriteRecovery.revalidate(
      () => catalog.projects.list.line({}),
      "refresh the list after a partial command",
    ),
  ],
});
```

When app UX needs severity rather than raw fallback heuristics, use the recovery
summary instead of branching on confirmation kinds manually:

```tsx
const result = await executeManagedResourceWrite(line);
const summary = result.recovery.summary();

if (summary.severity === "info") {
  // safe delayed refresh
}

if (summary.severity === "warning") {
  // meaningful stale risk
}
```

### `useManagedResourceWrite(...)`

```tsx
const createProject = useManagedResourceWrite({
  line(body) {
    return catalog.projects.createProject.line({ body });
  },
  feedback: {
    success: "Project created",
    partial: "Project created, refreshing list",
    error: "Unable to create project",
  },
  onPartial() {
    catalog.projects.list.line({}).revalidate();
  },
});

await createProject.execute({
  name: "Northstar Trail Pack",
});
```

The hook returns:

- `pending`
- `lastFeedback`
- `lastResult`
- `lastError`
- `execute(args)`
- `reset()`

This is the intended React lane when transient resource writes should behave
like a routine application operation rather than app-authored orchestration
folklore.

When multiple write flows should share one standard lifecycle-to-feedback
bridge, provide copy once and consume the standardized artifact:

```tsx
const createProject = useManagedResourceWrite({
  line(body) {
    return catalog.projects.createProject.line({ body });
  },
  feedback: {
    success: "Project created",
    partial: "Project created, refreshing list",
    error: "Unable to create project",
  },
  onFeedback(feedback) {
    toastBridge.consume(feedback);
  },
});
```

For lower-level execution lanes, use a managed write execution directly:

```tsx
const execution = createManagedResourceWriteExecution(
  catalog.projects.createProject.line({ body }),
  {
    feedback: {
      success: "Project created",
      partial: "Project created, refreshing list",
      error: "Unable to create project",
    },
  },
);

toastBridge.consume(await execution.feedback());
```

To consume that same execution as reactive async truth in React, use the shared
operation hook:

```tsx
const execution = createManagedResourceWriteExecution(
  catalog.projects.createProject.line({ body }),
);

const operation = useResourceOperation(execution, store);

if (operation.pending) {
  return <Spinner label="Saving project..." />;
}

if (operation.resultKind === "partial") {
  return <Banner tone="warning">Saved with follow-up refresh.</Banner>;
}
```

The settled result also carries `result.feedback`, and
`managedResourceWriteFeedback.create(result, messages?)` can derive the same
standardized artifact from any already-settled managed write.

## Small Example

```tsx
import { createSignals } from "forge-signal-wasm";
import { createReactSignalsStore, useSignalValue } from "forge-signal-wasm/react";

const signals = await createSignals();
const count = signals.input(1);
const store = createReactSignalsStore(signals);

function CounterValue() {
  const value = useSignalValue<number>(count, store);
  return <span>{value}</span>;
}
```

This is the smallest honest example because:

- the runtime owns `count`
- React only subscribes and renders
- nothing is copied into a second app store

Add `debugName` only if you want friendlier diagnostics while inspecting the
shared runtime.

## Real Example

```tsx
import { createSignals } from "forge-signal-wasm";
import {
  createReactSignalsStore,
  useOutputValue,
  useSignalValue,
  useSignalsDiagnostics,
} from "forge-signal-wasm/react";

const signals = await createSignals();

const itemWorkspace = signals.graph("itemWorkspace", (graph) => {
  const editor = graph.controller("editor", ({ input, computed }) => {
    const serverItem = input({
      id: "task-7",
      title: "Ship docs",
    });
    const draft = input({});
    const effectiveItem = computed(() => ({
      ...serverItem(),
      ...draft(),
    }));
    const dirtyState = computed(() => Object.keys(draft()).length > 0);

    return {
      inputs: { serverItem, draft },
      outputs: { effectiveItem, dirtyState },
    };
  });

  return graph.expose({
    inputs: {
      draft: graph.input.optional(editor.inputs.draft),
    },
    outputs: {
      effectiveItem: editor.outputs.effectiveItem,
      dirtyState: editor.outputs.dirtyState,
    },
  });
});

const store = createReactSignalsStore(signals);

function ItemEditor() {
  const effectiveItem = useOutputValue<{ title?: string }>(
    itemWorkspace.output("effectiveItem"),
    store,
  );
  const dirtyState = useOutputValue<boolean>(
    itemWorkspace.output("dirtyState"),
    store,
  );
const diagnostics = useSignalsDiagnostics(store);

  return (
    <section>
      <h2>{effectiveItem.title ?? "Untitled"}</h2>
      <button
        onClick={() => itemWorkspace.patchInput("draft", { title: "Ready to ship" })}
      >
        Patch Draft
      </button>
      <small>
        dirty: {String(dirtyState)}
        {" | "}
        latest flow: {diagnostics.latestFlow?.graph?.id ?? "none"}
      </small>
    </section>
  );
}
```

For a focused diagnostics lane, use:

```tsx
const latestFlow = useSignalsDiagnosticsValue(
  (snapshot) => snapshot.latestFlow,
);
```

What is authoritative here:

- the graph and its inputs/outputs still live in the signal runtime
- the React store only tracks subscriptions and snapshots
- component actions still mutate through graph helpers or runtime transactions

## How It Relates To Other Features

- Pair this with the main app surface when React is your view layer.
- Pair it with published graphs when you want stable output handles for
  components.
- Pair it with diagnostics when you want a lightweight dev panel.
- Use host capabilities in the shared signals instance, not by reading browser
  globals directly in React components.

## API Notes

### `createReactSignalsStore(signals)`

Creates the shared React subscription store.

The returned store exposes:

- `signals`
- `subscribeSignal(signal, listener)`
- `getSignalSnapshot(signal)`
- `subscribeDiagnostics(listener)`
- `getDiagnosticsSnapshot()`
- `transaction(callback)`
- `batch(callback)`
- `refreshDiagnostics()`
- `performanceSummary()`
- `dispose()`

### `ReactSignalsStoreProvider`

Provides one shared React signals store to descendant hooks.

### `useReactSignalsStore()`

Reads the nearest provided React signals store.

### `createResourceCatalog({ id, build })`

Legacy fully custom catalog builder.

### `createResourceCatalog({ id, scope, domains })`

Declares a runtime-scoped resource catalog definition.

### `getResourceCatalog(signals, catalog)`

Gets the cached catalog instance for one signals runtime.

### `useResourceCatalog(store, catalog)`

Explicit-store consumption lane.

### `useResourceCatalog(catalog, store?)`

Provider-backed consumption lane. If the tree is wrapped in
`ReactSignalsStoreProvider`, the `store` argument can be omitted.

### `useSignalsForm(declaration, store?, options?)`

Use this as the primary React form lane when a component should author a form
controller and consume field/action bindings directly.

If the tree is wrapped in `ReactSignalsStoreProvider`, the `store` argument can
be omitted.

Pass `options.remountKey` when the form should be rebuilt for a new dialog
identity or entity identity instead of preserving the existing controller.

The returned binding keeps the ordinary CRUD surfaces on the facade instead of
forcing the caller to dig into `form.controller` for routine work:

- `form.fieldState(...)`
- `form.source`
- `form.draft`
- `form.effective`
- `form.patchPlan`
- `form.reset()`

### `useFormField(form, fieldId, store, options?)`

Uses the existing form field and input-binding surfaces through a React-friendly
hook. Prefer `useSignalsForm(...)` unless the controller already exists and the
component truly only needs one low-level field binding.

### `useFormAction(form, actionId, store)`

Uses action-plan and action-debug truth through a React-friendly hook. Prefer
`useSignalsForm(...)` unless the controller already exists and the component
truly only needs one low-level action binding.

### `useSignalValue(signal, store)`

Use this for input or computed handles.

### `useOutputValue(output, store)`

Use this for output handles, including graph-published outputs.

### `useOptionalSignalValue(signal | null | undefined, store, options?)`

Use this when the underlying signal handle is intentionally absent in the
current UI posture.

### `useOptionalResourceLine(line | null | undefined, store, options?)`

Use this when the current React surface may have no selected or mounted
resource line yet.

### `optionalResourceLine(family, selection)`

Use this when a helper or non-React layer needs an honest optional line from a
family plus maybe-selected params.

### `useResourceLine(family, selection, store?, options?)`

Use this when React should consume a family directly with an explicit
`{ enabled: false }` inactive posture. If the tree is wrapped in
`ReactSignalsStoreProvider`, the `store` argument can be omitted.

### `useResourceView(line | null | undefined, store, options?)`

Use this when React needs a standard content-state projection over resource
line truth.

### `executeManagedResourceWrite(line, options?)`

Use this when you already have a transient resource line and want one
standard execution/settlement helper.

### `useManagedResourceWrite({ line, ...options })`

Use this when React should own pending state and last-result tracking for a
transient write workflow.

### `useRouterSession(routes, { history: "browser", ...options })`

Use this when React should own one retained browser-routing session instead of
hand-authoring ingress creation, route admission, story recording, and current
route mirroring separately.

### `useSignalsDiagnosticsValue(selector, store?)`

Use this when React should consume one diagnostics slice directly instead of
subscribing to the whole diagnostics snapshot and projecting it manually.

### `useSignalsDiagnostics(store)`

Returns:

- `latestObservation`
- `latestFlow`
- `performanceSummary`

## Inspection And Debugging

Useful store surfaces:

- `store.getSignalSnapshot(...)`
- `store.getDiagnosticsSnapshot()`
- `store.refreshDiagnostics()`
- `store.performanceSummary()`

Useful runtime surfaces behind the store:

- `signals.diagnostics()`
- graph `inspectDiagnostics()`

The React store is for consumption and fanout. If you need graph-contract or
history truth, inspect the graph or runtime directly.

## Anti-Patterns

- copying signal state into a second React store
- building component-local mirrors for values you already have as signal handles
- using React state as the authoritative source when the runtime should own it
- reading ambient browser state in components instead of host capability
- disposing the shared store while components still depend on it

## Current Limits

- the React adapter is intentionally thin
- it does not define a React-only mutation language
- it does not replace graph diagnostics/history surfaces
- it assumes you already have a shared `signals` instance to consume

## Related Docs

- [App Surface Overview](./overview.md)
- [Host Capabilities](./host-capabilities.md)
- [Diagnostics And History](./diagnostics-and-history.md)
