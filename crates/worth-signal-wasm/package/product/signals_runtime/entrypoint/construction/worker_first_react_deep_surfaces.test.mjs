import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadStoreModule } from "../../../host_capabilities_certification/module_loading/load_store_module.mjs";
import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

async function flushMicrotasks() {
  await Promise.resolve();
  await Promise.resolve();
  await new Promise((resolve) => setTimeout(resolve, 0));
}

async function waitUntil(predicate, label, timeoutMs = 2_000) {
  const started = Date.now();
  while (Date.now() - started < timeoutMs) {
    if (await predicate()) {
      return;
    }
    await flushMicrotasks();
  }
  throw new Error(`timed out waiting for ${label}`);
}

async function withWorkerFirstReactWorld(run) {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup: cleanupSignals, resourceParams, resourceParamIdentity } =
    await loadSignalsModule({ rawSurface: "real" });
  const { createReactSignalsStore, cleanup: cleanupStore } = await loadStoreModule();
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    const store = createReactSignalsStore(signals);
    await run({
      signals,
      store,
      createSignals,
      resourceParams,
      resourceParamIdentity,
    });
    store.dispose();
  } finally {
    if (signals) {
      await signals.terminate();
    }
    await cleanupStore();
    await cleanupSignals();
    globalThis.Worker = previousWorker;
  }
}

function defineSearchRoutes(signals) {
  return signals.router.define({
    home: signals.router.route("/"),
    search: signals.router.route("/search", {
      search: {
        q: signals.router.search.required.string(),
      },
    }),
    settings: signals.router.route("/settings"),
  });
}

test("DEEP1: worker-first empty React attach + object form bind/submit/fulfill", async () => {
  await withWorkerFirstReactWorld(async ({ signals, store }) => {
    assert.equal(store.getDiagnosticsSnapshot().latestObservation, null);

    const form = signals.form({
      source: { title: "Draft", status: "editing" },
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
      actions: ({ submit }) => ({ submit: submit() }),
    });

    await form.fields.title.set("Ship docs");
    await signals.settleAuthoredWork();
    assert.equal(form.effective().title, "Ship docs");

    await waitUntil(
      () => form.summarySignal()().effective?.title === "Ship docs",
      "form summarySignal catch-up after field set",
    );
    const summary = form.summarySignal();
    assert.equal(store.getSignalSnapshot(summary).effective.title, "Ship docs");
    assert.equal(store.getSignalSnapshot(summary).readiness.canSubmit, true);

    const pending = form.executeAction("submit");
    assert.equal(pending.resultKind, "pending");
    form.fulfillAction(pending.operationId, {
      canonicalValue: { title: "Ship docs", status: "published" },
    });
    assert.equal(form.source().status, "published");
    assert.equal(form.effective().status, "published");

    store.subscribeDiagnostics(() => {});
    assert.equal(
      typeof store.getDiagnosticsSnapshot().performanceSummary?.deliveredObservationCount,
      "number",
    );
  });
});

test("DEEP2: form over imported graph fails closed after importGraph supersession", async () => {
  await withWorkerFirstReactWorld(async ({ signals, store, createSignals }) => {
    const compatibility = await createSignals({ deployment: "mainThreadCompatibility" });
    try {
      const document = compatibility.input(
        { title: "Alpha", status: "editing" },
        { debugName: "deep2.document" },
      );
      const graph = compatibility.graph("deep2FormGraph", {
        inputs: { document: compatibility.publicInput(document) },
        outputs: { document },
      });
      const definition = graph.exportDefinition();
      const firstSnapshot = graph.exportSnapshot();
      document.set({ title: "Gamma", status: "published" });
      const secondSnapshot = graph.exportSnapshot();

      const imported = signals.importGraph(definition, firstSnapshot);
      await imported.ready();
      store.subscribeDiagnostics(() => {});
      assert.notEqual(store.getDiagnosticsSnapshot().latestObservation, undefined);

      const form = signals.form({
        source: signals.form.source.signal(imported.input("document"), {
          id: "deep2-form-source",
        }),
        fields: ({ field }) => ({
          title: field("title"),
          status: field("status"),
        }),
        actions: ({ submit }) => ({ submit: submit() }),
      });
      assert.equal(form.source().title, "Alpha");
      await form.fields.title.set("Beta");
      await signals.settleAuthoredWork();
      assert.equal(form.effective().title, "Beta");

      const replacement = signals.importGraph(definition, secondSnapshot);
      await replacement.ready();

      // Drain any stale publish chains before probing fail-closed mutations.
      await Promise.resolve();
      await assert.rejects(
        async () => {
          await form.fields.title.set("Delta");
        },
        /superseded|invalidated|not currently available|active imported graph|replaced the worker-owned runtime/u,
      );
      assert.throws(
        () => form.executeAction("submit"),
        /superseded|invalidated|not currently available|active imported graph|replaced the worker-owned runtime/u,
      );
    } finally {
      compatibility.free();
    }
  });
});

test("DEEP3: resource line.signal() matches value through React store at settlement", async () => {
  await withWorkerFirstReactWorld(async ({
    signals,
    store,
    resourceParams,
    resourceParamIdentity,
  }) => {
    const detail = signals.resource.detail({
      params: resourceParams(),
      normalizeParams: ({ taskId }) => resourceParamIdentity({ taskId }, taskId),
      load: ({ taskId }) => ({ id: taskId, title: `Task:${taskId}` }),
    });
    const line = detail.line({ taskId: "task-deep3" });
    const settled = await line.awaitSettlement({ timeoutMs: 5_000 });
    assert.equal(settled.resultKind, "fulfilled");
    assert.deepEqual(line.value(), { id: "task-deep3", title: "Task:task-deep3" });

    const valueSignal = line.signal();
    assert.deepEqual(valueSignal(), line.value());
    assert.deepEqual(store.getSignalSnapshot(valueSignal), line.value());

    const summary = line.summarySignal();
    assert.equal(store.getSignalSnapshot(summary).current.status.kind, "fulfilled");
    assert.equal(store.getSignalSnapshot(summary).current.hasVisibleValue, true);

    const unsubscribe = store.subscribeSignal(valueSignal, () => {});
    assert.deepEqual(store.getSignalSnapshot(valueSignal), line.value());
    store.subscribeDiagnostics(() => {});
    assert.equal(
      typeof store.getDiagnosticsSnapshot().performanceSummary?.deliveredObservationCount,
      "number",
    );
    unsubscribe();
    line.free();
  });
});

test("DEEP4: router admit + story on worker-first with React store attached", async () => {
  await withWorkerFirstReactWorld(async ({ signals, store }) => {
    const routes = defineSearchRoutes(signals);
    const story = signals.router.browserHistory.story();
    store.subscribeDiagnostics(() => {});

    const homeReport = await routes.admitBrowserHistoryIngress(
      signals.router.browserHistory.load("/", {
        routeIdentity: "homeRoute",
        runtimeRouteSourceId: "routeIdentity",
        routeValue: "homeRoute",
      }),
    );
    assert.equal(homeReport.outcome().kind, "admitted");
    assert.equal(homeReport.outcome().routeId, "home");
    story.record(homeReport);
    assert.equal(story.current()?.routeId, "home");

    const searchReport = await routes.admitBrowserHistoryIngress(
      signals.router.browserHistory.push("/search?q=Worth", {
        routeIdentity: "searchRoute:WORTH",
        runtimeRouteSourceId: "routeIdentity",
        routeValue: "searchRoute:WORTH",
        runtimeContinuitySourceId: "routeContinuity",
        continuityValue: "restored",
      }),
    );
    assert.equal(searchReport.outcome().kind, "admitted");
    assert.equal(searchReport.outcome().routeId, "search");
    assert.equal(searchReport.outcome().href, "/search?q=Worth");
    story.record(searchReport);
    assert.equal(story.current()?.routeId, "search");
    assert.equal(story.back()?.routeId, "home");

    assert.equal(
      typeof store.getDiagnosticsSnapshot().performanceSummary?.deliveredObservationCount,
      "number",
    );
  });
});

test("DEEP5: one React store hosts form + resource + router without crosstalk", async () => {
  await withWorkerFirstReactWorld(async ({
    signals,
    store,
    resourceParams,
    resourceParamIdentity,
  }) => {
    const quantity = signals.input(2, { debugName: "deep5.quantity" });
    const total = signals.computed(() => quantity() * 10, {
      debugName: "deep5.total",
    });
    assert.equal(store.getSignalSnapshot(total), 20);

    const form = signals.form({
      source: { title: "Shared", status: "editing" },
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
      actions: ({ submit }) => ({ submit: submit() }),
    });
    await form.fields.title.set("Shared-edited");
    await signals.settleAuthoredWork();

    const detail = signals.resource.detail({
      params: resourceParams(),
      normalizeParams: ({ taskId }) => resourceParamIdentity({ taskId }, taskId),
      load: ({ taskId }) => ({ id: taskId, title: `Line:${taskId}` }),
    });
    const line = detail.line({ taskId: "deep5" });
    await line.awaitSettlement({ timeoutMs: 5_000 });
    const lineSignal = line.signal();
    assert.deepEqual(store.getSignalSnapshot(lineSignal), {
      id: "deep5",
      title: "Line:deep5",
    });

    const routes = defineSearchRoutes(signals);
    const searchReport = await routes.admitBrowserHistoryIngress(
      signals.router.browserHistory.push("/search?q=multi", {
        routeIdentity: "searchRoute:multi",
        runtimeRouteSourceId: "routeIdentity",
        routeValue: "searchRoute:multi",
      }),
    );
    assert.equal(searchReport.outcome().routeId, "search");

    const totalPulses = [];
    const unsubscribeTotal = store.subscribeSignal(total, () => {
      totalPulses.push(store.getSignalSnapshot(total));
    });
    await signals.transaction((tx) => {
      tx.set(quantity, 7);
    });
    await waitUntil(
      () => store.getSignalSnapshot(total) === 70,
      "authored total catch-up after transaction",
    );
    assert.equal(totalPulses.includes(70), true);
    unsubscribeTotal();
    assert.equal(form.effective().title, "Shared-edited");
    assert.deepEqual(store.getSignalSnapshot(lineSignal), {
      id: "deep5",
      title: "Line:deep5",
    });
    assert.equal(searchReport.outcome().href, "/search?q=multi");

    const pending = form.executeAction("submit");
    form.fulfillAction(pending.operationId, {
      canonicalValue: { title: "Shared-edited", status: "published" },
    });
    assert.equal(form.source().status, "published");
    assert.equal(store.getSignalSnapshot(total), 70);
    assert.deepEqual(line.value(), { id: "deep5", title: "Line:deep5" });

    line.free();
  });
});
