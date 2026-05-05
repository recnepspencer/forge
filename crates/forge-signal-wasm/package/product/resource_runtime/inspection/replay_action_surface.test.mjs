import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";
import { projectBasisProof } from "../delivery/delivery_basis_history_proof_helpers.mjs";

function createReplayDetailLine(mod, replayState, historyOverrides = {}) {
  return mod.createResourceNamespace(
    createFakeSignalNamespace("root", historyOverrides),
    {},
  ).detail({
    params: mod.resourceParams(),
    normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
    load: ({ id }) => ({
      id,
      state: replayState.active ? "replayed" : "live",
    }),
  }).line({ id: "replayable" });
}

test("line history replayExact reconstructs line state and records replay-specific lifecycle evidence", async () => {
  const mod = await loadResourceModule();
  try {
    const replayState = { active: false, signalIds: [] };
    const line = createReplayDetailLine(mod, replayState, {
      replay_signal_by_id(signalId) {
        replayState.active = true;
        replayState.signalIds.push(signalId);
      },
    });

    const replayAvailability = line.history().availability.replayExact;
    const result = line.history().replayExact();

    assert.deepEqual(replayAvailability, {
      kind: "available",
      mode: "SameRuntimeSignalExact",
      signalId: line.signal().id,
    });
    assert.deepEqual(result, {
      kind: "replayed",
      mode: "SameRuntimeSignalExact",
      signalId: line.signal().id,
      basisCurrentId: null,
      basisAdvanceCount: 0,
      reloadStatus: {
        kind: "fulfilled",
        operation: "replay",
      },
    });
    assert.deepEqual(replayState.signalIds, [line.signal().id]);
    assert.deepEqual(line.value(), {
      id: "replayable",
      state: "replayed",
    });
    assert.deepEqual(line.status(), {
      kind: "fulfilled",
      operation: "replay",
    });
    assert.equal(line.history().lifecycle.at(-1)?.event, "replayed");
  } finally {
    await mod.cleanup();
  }
});

test("line history replayExact returns explicit unavailable or runtimeRejected artifacts without rewriting basis proof", async () => {
  const mod = await loadResourceModule();
  try {
    const unsupportedState = { active: false };
    const unsupported = createReplayDetailLine(mod, unsupportedState);

    assert.deepEqual(unsupported.history().availability.replayExact, {
      kind: "unavailable",
      reason: "unsupportedByRuntime",
      detail:
        "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
    });
    assert.deepEqual(unsupported.history().replayExact(), {
      kind: "unavailable",
      reason: "unsupportedByRuntime",
      detail:
        "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
      basisCurrentId: null,
      basisAdvanceCount: 0,
    });

    const retainedState = { active: false };
    const retained = createReplayDetailLine(mod, retainedState, {
      replay_signal_by_id() {
        retainedState.active = true;
      },
      replay_execution_availability_for(signalId) {
        return {
          kind: "unavailable",
          reason: "runtimeRejected",
          detail:
            `retained replay execution for ${signalId} is unavailable because replay frames were truncated`,
        };
      },
    });

    assert.deepEqual(retained.history().availability.replayExact, {
      kind: "unavailable",
      reason: "runtimeRejected",
      detail:
        `retained replay execution for ${retained.signal().id} is unavailable because replay frames were truncated`,
    });
    assert.deepEqual(retained.history().replayExact(), {
      kind: "unavailable",
      reason: "runtimeRejected",
      detail:
        `retained replay execution for ${retained.signal().id} is unavailable because replay frames were truncated`,
      basisCurrentId: null,
      basisAdvanceCount: 0,
    });

    const rejected = mod.createResourceNamespace(
      createFakeSignalNamespace("root", {
        replay_signal_by_id() {
          throw new Error("replay frames for signal were evicted");
        },
      }),
      {},
    ).collection({
      params: mod.resourceParams(),
      requestContext: mod.resourceRequestContext({ basisId: "basis-1" }),
      normalizeParams: ({ workspaceId }) =>
        mod.resourceParamIdentity({ workspaceId }, workspaceId),
      itemIdentity: (item) => item.id,
      reconcile: mod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      }),
      load: (_params, request) => ({
        items: [{ id: "demo:1", title: `Load:${request.context.basisId}` }],
      }),
    }).line({ workspaceId: "demo" });
    rejected.deliver(
      mod.resourceDelivery.replace({
        packetId: "pkt-basis-2",
        basisId: "basis-1",
        nextBasisId: "basis-2",
        nextValue: {
          items: [{ id: "demo:1", title: "Delivered Basis 2" }],
        },
      }),
    );
    const beforeRejected = projectBasisProof(rejected);
    const rejectedResult = rejected.history().replayExact();

    assert.deepEqual(rejectedResult, {
      kind: "unavailable",
      reason: "runtimeRejected",
      detail:
        "resource line exact replay is unavailable because replay execution failed: replay frames for signal were evicted",
      basisCurrentId: "basis-2",
      basisAdvanceCount: 1,
    });
    assert.deepEqual(projectBasisProof(rejected), beforeRejected);
  } finally {
    await mod.cleanup();
  }
});
