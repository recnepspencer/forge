import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

test("public collection shapes cannot manually attach response lens proof", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const response = signals.resource.response.array({
      itemId: (task) => task.id,
      aspects: signals.resource.response.objectAspects()({
        title: "title",
      }),
    });

    assert.throws(
      () =>
        signalsMod.resourceCollectionShape({
          items: (value) => value,
          replaceItems: (_value, nextItems) => [...nextItems],
          aspects: signalsMod.resourceItemAspects({
            title: {
              read: (task) => task.title,
              write: (task, title) => ({ ...task, title }),
            },
          }),
          responseLensProof: response.lensProof,
        }),
      /does not accept responseLensProof/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("public collection shapes cannot borrow proof for a different topology", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const directArrayResponse = signals.resource.response.array({
      itemId: (task) => task.id,
    });

    assert.throws(
      () =>
        signalsMod.resourceCollectionShape({
          items: (value) => value.edges.map((edge) => edge.node),
          replaceItems: (value, nextItems) => ({
            ...value,
            edges: nextItems.map((node) => ({ node })),
          }),
          responseLensProof: directArrayResponse.lensProof,
        }),
      /does not accept responseLensProof/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("response-derived reconciliation keeps sealed proof attachment internal", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const response = signals.resource.response.collection({
      itemId: (task) => task.id,
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      aspects: signals.resource.response.objectAspects()({
        title: "title",
      }),
    });
    const tasks = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/response-owned-proof")
      .response(response)
      .list({
        load: () => ({
          items: [{ id: "t1", title: "First" }],
          total: 1,
        }),
      });

    const line = tasks.line({});
    line.patch(
      tasks.patch.itemAspect({
        itemId: "t1",
        aspect: "title",
        value: "Renamed",
      }),
    );

    assert.deepEqual(line.value(), {
      items: [{ id: "t1", title: "Renamed" }],
      total: 1,
    });
    const locusProof = line.diagnostics().lastEffect.locusProof;
    assert.equal(
      locusProof.declarationDigest,
      response.lensProof.declarationDigest,
    );
    assert.equal(
      locusProof.capabilityDigest,
      response.lensProof.capabilityDigest,
    );
    assert.equal(
      locusProof.compiledLensDigest,
      response.lensProof.compiledLensDigest,
    );
    assert.equal(
      locusProof.capabilityRowDigest,
      "response-capability-row|itemAspect|aspect|admitted|none",
    );
    assert.equal(
      locusProof.effectLocusDigest,
      [
        "response-effect-locus",
        response.lensProof.compiledLensDigest,
        "response-capability-row|itemAspect|aspect|admitted|none",
        "itemAspect",
        "none",
        "title",
        "none",
        "none",
      ].join("|"),
    );
    assert.deepEqual(
      {
        version: locusProof.version,
        lensVersion: locusProof.lensVersion,
        lensSource: locusProof.lensSource,
        topology: locusProof.topology,
        itemField: locusProof.itemField,
        locus: locusProof.locus,
        patchScope: locusProof.patchScope,
        aspect: locusProof.aspect,
        summary: locusProof.summary,
        summaryPatchScope: locusProof.summaryPatchScope,
        proofBreadth: locusProof.proofBreadth,
      },
      {
      version: "resource-effect-locus-proof-v1",
      lensVersion: "resource-response-lens-proof-v1",
      lensSource: "resource.response.collection(...)",
      topology: "customCollection",
      itemField: null,
      locus: "itemAspect",
      patchScope: "aspect",
      aspect: "title",
      summary: null,
      summaryPatchScope: null,
      proofBreadth: 1,
      },
    );
  } finally {
    await runtime.cleanup();
  }
});

test("response-derived reconciliation still validates proof and summary shape", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const response = signals.resource.response.objectItems()({
      field: "tasks",
      itemId: (task) => task.id,
      summaries: signalsMod.resourceValueSummaries({
        total: {
          read: (value) => value.total,
          write: (value, total) => ({ ...value, total }),
        },
      }),
    });
    const tasks = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/response-owned-summary-proof")
      .response(response)
      .list({
        load: () => ({
          tasks: [{ id: "t1", title: "First" }],
          total: 1,
        }),
      });

    const line = tasks.line({});
    line.patch(tasks.patch.summary({ summary: "total", value: 2 }));

    assert.deepEqual(line.value(), {
      tasks: [{ id: "t1", title: "First" }],
      total: 2,
    });
    assert.equal(line.diagnostics().lastEffect.locusProof.locus, "summary");
    assert.equal(
      line.diagnostics().lastEffect.locusProof.summaryPatchScope,
      "line",
    );
  } finally {
    await runtime.cleanup();
  }
});
