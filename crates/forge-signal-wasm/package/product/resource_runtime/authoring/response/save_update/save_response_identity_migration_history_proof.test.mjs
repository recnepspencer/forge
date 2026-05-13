import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("identity-migrated target lines retain structured migration proof in history and verification reads", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .detail({
        load: ({ taskId }) => ({ id: taskId, title: "Draft" }),
      });
    const draftLine = taskRead.line({ taskId: "tmp-history-1" });
    const firstRuntimeLineId = draftLine.descriptor().runtimeLineId;
    const createTask = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          targets: [{
            family: taskRead,
            params: ({ body }) => ({ taskId: body.id }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              taskId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }],
        },
        load: ({ body }) => ({
          id: `task:${body.id}`,
          title: body.title,
        }),
      });

    const saveLine = createTask.line({
      body: { id: "tmp-history-1", title: "Draft" },
    });
    const plan = saveLine.mutationResponse();

    const diagnosticsLatest = saveLine.summary().diagnostics.latest;
    const migratedDiagnosticsLatest = draftLine.summary().diagnostics.latest;
    const identityMigrationEntry = draftLine.history().lifecycle.at(-1);
    const verification = draftLine.history().verificationPackage();
    const saveVerification = saveLine.history().verificationPackage();

    assert.equal(identityMigrationEntry.event, "identityMigrated");
    assert.deepEqual(identityMigrationEntry.identityMigration, {
      previousCanonicalKey: "/tasks/tmp-history-1",
      nextCanonicalKey: "/tasks/task%3Atmp-history-1",
      previousRuntimeLineId: firstRuntimeLineId,
      nextRuntimeLineId: draftLine.descriptor().runtimeLineId,
      basisId: null,
      requestPath: "/tasks/task%3Atmp-history-1",
    });
    assert.equal(verification.historyReplayRestore.lastLifecycleEvent, "identityMigrated");
    assert.equal(verification.historyReplayRestore.identityMigrationCount, 1);
    assert.deepEqual(
      verification.historyReplayRestore.latestIdentityMigration,
      identityMigrationEntry.identityMigration,
    );
    assert.equal(migratedDiagnosticsLatest.identityMigrationCount, 1);
    assert.deepEqual(
      migratedDiagnosticsLatest.lastIdentityMigration,
      identityMigrationEntry.identityMigration,
    );
    assert.deepEqual(
      verification.diagnostics.summary.latest.lastIdentityMigration,
      identityMigrationEntry.identityMigration,
    );
    assert.equal(
      diagnosticsLatest.mutationResponseIdentityMigrationDigest,
      plan.identityMigration.digest,
    );
    assert.equal(
      diagnosticsLatest.mutationResponseIdentityMigrationNeeded,
      true,
    );
    assert.equal(
      diagnosticsLatest.mutationResponseIdentityMigrationPartialAdmission,
      plan.identityMigration.partialAdmission,
    );
    assert.equal(
      diagnosticsLatest.mutationResponseIdentityMigrationTargetCount,
      plan.identityMigration.targetCount,
    );
    assert.equal(
      diagnosticsLatest.mutationResponseIdentityMigrationExactTargetCount,
      plan.identityMigration.exactTargetCount,
    );
    assert.equal(
      diagnosticsLatest.mutationResponseIdentityMigrationExecutionDigest,
      plan.identityMigration.executionDigest,
    );
    assert.equal(
      diagnosticsLatest.mutationResponseIdentityMigrationFallbackDigest,
      plan.identityMigration.fallbackDigest,
    );
    assert.deepEqual(
      saveVerification.diagnostics.summary.latest,
      diagnosticsLatest,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("identity-migrated target lines deny exact replay and exact restore with typed migration-specific posture", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .detail({
        load: ({ taskId }) => ({ id: taskId, title: "Draft" }),
      });
    const draftLine = taskRead.line({ taskId: "tmp-history-2" });
    runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          targets: [{
            family: taskRead,
            params: ({ body }) => ({ taskId: body.id }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              taskId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }],
        },
        load: ({ body }) => ({
          id: `task:${body.id}`,
          title: body.title,
        }),
      })
      .line({
        body: { id: "tmp-history-2", title: "Draft" },
      })
      .mutationResponse();

    const replayAvailability = draftLine.history().availability.replayExact;
    const restoreAvailability = draftLine.history().availability.restoreExact;
    const replayResult = draftLine.history().replayExact();
    const restoreResult = draftLine.history().restoreExact();
    const verification = draftLine.history().verificationPackage();

    assert.equal(replayAvailability.kind, "unavailable");
    assert.equal(replayAvailability.reason, "identityMigrationUnavailable");
    assert.match(replayAvailability.detail, /identity migration rewrote/);
    assert.equal(restoreAvailability.kind, "unavailable");
    assert.equal(restoreAvailability.reason, "identityMigrationUnavailable");
    assert.match(restoreAvailability.detail, /resident rematerialization/);
    assert.deepEqual(replayResult, {
      kind: "unavailable",
      reason: "identityMigrationUnavailable",
      detail: replayAvailability.detail,
      basisCurrentId: null,
      basisAdvanceCount: 0,
    });
    assert.deepEqual(restoreResult, {
      kind: "unavailable",
      reason: "identityMigrationUnavailable",
      detail: restoreAvailability.detail,
      basisCurrentId: null,
      basisAdvanceCount: 0,
    });
    assert.deepEqual(verification.typedDenials.replayExact, replayAvailability);
    assert.deepEqual(verification.typedDenials.restoreExact, restoreAvailability);
    assert.deepEqual(
      verification.historyReplayRestore.availability.replayExact,
      replayAvailability,
    );
    assert.deepEqual(
      verification.historyReplayRestore.availability.restoreExact,
      restoreAvailability,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("multi-target exact identity migration keeps write-line proof and both migrated line histories aligned", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId").detail({
      load: ({ taskId }) => ({ id: taskId, title: "Draft" }),
    });
    const taskList = runtime.signals.api({}).url("/task-lists/:listId").list({
      itemIdentity: (item) => item.id,
      load: ({ listId }) => [{ id: `${listId}:1`, title: "Draft item" }],
    });
    const draftDetailLine = taskRead.line({ taskId: "tmp-history-batch-1" });
    const draftCollectionLine = taskList.line({ listId: "tmp-history-batch-1" });
    const saveLine = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          targets: [{
            family: taskRead,
            params: ({ body }) => ({ taskId: body.id }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              taskId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }, {
            family: taskList,
            params: ({ body }) => ({ listId: body.id }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              listId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }],
        },
        load: ({ body }) => ({
          id: `task:${body.id}`,
          title: body.title,
        }),
      })
      .line({
        body: { id: "tmp-history-batch-1", title: "Draft" },
      });

    const plan = saveLine.mutationResponse();
    const saveDiagnosticsLatest = saveLine.summary().diagnostics.latest;
    const detailEntry = draftDetailLine.history().lifecycle.at(-1);
    const collectionEntry = draftCollectionLine.history().lifecycle.at(-1);

    assert.equal(plan.identityMigration.exactTargetCount, 2);
    assert.equal(plan.lifecycleProof.count, 2);
    assert.equal(
      plan.counters.identityMigrationLifecycleProofBreadth,
      2,
    );
    assert.equal(
      saveDiagnosticsLatest.mutationResponseIdentityMigrationExactTargetCount,
      2,
    );
    assert.equal(detailEntry.event, "identityMigrated");
    assert.equal(collectionEntry.event, "identityMigrated");
    assert.equal(
      draftDetailLine.history().verificationPackage().historyReplayRestore.identityMigrationCount,
      1,
    );
    assert.equal(
      draftCollectionLine.history().verificationPackage().historyReplayRestore.identityMigrationCount,
      1,
    );
    assert.equal(
      draftDetailLine.summary().diagnostics.latest.identityMigrationCount,
      1,
    );
    assert.equal(
      draftCollectionLine.summary().diagnostics.latest.identityMigrationCount,
      1,
    );
  } finally {
    await runtime.cleanup();
  }
});
