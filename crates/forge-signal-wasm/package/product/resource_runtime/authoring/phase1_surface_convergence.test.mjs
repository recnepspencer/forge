import assert from "node:assert/strict";
import test from "node:test";

import { createGraphPublicationRuntime } from "../../signals_runtime/runtime_fixture/graph_publication_runtime.mjs";
import { loadSignalsModule } from "../../signals_runtime/module_loading/load_signals_module.mjs";
import { createPhase1FamilyCases } from "../runtime_fixture/phase1_family_cases.mjs";

test("one semantic declaration keeps one Phase 1 identity story across line, view, publication, rematerialization, replay, and release", async () => {
  const {
    wrapSignals,
    resourceParams,
    resourceParamIdentity,
    cleanup,
  } = await loadSignalsModule();
  const rawSignals = createGraphPublicationRuntime();

  try {
    const signals = wrapSignals(rawSignals);
    const resource = signals.resource;
    const mod = {
      resourceParams,
      resourceParamIdentity,
    };
    const familyCases = createPhase1FamilyCases(resource, mod);

    for (const familyCase of familyCases) {
      const directFamily = familyCase.build(familyCase.directLoad);
      const helperFamily = familyCase.build(familyCase.helperLoad);
      const directLine = directFamily.line({ productId: "p1" });
      const helperLine = helperFamily.line({ productId: "p1" });

      assert.equal(directLine.descriptor().family.kind, familyCase.kind);
      assert.equal(helperLine.descriptor().family.kind, familyCase.kind);
      assert.equal(
        directLine.descriptor().canonicalParams.canonicalKey,
        helperLine.descriptor().canonicalParams.canonicalKey,
      );
      assert.deepEqual(directLine.status(), helperLine.status());
      assert.deepEqual(directLine.freshness(), helperLine.freshness());
      assert.deepEqual(directLine.value(), helperLine.value());

      const firstView = familyCase.view(directLine);
      const firstGraph = signals.graph(`${familyCase.kind}Detail`, {
        outputs: {
          product: directLine.signal(),
        },
      });
      const firstHistory = directLine.history();
      const firstDescriptor = directLine.descriptor();
      const firstSignalId = directLine.signal().id;

      assert.notEqual(firstView.id, firstSignalId);
      assert.equal(firstGraph.outputs.product.id, `${familyCase.kind}Detail.product`);
      assert.equal(firstHistory.replay.id, firstSignalId);
      assert.equal(firstHistory.lineage.id, firstSignalId);
      assert.equal(firstDescriptor.canonicalParams.canonicalKey, "p1");

      directLine.free();
      assert.throws(
        () => directLine.value(),
        /cannot be used after line\.free\(\)/,
      );

      const rematerializedLine = directFamily.line({ productId: "p1" });
      const rematerializedGraph = signals.graph(
        `${familyCase.kind}DetailReloaded`,
        {
          outputs: {
            product: rematerializedLine.signal(),
          },
        },
      );
      const rematerializedHistory = rematerializedLine.history();

      assert.equal(
        rematerializedLine.descriptor().family.familyId,
        firstDescriptor.family.familyId,
      );
      assert.equal(
        rematerializedLine.descriptor().canonicalParams.canonicalKey,
        firstDescriptor.canonicalParams.canonicalKey,
      );
      assert.notEqual(
        rematerializedLine.descriptor().runtimeLineId,
        firstDescriptor.runtimeLineId,
      );
      assert.notEqual(rematerializedLine.signal().id, firstSignalId);
      assert.equal(
        rematerializedHistory.replay.id,
        rematerializedLine.signal().id,
      );
      assert.equal(
        rematerializedHistory.lineage.id,
        rematerializedLine.signal().id,
      );
      assert.equal(
        rematerializedGraph.descriptors()[0].publishedId,
        `${familyCase.kind}DetailReloaded.product`,
      );
      assert.notEqual(
        rematerializedGraph.descriptors()[0].publishedId,
        rematerializedLine.descriptor().runtimeLineId,
      );
    }
  } finally {
    await cleanup();
  }
});
