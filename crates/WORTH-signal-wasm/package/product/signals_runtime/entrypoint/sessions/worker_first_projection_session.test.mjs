import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("worker-first projection session bootstraps worker truth, primes diagnostics, and caches projected outputs", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { importProductModule, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const { createWorkerFirstProjectionSession } = await importProductModule(
    "entrypoint/worker_first_projection_session.js",
  );
  const session = await createWorkerFirstProjectionSession({
    publication: counterPublicationWithOutput(),
    outputIds: ["doubleCounter"],
  });
  try {
    assert.equal(
      session.bootstrapRecord().shellLock.identity.runtimeAuthority,
      "workerOwnedRuntime",
    );
    assert.equal(
      session.workerRuntimeShellLock().identity.deploymentPosture,
      "workerFirst",
    );
    assert.equal(session.diagnosticsSummary().profile, "Operational");
    assert.equal(session.diagnosticsHistory().history.profile, "Operational");

    const projection = await session.projectCommittedTransaction({
      transactionOps: [{ kind: "set", id: "counter", value: 9 }],
    });

    assert.equal(projection.workerFirstTruthDigest, projection.transaction.committedTruthDigest);
    assert.equal(session.readProjectedOutput("doubleCounter"), 18);
    assert.deepEqual(session.trackedOutputIds(), ["doubleCounter"]);
    assert.equal(
      session.diagnosticsSummary().active_node_count,
      projection.diagnosticsSummary.summary.active_node_count,
    );
  } finally {
    await session.terminate();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first projection session materializes immutable cached truth and clears stale output cache when tracking is emptied", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { importProductModule, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const { createWorkerFirstProjectionSession } = await importProductModule(
    "entrypoint/worker_first_projection_session.js",
  );
  const session = await createWorkerFirstProjectionSession({
    publication: objectOutputPublication(),
    outputIds: ["counterObject"],
  });
  try {
    const original = session.readProjectedOutput("counterObject");
    assert.equal(Object.isFrozen(original), true);
    assert.throws(() => {
      original.count = 999;
    }, /Cannot assign/);
    assert.deepEqual(session.readProjectedOutput("counterObject"), { count: 1 });

    const summary = session.diagnosticsSummary();
    assert.equal(Object.isFrozen(summary), true);
    assert.throws(() => {
      summary.active_node_count = 999;
    }, /Cannot assign/);
    assert.notEqual(session.diagnosticsSummary().active_node_count, 999);

    const history = session.diagnosticsHistory();
    assert.equal(Object.isFrozen(history), true);
    assert.equal(Object.isFrozen(history.history), true);
    assert.throws(() => {
      history.history.profile = "MutatedLocally";
    }, /Cannot assign/);
    assert.notEqual(session.diagnosticsHistory().history.profile, "MutatedLocally");

    await session.refreshProjection({ outputIds: [] });

    assert.deepEqual(session.trackedOutputIds(), []);
    assert.throws(
      () => session.readProjectedOutput("counterObject"),
      /has no cached output/,
    );
  } finally {
    await session.terminate();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first projection session denies uncached output reads and disposed access", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { importProductModule, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const { createWorkerFirstProjectionSession } = await importProductModule(
    "entrypoint/worker_first_projection_session.js",
  );
  const session = await createWorkerFirstProjectionSession({
    publication: counterPublicationWithOutput(),
  });
  try {
    assert.throws(
      () => session.readProjectedOutput("doubleCounter"),
      /has no cached output/,
    );

    await session.terminate();

    assert.throws(
      () => session.diagnosticsSummary(),
      /cannot be used after terminate/,
    );
    await assert.rejects(
      () =>
        session.projectCommittedTransaction({
          transactionOps: [{ kind: "set", id: "counter", value: 3 }],
          outputIds: ["doubleCounter"],
        }),
      /cannot be used after terminate/,
    );
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first projection session rejects malformed tracked output ids before worker projection begins", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { importProductModule, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const { createWorkerFirstProjectionSession } = await importProductModule(
    "entrypoint/worker_first_projection_session.js",
  );
  await assert.rejects(
    () =>
      createWorkerFirstProjectionSession({
        publication: counterPublicationWithOutput(),
        outputIds: ["doubleCounter", "doubleCounter"],
      }),
    /duplicate output id/,
  );
  await cleanup();
  globalThis.Worker = previousWorker;
});

test("worker-first projection session refreshes cached worker truth after host-boundary mutations only when refresh is explicitly requested", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, importProductModule, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const { createWorkerFirstProjectionSession } = await importProductModule(
    "entrypoint/worker_first_projection_session.js",
  );
  const routerSignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const session = await createWorkerFirstProjectionSession({
    publication: routeAndOutputPublication(),
    outputIds: ["routeProjection"],
  });
  try {
    assert.equal(session.readProjectedOutput("routeProjection"), "homeRoute");

    const ingress = await session.admitBrowserHistoryIngress(
      routerSignals.router.browserHistory.push("/search?q=WORTH", {
        routeIdentity: "searchRoute:WORTH",
        runtimeRouteSourceId: "routeIdentity",
        routeValue: "searchRoute:WORTH",
        runtimeContinuitySourceId: "routeContinuity",
        continuityValue: "restored",
      }),
      { refreshProjection: true },
    );

    assert.equal(ingress.envelopeFamily, "browserHistoryIngress");
    assert.equal(session.readProjectedOutput("routeProjection"), "searchRoute:WORTH");
    assert.equal(session.diagnosticsSummary().profile, "Operational");
  } finally {
    await session.terminate();
    routerSignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first projection session preserves stale cached truth by default after host-boundary mutations", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, importProductModule, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const { createWorkerFirstProjectionSession } = await importProductModule(
    "entrypoint/worker_first_projection_session.js",
  );
  const routerSignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const session = await createWorkerFirstProjectionSession({
    publication: routeAndOutputPublication(),
    outputIds: ["routeProjection"],
  });
  try {
    assert.equal(session.readProjectedOutput("routeProjection"), "homeRoute");

    await session.admitBrowserHistoryIngress(
      routerSignals.router.browserHistory.push("/search?q=WORTH", {
        routeIdentity: "searchRoute:WORTH",
        runtimeRouteSourceId: "routeIdentity",
        routeValue: "searchRoute:WORTH",
        runtimeContinuitySourceId: "routeContinuity",
        continuityValue: "restored",
      }),
    );

    assert.equal(session.readProjectedOutput("routeProjection"), "homeRoute");

    await session.refreshProjection();

    assert.equal(session.readProjectedOutput("routeProjection"), "searchRoute:WORTH");
  } finally {
    await session.terminate();
    routerSignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first projection session rejects malformed per-transaction output tracking without drifting cached truth", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { importProductModule, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const { createWorkerFirstProjectionSession } = await importProductModule(
    "entrypoint/worker_first_projection_session.js",
  );
  const session = await createWorkerFirstProjectionSession({
    publication: counterPublicationWithOutput(),
    outputIds: ["doubleCounter"],
  });
  try {
    assert.equal(session.readProjectedOutput("doubleCounter"), 2);

    await assert.rejects(
      () =>
        session.projectCommittedTransaction({
          transactionOps: [{ kind: "set", id: "counter", value: 5 }],
          outputIds: ["doubleCounter", "doubleCounter"],
        }),
      /duplicate output id/,
    );

    assert.deepEqual(session.trackedOutputIds(), ["doubleCounter"]);
    assert.equal(session.readProjectedOutput("doubleCounter"), 2);
  } finally {
    await session.terminate();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

function counterPublicationWithOutput() {
  return {
    policy: { preset: "development" },
    sources: [{ id: "counter", initial: 1 }],
    recipes: [
      {
        id: "doubleCounter",
        reads: ["counter"],
        expr: {
          kind: "sum",
          args: [
            { kind: "read", id: "counter" },
            { kind: "read", id: "counter" },
          ],
        },
        identity: { kind: "exact" },
      },
    ],
    outputIds: ["doubleCounter"],
  };
}

function routeAndOutputPublication() {
  return {
    policy: { preset: "development" },
    sources: [
      { id: "routeIdentity", initial: "homeRoute" },
      { id: "routeContinuity", initial: "fresh" },
    ],
    recipes: [
      {
        id: "routeProjection",
        reads: ["routeIdentity"],
        expr: { kind: "read", id: "routeIdentity" },
        identity: { kind: "exact" },
      },
    ],
    outputIds: ["routeProjection"],
  };
}

function objectOutputPublication() {
  return {
    policy: { preset: "development" },
    sources: [{ id: "counter", initial: 1 }],
    recipes: [
      {
        id: "counterObject",
        reads: ["counter"],
        expr: {
          kind: "object",
          fields: [["count", { kind: "read", id: "counter" }]],
        },
        identity: { kind: "exact" },
      },
    ],
    outputIds: ["counterObject"],
  };
}
