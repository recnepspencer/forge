import assert from "node:assert/strict";
import test from "node:test";

import { createRealTransferRuntime } from "../../../runtime_fixture/real_transfer_runtime.mjs";
import { projectLineDiagnosticsHistoryDigest } from "../../../runtime_fixture/proof/line_diagnostics_history_artifacts.mjs";
import { projectAuthoringConvergenceDigest } from "../../../closeout/resource_verification_package_helpers.mjs";

test("api.url(...).detail(...) keeps grouped summary, diagnostics, and history parity with the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeDetail = signals.api({}).url("/reports/:reportId")
      .headers(({ reportId }) => ({
        "x-report-id": String(reportId),
      }))
      .detail({
        load: ({ reportId }) => ({ id: reportId, title: `Report ${reportId}` }),
      });
    const rawDetail = signals.resource.detail({
      params: signalsMod.resourceParams(),
      requestContext: ({ reportId }) =>
        signalsMod.resourceRequestContext({
          headers: { "x-report-id": String(reportId) },
        }),
      normalizeParams: ({ reportId }) =>
        signalsMod.resourceParamIdentity({ reportId }, `/reports/${reportId}`),
      load: ({ reportId }) => ({ id: reportId, title: `Report ${reportId}` }),
    });

    const routeLine = routeDetail.line({ reportId: "r1" });
    const rawLine = rawDetail.line({ reportId: "r1" });

    assert.deepEqual(
      projectLineDiagnosticsHistoryDigest(routeLine),
      projectLineDiagnosticsHistoryDigest(rawLine),
    );
    assert.deepEqual(
      projectAuthoringConvergenceDigest(routeLine.history().verificationPackage()),
      projectAuthoringConvergenceDigest(rawLine.history().verificationPackage()),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).items(...).list(...) keeps grouped summary, diagnostics, and history parity with the raw lane through patch, invalidation, and rejected refresh", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    let shouldFail = false;
    const loadTasks = ({ workspaceId }) => {
      if (shouldFail) {
        throw new Error(`refresh failed for ${workspaceId}`);
      }
      return [{ id: `${workspaceId}:1`, title: "First" }];
    };
    const routeTasks = signals.api({}).url("/workspaces/:workspaceId/tasks")
      .items((item) => item.id)
      .aspect(
        "title",
        (item) => item.title,
        (item, title) => ({ ...item, title: String(title) }),
      )
      .list({
        load: loadTasks,
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
      }),
      load: loadTasks,
    });

    const routeLine = routeTasks.line({ workspaceId: "demo" });
    const rawLine = rawTasks.line({ workspaceId: "demo" });
    const titlePatch = signalsMod.resourcePatch.itemAspect({
      itemId: "demo:1",
      aspect: "title",
      value: "Patched",
    });

    routeLine.patch(titlePatch);
    rawLine.patch(titlePatch);
    routeLine.invalidate();
    rawLine.invalidate();
    shouldFail = true;
    routeLine.refresh();
    rawLine.refresh();

    assert.deepEqual(
      projectLineDiagnosticsHistoryDigest(routeLine),
      projectLineDiagnosticsHistoryDigest(rawLine),
    );
    assert.deepEqual(
      projectAuthoringConvergenceDigest(routeLine.history().verificationPackage()),
      projectAuthoringConvergenceDigest(rawLine.history().verificationPackage()),
    );
  } finally {
    await runtime.cleanup();
  }
});
