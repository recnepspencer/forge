import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";
import {
  createMutationResponsePlanFixture,
  createReadOnlyResourceLineFixture,
} from "./resource_line_fixture.mjs";

test("signals.form exposes resource-backed source readback and readiness blockers", async () => {
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
    assert.equal(report.effectProfile.profile.name, "branchNative");
    assert.equal(typeof report.effectProfile.closeoutMatrixDigest, "string");
    assert.equal(report.rollback, null);
    assert.equal(report.visibleSelection.kind, "unavailable");
    assert.equal(report.history.branch?.id, 7);
    assert.equal(report.history.availability.restoreExact.kind, "available");
    assert.equal(typeof report.verification.packageDigest, "string");
    assert.equal(typeof report.verification.mutationResponseCloseoutMatrixDigest, "string");
    assert.equal(report.mutationResponse.confirmationKind, "deliveryAwaited");
    assert.equal(report.mutationResponse.planCount, 3);
    assert.equal(report.mutationResponse.fallbackTargetCount, 1);
    assert.equal(typeof report.mutationResponse.digest, "string");
    assert.deepEqual(
      pendingForm.readiness().blockers.map((blocker) => blocker.kind),
      ["resource:pending", "resource:stale", "unchanged"],
    );
    assert.equal(pendingForm.diagnostics().resourceSource.digest, report.digest);
    assert.equal(pendingForm.verification().digests.resourceSourceDigest, report.digest);
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
      ["resource:rejected", "unchanged"],
    );
    assert.equal(rejectedForm.resourceSource().visibleSelection.kind, "committed");

    const plainForm = signals.form({
      source: { title: "Plain source" },
      fields: ({ field }) => ({ title: field("title") }),
    });
    assert.equal(plainForm.resourceSource(), null);
    assert.equal(plainForm.diagnostics().resourceSource, null);
    assert.equal(plainForm.verification().digests.resourceSourceDigest, null);
  } finally {
    await cleanup();
  }
});
