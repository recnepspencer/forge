import assert from "node:assert/strict";
import test from "node:test";

import { withSignals } from "../../action_execution_test_helpers.mjs";
import {
  createDetailPatchLineFixture,
  createMutationResponsePlanFixture,
  createReadOnlyResourceLineFixture,
} from "../fixtures/resource_line_fixture.mjs";

test("signals.form lowers resource-line submit through detail resource patches", async () => {
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
    assert.equal(
      execution.resourceSubmission.mutationResponse.outOfContractTargetDigest,
      "mutation-response-unsupportedTarget-targets|none",
    );
    assert.equal(execution.resourceSubmission.mutationResponse.completion.multiFamily, false);
    assert.equal(typeof execution.resourceSubmission.verification.packageDigest, "string");
    assert.equal(typeof execution.resourceSubmission.verification.mutationResponseCloseoutMatrixDigest, "string");
    assert.deepEqual(execution.resourceSubmission.patches[0], {
      field: "title",
      path: "title",
      locusKind: "field",
      locus: "title",
      operationKind: "set",
      patchKind: "field",
      patchResultKind: "narrowed",
      patchScope: "field",
      effectDigest: execution.resourceSubmission.patches[0].effectDigest,
      basisId: "basis-1",
    });
    assert.equal(form.canonicalizationHistory().length, 1);
    assert.equal(form.canonicalizationHistory()[0].resourceLine.sourceKind, "resourceLine");
    assert.equal(form.canonicalizationHistory()[0].resourceLine.rollback.kind, "compactInverseAvailable");
    assert.equal(
      form.canonicalizationHistory()[0].resourceLine.mutationResponse.confirmationKind,
      "partialCanonicalTruth",
    );
    assert.equal(
      form.canonicalizationHistory()[0].resourceLine.mutationResponse.contract.digest,
      execution.resourceSubmission.mutationResponse.contract.digest,
    );
    assert.equal(
      form.canonicalizationHistory()[0].resourceLine.mutationResponse.completion.digest,
      execution.resourceSubmission.mutationResponse.completion.digest,
    );
    assert.equal(
      form.canonicalizationHistory()[0].sourceProjection,
      "resourceMutationResponsePartialCanonicalTruth",
    );
    assert.equal(
      form.canonicalizationHistory()[0].resourceLine.resourceSubmissionDigest,
      execution.resourceSubmission.digest,
    );
    assert.deepEqual(form.source(), {
      title: "Published docs",
      status: "draft",
    });
    assert.deepEqual(form.draft(), {});
  });
});

test("signals.form denies resource-line submit when no declared resource locus exists", async () => {
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
    const plan = form.actionPlan("submit");
    assert.equal(plan.status, "denied");
    assert.equal(plan.resourceAction.source, "submitWithoutResourcePatchAdmission");
    assert.equal(
      plan.readiness.blockers[0]?.reason,
      'resource-line action "submit" has no declared resource locus for form field path "statusText"',
    );
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "denied");
    assert.equal(execution.action, "submit");
    assert.equal(execution.effectStarted, false);
    assert.equal(execution.resourceSubmission, undefined);
    assert.deepEqual(form.source(), {
      title: "Ship docs",
      status: "draft",
    });
    assert.deepEqual(form.draft(), { statusText: "published" });
  });
});

test("signals.form denies mixed resource-line submit plans before any resource patch begins", async () => {
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
    const plan = form.actionPlan("submit");
    assert.equal(plan.status, "denied");
    assert.equal(plan.resourceAction.source, "submitWithoutResourcePatchAdmission");
    assert.equal(
      plan.readiness.blockers[0]?.reason,
      'resource-line action "submit" has no declared resource locus for form field path "statusText"',
    );
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "denied");
    assert.equal(execution.effectStarted, false);
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

test("signals.form lowers declared resource-line custom actions through the same resource patch execution path", async () => {
  await withSignals((signals) => {
    const source = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        status: "draft",
      },
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-custom-action" }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
      actions: ({ action }) => ({
        saveResourceDraft: action("saveResourceDraft", {
          resourceAction: { kind: "patchPlan" },
          resourceEffectProfile: signals.resource.effects.branchNative(),
        }),
      }),
    });

    form.fields.title.set("Queued draft");
    const execution = form.executeAction("saveResourceDraft");
    assert.equal(execution.resultKind, "fulfilled");
    assert.equal(execution.action, "saveResourceDraft");
    assert.equal(execution.effectStarted, true);
    assert.equal(execution.resourceSubmission.sourceKind, "resourceLine");
    assert.equal(execution.resourceSubmission.patchCount, 1);
    assert.equal(execution.resourceSubmission.effectProfile.profile.name, "branchNative");
    assert.deepEqual(form.source(), {
      title: "Queued draft",
      status: "draft",
    });
    assert.deepEqual(form.draft(), {});
  });
});

test("signals.form lowers declared resource-line lifecycle actions into resource line refresh and revalidate requests", async () => {
  await withSignals((signals) => {
    const refreshSource = createReadOnlyResourceLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      status: {
        kind: "rejected",
        operation: "refresh",
        message: "network down",
        continuity: "preservedVisibleValue",
      },
      freshness: { kind: "fresh" },
    });
    const refreshForm = signals.form({
      source: signals.form.source.resourceLine(refreshSource, { id: "task-resource-lifecycle-refresh" }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action }) => ({
        refreshResourceSource: action("refreshResourceSource", {
          resourceAction: { kind: "refresh" },
        }),
      }),
    });

    const refreshExecution = refreshForm.executeAction("refreshResourceSource");
    assert.equal(refreshExecution.resultKind, "fulfilled");
    assert.equal(refreshExecution.action, "refreshResourceSource");
    assert.equal(refreshExecution.effectStarted, true);
    assert.equal(refreshExecution.resourceSubmission, null);
    assert.equal(refreshExecution.resourceLifecycle.operation, "refresh");
    assert.equal(refreshExecution.resourceLifecycle.status.kind, "pending");
    assert.equal(refreshExecution.resourceLifecycle.status.operation, "refresh");
    assert.equal(refreshForm.resourceSource().status.kind, "pending");
    assert.equal(refreshForm.resourceSource().status.operation, "refresh");
    assert.equal(refreshForm.canonicalizationHistory().length, 0);

    const revalidateSource = createReadOnlyResourceLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      status: {
        kind: "fulfilled",
        operation: "initialLoad",
      },
      freshness: { kind: "stale", reason: "revalidationRequired" },
    });
    const revalidateForm = signals.form({
      source: signals.form.source.resourceLine(revalidateSource, { id: "task-resource-lifecycle-revalidate" }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action }) => ({
        revalidateResourceSource: action("revalidateResourceSource", {
          resourceAction: { kind: "revalidate" },
        }),
      }),
    });

    const revalidateExecution = revalidateForm.executeAction("revalidateResourceSource");
    assert.equal(revalidateExecution.resultKind, "fulfilled");
    assert.equal(revalidateExecution.resourceLifecycle.operation, "revalidate");
    assert.equal(revalidateExecution.resourceLifecycle.status.kind, "pending");
    assert.equal(revalidateExecution.resourceLifecycle.status.operation, "revalidate");
    assert.equal(revalidateForm.resourceSource().status.operation, "revalidate");
    assert.equal(revalidateForm.canonicalizationHistory().length, 0);
  });
});

test("signals.form lowers declared resource-line recovery actions into exact replay restore and rollback truth", async () => {
  await withSignals((signals) => {
    const source = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        status: "draft",
      },
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-recovery-actions" }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
      actions: ({ action }) => ({
        replayResourceSource: action("replayResourceSource", {
          resourceAction: { kind: "replayExact" },
        }),
        restoreResourceSource: action("restoreResourceSource", {
          resourceAction: { kind: "restoreExact" },
        }),
        rollbackResourceEffect: action("rollbackResourceEffect", {
          resourceAction: { kind: "rollbackLastEffect" },
        }),
      }),
    });

    form.fields.title.set("Local draft");
    const replayExecution = form.executeAction("replayResourceSource");
    assert.equal(replayExecution.resultKind, "fulfilled");
    assert.equal(replayExecution.effectStarted, true);
    assert.equal(replayExecution.resourceSubmission, null);
    assert.equal(replayExecution.resourceLifecycle, null);
    assert.equal(replayExecution.resourceRecovery.mode, "resourceReplayExact");
    assert.equal(replayExecution.resourceRecovery.resultKind, "replayed");
    assert.deepEqual(form.source(), {
      title: "Ship docs",
      status: "draft",
    });
    assert.deepEqual(form.draft(), { title: "Local draft" });
    assert.equal(form.canonicalizationHistory().length, 0);

    form.fields.title.set("Published docs");
    form.fields.status.set("review");
    const submitExecution = form.executeAction("submit");
    assert.equal(submitExecution.resultKind, "fulfilled");

    const rollbackExecution = form.executeAction("rollbackResourceEffect");
    assert.equal(rollbackExecution.resultKind, "fulfilled");
    assert.equal(rollbackExecution.resourceRecovery.mode, "resourceRollback");
    assert.equal(rollbackExecution.resourceRecovery.resultKind, "effectRejected");
    assert.deepEqual(form.draft(), {
      title: "Published docs",
      status: "review",
    });

    form.fields.title.set("Local draft after rollback");
    const restoreExecution = form.executeAction("restoreResourceSource");
    assert.equal(restoreExecution.resultKind, "fulfilled");
    assert.equal(restoreExecution.resourceRecovery.mode, "resourceRestoreExact");
    assert.equal(restoreExecution.resourceRecovery.resultKind, "restored");
    assert.deepEqual(form.source(), {
      title: "Ship docs",
      status: "draft",
    });
    assert.deepEqual(form.draft(), {
      title: "Local draft after rollback",
      status: "review",
    });
  });
});
