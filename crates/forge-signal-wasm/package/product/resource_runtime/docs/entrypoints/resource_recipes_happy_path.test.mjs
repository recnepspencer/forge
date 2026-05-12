import assert from "node:assert/strict";
import test from "node:test";

import { createRealTransferRuntime } from "../../runtime_fixture/real_transfer_runtime.mjs";

test("resource recipes doc happy path covers route-first detail, branch-native effects, upload, and downloads as copyable entrypoints", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const api = runtime.signals.api({});
    const branchNativeApi = runtime.signals.api({
      effects: runtime.signals.resource.effects.branchNative(),
    });
    const userDetail = api.url("/users/:userId").detail({
      load: ({ userId }) => ({ id: userId, name: `User ${userId}` }),
    });
    const branchNativeTasks = branchNativeApi.url("/branch-native-tasks")
      .items((task) => task.id)
      .aspect("title", (task) => task.title, (task, title) => ({
        ...task,
        title,
      }))
      .list({
        load: () => [{ id: "task:1", title: "First" }],
      });
    const receiptUpload = api.url("/receipts/upload")
      .signedUpload({ method: "POST", finalizeRequired: true })
      .processing("poll")
      .create({
        load: ({ body }) => ({ receiptId: body.receiptId }),
      });
    const reportDetail = api.url("/reports/:reportId")
      .downloads(({ reportId }, _value, download) => [
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

    assert.equal(userDetail.line({ userId: "u1" }).status().kind, "fulfilled");
    const branchNativeLine = branchNativeTasks.line({});
    branchNativeLine.patch(branchNativeTasks.patch.itemAspect({
      itemId: "task:1",
      aspect: "title",
      value: "Draft",
    }));
    assert.equal(
      branchNativeLine.diagnostics().lastEffect.profile.name,
      "branchNative",
    );
    assert.equal(receiptUpload.line({ body: { receiptId: "r1" } }).upload().transportKind, "signed");
    assert.equal(reportDetail.line({ reportId: "r1" }).download().readyCount, 1);
  } finally {
    await runtime.cleanup();
  }
});
