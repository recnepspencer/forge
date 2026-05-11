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

    const error = catchThrown(() =>
      line.patch(
        runtime.signalsMod.resourcePatch.summary({
          summary: "visibleCount",
          value: 2,
        }),
      ),
    );

    assertResponseLensDenial(error, {
      message: /do not admit resourceValueSummaries\.pageWindow/,
      reason: "listSummaryScopeMismatch",
      summary: "visibleCount",
    });
    assert.equal(currentBranchReadCount, 0);
    assert.deepEqual(line.value(), beforeValue);
    assert.equal(line.diagnostics().lastEffect, beforeEffect);
  } finally {
    await runtime.cleanup();
  }
});

test("response lens unsupported locus denial carries compile-boundary proof", async () => {
  let currentBranchReadCount = 0;
  const runtime = await createRealRequestRuntime({
    current_branch(history) {
      currentBranchReadCount += 1;
      return history.current_branch();
    },
  });
  try {
    const { signals, signalsMod } = runtime;
    const response = signals.resource.response.array({
      itemId: (task) => task.id,
    });
    const tasks = signals.api({
      effects: signals.resource.effects.branchNative(),
    }).url("/unsupported-summary-locus")
      .response(response)
      .list({
        load: () => [{ id: "t1", title: "First" }],
      });
    const line = tasks.line({});
    const beforeValue = line.value();
    const beforeEffect = line.diagnostics().lastEffect;

    const error = catchThrown(() =>
      line.patch(
        signalsMod.resourcePatch.summary({
          summary: "total",
          value: 2,
        }),
      ),
    );

    assertResponseLensDenial(error, {
      message: /cannot lower effect locus "summary"/,
      reason: "unsupportedCapability",
      summary: "total",
    });
    assert.equal(error.denialProof.compiledLensDigest, response.lensProof.compiledLensDigest);
    assert.equal(error.denialProof.parityDigest, response.lensProof.parityDigest);
    assert.equal(
      error.denialProof.compileBoundaryDigest,
      response.lensProof.compileBoundaryDigest,
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

    const error = catchThrown(() =>
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
    );

    assertResponseLensDenial(error, {
      message: /do not admit resourceValueSummaries\.pageWindow/,
      reason: "listSummaryScopeMismatch",
      summary: "visibleCount",
    });
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

function catchThrown(action) {
  try {
    action();
  } catch (error) {
    return error;
  }
  assert.fail("expected action to throw");
}

function assertResponseLensDenial(error, expected) {
  assert.match(error.message, expected.message);
  assert.equal(error.name, "ResourceResponseLensDenialError");
  assert.equal(error.denialProof.version, "resource-response-lens-denial-proof-v1");
  assert.equal(error.denialProof.reason, expected.reason);
  assert.equal(error.denialProof.requestedLocus, "summary");
  assert.equal(error.denialProof.requestedPatchScope, "summary");
  assert.equal(error.denialProof.summary, expected.summary);
  assert.equal(error.denialProof.denialDigest.includes("response-lens-denial"), true);
}

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
