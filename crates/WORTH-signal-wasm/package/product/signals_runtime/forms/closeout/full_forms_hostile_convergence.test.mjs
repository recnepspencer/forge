import assert from "node:assert/strict";
import test from "node:test";

import { withSignals } from "../action_execution_test_helpers.mjs";
import { createDetailPatchLineFixture } from "../resource_source/fixtures/resource_line_fixture.mjs";

function projectStableSemanticSnapshot(form) {
  const verification = form.verification();
  return {
    source: form.source(),
    draft: form.draft(),
    effective: form.effective(),
    readiness: verification.digests.readinessDigest,
    validation: verification.digests.validationDigest,
    semanticEquality: verification.digests.semanticEqualityDigest,
  };
}

test("forms closeout keeps local, presentation, async, collaboration, and resource recovery surfaces coherent under hostile sequencing", async () => {
  await withSignals((signals) => {
    const source = signals.input({
      title: "",
      slug: "ship-docs",
      status: "draft",
    });
    const schemaVersion = signals.input("v1");
    const localForm = signals.form({
      source: {
        value: source,
        schemaVersion,
        sourceAdmission: { status: "ready", reason: "source admitted" },
        draftRestore: { status: "ready", reason: "draft restored" },
      },
      fields: ({ field }) => ({
        title: field("title", { row: "hero" }),
        slug: field("slug"),
        status: field("status"),
      }),
      validation: ({ field, asyncField }) => ({
        titleRequired: field("title", (value) => (
          value.length > 0
            ? { kind: "valid", field: "title", digest: value }
            : {
                kind: "invalid",
                field: "title",
                message: {
                  code: "title.required",
                  severity: "error",
                  audience: "user",
                  visibility: "visible",
                },
              }
        )),
        slugUnique: asyncField("slug", {
          id: "slugUnique",
          triggers: ["submit"],
        }),
      }),
      actions: ({ submit }) => ({
        submit: submit({
          hostRequirements: ["online"],
        }),
      }),
      collaboration: {
        mode: "fieldLease",
        actorId: "local-author",
        supportsPresence: true,
      },
      host: {
        online: true,
        focus: "title",
      },
      presentation: {
        entry: {
          bootstrap: {
            validation: true,
            readiness: true,
            layoutMeasurement: true,
          },
        },
        action: {
          settleOn: ["messages", "layout"],
        },
      },
    });

    assert.equal(localForm.presentationLifecycle("entry").status, "pending");
    localForm.recordLayoutMeasurement([{ row: "hero", controlHeight: 32 }], {
      cause: "animationFrame",
    });
    assert.equal(localForm.presentationLifecycle("entry").status, "ready");

    const beforeUiOnly = projectStableSemanticSnapshot(localForm);
    localForm.reportMessages({
      status: "settling",
      reason: "save toast visible",
      channel: "toast",
    });
    localForm.reportCollaboration({
      posture: "blocked",
      leasedFields: [{ field: "title", ownerId: "peer-1" }],
      reason: "peer owns the title field lease",
    });
    const afterUiOnly = projectStableSemanticSnapshot(localForm);

    assert.deepEqual(afterUiOnly, beforeUiOnly);
    assert.equal(localForm.fieldWritePosture("title").canWrite, false);

    localForm.clearMessages({ reason: "toast dismissed" });
    localForm.clearCollaboration({ reason: "peer lease released" });
    assert.equal(localForm.fieldWritePosture("title").canWrite, true);

    localForm.fields.title.set("Ship docs");
    const asyncStart = localForm.startAsyncValidation("slugUnique");
    const duringAsync = localForm.presentationLifecycle("entry");
    assert.equal(duringAsync.status, "busy");
    assert.equal(duringAsync.bootstrap?.posture, "pending");

    localForm.fulfillAsyncValidation(asyncStart.operationId, {
      reason: "slug is unique",
    });
    assert.equal(localForm.presentationLifecycle("entry").status, "ready");

    const execution = localForm.executeAction("submit");
    assert.equal(execution.resultKind, "pending");
    const fulfilled = localForm.fulfillAction(execution.operationId, {
      reason: "server canonicalized title",
      canonicalValue: {
        title: "Ship docs",
        slug: "ship-docs",
        status: "published",
      },
    });
    assert.equal(fulfilled.resultKind, "fulfilled");
    assert.equal(localForm.source().status, "published");
    assert.equal(localForm.verification().canonicalizationHistory.operations, 1);

    const reset = localForm.reset({ reason: "accept canonical truth" });
    assert.equal(reset.resultKind, "noOp");
    assert.deepEqual(localForm.draft(), {});

    const resourceLine = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        status: "draft",
      },
    });
    const resourceForm = signals.form({
      source: signals.form.source.resourceLine(resourceLine, { id: "closeout-resource-form" }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
      actions: ({ submit }) => ({
        submit: submit({
          resourceEffectProfile: signals.resource.effects.branchNative(),
        }),
      }),
    });

    resourceForm.fields.title.set("Published docs");
    resourceForm.fields.status.set("review");
    const resourceExecution = resourceForm.executeAction("submit");
    assert.equal(resourceExecution.resultKind, "fulfilled");
    assert.equal(resourceForm.source().status, "review");
    assert.equal(typeof resourceForm.resourceSource()?.effectProfile.closeoutMatrixDigest, "string");

    const restored = resourceForm.restoreExactResourceSource({
      reason: "branch restore after hostile sequence",
    });
    assert.equal(restored.resultKind, "restored");
    assert.equal(resourceForm.replayRestoreHistory().length, 1);
    assert.equal(resourceForm.verification().performanceEnvelope.replayRestoreOperations, 1);

    const replay = resourceForm.replayExactResourceSource({
      reason: "replay proof stays explicit when runtime support is absent",
    });
    assert.ok(["replayed", "unavailable"].includes(replay.resultKind));

    const rollback = resourceForm.rollbackLastResourceEffect({
      reason: "undo speculative branch effect",
    });
    assert.ok(["rolledBack", "unavailable"].includes(rollback.resultKind));
    assert.equal(resourceForm.resetHistory().length >= 1, true);
    assert.equal(typeof resourceForm.resourceSource()?.digest, "string");
    assert.equal(typeof resourceForm.resourceSource()?.visibleSelection.digest, "string");
  });
});
