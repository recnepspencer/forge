import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { QueryClient, QueryClientProvider, useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { createSignals } from "forge-signal-wasm";
import {
  createReactSignalsStore,
  useManagedResourceWrite,
  useResourceLine,
} from "forge-signal-wasm/react";

import { DemoPreface } from "./DemoPreface";
import { ResourcesSectionCodeSample } from "./ResourcesSectionCodeSample";
import { DiagnosticsComparison, ResourceFeedbackGuide, ResourcePanel } from "./ResourcesSectionParts";
import "./resourcesSection.css";
import {
  comparisonApiUrls,
  createComparisonStore,
  createPanelEvent,
  formatItemRows,
  optimisticItem,
  summarizeRecovery,
  type ListItem,
  type PanelEvent,
  type ResourceDemoDiagnostics,
  type SaveItemBody,
} from "./resourcesSectionSupport";

const QueryProvider = QueryClientProvider as unknown as (props: { children?: React.ReactNode; client: QueryClient }) =>
  React.ReactNode | Promise<React.ReactNode>;

const inactiveResourceFamily = Object.freeze({ line() { throw new Error("inactive resource line"); } });
const inactiveReactStore = Object.freeze({ subscribeSignal: () => () => {}, getSignalSnapshot: () => null });

interface ResourcesSectionProps { onNavigate: (path: string) => void; }

interface Deferred<T> {
  promise: Promise<T>;
  resolve: (value: T) => void;
  reject: (reason?: unknown) => void;
}

interface PanelController {
  apply: () => Promise<void>; confirm: () => Promise<void>; fail: () => Promise<void>; reset: () => Promise<void>;
}

type SignalsRuntime = Awaited<ReturnType<typeof createSignals>>;
type ScenarioPhase = "idle" | "optimistic" | "confirmed" | "rolledBack";
function createDeferred<T>(): Deferred<T> {
  let resolve!: (value: T) => void;
  let reject!: (reason?: unknown) => void;
  const promise = new Promise<T>((nextResolve, nextReject) => {
    resolve = nextResolve;
    reject = nextReject;
  });
  return { promise, resolve, reject };
}

function pushEvent(setter: (value: PanelEvent[] | ((current: PanelEvent[]) => PanelEvent[])) => void, event: PanelEvent): void {
  setter((current) => [event, ...current].slice(0, 4));
}

function TanStackPanel({
  onController,
  onDiagnosticsChange,
}: {
  onController: (controller: PanelController | null) => void;
  onDiagnosticsChange: (diagnostics: ResourceDemoDiagnostics) => void;
}) {
  const store = useMemo(() => createComparisonStore(), []);
  const queryClient = useQueryClient();
  const pendingDeferredRef = useRef<Deferred<ListItem> | null>(null);
  const [pendingDeferred, setPendingDeferred] = useState<Deferred<ListItem> | null>(null);
  const [rollbackSnapshot, setRollbackSnapshot] = useState<readonly ListItem[] | null>(null);
  const [events, setEvents] = useState<PanelEvent[]>([]);
  const items = useQuery({ queryKey: ["section4", "items"], queryFn: () => store.fetchItems() });
  const mutation = useMutation({
    mutationFn: (deferred: Deferred<ListItem>) => deferred.promise,
    onMutate: async () => {
      const previous = queryClient.getQueryData<readonly ListItem[]>(["section4", "items"]) ?? null;
      setRollbackSnapshot(previous);
      queryClient.setQueryData(["section4", "items"], (current: readonly ListItem[] = []) => [...current, optimisticItem]);
      pushEvent(setEvents, createPanelEvent("info", "Adding item...", "onMutate inserted the optimistic row."));
      return { previous };
    },
    onSuccess: (saved) => {
      queryClient.setQueryData(["section4", "items"], (current: readonly ListItem[] = []) =>
        current.map((item) => (item.id === saved.id ? saved : item)),
      );
      setRollbackSnapshot(null);
      pendingDeferredRef.current = null;
      setPendingDeferred(null);
      pushEvent(setEvents, createPanelEvent("success", "Item added successfully", "onSuccess replaced the optimistic row."));
    },
    onError: (_error, _vars, context) => {
      queryClient.setQueryData(["section4", "items"], context?.previous ?? []);
      setRollbackSnapshot(null);
      pendingDeferredRef.current = null;
      setPendingDeferred(null);
      pushEvent(setEvents, createPanelEvent("error", "Failed to add item", "onError restored the previous cache snapshot."));
    },
  });

  const diagnostics = useMemo<ResourceDemoDiagnostics>(() => ({
    schema: "resource-demo-diagnostics.v1",
    runtime: "tanstack",
    blocks: [
        { label: "queryClient.getQueryData(['section4', 'items'])", mode: "rows", value: formatItemRows(items.data ?? null) },
      { label: "mutation.status", value: mutation.status },
      { label: "rollback source", value: rollbackSnapshot ? `feature snapshot with ${rollbackSnapshot.length} rows` : "none captured" },
      { label: "toastFeed[0]", value: events[0] ? `${events[0].tone}: ${events[0].title}` : "none" },
    ],
  }), [events, items.data, mutation.status, rollbackSnapshot]);

  useEffect(() => onDiagnosticsChange(diagnostics), [diagnostics, onDiagnosticsChange]);

  const reset = useCallback(async () => {
    store.reset();
    pendingDeferredRef.current = null;
    setPendingDeferred(null);
    setRollbackSnapshot(null);
    setEvents([]);
    await queryClient.invalidateQueries({ queryKey: ["section4", "items"] });
  }, [queryClient, store]);

  const controller = useMemo<PanelController>(() => ({
    apply: async () => {
      if (mutation.isPending) return;
      await queryClient.ensureQueryData({ queryKey: ["section4", "items"], queryFn: () => store.fetchItems() });
      const deferred = createDeferred<ListItem>();
      pendingDeferredRef.current = deferred;
      setPendingDeferred(deferred);
      mutation.mutate(deferred);
    },
    confirm: async () => {
      const deferred = pendingDeferredRef.current ?? pendingDeferred;
      if (!deferred) return;
      deferred.resolve(await store.confirmItem(optimisticItem.id));
    },
    fail: async () => {
      const deferred = pendingDeferredRef.current ?? pendingDeferred;
      if (!deferred) return;
      deferred.reject(new Error("Server rejected the item."));
    },
    reset,
  }), [mutation, pendingDeferred, queryClient, reset, store]);

  useEffect(() => {
    onController(controller);
    return () => onController(null);
  }, [controller, onController]);

  return (
    <ResourcePanel
      error={items.error instanceof Error ? items.error.message : null}
      events={events}
      items={items.data ?? null}
      loading={items.isLoading}
      title="App-owned optimistic callbacks"
      variant="TanStack"
    />
  );
}

function ForgePanel({
  onController,
  onDiagnosticsChange,
}: {
  onController: (controller: PanelController | null) => void;
  onDiagnosticsChange: (diagnostics: ResourceDemoDiagnostics) => void;
}) {
  const store = useMemo(() => createComparisonStore(), []);
  const [signals, setSignals] = useState<SignalsRuntime | null>(null);
  const reactStore = useMemo(() => signals ? createReactSignalsStore(signals) : null, [signals]);
  const [bootError, setBootError] = useState<string | null>(null);
  const [events, setEvents] = useState<PanelEvent[]>([]);
  const [lastRollback, setLastRollback] = useState<unknown>(null);

  useEffect(() => {
    createSignals({ deployment: "mainThreadCompatibility" })
      .then(setSignals)
      .catch((error) => setBootError(error instanceof Error ? error.message : "Could not boot the Forge runtime."));
  }, []);

  const forge = useMemo(() => {
    if (!signals) return null;
    const api = signals.api({
      baseUrl: comparisonApiUrls.items.replace(/\/items\.json$/, ""),
      effects: signals.resource.effects.branchNative(),
    });
    const itemsResponse = signals.resource.response.array({ itemId: (item: ListItem) => item.id });
    const itemsFamily = api.url("/items").response(itemsResponse).list({
      load: () => store.fetchItems(),
    });
    const saveItem = api.url("/items/:itemId")
      .response(signals.resource.response.detail<ListItem>()({ label: "label", sync: "sync" }))
      .update({
        reconciles: [{
          family: itemsFamily,
          params: () => ({}),
          collection: { kind: "item" },
          fallback: "partialReconciliation",
        }],
        load: ({ body, itemId }: { body: SaveItemBody; itemId: string | number }) =>
          store.confirmItem(String(itemId), body.outcome),
      });
    return { itemsFamily, itemsLine: itemsFamily.line({}), saveItem };
  }, [signals, store]);

  useEffect(() => {
    forge?.itemsLine.refresh();
  }, [forge]);

  const write = useManagedResourceWrite<SaveItemBody, ReturnType<NonNullable<typeof forge>["saveItem"]["line"]>>({
    line(body) {
      if (!forge) {
        throw new Error("Forge runtime is not ready.");
      }
      return forge.saveItem.line({ body, itemId: body.id });
    },
    feedback: {
      success: "Item added successfully",
      error: "Failed to add item",
    },
    onFeedback(feedback) {
      pushEvent(
        setEvents,
        createPanelEvent(
          feedback.kind === "success" ? "success" : feedback.kind === "error" ? "error" : "info",
          feedback.title,
          feedback.description ?? `managed write result: ${feedback.resultKind}`,
        ),
      );
    },
  });

  const lineView = useResourceLine<readonly ListItem[] | null, null>(
    forge?.itemsFamily ?? inactiveResourceFamily,
    forge ? {} : { enabled: false },
    (reactStore ?? inactiveReactStore) as Parameters<typeof useResourceLine>[2],
    { inactiveValue: null },
  );
  const items = lineView.value;
  const statusKind = forge ? forge.itemsLine.status().kind : "booting";
  const diagnostics = useMemo<ResourceDemoDiagnostics>(() => {
    const latestResult = write.lastResult;
    const lastEffect = forge?.itemsLine.diagnostics().lastEffect ?? null;
    const lifecycle = forge?.itemsLine.history().lifecycle ?? [];
    return {
      schema: "resource-demo-diagnostics.v1",
      runtime: "forge",
      blocks: [
        { label: "useResourceLine(...).value", mode: "rows", value: formatItemRows(items ?? null) },
        { label: "line.diagnostics().lastEffect", value: lastEffect ? {
          provenance: lastEffect.provenance ?? null,
          optimisticKind: lastEffect.optimistic?.kind ?? null,
          rollbackKind: lastEffect.optimistic?.rollback?.kind ?? null,
        } : null },
        { label: "useManagedResourceWrite(...).lastResult", value: latestResult ? {
          resultKind: latestResult.resultKind,
          confirmation: latestResult.confirmationKind,
          recovery: summarizeRecovery(latestResult),
        } : null },
        { label: "line.history().rollbackLastEffect()", value: lastRollback },
        { label: "line.history().lifecycle.slice(-3)", value: lifecycle.slice(-3).map((entry: any) => ({
          event: entry.event,
          lastPatchKind: entry.lastPatchKind,
          lastPatchedItemId: entry.lastPatchedItemId,
          visibleSelection: entry.visibleSelection ? {
            kind: entry.visibleSelection.kind,
            source: entry.visibleSelection.source,
            effectId: entry.visibleSelection.effectId ?? null,
            branchId: entry.visibleSelection.branchId ?? null,
            snapshotId: entry.visibleSelection.snapshotId ?? null,
            rollbackKind: entry.visibleSelection.rollbackKind ?? null,
          } : null,
        })) },
        { label: "toastFeed[0]", value: events[0] ? `${events[0].tone}: ${events[0].title}` : "none" },
      ],
    };
  }, [events, forge, items, lastRollback, lineView.diagnosticsSummary, write.lastResult]);

  useEffect(() => onDiagnosticsChange(diagnostics), [diagnostics, onDiagnosticsChange]);

  const reset = useCallback(async () => {
    if (!forge) return;
    store.reset();
    setEvents([]);
    setLastRollback(null);
    write.reset();
    forge.itemsLine.invalidate();
    forge.itemsLine.refresh();
    await forge.itemsLine.awaitSettlement();
  }, [forge, store, write]);

  const controller = useMemo<PanelController>(() => ({
    apply: async () => {
      if (!forge) return;
      write.reset();
      setLastRollback(null);
      forge.itemsLine.patch(forge.itemsFamily.patch.insert({
        itemId: optimisticItem.id,
        placement: "append",
        nextItem: optimisticItem,
      }));
      pushEvent(setEvents, createPanelEvent("info", "Adding item...", "branchNative admitted an optimistic list insert."));
    },
    confirm: async () => {
      await write.execute({ id: optimisticItem.id, outcome: "confirm" });
    },
    fail: async () => {
      const result = await write.execute({ id: optimisticItem.id, outcome: "reject" });
      if (result.resultKind === "rejected") {
        const rollback = forge?.itemsLine.history().rollbackLastEffect() ?? null;
        setLastRollback(rollback);
      }
    },
    reset,
  }), [forge, reset, write]);

  useEffect(() => {
    onController(controller);
    return () => onController(null);
  }, [controller, onController]);

  return (
    <ResourcePanel
      error={bootError}
      events={events}
      items={items}
      loading={!forge || statusKind === "booting" || statusKind === "pending"}
      title="Runtime-owned optimistic lifecycle"
      variant="Forge"
    />
  );
}

export function ResourcesSection({ onNavigate }: ResourcesSectionProps) {
  const tanstackController = useRef<PanelController | null>(null);
  const forgeController = useRef<PanelController | null>(null);
  const queryClient = useMemo(() => new QueryClient(), []);
  const [phase, setPhase] = useState<ScenarioPhase>("idle");
  const [tanstackDiagnostics, setTanstackDiagnostics] = useState<ResourceDemoDiagnostics | null>(null);
  const [forgeDiagnostics, setForgeDiagnostics] = useState<ResourceDemoDiagnostics | null>(null);

  const runBoth = useCallback(async (action: keyof PanelController, nextPhase: ScenarioPhase) => {
    await tanstackController.current?.[action]();
    await forgeController.current?.[action]();
    setPhase(nextPhase);
  }, []);

  return (
    <div className="xai-section-band accent-resources">
      <div className="xai-section-heading">
        <span className="xai-section-eyebrow">05 / Optimistic Resources</span>
        <h2>Optimistic updates should be inspectable.</h2>
        <p>Add one item to the same list in both implementations, then inspect the actual runtime output below.</p>
      </div>

      <DemoPreface demoId={5} />

      <ResourceFeedbackGuide />

      <div className="resources-control-bar">
        <span>One shared add-item flow. Both windows move together.</span>
        <div className="resources-control-actions">
          <button className="resources-control-button" disabled={phase !== "idle"} onClick={() => void runBoth("apply", "optimistic")} type="button">
            Add item optimistically
          </button>
          <button className="resources-control-button" disabled={phase !== "optimistic"} onClick={() => void runBoth("confirm", "confirmed")} type="button">
            Confirm from server
          </button>
          <button className="resources-control-button" disabled={phase !== "optimistic"} onClick={() => void runBoth("fail", "rolledBack")} type="button">
            Fail from server
          </button>
          <button className="resources-control-button resources-control-button-ghost" onClick={() => void runBoth("reset", "idle")} type="button">
            Reset both
          </button>
          <button className="resources-control-button resources-control-button-ghost" onClick={() => onNavigate("#/demos/5")} type="button">
            Open full demo
          </button>
        </div>
      </div>

      <div className="resources-live-grid">
        <QueryProvider client={queryClient}>
          <TanStackPanel onController={(controller) => { tanstackController.current = controller; }} onDiagnosticsChange={setTanstackDiagnostics} />
        </QueryProvider>
        <ForgePanel onController={(controller) => { forgeController.current = controller; }} onDiagnosticsChange={setForgeDiagnostics} />
      </div>

      <ResourcesSectionCodeSample />

      <DiagnosticsComparison forgeDiagnostics={forgeDiagnostics} tanstackDiagnostics={tanstackDiagnostics} />
    </div>
  );
}

