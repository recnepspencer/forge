import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form action plan digests include effect and action schema proof", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const createFormWithEffect = (hostEffect) => signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action }) => ({
        workflow: action("workflow", {
          patchPolicy: "allowEmpty",
          hostEffect,
          schema: { version: 1, effect: hostEffect },
        }),
      }),
    });

    const approvePlan = createFormWithEffect("workflow.approve").actionPlan("workflow");
    const rejectPlan = createFormWithEffect("workflow.reject").actionPlan("workflow");
    assert.notEqual(approvePlan.planDigest, rejectPlan.planDigest);
    assert.notEqual(approvePlan.proof.effectDigest, rejectPlan.proof.effectDigest);
    assert.notEqual(approvePlan.proof.actionSchemaDigest, rejectPlan.proof.actionSchemaDigest);
  } finally {
    await cleanup();
  }
});

test("signals.form verification package separates action planning from lifecycle history", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs", approved: false },
      fields: ({ field }) => ({
        title: field("title"),
        approved: field("approved"),
      }),
      admission: ({ action }) => ({
        approveAdmission: action("approve", "approval", ["approved"], () => ({
          posture: "requiresApproval",
          actorDigest: "actor:reviewer",
          policyDigest: "policy:approve",
        })),
      }),
      actions: ({ action }) => ({
        approve: action("approve", {
          patchPolicy: "allowEmpty",
          hostEffect: "workflow.approve",
          idempotency: "deny",
        }),
      }),
    });

    const beforeAttempt = form.verification();
    assert.equal(beforeAttempt.kind, "formVerification");
    assert.equal(
      beforeAttempt.digests.actionCatalogDigest,
      form.actions().digests.catalogDigest,
    );
    assert.equal(
      beforeAttempt.digests.actionReadinessAdmissionDigest,
      form.actions().digests.readinessAdmissionDigest,
    );
    assert.equal(beforeAttempt.digests.submitPlanDigest, form.actionPlan("submit").planDigest);
    assert.equal(beforeAttempt.actionHistory.attempts, 0);
    assert.equal(beforeAttempt.performanceEnvelope.actions.plans, 2);

    const deniedAttempt = form.attemptAction("approve");
    const afterAttempt = form.verification();
    assert.equal(deniedAttempt.resultKind, "denied");
    assert.equal(afterAttempt.actionHistory.attempts, 1);
    assert.equal(
      afterAttempt.digests.actionCatalogDigest,
      beforeAttempt.digests.actionCatalogDigest,
    );
    assert.equal(
      afterAttempt.digests.actionReadinessAdmissionDigest,
      beforeAttempt.digests.actionReadinessAdmissionDigest,
    );
    assert.notEqual(
      afterAttempt.digests.actionLifecycleDigest,
      beforeAttempt.digests.actionLifecycleDigest,
    );
    assert.notEqual(afterAttempt.packageDigest, beforeAttempt.packageDigest);
    assert.equal(form.diagnostics().verification.packageDigest, afterAttempt.packageDigest);
  } finally {
    await cleanup();
  }
});

test("signals.form verification package carries source compatibility history and counters", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const source = signals.input({ title: "Ship docs" });
    const schemaVersion = signals.input("v1");
    const form = signals.form({
      source: {
        value: source,
        schemaVersion,
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    form.fields.title.set("Client title");
    source.set({ title: "Server title" });
    schemaVersion.set("v2");

    const verification = form.verification();
    assert.equal(form.sourceCompatibility().posture, "unavailable");
    assert.equal(verification.sourceCompatibilityHistory.operations, 1);
    assert.equal(verification.performanceEnvelope.sourceCompatibilityOperations, 1);
    assert.equal(verification.performanceEnvelope.sourceCompatibility.unavailableDrifts, 1);
    assert.equal(typeof verification.digests.sourceCompatibilityDigest, "string");
    assert.equal(typeof verification.digests.sourceCompatibilityHistoryDigest, "string");
  } finally {
    await cleanup();
  }
});

test("signals.form verification package carries source admission and draft restore digests", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: {
        value: { title: "Ship docs" },
        sourceAdmission: {
          status: "ready",
          reason: "source admission is settled",
        },
        draftRestore: {
          status: "pending",
          reason: "draft restore is replaying local edits",
          token: "draft-restore-1",
        },
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      presentation: {
        entry: {
          bootstrap: {
            sourceAdmission: true,
            draftRestore: true,
          },
        },
      },
    });

    const verification = form.verification();
    assert.equal(typeof verification.digests.sourceAdmissionDigest, "string");
    assert.equal(typeof verification.digests.draftRestoreDigest, "string");
  } finally {
    await cleanup();
  }
});

test("signals.form verification package carries host fact digests and counters", async () => {
  const loaded = await loadSignalsModule();
  const {
    cleanup,
    hostCapabilityPlan,
    onlineCapability,
    wrapSignals,
  } = loaded;
  try {
    const state = {
      online: true,
      credentialsAvailable: true,
    };
    const signals = wrapSignals(createGraphOperationalRuntime(), {
      hostCapabilities: hostCapabilityPlan({
        online: onlineCapability({
          source: {
            current() {
              return state.online;
            },
            subscribe() {
              return () => {};
            },
          },
        }),
      }),
    });
    const form = signals.form({
      source: { title: "Ship docs" },
      host: {
        online: signals.host.online,
        credentials: () => state.credentialsAvailable,
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ submit }) => ({
        submit: submit({
          hostRequirements: ["online", "credentials"],
        }),
      }),
    });

    form.fields.title.set("Ship docs now");
    const verification = form.verification();
    assert.equal(verification.digests.hostFactDigest, form.host().digest);
    assert.equal(verification.digests.interactionDigest, form.interaction().digest);
    assert.equal(verification.digests.layoutDigest, form.layout().digest);
    assert.equal(verification.digests.layoutMeasurementDigest, form.layoutMeasurement().digest);
    assert.equal(verification.digests.presentationDigest, form.presentation().digest);
    assert.equal(verification.interactionHistory.operations, 0);
    assert.equal(verification.performanceEnvelope.hostFacts.declaredFacts, 2);
    assert.equal(verification.performanceEnvelope.interaction.fields, 1);
    assert.equal(verification.performanceEnvelope.layout.fields, 1);
    assert.equal(verification.performanceEnvelope.layoutMeasurement.retainedSnapshots, 0);
    assert.equal(verification.performanceEnvelope.presentation.lanes, form.presentation().lanes.length);
    assert.equal(verification.performanceEnvelope.hostFacts.supportedFacts, 2);
    assert.equal(verification.performanceEnvelope.actions.hostRequiredPlans, 1);
  } finally {
    await cleanup();
  }
});

test("signals.form verification package carries collaboration digests and counters", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      collaboration: {
        mode: "fieldLease",
        actorId: "me",
        supportsPresence: true,
        supportsComments: true,
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      presentation: {
        collaboration: { scope: "wholeForm" },
      },
    });

    form.reportCollaboration({
      posture: "blocked",
      leasedFields: [{ field: "title", ownerId: "peer-1" }],
      presence: [{ actorId: "peer-1", status: "active" }],
      comments: [{ id: "comment-1", authorId: "peer-1", target: "title" }],
      reason: "peer-1 owns the title lease",
    });

    const verification = form.verification();
    assert.equal(verification.digests.collaborationDigest, form.collaboration().digest);
    assert.equal(verification.performanceEnvelope.collaboration.blockingFields, 1);
    assert.equal(verification.performanceEnvelope.collaboration.presenceActors, 1);
    assert.equal(verification.performanceEnvelope.collaboration.commentArtifacts, 1);
    assert.equal(form.diagnostics().collaboration.digest, form.collaboration().digest);
  } finally {
    await cleanup();
  }
});

test("signals.form verification package carries reset and rollback digests for resource canonicalization proof", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const {
      createDetailPatchLineFixture,
      createMutationResponsePlanFixture,
    } = await import("./resource_source/fixtures/resource_line_fixture.mjs");
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
      source: signals.form.source.resourceLine(source, { id: "verification-resource-submit" }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
    });

    form.fields.title.set("Published docs");
    const execution = form.executeAction("submit");
    const verification = form.verification();
    assert.equal(execution.resultKind, "fulfilled");
    assert.equal(typeof verification.digests.resetRollbackDigest, "string");
    assert.equal(typeof verification.digests.resourceMutationResponseDigest, "string");
    assert.equal(typeof verification.digests.resourceMutationResponseConfirmationDigest, "string");
    assert.equal(typeof verification.digests.resourceMutationResponseContractDigest, "string");
    assert.equal(typeof verification.digests.resourceMutationResponseTargetOutcomeDigest, "string");
    assert.equal(typeof verification.digests.mutationResponseReconciliationDigest, "string");
    assert.equal(
      verification.digests.resetRollbackDigest,
      form.verification().digests.resetRollbackDigest,
    );
    assert.equal(form.canonicalizationHistory()[0].resourceLine.rollback.kind, "compactInverseAvailable");
    assert.equal(
      verification.digests.resourceMutationResponseContractDigest,
      form.resourceSource().mutationResponse.contract.digest,
    );
    assert.equal(
      verification.digests.resourceMutationResponseDigest,
      form.resourceSource().mutationResponse.digest,
    );
    assert.equal(
      form.canonicalizationHistory()[0].resourceLine.mutationResponse.confirmationKind,
      "partialCanonicalTruth",
    );
  } finally {
    await cleanup();
  }
});
