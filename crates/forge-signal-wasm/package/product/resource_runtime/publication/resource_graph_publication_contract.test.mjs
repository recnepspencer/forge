import assert from "node:assert/strict";
import test from "node:test";

import { createGraphPublicationRuntime } from "../../signals_runtime/runtime_fixture/graph_publication_runtime.mjs";
import { loadSignalsModule } from "../../signals_runtime/module_loading/load_signals_module.mjs";

test("resource line publication keeps the graph output name explicit before and after rematerialization", async () => {
  const {
    wrapSignals,
    resourceParams,
    resourceParamIdentity,
    cleanup,
  } = await loadSignalsModule();
  const rawSignals = createGraphPublicationRuntime();

  try {
    const signals = wrapSignals(rawSignals);
    const productResource = signals.resource.detail({
      params: resourceParams(),
      normalizeParams: ({ productId }) =>
        resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({ id: productId, label: `product:${productId}` }),
    });

    const firstLine = productResource.line({ productId: "p1" });
    const firstGraph = signals.graph("productDetail", {
      outputs: {
        product: firstLine.signal(),
      },
    });

    assert.match(
      firstLine.descriptor().runtimeLineId,
      /^__resourceFamily\.detail\.\d+\.line1$/,
    );
    assert.equal(firstGraph.outputs.product.id, "productDetail.product");
    assert.deepEqual(firstGraph.descriptors(), [
      {
        outputName: "product",
        sourceId: firstLine.signal().id,
        sourceKind: "computed",
        publishedId: "productDetail.product",
        publicationKind: "synthesizedOutput",
      },
    ]);
    assert.notEqual(
      firstGraph.descriptors()[0].publishedId,
      firstLine.descriptor().runtimeLineId,
    );

    firstLine.free();

    const secondLine = productResource.line({ productId: "p1" });
    const secondGraph = signals.graph("productDetailRematerialized", {
      outputs: {
        product: secondLine.signal(),
      },
    });

    assert.equal(
      secondLine.descriptor().family.familyId,
      firstLine.descriptor().family.familyId,
    );
    assert.equal(
      secondLine.descriptor().canonicalParams.canonicalKey,
      firstLine.descriptor().canonicalParams.canonicalKey,
    );
    assert.notEqual(
      secondLine.descriptor().runtimeLineId,
      firstLine.descriptor().runtimeLineId,
    );
    assert.equal(secondGraph.outputs.product.id, "productDetailRematerialized.product");
    assert.notEqual(
      secondGraph.descriptors()[0].publishedId,
      secondLine.descriptor().runtimeLineId,
    );
  } finally {
    await cleanup();
  }
});
