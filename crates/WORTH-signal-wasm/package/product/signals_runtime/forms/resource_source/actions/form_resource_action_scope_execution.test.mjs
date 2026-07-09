import assert from "node:assert/strict";
import test from "node:test";

import { withSignals } from "../../action_execution_test_helpers.mjs";
import {
  createAttachmentTransferLineFixture,
  createDownloadDescriptors,
} from "../fixtures/resource_attachment_transfer_line_fixture.mjs";
import { createDeclaredLocusDetailLineFixture } from "../fixtures/resource_declared_locus_line_fixture.mjs";

test("signals.form scopes resource-line custom patch actions to declared evidence fields and preserves unrelated draft truth", async () => {
  await withSignals((signals) => {
    const source = createDeclaredLocusDetailLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        evidence: null,
      },
      regionNames: ["evidence"],
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-evidence-scope" }),
      fields: ({ field, evidence }) => ({
        title: field("title"),
        evidence: evidence("evidence", {
          attachmentIdentity: "digest",
          resourceLocus: { kind: "region", region: "evidence" },
        }),
      }),
      actions: ({ action }) => ({
        addEvidence: action("addEvidence", {
          resourceAction: { kind: "patchPlan", fields: ["evidence"] },
          resourceEffectProfile: signals.resource.effects.branchNative(),
        }),
      }),
    });

    form.fields.title.set("Retitle locally");
    form.fields.evidence.set({ digest: "file-2", name: "proof.pdf" });

    const plan = form.actionPlan("addEvidence");
    assert.equal(plan.status, "accepted");
    assert.equal(plan.patch.operations.length, 1);
    assert.equal(plan.patch.operations[0].field, "evidence");
    assert.equal(plan.patch.operations[0].kind, "attach");
    assert.equal(form.patchPlan().operations.length, 2);
    assert.notEqual(plan.proof.patchDigest, form.patchPlan().equivalenceDigest);

    const execution = form.executeAction("addEvidence");
    assert.equal(execution.resultKind, "fulfilled");
    assert.equal(execution.resourceSubmission.patchCount, 1);
    assert.deepEqual(execution.resourceSubmission.patches[0], {
      field: "evidence",
      path: "evidence",
      locusKind: "region",
      locus: "evidence",
      operationKind: "attach",
      patchKind: "region",
      patchResultKind: "narrowed",
      patchScope: "region",
      effectDigest: execution.resourceSubmission.patches[0].effectDigest,
      basisId: "basis-1",
    });
    assert.deepEqual(form.source(), {
      title: "Ship docs",
      evidence: { digest: "file-2", name: "proof.pdf" },
    });
    assert.deepEqual(form.draft(), {
      title: "Retitle locally",
    });
    assert.equal(form.canonicalizationHistory()[0].draftReset, false);
    assert.deepEqual(form.canonicalizationHistory()[0].draftClearedFields, ["evidence"]);
    assert.deepEqual(form.canonicalizationHistory()[0].nextDraftValue, {
      title: "Retitle locally",
    });
  });
});

test("signals.form lowers scoped repeated-field resource actions through explicit whole-resource replace when the scoped field owns the full change", async () => {
  await withSignals((signals) => {
    const source = createDeclaredLocusDetailLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        items: [
          { id: "a", label: "First" },
          { id: "b", label: "Second" },
        ],
      },
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-repeat-scope" }),
      fields: ({ field, repeated }) => ({
        title: field("title"),
        items: repeated("items", {
          itemIdentity: "id",
          resourceLocus: { kind: "collectionItems", placement: "append" },
        }),
      }),
      actions: ({ action }) => ({
        saveOrdering: action("saveOrdering", {
          resourceAction: { kind: "patchPlan", fields: ["items"] },
          resourceEffectProfile: signals.resource.effects.branchNative(),
        }),
      }),
    });

    form.fields.items.moveItem("b", "a");
    const plan = form.actionPlan("saveOrdering");
    assert.equal(plan.status, "accepted");
    assert.equal(plan.patch.broadReplacement, true);
    assert.deepEqual(plan.patch.replacement?.fields, ["items"]);

    const execution = form.executeAction("saveOrdering");
    assert.equal(execution.resultKind, "fulfilled");
    assert.equal(execution.resourceSubmission.patchCount, 1);
    assert.equal(execution.resourceSubmission.patches[0].patchKind, "replace");
    assert.equal(execution.resourceSubmission.patches[0].patchResultKind, "replaced");
    assert.deepEqual(form.source().items.map((item) => item.id), ["b", "a"]);
    assert.deepEqual(form.draft(), {});
  });
});

test("signals.form denies scoped repeated-field resource actions before effects when whole-resource replace would also consume out-of-scope fields", async () => {
  await withSignals((signals) => {
    const source = createDeclaredLocusDetailLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        items: [
          { id: "a", label: "First" },
          { id: "b", label: "Second" },
        ],
      },
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-repeat-scope-denied" }),
      fields: ({ field, repeated }) => ({
        title: field("title"),
        items: repeated("items", {
          itemIdentity: "id",
          resourceLocus: { kind: "collectionItems", placement: "append" },
        }),
      }),
      actions: ({ action }) => ({
        saveOrdering: action("saveOrdering", {
          resourceAction: { kind: "patchPlan", fields: ["items"] },
          resourceEffectProfile: signals.resource.effects.branchNative(),
        }),
      }),
    });

    form.fields.items.moveItem("b", "a");
    form.fields.title.set("Retitle locally");
    const plan = form.actionPlan("saveOrdering");
    assert.equal(plan.status, "denied");
    assert.equal(plan.resourceAction.source, "declaredWithoutResourcePatchAdmission");
    assert.equal(
      plan.readiness.blockers[0]?.reason,
      'resource-line action "saveOrdering" cannot lower a whole-resource replace because it would also consume out-of-scope fields: title',
    );

    const execution = form.executeAction("saveOrdering");
    assert.equal(execution.resultKind, "denied");
    assert.equal(execution.effectStarted, false);
    assert.equal(execution.resourceSubmission, undefined);
    assert.deepEqual(form.source().items.map((item) => item.id), ["a", "b"]);
    assert.deepEqual(form.draft().items.map((item) => item.id), ["b", "a"]);
    assert.equal(form.draft().title, "Retitle locally");
  });
});

test("signals.form does not let unrelated field blockers deny a scoped resource-line patch action", async () => {
  await withSignals((signals) => {
    const source = createDeclaredLocusDetailLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        evidence: null,
      },
      regionNames: ["evidence"],
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-scope-blockers" }),
      fields: ({ field, evidence }) => ({
        title: field("title"),
        evidence: evidence("evidence", {
          attachmentIdentity: "digest",
          resourceLocus: { kind: "region", region: "evidence" },
        }),
      }),
      availability: ({ field }) => ({
        titleAvailability: field("title", ["title"], (values) => (
          values.title === "Locked title"
            ? { state: "blocked", reason: "title is locked" }
            : "enabled"
        )),
      }),
      actions: ({ action }) => ({
        addEvidence: action("addEvidence", {
          resourceAction: { kind: "patchPlan", fields: ["evidence"] },
          resourceEffectProfile: signals.resource.effects.branchNative(),
        }),
      }),
    });

    form.fields.title.set("Locked title");
    form.fields.evidence.set({ digest: "file-2", name: "proof.pdf" });

    assert.equal(form.readiness().canSubmit, false);
    assert.deepEqual(
      form.readiness().blockers.filter((blocker) => blocker.field === "title").map((blocker) => blocker.kind),
      ["availability:blocked"],
    );

    const plan = form.actionPlan("addEvidence");
    assert.equal(plan.status, "accepted");
    assert.equal(plan.readiness.canRun, true);
    assert.deepEqual(plan.readiness.blockers, []);

    const execution = form.executeAction("addEvidence");
    assert.equal(execution.resultKind, "fulfilled");
    assert.deepEqual(form.source(), {
      title: "Ship docs",
      evidence: { digest: "file-2", name: "proof.pdf" },
    });
    assert.deepEqual(form.draft(), {
      title: "Locked title",
    });
  });
});

test("signals.form does not let unrelated resource transfer blockers deny a scoped resource-line patch action", async () => {
  await withSignals((signals) => {
    const source = createAttachmentTransferLineFixture({
      value: {
        title: "Ship docs",
        appendix: { digest: "file-2", name: "appendix.pdf" },
        supplement: { digest: "file-3", name: "supplement.pdf" },
      },
      upload: {
        kind: "prepared",
        transportKind: "signed",
        uploadId: "upload-1",
        descriptor: {
          kind: "signed",
          url: "https://example.test/upload",
          method: "PUT",
          headers: {},
          fields: {},
          objectKey: "object-1",
          expiresAt: null,
        },
        finalizeRequired: true,
        awaitingProcessing: false,
        message: "ready to upload",
      },
      processing: { kind: "ready", completionKind: "none", jobId: null, message: null },
      download: createDownloadDescriptors("file-2"),
      fieldNames: ["title"],
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-scope-transfer-blockers" }),
      fields: ({ field, attachment }) => ({
        title: field("title", {
          resourceLocus: { kind: "field", field: "title" },
        }),
        appendix: attachment("appendix", {
          attachmentIdentity: "digest",
        }),
        supplement: attachment("supplement", {
          attachmentIdentity: "digest",
        }),
      }),
      actions: ({ action }) => ({
        saveTitle: action("saveTitle", {
          resourceAction: { kind: "patchPlan", fields: ["title"] },
        }),
      }),
    });

    form.fields.title.set("Published docs");

    assert.equal(form.readiness().canSubmit, false);
    assert.equal(
      form.readiness().blockers.some((blocker) => blocker.kind === "resource:transferMappingUnavailable"),
      true,
    );

    const plan = form.actionPlan("saveTitle");
    assert.equal(plan.status, "accepted");
    assert.equal(plan.readiness.canRun, true);
    assert.deepEqual(plan.readiness.blockers, []);

    const execution = form.executeAction("saveTitle");
    assert.equal(execution.resultKind, "fulfilled");
    assert.deepEqual(form.source(), {
      title: "Published docs",
      appendix: { digest: "file-2", name: "appendix.pdf" },
      supplement: { digest: "file-3", name: "supplement.pdf" },
    });
    assert.deepEqual(form.draft(), {});
  });
});
