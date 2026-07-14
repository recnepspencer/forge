import assert from "node:assert/strict";
import test from "node:test";

import { createRealTransferRuntime } from "../../runtime_fixture/real_transfer_runtime.mjs";
import { normalizeRouteLineArtifact } from "./route_line_artifact_proof.mjs";

test("api.url(...).verb(...).body().detail(...) lowers custom method and body truth into the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeReceipt = signals.api({}).url("/receipts/:receiptId/finalize")
      .verb("POST")
      .body()
      .detail({
        load: ({ receiptId, body }) => ({
          id: receiptId,
          submittedAmount: body.amount,
        }),
      });
    const rawReceipt = signals.resource.detail({
      params: signalsMod.resourceParams(),
      method: "POST",
      requestBody: (params) => params.body,
      normalizeParams: ({ receiptId, body }) =>
        signalsMod.resourceParamIdentity(
          { receiptId, body },
          `/receipts/${receiptId}/finalize#body=${JSON.stringify(body)}`,
        ),
      load: ({ receiptId, body }) => ({
        id: receiptId,
        submittedAmount: body.amount,
      }),
    });

    assert.deepEqual(
      normalizeRouteLineArtifact(routeReceipt.line({
        receiptId: "r1",
        body: { amount: 42 },
      })),
      normalizeRouteLineArtifact(rawReceipt.line({
        receiptId: "r1",
        body: { amount: 42 },
      })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).headers(...).detail(...) lowers owned endpoint headers into the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeReport = signals.api({}).url("/reports/:reportId")
      .headers(({ reportId }) => ({
        "x-report-id": String(reportId),
      }))
      .detail({
        load: ({ reportId }) => ({ id: reportId }),
      });
    const rawReport = signals.resource.detail({
      params: signalsMod.resourceParams(),
      requestContext: ({ reportId }) =>
        signalsMod.resourceRequestContext({
          headers: {
            "x-report-id": String(reportId),
          },
        }),
      normalizeParams: ({ reportId }) =>
        signalsMod.resourceParamIdentity({ reportId }, `/reports/${reportId}`),
      load: ({ reportId }) => ({ id: reportId }),
    });

    assert.deepEqual(
      normalizeRouteLineArtifact(routeReport.line({ reportId: "r2" })),
      normalizeRouteLineArtifact(rawReport.line({ reportId: "r2" })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).verb(\"DELETE\").detail(...) lowers custom delete detail truth into the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeReceipt = signals.api({}).url("/receipts/:receiptId")
      .verb("DELETE")
      .detail({
        load: ({ receiptId }) => ({ removedId: receiptId }),
      });
    const rawReceipt = signals.resource.detail({
      params: signalsMod.resourceParams(),
      method: "DELETE",
      normalizeParams: ({ receiptId }) =>
        signalsMod.resourceParamIdentity({ receiptId }, `/receipts/${receiptId}`),
      load: ({ receiptId }) => ({ removedId: receiptId }),
    });

    assert.deepEqual(
      normalizeRouteLineArtifact(routeReceipt.line({ receiptId: "r3" })),
      normalizeRouteLineArtifact(rawReceipt.line({ receiptId: "r3" })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).items(...).verb(...).body().paged(...) lowers collection request-shape truth into the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeTasks = signals.api({}).url("/workspaces/:workspaceId/tasks/search")
      .items((item) => item.id)
      .verb("POST")
      .body()
      .paged({
        accumulatePage: (existing, next) => [...existing, ...next],
        load: ({ workspaceId, body }) => [{ id: `${workspaceId}:${body.query}` }],
      });
    const rawTasks = signals.resource.paged({
      params: signalsMod.resourceParams(),
      method: "POST",
      requestBody: (params) => params.body,
      normalizeParams: ({ workspaceId, body }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId, body },
          `/workspaces/${workspaceId}/tasks/search#body=${JSON.stringify(body)}`,
        ),
      itemIdentity: (item) => item.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value,
        replaceItems: (_value, nextItems) => [...nextItems],
      }),
      accumulatePage: (existing, next) => [...existing, ...next],
      load: ({ workspaceId, body }) => [{ id: `${workspaceId}:${body.query}` }],
    });

    assert.deepEqual(
      normalizeRouteLineArtifact(routeTasks.line({
        workspaceId: "demo",
        body: { query: "open" },
      })),
      normalizeRouteLineArtifact(rawTasks.line({
        workspaceId: "demo",
        body: { query: "open" },
      })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).items(...).headers(...).list(...) lowers direct-array request headers into the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeTasks = signals.api({}).url("/workspaces/:workspaceId/tasks")
      .items((item) => item.id)
      .headers(({ workspaceId }) => ({ "x-workspace-id": String(workspaceId) }))
      .list({
        load: ({ workspaceId }) => [{ id: `${workspaceId}:t1` }],
      });
    const rawTasks = signals.resource.collection({
      params: signalsMod.resourceParams(),
      requestContext: ({ workspaceId }) =>
        signalsMod.resourceRequestContext({
          headers: { "x-workspace-id": String(workspaceId) },
        }),
      normalizeParams: ({ workspaceId }) =>
        signalsMod.resourceParamIdentity({ workspaceId }, `/workspaces/${workspaceId}/tasks`),
      itemIdentity: (item) => item.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value,
        replaceItems: (_value, nextItems) => [...nextItems],
      }),
      load: ({ workspaceId }) => [{ id: `${workspaceId}:t1` }],
    });

    assert.deepEqual(
      normalizeRouteLineArtifact(routeTasks.line({ workspaceId: "demo" })),
      normalizeRouteLineArtifact(rawTasks.line({ workspaceId: "demo" })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).items(...).reconcile(...).headers(...).list(...) lowers envelope-shaped request headers into the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeCatalog = signals.api({}).url("/workspaces/:workspaceId/catalog")
      .items((item) => item.id)
      .reconcile(
        (value) => value.items,
        (value, nextItems) => ({ ...value, items: [...nextItems] }),
      )
      .headers(({ workspaceId }) => ({ "x-workspace-id": String(workspaceId) }))
      .list({
        load: ({ workspaceId }) => ({ items: [{ id: `${workspaceId}:c1` }] }),
      });
    const rawCatalog = signals.resource.collection({
      params: signalsMod.resourceParams(),
      requestContext: ({ workspaceId }) =>
        signalsMod.resourceRequestContext({
          headers: { "x-workspace-id": String(workspaceId) },
        }),
      normalizeParams: ({ workspaceId }) =>
        signalsMod.resourceParamIdentity({ workspaceId }, `/workspaces/${workspaceId}/catalog`),
      itemIdentity: (item) => item.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      }),
      load: ({ workspaceId }) => ({ items: [{ id: `${workspaceId}:c1` }] }),
    });

    assert.deepEqual(
      normalizeRouteLineArtifact(routeCatalog.line({ workspaceId: "demo" })),
      normalizeRouteLineArtifact(rawCatalog.line({ workspaceId: "demo" })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).items(...).reconcile(...).verb(...).body().paged(...) lowers envelope-shaped request body truth into the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeCatalog = signals.api({}).url("/workspaces/:workspaceId/catalog/search")
      .items((item) => item.id)
      .reconcile(
        (value) => value.items,
        (value, nextItems) => ({ ...value, items: [...nextItems] }),
      )
      .verb("POST")
      .body()
      .paged({
        accumulatePage: (existing, next) => ({
          ...next,
          items: [...existing.items, ...next.items],
        }),
        load: ({ workspaceId, body }) => ({
          items: [{ id: `${workspaceId}:${body.query}` }],
        }),
      });
    const rawCatalog = signals.resource.paged({
      params: signalsMod.resourceParams(),
      method: "POST",
      requestBody: (params) => params.body,
      normalizeParams: ({ workspaceId, body }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId, body },
          `/workspaces/${workspaceId}/catalog/search#body=${JSON.stringify(body)}`,
        ),
      itemIdentity: (item) => item.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      }),
      accumulatePage: (existing, next) => ({
        ...next,
        items: [...existing.items, ...next.items],
      }),
      load: ({ workspaceId, body }) => ({
        items: [{ id: `${workspaceId}:${body.query}` }],
      }),
    });

    assert.deepEqual(
      normalizeRouteLineArtifact(routeCatalog.line({
        workspaceId: "demo",
        body: { query: "open" },
      })),
      normalizeRouteLineArtifact(rawCatalog.line({
        workspaceId: "demo",
        body: { query: "open" },
      })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("advanced request-shaping builders keep final declaration ownership honest", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    assert.throws(
      () =>
        runtime.signals.api({}).url("/reports/:reportId")
          .verb("POST")
          .body()
          .detail({
            requestBody: (params) => params.body,
            load: ({ reportId, body }) => ({ reportId, body }),
          }),
      /owns requestBody/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/reports/:reportId")
          .verb("DELETE")
          .detail({
            method: "DELETE",
            load: ({ reportId }) => ({ reportId }),
          }),
      /owns request method selection/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/reports/:reportId")
          .headers({ "x-report-id": "r4" })
          .detail({
            headers: { "x-extra": "1" },
            load: ({ reportId }) => ({ reportId }),
          }),
      /owns headers/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks/search")
          .items((item) => item.id)
          .body()
          .list({
            requestBody: (params) => params.body,
            load: ({ body }) => [{ id: body.query }],
          }),
      /owns requestBody/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks")
          .items((item) => item.id)
          .headers({ "x-tasks": "1" })
          .list({
            headers: { "x-extra": "1" },
            load: () => [{ id: "t1" }],
          }),
      /owns headers/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/catalog")
          .items((item) => item.id)
          .reconcile(
            (value) => value.items,
            (value, nextItems) => ({ ...value, items: [...nextItems] }),
          )
          .headers({ "x-catalog": "1" })
          .list({
            headers: { "x-extra": "2" },
            load: () => ({ items: [{ id: "c1" }] }),
          }),
      /owns headers/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/catalog/search")
          .items((item) => item.id)
          .reconcile(
            (value) => value.items,
            (value, nextItems) => ({ ...value, items: [...nextItems] }),
          )
          .body()
          .paged({
            requestBody: (params) => params.body,
            accumulatePage: (existing, next) => next,
            load: ({ body }) => ({ items: [{ id: body.query }] }),
          }),
      /owns requestBody/,
    );
  } finally {
    await runtime.cleanup();
  }
});
