import assert from "node:assert/strict";
import test from "node:test";

import { projectBasisProof } from "../delivery/delivery_basis_history_proof_helpers.mjs";
import {
  createRealResourceNamespace,
  createRealResourceRuntime,
} from "../runtime_fixture/real_resource_signals.mjs";

function createReplayDetailLine(resourceMod, signals) {
  return createRealResourceNamespace(resourceMod, signals)
    .detail({
      params: resourceMod.resourceParams(),
      normalizeParams: ({ id }) => resourceMod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({
        id,
        state: "live",
      }),
    })
    .line({ id: "replayable" });
}

test("line history replayExact is explicitly unsupported on the shipped real runtime", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const line = createReplayDetailLine(runtime.resourceMod, runtime.signals);

    assert.deepEqual(line.history().availability.replayExact, {
      kind: "unavailable",
      reason: "unsupportedByRuntime",
      detail:
        "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
    });
    assert.deepEqual(line.history().replayExact(), {
      kind: "unavailable",
      reason: "unsupportedByRuntime",
      detail:
        "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
      basisCurrentId: null,
      basisAdvanceCount: 0,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("unsupported exact replay does not rewrite basis-bearing proof surfaces", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const rejected = createRealResourceNamespace(
      runtime.resourceMod,
      runtime.signals,
    )
      .collection({
        params: runtime.resourceMod.resourceParams(),
        requestContext: runtime.resourceMod.resourceRequestContext({
          basisId: "basis-1",
        }),
        normalizeParams: ({ workspaceId }) =>
          runtime.resourceMod.resourceParamIdentity(
            { workspaceId },
            workspaceId,
          ),
        itemIdentity: (item) => item.id,
        reconcile: runtime.resourceMod.resourceCollectionShape({
          items: (value) => value.items,
          replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        }),
        load: (_params, request) => ({
          items: [{ id: "demo:1", title: `Load:${request.context.basisId}` }],
        }),
      })
      .line({ workspaceId: "demo" });

    rejected.deliver(
      runtime.resourceMod.resourceDelivery.replace({
        packetId: "pkt-basis-2",
        basisId: "basis-1",
        nextBasisId: "basis-2",
        nextValue: {
          items: [{ id: "demo:1", title: "Delivered Basis 2" }],
        },
      }),
    );
    const before = projectBasisProof(rejected);
    const result = rejected.history().replayExact();
    const after = projectBasisProof(rejected);

    assert.deepEqual(result, {
      kind: "unavailable",
      reason: "unsupportedByRuntime",
      detail:
        "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
      basisCurrentId: "basis-2",
      basisAdvanceCount: 1,
    });
    assert.deepEqual(after.requestBasisId, before.requestBasisId);
    assert.deepEqual(after.diagnosticsBasis, before.diagnosticsBasis);
    assert.deepEqual(after.summaryBasis, before.summaryBasis);
    assert.deepEqual(after.historyBasis, before.historyBasis);
    assert.deepEqual(after.lifecycleBasis, before.lifecycleBasis);
  } finally {
    await runtime.cleanup();
  }
});
