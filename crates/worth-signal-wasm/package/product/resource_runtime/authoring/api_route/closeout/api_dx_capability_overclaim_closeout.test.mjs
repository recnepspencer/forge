import assert from "node:assert/strict";
import test from "node:test";

import { createRealTransferRuntime } from "../../../runtime_fixture/real_transfer_runtime.mjs";

test("api.url(...).detail(...) keeps runtime line capability truth aligned with the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeDetail = signals.api({}).url("/reports/:reportId").detail({
      load: ({ reportId }) => ({ id: reportId }),
    });
    const rawDetail = signals.resource.detail({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ reportId }) =>
        signalsMod.resourceParamIdentity({ reportId }, `/reports/${reportId}`),
      load: ({ reportId }) => ({ id: reportId }),
    });

    const routeLine = routeDetail.line({ reportId: "r1" });
    const rawLine = rawDetail.line({ reportId: "r1" });

    assert.equal("patch" in routeLine, false);
    assert.equal("deliver" in routeLine, false);
    assert.equal("reconciliation" in routeLine, false);
    assert.deepEqual(
      routeLine.history().verificationPackage().capabilities,
      rawLine.history().verificationPackage().capabilities,
    );
    assert.deepEqual(routeLine.history().verificationPackage().capabilities, {
      summary: true,
      diagnosticsSummary: true,
      requestRead: true,
      processingRead: true,
      uploadRead: true,
      downloadRead: true,
      historyRead: true,
      patch: false,
      deliver: false,
      reconciliationRead: false,
      broadReplace: false,
      narrowItem: false,
      narrowSummary: false,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).items(...).aspect(...).summary(...).list(...) keeps collection capability truth aligned with the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeTasks = signals.api({}).url("/workspaces/:workspaceId/tasks")
      .items((item) => item.id)
      .aspect(
        "title",
        (item) => item.title,
        (item, title) => ({ ...item, title: String(title) }),
      )
      .summary(
        "total",
        (value) => value.length,
        (value, total) => value.slice(0, Number(total)),
      )
      .list({
        load: ({ workspaceId }) => [{ id: `${workspaceId}:1`, title: "First" }],
      });
    const rawTasks = signals.resource.collection({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId },
          `/workspaces/${workspaceId}/tasks`,
        ),
      itemIdentity: (item) => item.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value,
        replaceItems: (_value, nextItems) => [...nextItems],
        aspects: signalsMod.resourceItemAspects({
          title: {
            read: (item) => item.title,
            write: (item, title) => ({ ...item, title: String(title) }),
          },
        }),
        summaries: signalsMod.resourceValueSummaries({
          total: {
            read: (value) => value.length,
            write: (value, total) => value.slice(0, Number(total)),
          },
        }),
      }),
      load: ({ workspaceId }) => [{ id: `${workspaceId}:1`, title: "First" }],
    });

    const routeLine = routeTasks.line({ workspaceId: "demo" });
    const rawLine = rawTasks.line({ workspaceId: "demo" });

    assert.equal("patch" in routeLine, true);
    assert.equal("deliver" in routeLine, true);
    assert.equal("reconciliation" in routeLine, true);
    assert.deepEqual(routeLine.reconciliation(), {
      broadReplace: true,
      narrowItem: true,
      narrowSummary: true,
      aspectNames: ["title"],
      summaryNames: ["total"],
    });
    assert.deepEqual(
      routeLine.history().verificationPackage().capabilities,
      rawLine.history().verificationPackage().capabilities,
    );
    assert.deepEqual(routeLine.history().verificationPackage().capabilities, {
      summary: true,
      diagnosticsSummary: true,
      requestRead: true,
      processingRead: true,
      uploadRead: true,
      downloadRead: true,
      historyRead: true,
      patch: true,
      deliver: true,
      reconciliationRead: true,
      broadReplace: true,
      narrowItem: true,
      narrowSummary: true,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).items(...).paged(...) keeps paged summary capability truth aligned with the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeFeed = signals.api({}).url("/feeds/:feedId")
      .items((item) => item.id)
      .paged({
        accumulatePage: (existing, next) => [...existing, ...next],
        load: ({ feedId }) => [{ id: `${feedId}:1`, title: "First" }],
      });
    const rawFeed = signals.resource.paged({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ feedId }) =>
        signalsMod.resourceParamIdentity({ feedId }, `/feeds/${feedId}`),
      itemIdentity: (item) => item.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value,
        replaceItems: (_value, nextItems) => [...nextItems],
      }),
      accumulatePage: (existing, next) => [...existing, ...next],
      load: ({ feedId }) => [{ id: `${feedId}:1`, title: "First" }],
    });

    const routeLine = routeFeed.line({ feedId: "demo" });
    const rawLine = rawFeed.line({ feedId: "demo" });

    assert.equal("patch" in routeLine, true);
    assert.equal("deliver" in routeLine, true);
    assert.equal("reconciliation" in routeLine, true);
    assert.deepEqual(routeLine.reconciliation(), {
      broadReplace: true,
      narrowItem: true,
      narrowSummary: false,
      aspectNames: [],
      summaryNames: [],
    });
    assert.deepEqual(
      routeLine.history().verificationPackage().capabilities,
      rawLine.history().verificationPackage().capabilities,
    );
    assert.deepEqual(routeLine.history().verificationPackage().capabilities, {
      summary: true,
      diagnosticsSummary: true,
      requestRead: true,
      processingRead: true,
      uploadRead: true,
      downloadRead: true,
      historyRead: true,
      patch: true,
      deliver: true,
      reconciliationRead: true,
      broadReplace: true,
      narrowItem: true,
      narrowSummary: false,
    });
  } finally {
    await runtime.cleanup();
  }
});
