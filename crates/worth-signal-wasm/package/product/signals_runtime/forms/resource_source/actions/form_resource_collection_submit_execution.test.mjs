import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../../runtime_fixture/graph_operational_runtime.mjs";
import { createCollectionPatchLineFixture } from "../fixtures/resource_collection_line_fixture.mjs";

test("signals.form lowers repeated-field collection writes into resource item patches when the field declares collection item resource locus", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const line = createCollectionPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialItems: [
        { id: "a", label: "Alpha" },
        { id: "b", label: "Beta" },
      ],
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(line, { id: "collection-resource-submit" }),
      fields: ({ repeated }) => ({
        items: repeated("items", {
          itemIdentity: "id",
          resourceLocus: { kind: "collectionItems", placement: "append" },
        }),
      }),
    });

    form.fields.items.replaceItem("a", { id: "a", label: "Alpha+" });
    form.fields.items.removeItem("b");
    form.fields.items.addItem({ id: "c", label: "Gamma" });

    assert.equal(form.fieldContract()[0]?.resourceLocus?.kind, "collectionItems");
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "fulfilled");
    assert.deepEqual(
      execution.resourceSubmission?.patches.map((patch) => patch.patchKind),
      ["delete", "item", "insert"],
    );
    assert.deepEqual(line.value().items, [
      { id: "a", label: "Alpha+" },
      { id: "c", label: "Gamma" },
    ]);
  } finally {
    await cleanup();
  }
});

test("signals.form denies collection-backed submit before effects when repeated field resource locus is not declared", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const line = createCollectionPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialItems: [{ id: "a", label: "Alpha" }],
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(line, { id: "collection-resource-denial" }),
      fields: ({ repeated }) => ({
        items: repeated("items", {
          itemIdentity: "id",
        }),
      }),
    });

    form.fields.items.addItem({ id: "b", label: "Beta" });
    const plan = form.actionPlan("submit");
    assert.equal(plan.status, "denied");
    assert.equal(plan.resourceAction.source, "submitWithoutResourcePatchAdmission");
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "denied");
    assert.equal(execution.effectStarted, false);
  } finally {
    await cleanup();
  }
});

test("signals.form uses explicit whole-resource replace when repeated collection order changes or paged placement needs broad replacement", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const line = createCollectionPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      familyKind: "paged",
      familyId: "task-page",
      runtimeLineId: "task:page:1",
      canonicalKey: "page=1",
      initialItems: [
        { id: "a", label: "Alpha" },
        { id: "b", label: "Beta" },
      ],
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(line, { id: "paged-resource-submit" }),
      fields: ({ repeated }) => ({
        items: repeated("items", {
          itemIdentity: "id",
          resourceLocus: { kind: "collectionItems", placement: "append" },
        }),
      }),
    });

    form.fields.items.moveItem("b", "a");
    const patchPlan = form.patchPlan();
    assert.equal(patchPlan.broadReplacement, true);
    assert.equal(patchPlan.replacement?.reason, "repeatedReorderRequiresWholeReplace");

    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "fulfilled");
    assert.deepEqual(
      execution.resourceSubmission?.patches.map((patch) => patch.patchKind),
      ["replace"],
    );
    assert.deepEqual(line.value().items, [
      { id: "b", label: "Beta" },
      { id: "a", label: "Alpha" },
    ]);
  } finally {
    await cleanup();
  }
});
