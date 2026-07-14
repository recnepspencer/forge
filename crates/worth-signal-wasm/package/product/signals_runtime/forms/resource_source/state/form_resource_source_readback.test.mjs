import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../../runtime_fixture/graph_operational_runtime.mjs";
import {
  createMutationResponsePlanFixture,
  createReadOnlyResourceLineFixture,
} from "../fixtures/resource_line_fixture.mjs";

test("signals.form exposes resource line source readback and readiness blockers", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const branchNativeProfile = signals.resource.effects.branchNative();
    const pendingForm = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          effectProfile: branchNativeProfile,
          status: { kind: "pending", operation: "initialLoad", continuity: "noVisibleValueYet" },
          freshness: { kind: "stale", reason: "initialLoadPending" },
          visibleSelection: {
            kind: "unavailable",
            source: "initialLoad",
            effectId: null,
            branchId: null,
            snapshotId: null,
            basisId: null,
            detail: "initial load has not materialized visible branch truth yet",
          },
          mutationResponse: createMutationResponsePlanFixture({
            confirmationKind: "deliveryAwaited",
            fallbackKind: "deliveryAwaited",
            planCount: 3,
          }),
        }),
        { id: "resource-task" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
    });

    const report = pendingForm.resourceSource();
    assert.equal(report.sourceKind, "resourceLine");
    assert.equal(report.request.method, "GET");
    assert.equal(report.status.kind, "pending");
    assert.equal(report.freshness.kind, "stale");
    assert.equal(report.shape.familyKind, "detail");
    assert.equal(report.shape.patchLowering, "detailFieldJsonPathRegion");
    assert.equal(report.externalCompatibility.kind, "native");
    assert.equal(report.externalCompatibility.deliveryContract, "nativeInternalLine");
    assert.equal(report.effectProfile.profile.name, "branchNative");
    assert.equal(typeof report.effectProfile.closeoutMatrixDigest, "string");
    assert.equal(report.rollback, null);
    assert.equal(report.visibleSelection.kind, "unavailable");
    assert.equal(report.visibleSelection.branchProof.admitted, false);
    assert.equal(report.visibleSelection.rebaseProof.admitted, false);
    assert.equal(
      report.visibleSelection.branchProof.reason,
      "resource line visible selection does not carry admitted native branch-visible proof",
    );
    assert.equal(
      report.visibleSelection.rebaseProof.reason,
      "resource line visible selection does not carry admitted merge/rebase-visible proof",
    );
    assert.equal(report.history.branch?.id, 7);
    assert.equal(report.history.availability.restoreExact.kind, "available");
    assert.equal(typeof report.verification.packageDigest, "string");
    assert.equal(typeof report.verification.mutationResponseCloseoutMatrixDigest, "string");
    assert.equal(report.mutationResponse.confirmationKind, "deliveryAwaited");
    assert.equal(report.mutationResponse.planCount, 3);
    assert.equal(report.mutationResponse.fallbackTargetCount, 1);
    assert.equal(typeof report.mutationResponse.digest, "string");
    assert.equal(report.lifecycle.retry.kind, "notNeeded");
    assert.equal(report.lifecycle.supersession.kind, "none");
    assert.equal(report.lifecycle.deliveryBasis.kind, "stable");
    assert.equal(typeof report.lifecycle.digest, "string");
    assert.deepEqual(
      pendingForm.readiness().blockers.map((blocker) => blocker.kind),
      ["resource:pending", "resource:stale", "resource:actionUnavailable", "unchanged"],
    );
    assert.equal(pendingForm.diagnostics().resourceSource.digest, report.digest);
    assert.equal(pendingForm.diagnostics().resourceSource.shape.digest, report.shape.digest);
    assert.equal(pendingForm.diagnostics().resourceSource.lifecycle.digest, report.lifecycle.digest);
    assert.equal(pendingForm.verification().digests.resourceSourceDigest, report.digest);
    assert.equal(pendingForm.verification().digests.resourceShapeDigest, report.shape.digest);
    assert.equal(pendingForm.verification().digests.resourceLifecycleDigest, report.lifecycle.digest);
    assert.equal(
      pendingForm.verification().digests.resourceExternalCompatibilityDigest,
      report.externalCompatibility.digest,
    );
    assert.equal(typeof pendingForm.verification().digests.resourceEffectProfileDigest, "string");
    assert.equal(typeof pendingForm.verification().digests.resourceVisibleBranchSelectionDigest, "string");
    assert.equal(
      pendingForm.verification().digests.resourceVerificationPackageDigest,
      report.verification.packageDigest,
    );
    assert.equal(
      pendingForm.verification().digests.resourceEffectCloseoutMatrixDigest,
      report.effectProfile.closeoutMatrixDigest,
    );
    assert.equal(
      pendingForm.verification().digests.resourceMutationResponseDigest,
      report.mutationResponse.digest,
    );
    assert.equal(
      pendingForm.verification().digests.resourceMutationResponseConfirmationDigest,
      report.mutationResponse.confirmationDigest,
    );
    assert.equal(
      pendingForm.verification().digests.resourceMutationResponseTargetOutcomeDigest,
      report.mutationResponse.targetOutcomeDigest,
    );
    assert.equal(
      pendingForm.verification().digests.resourceMutationResponseCloseoutMatrixDigest,
      report.verification.mutationResponseCloseoutMatrixDigest,
    );
    assert.equal(
      pendingForm.verification().performanceEnvelope.resourceSource.costBasis,
      "resourceLineProofRead",
    );

    const externalCompatibility = Object.freeze({
      kind: "externalDefinition",
      version: "worth-resource-external-v1",
      definitionId: "tasks.external.search",
      requestContract: "native-v1",
      reconciliationContract: "paged-v1",
    });
    const externalForm = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: { kind: "fulfilled", operation: "initialLoad" },
          freshness: { kind: "fresh" },
          familyKind: "paged",
          familyId: "task-search",
          runtimeLineId: "task:page:1",
          canonicalKey: "query=task&page=1",
          compatibility: externalCompatibility,
        }),
        { id: "resource-task-external-compatibility" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
    });
    assert.equal(externalForm.resourceSource().externalCompatibility.kind, "externalDefinition");
    assert.equal(
      externalForm.resourceSource().externalCompatibility.definitionId,
      "tasks.external.search",
    );
    assert.equal(
      externalForm.resourceSource().externalCompatibility.reconciliationContract,
      "paged-v1",
    );
    assert.equal(
      externalForm.resourceSource().externalCompatibility.deliveryContract,
      "basisCompatV1",
    );
    assert.equal(
      externalForm.verification().digests.resourceExternalCompatibilityDigest,
      externalForm.resourceSource().externalCompatibility.digest,
    );
    assert.equal(
      externalForm.diagnostics().resourceSource.externalCompatibility.digest,
      externalForm.resourceSource().externalCompatibility.digest,
    );

    const rejectedForm = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: {
            kind: "rejected",
            operation: "refresh",
            message: "network down",
            continuity: "preservedVisibleValue",
          },
          freshness: { kind: "fresh" },
        }),
        { id: "resource-task-rejected" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
    });
    assert.deepEqual(
      rejectedForm.readiness().blockers.map((blocker) => blocker.kind),
      ["resource:rejected", "resource:actionUnavailable", "unchanged"],
    );
    assert.equal(rejectedForm.resourceSource().lifecycle.retry.kind, "recommended");
    assert.equal(rejectedForm.resourceSource().lifecycle.retry.operation, "refresh");

    const driftBaseLine = createReadOnlyResourceLineFixture({
      status: { kind: "fulfilled", operation: "delivery" },
      freshness: { kind: "stale", reason: "deliveryInvalidate" },
    });
    const driftLine = Object.freeze({
      ...driftBaseLine,
      summary() {
        const summary = driftBaseLine.summary();
        return Object.freeze({
          ...summary,
          current: Object.freeze({
            ...summary.current,
            freshness: Object.freeze({ kind: "stale", reason: "deliveryInvalidate" }),
          }),
          diagnostics: Object.freeze({
            ...summary.diagnostics,
            current: Object.freeze({
              ...summary.diagnostics.current,
              freshness: Object.freeze({ kind: "stale", reason: "deliveryInvalidate" }),
            }),
            counts: Object.freeze({
              ...summary.diagnostics.counts,
              retryAttemptCount: 2,
              supersessionCount: 1,
              deliveryCount: 3,
            }),
            latest: Object.freeze({
              ...summary.diagnostics.latest,
              basisCurrentId: "basis-2",
              supersededOperation: "refresh",
              deliveryKind: "basisRefresh",
              deliveryScope: "basis",
              deliveryBasisId: "basis-2",
              invalidationCause: "deliveryInvalidate",
              invalidationScope: "line",
            }),
          }),
        });
      },
      freshness() {
        return Object.freeze({ kind: "stale", reason: "deliveryInvalidate" });
      },
    });
    const driftForm = signals.form({
      source: signals.form.source.resourceLine(driftLine, { id: "resource-task-delivery-drift" }),
      fields: ({ field }) => ({ title: field("title") }),
    });
    assert.equal(driftForm.resourceSource().lifecycle.deliveryBasis.kind, "drifted");
    assert.equal(driftForm.resourceSource().lifecycle.supersession.kind, "observed");
    assert.equal(driftForm.resourceSource().lifecycle.supersession.lastOperation, "refresh");
    assert.equal(driftForm.resourceSource().lifecycle.counts.retryAttemptCount, 2);
    assert.deepEqual(
      driftForm.readiness().blockers.map((blocker) => blocker.kind),
      ["resource:deliveryBasisDrift", "resource:actionUnavailable", "unchanged"],
    );
    assert.equal(
      driftForm.verification().digests.resourceLifecycleDigest,
      driftForm.resourceSource().lifecycle.digest,
    );
    assert.equal(
      driftForm.diagnostics().resourceSource.lifecycle.digest,
      driftForm.resourceSource().lifecycle.digest,
    );

    const collectionForm = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: { kind: "fulfilled", operation: "initialLoad" },
          freshness: { kind: "fresh" },
          familyKind: "collection",
          familyId: "task-list",
          runtimeLineId: "task:list",
          canonicalKey: "workspace=current",
        }),
        { id: "resource-task-collection" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
    });
    assert.equal(collectionForm.resourceSource().shape.familyKind, "collection");
    assert.equal(
      collectionForm.resourceSource().shape.patchLowering,
      "collectionMembershipItemFieldJsonPathRegion",
    );
    assert.equal(
      collectionForm.verification().digests.resourceShapeDigest,
      collectionForm.resourceSource().shape.digest,
    );

    const pagedForm = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: { kind: "fulfilled", operation: "initialLoad" },
          freshness: { kind: "fresh" },
          familyKind: "paged",
          familyId: "task-search",
          runtimeLineId: "task:page:1",
          canonicalKey: "query=task&page=1",
        }),
        { id: "resource-task-paged" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
    });
    assert.equal(pagedForm.resourceSource().shape.familyKind, "paged");
    assert.equal(
      pagedForm.resourceSource().shape.patchLowering,
      "pagedWindowMembershipItemFieldJsonPathRegion",
    );
    assert.equal(
      pagedForm.diagnostics().resourceSource.shape.digest,
      pagedForm.resourceSource().shape.digest,
    );

    const plainForm = signals.form({
      source: { title: "Plain source" },
      fields: ({ field }) => ({ title: field("title") }),
    });
    assert.equal(plainForm.resourceSource(), null);
    assert.equal(plainForm.diagnostics().resourceSource, null);
    assert.equal(plainForm.verification().digests.resourceSourceDigest, null);
    assert.equal(plainForm.verification().digests.resourceShapeDigest, null);
  } finally {
    await cleanup();
  }
});
