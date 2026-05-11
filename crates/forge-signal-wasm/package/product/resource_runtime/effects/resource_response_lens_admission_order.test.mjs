import assert from "node:assert/strict";
import test from "node:test";

import { createDeferred } from "../runtime_fixture/async/deferred.mjs";
import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

test("response summary scope denial precedes branch effect planning", async () => {
  let currentBranchReadCount = 0;
  const runtime = await createRealRequestRuntime({
    current_branch(history) {
      currentBranchReadCount += 1;
      return history.current_branch();
    },
  });
  try {
    const line = createPageWindowListLine(runtime, {
      api: {
        effects: runtime.signals.resource.effects.branchNative(),
      },
    });
    const beforeValue = line.value();
    const beforeEffect = line.diagnostics().lastEffect;

    assert.throws(
      () =>
        line.patch(
          runtime.signalsMod.resourcePatch.summary({
            summary: "visibleCount",
            value: 2,
          }),
        ),
      /do not admit resourceValueSummaries\.pageWindow/,
    );

    assert.equal(currentBranchReadCount, 0);
    assert.deepEqual(line.value(), beforeValue);
    assert.equal(line.diagnostics().lastEffect, beforeEffect);
  } finally {
    await runtime.cleanup();
  }
});

test("response summary scope denial precedes delivery reload supersession", async () => {
  const runtime = await createRealRequestRuntime();
  const refreshDeferred = createDeferred();
  let loadCount = 0;
  try {
    const line = createPageWindowListLine(runtime, {
      list: {
        load: () => {
          loadCount += 1;
          if (loadCount === 1) {
            return {
              items: [{ id: "window:1", title: "First" }],
              visibleCount: 1,
            };
          }
          return refreshDeferred.promise;
        },
      },
    });
    line.refresh();
    const beforeDiagnostics = line.diagnostics();
    const beforeLifecycleLength = line.history().lifecycle.length;

    assert.throws(
      () =>
        line.deliver(
          runtime.signalsMod.resourceDelivery.patch({
            packetId: "pkt-denied-page-window",
            basisId: null,
            patch: runtime.signalsMod.resourcePatch.summary({
              summary: "visibleCount",
              value: 2,
            }),
          }),
        ),
      /do not admit resourceValueSummaries\.pageWindow/,
    );

    assert.equal(line.diagnostics().pendingOperation, beforeDiagnostics.pendingOperation);
    assert.equal(line.history().lifecycle.length, beforeLifecycleLength);

    refreshDeferred.resolve({
      items: [{ id: "window:1", title: "Settled" }],
      visibleCount: 1,
    });
    await refreshDeferred.promise;
    await Promise.resolve();
  } finally {
    await runtime.cleanup();
  }
});

function createPageWindowListLine(runtime, overrides = {}) {
  const { signals, signalsMod } = runtime;
  const response = signals.resource.response.collection({
    itemId: (task) => task.id,
    items: (value) => value.items,
    replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
    summaries: signalsMod.resourceValueSummaries.pageWindow({
      visibleCount: {
        read: (value) => value.visibleCount,
        write: (value, visibleCount) => ({ ...value, visibleCount }),
      },
    }),
  });
  const list = signals.api({
    effects: signals.resource.effects.pessimistic(),
    ...(overrides.api ?? {}),
  }).url("/page-window-list")
    .response(response)
    .list({
      load: () => ({
        items: [{ id: "window:1", title: "First" }],
        visibleCount: 1,
      }),
      ...(overrides.list ?? {}),
    });
  return list.line({});
}
