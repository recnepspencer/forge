import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form certifies phase one source authority and field loci", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);
    const signalSource = signals.input({
      title: "Phase 1",
      items: [{ id: "i1", label: "One" }],
      evidence: { digest: "file-1", name: "audit.pdf" },
    });
    const graphInput = signals.publicInput(signalSource, {
      authority: "readOnly",
      requiredness: "required",
    });
    const resourceLine = {
      value: () => ({ title: "Resource backed" }),
      descriptor: () => ({ family: "detail", member: "task", params: { id: "t1" } }),
      request: () => ({ method: "GET", target: "/tasks/t1" }),
      summary: () => ({ state: "fulfilled" }),
    };

    const signalForm = signals.form({
      id: "phase-one-task-form",
      source: signals.form.source.signal(signalSource, { id: "task-signal" }),
      fields: ({ field, repeated, attachment }) => ({
        title: field("title"),
        items: repeated("items", { itemIdentity: "id" }),
        evidence: attachment("evidence", {
          attachmentIdentity: "digest",
          metadata: { required: true },
        }),
      }),
    });

    assert.equal(signalForm.sourceAuthority().kind, "signal");
    assert.equal(signalForm.sourceAuthority().explicit, true);
    assert.equal(signalForm.sourceAuthority().sourceId, "task-signal");
    assert.equal(signalForm.declaration().formId, "phase-one-task-form");
    assert.deepEqual(signalForm.declaration().fieldFamilies, {
      scalar: 1,
      repeated: 1,
      attachment: 1,
    });
    assert.deepEqual(fieldContractSummary(signalForm), [
      { id: "title", family: "scalar", path: "title" },
      { id: "items", family: "repeated", path: "items" },
      { id: "evidence", family: "attachment", path: "evidence" },
    ]);
    assert.equal(
      signalForm.diagnostics().fieldContract[1].collectionIdentity.posture,
      "stableItemIdentityRequired",
    );
    assert.equal(
      signalForm.diagnostics().fieldContract[2].attachment.posture,
      "fileBlobIdentityAndMetadataDeclared",
    );
    assert.equal(Object.hasOwn(signalForm.fields.title, "addItem"), false);
    assert.equal(Object.hasOwn(signalForm.fields.items, "attachmentIdentity"), false);
    assert.equal(Object.hasOwn(signalForm.fields.evidence, "addItem"), false);

    signalForm.fields.items.addItem({ id: "i2", label: "Two" });
    signalForm.fields.items.moveItem("i2", "i1");
    signalForm.fields.items.replaceItem("i1", { id: "i1", label: "One updated" });
    assert.deepEqual(collectionItemIds(signalForm), ["i2", "i1"]);
    signalForm.fields.items.removeItem("i2");
    assert.deepEqual(collectionItemIds(signalForm), ["i1"]);
    assert.throws(
      () => signalForm.fields.items.replaceItem("i1", { id: "wrong", label: "Wrong" }),
      /preserve item identity/,
    );
    assert.throws(
      () => signalForm.fields.items.removeItem("missing"),
      /remove target was not found/,
    );
    assert.throws(
      () => signalForm.fields.items.replaceItem("missing", { id: "missing", label: "Missing" }),
      /replacement target was not found/,
    );

    assert.equal(signalForm.fields.evidence.attachmentIdentity().attachmentDigest, "file-1");
    signalForm.fields.evidence.set({ digest: "file-2", name: "signed.pdf" });
    assert.deepEqual(signalForm.fields.evidence.attachmentIdentity().metadata, {
      required: true,
    });
    assert.equal(signalForm.fields.evidence.attachmentIdentity().attachmentDigest, "file-2");
    assert.equal(
      signalForm.verification().digests.sourceAuthorityDigest,
      signalForm.sourceAuthority().sourceAuthorityDigest,
    );
    assert.ok(signalForm.verification().digests.fieldContractDigest.length > 0);
    assert.deepEqual(inputAdapterSummary(signalForm), [
      { field: "title", family: "scalar", tier: "signalNative" },
      { field: "items", family: "repeated", tier: "signalNative" },
      { field: "evidence", family: "attachment", tier: "signalNative" },
    ]);
    assert.ok(signalForm.verification().digests.inputAdapterCapabilityDigest.length > 0);

    const graphForm = signals.form({
      source: signals.form.source.graphPublicInput(graphInput, { id: "graph-task" }),
      fields: ({ field }) => ({ title: field("title") }),
    });
    assert.equal(graphForm.sourceAuthority().kind, "graphPublicInput");
    assert.equal(graphForm.sourceAuthority().identity.authority, "readOnly");

    const resourceForm = signals.form({
      source: signals.form.source.resourceLine(resourceLine, { id: "resource-task" }),
      fields: ({ field }) => ({ title: field("title") }),
    });
    assert.equal(resourceForm.sourceAuthority().kind, "resourceLine");
    assert.deepEqual(resourceForm.source(), { title: "Resource backed" });

    assert.throws(
      () => signals.form.source.graphPublicInput(signalSource),
      /expects signals\.publicInput/,
    );
    assert.throws(
      () => signals.form.source.resourceLine({ value: () => ({}) }),
      /resource line handle/,
    );
    assert.throws(
      () => signals.form({
        source: {},
        fields: ({ repeated }) => ({ items: repeated("items") }),
      }),
      /explicit itemIdentity or key/,
    );
    assert.throws(
      () => signals.form({
        source: {},
        fields: ({ repeated }) => ({ items: repeated("items", { itemIdentity: 7 }) }),
      }),
      /itemIdentity/,
    );
    assert.throws(
      () => signals.form({
        source: {},
        fields: ({ attachment }) => ({ evidence: attachment("evidence") }),
      }),
      /explicit attachmentIdentity or digest/,
    );
    assert.throws(
      () => signals.form({
        source: {},
        fields: ({ attachment }) => ({
          evidence: attachment("evidence", { attachmentIdentity: 7 }),
        }),
      }),
      /attachmentIdentity/,
    );
    const resolverForm = signals.form({
      source: { items: [] },
      fields: ({ repeated }) => ({
        items: repeated("items", { itemIdentity: () => "" }),
      }),
    });
    assert.throws(
      () => resolverForm.fields.items.addItem({ id: "bad" }),
      /resolver returned an invalid item identity/,
    );
  } finally {
    await cleanup();
  }
});

function fieldContractSummary(form) {
  return form.diagnostics().fieldContract.map((field) => ({
    id: field.id,
    family: field.family,
    path: field.path,
  }));
}

function collectionItemIds(form) {
  return form.fields.items.collectionIdentity().items.map((item) => item.itemId);
}

function inputAdapterSummary(form) {
  return form.inputAdapters().map((adapter) => ({
    field: adapter.field,
    family: adapter.family,
    tier: adapter.tier,
  }));
}
