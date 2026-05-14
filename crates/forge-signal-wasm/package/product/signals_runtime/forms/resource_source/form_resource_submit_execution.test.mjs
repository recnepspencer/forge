import assert from "node:assert/strict";
import test from "node:test";

import { withSignals } from "../action_execution_test_helpers.mjs";
import {
  createDetailPatchLineFixture,
  createMutationResponsePlanFixture,
} from "./resource_line_fixture.mjs";

test("signals.form lowers resource-backed submit through detail resource patches", async () => {
  await withSignals((signals) => {
    const source = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        status: "draft",
      },
      mutationResponse: createMutationResponsePlanFixture({
        confirmationKind: "partialCanonicalTruth",
        fallbackKind: "partialReconciliation",
      }),
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-submit" }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
    });

    form.fields.title.set("Published docs");
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "fulfilled");
    assert.equal(execution.action, "submit");
    assert.equal(execution.effectStarted, true);
    assert.deepEqual(execution.canonicalValue, {
      title: "Published docs",
      status: "draft",
    });
    assert.equal(execution.resourceSubmission.sourceKind, "resourceLine");
    assert.equal(execution.resourceSubmission.patchCount, 1);
    assert.equal(execution.resourceSubmission.effectProfile.profile.name, "branchNative");
    assert.equal(typeof execution.resourceSubmission.effectProfile.closeoutMatrixDigest, "string");
    assert.equal(execution.resourceSubmission.rollback.kind, "compactInverseAvailable");
    assert.equal(execution.resourceSubmission.rollback.mode, "CompactInversePatch");
    assert.equal(execution.resourceSubmission.visibleSelection.kind, "speculative");
    assert.equal(execution.resourceSubmission.mutationResponse.confirmationKind, "partialCanonicalTruth");
    assert.equal(execution.resourceSubmission.mutationResponse.fallbackTargetCount, 1);
    assert.equal(typeof execution.resourceSubmission.verification.packageDigest, "string");
    assert.equal(typeof execution.resourceSubmission.verification.mutationResponseCloseoutMatrixDigest, "string");
    assert.deepEqual(execution.resourceSubmission.patches[0], {
      field: "title",
      path: "title",
      locusKind: "field",
      locus: "title",
      patchKind: "field",
      patchResultKind: "narrowed",
      patchScope: "field",
      effectDigest: execution.resourceSubmission.patches[0].effectDigest,
      basisId: "basis-1",
    });
    assert.equal(form.canonicalizationHistory().length, 1);
    assert.equal(form.canonicalizationHistory()[0].resourceBacked.sourceKind, "resourceLine");
    assert.equal(form.canonicalizationHistory()[0].resourceBacked.rollback.kind, "compactInverseAvailable");
    assert.equal(
      form.canonicalizationHistory()[0].resourceBacked.mutationResponse.confirmationKind,
      "partialCanonicalTruth",
    );
    assert.equal(
      form.canonicalizationHistory()[0].sourceProjection,
      "resourceMutationResponsePartialCanonicalTruth",
    );
    assert.equal(
      form.canonicalizationHistory()[0].resourceBacked.resourceSubmissionDigest,
      execution.resourceSubmission.digest,
    );
    assert.deepEqual(form.source(), {
      title: "Published docs",
      status: "draft",
    });
    assert.deepEqual(form.draft(), {});
  });
});

test("signals.form denies resource-backed submit when no declared resource locus exists", async () => {
  await withSignals((signals) => {
    const source = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        status: "draft",
      },
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-denied" }),
      fields: ({ field }) => ({
        title: field("title"),
        state: field("statusText"),
      }),
    });

    form.fields.state.set("published");
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "denied");
    assert.equal(execution.action, "submit");
    assert.equal(execution.effectStarted, false);
    assert.equal(
      execution.reason,
      'resource-backed submit has no declared resource locus for form field path "statusText"',
    );
    assert.equal(execution.resourceSubmission, null);
    assert.deepEqual(form.source(), {
      title: "Ship docs",
      status: "draft",
    });
    assert.deepEqual(form.draft(), { statusText: "published" });
  });
});

test("signals.form denies mixed resource-backed submit plans before any resource patch begins", async () => {
  await withSignals((signals) => {
    const source = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        status: "draft",
      },
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-atomic-denial" }),
      fields: ({ field }) => ({
        title: field("title"),
        state: field("statusText"),
      }),
    });

    form.fields.title.set("Published docs");
    form.fields.state.set("published");
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "denied");
    assert.equal(execution.effectStarted, false);
    assert.equal(
      execution.reason,
      'resource-backed submit has no declared resource locus for form field path "statusText"',
    );
    assert.deepEqual(form.source(), {
      title: "Ship docs",
      status: "draft",
    });
    assert.deepEqual(form.effective(), {
      title: "Published docs",
      status: "draft",
      statusText: "published",
    });
    assert.equal(source.patchHistory().length, 0);
  });
});

test("signals.form keeps delivery-awaited resource completion explicit in canonicalization history", async () => {
  await withSignals((signals) => {
    const source = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        status: "draft",
      },
      mutationResponse: createMutationResponsePlanFixture({
        confirmationKind: "deliveryAwaited",
        fallbackKind: "deliveryAwaited",
      }),
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-delivery-awaited" }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
    });

    form.fields.title.set("Published docs");
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "fulfilled");
    assert.equal(execution.resourceSubmission.mutationResponse.confirmationKind, "deliveryAwaited");
    assert.equal(
      form.canonicalizationHistory()[0].sourceProjection,
      "resourceMutationResponseDeliveryAwaited",
    );
    assert.notEqual(
      form.canonicalizationHistory()[0].sourceProjection,
      "serverCanonicalUntilAuthoritativeSourceDrift",
    );
  });
});
