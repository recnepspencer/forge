import assert from "node:assert/strict";
import test from "node:test";

import { createDeferred } from "../../../runtime_fixture/async/deferred.mjs";
import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("save response identity migration rewrites a resident detail line to canonical identity", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()({ title: "title" }))
      .detail({
        load: ({ taskId }) => ({
          id: taskId,
          title: "Draft",
        }),
      });
    const draftLine = taskRead.line({ taskId: "tmp-1" });
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
    const firstRuntimeLineId = draftLine.descriptor().runtimeLineId;

    const plan = createTask.line({
      body: {
        id: "tmp-1",
        title: "Draft",
      },
    }).mutationResponse();
    const canonicalLine = taskRead.line({ taskId: "task:tmp-1" });

    assert.equal(plan.identityMigration.submittedIdentity, "tmp-1");
    assert.equal(plan.identityMigration.responseIdentity, "task:tmp-1");
    assert.equal(plan.identityMigration.canonicalIdentity, "task:tmp-1");
    assert.equal(plan.identityMigration.migrationNeeded, true);
    assert.deepEqual(plan.identityMigration.fallbackKinds, []);
    assert.equal(plan.identityMigration.fallbackDigest, "mutation-response-identity-fallbacks|none");
    assert.equal(plan.identityMigration.exactTargetCount, 1);
    assert.equal(plan.identityMigration.targetCount, 1);
    assert.equal(plan.identityMigration.targets[0].targetId, "migrationTarget1");
    assert.equal(
      plan.identityMigration.targets[0].line.canonicalKey,
      canonicalLine.descriptor().canonicalParams.canonicalKey,
    );
    assert.equal(plan.identityMigration.targets[0].staleness, null);
    assert.equal(plan.identityMigration.targets[0].outcome, "exactResidentLine");
    assert.equal(plan.identityMigration.targets[0].execution.kind, "exactResidentLine");
    assert.equal(plan.identityMigration.targets[0].execution.outcomeKind, "applied");
    assert.equal(
      plan.identityMigration.targets[0].execution.previousCanonicalKey,
      "/tasks/tmp-1",
    );
    assert.equal(
      plan.identityMigration.targets[0].execution.nextCanonicalKey,
      "/tasks/task%3Atmp-1",
    );
    assert.equal(
      plan.identityMigration.targets[0].execution.requestPath,
      "/tasks/task%3Atmp-1",
    );
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.deepEqual(plan.confirmation.fallbackKinds, []);
    assert.equal(plan.confirmation.exactTargetCount, 1);
    assert.equal(plan.counters.identityResponseExtractionBreadth, 1);
    assert.equal(plan.counters.identityMigrationTargetFanoutBreadth, 1);
    assert.equal(plan.counters.identityMigrationStaleDenialBreadth, 0);
    assert.equal(plan.counters.identityMigrationExecutionBreadth, 1);
    assert.equal(
      plan.identityMigration.counters.requestDescriptorRewriteBreadth,
      1,
    );
    assert.equal(
      plan.identityMigration.counters.exactTargetCount,
      1,
    );
    assert.equal(canonicalLine, draftLine);
    assert.equal(
      draftLine.descriptor().canonicalParams.canonicalKey,
      "/tasks/task%3Atmp-1",
    );
    assert.notEqual(draftLine.descriptor().runtimeLineId, firstRuntimeLineId);
    assert.equal(
      draftLine.descriptor().runtimeLineId,
      plan.identityMigration.targets[0].execution.nextRuntimeLineId,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("save response identity migration falls back when the canonical destination is already resident", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()({ title: "title" }))
      .detail({
        load: ({ taskId }) => ({
          id: taskId,
          title: "Draft",
        }),
      });
    const draftLine = taskRead.line({ taskId: "tmp-1" });
    const canonicalLine = taskRead.line({ taskId: "task:tmp-1" });
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

    const plan = createTask.line({
      body: {
        id: "tmp-1",
        title: "Draft",
      },
    }).mutationResponse();

    assert.equal(plan.identityMigration.migrationNeeded, true);
    assert.equal(plan.identityMigration.exactTargetCount, 0);
    assert.deepEqual(plan.identityMigration.fallbackKinds, [
      "identityMigrationUnavailable",
    ]);
    assert.equal(plan.identityMigration.targets[0].outcome, "fallback");
    assert.equal(plan.identityMigration.targets[0].execution.kind, "fallback");
    assert.match(
      plan.identityMigration.targets[0].detail,
      /canonical destination is already resident/,
    );
    assert.equal(plan.confirmation.kind, "partialCanonicalTruth");
    assert.equal(plan.confirmation.exactTargetCount, 0);
    assert.equal(plan.counters.identityMigrationExecutionBreadth, 0);
    assert.equal(
      plan.identityMigration.counters.requestDescriptorRewriteBreadth,
      0,
    );
    assert.equal(
      draftLine.descriptor().canonicalParams.canonicalKey,
      "/tasks/tmp-1",
    );
    assert.equal(
      canonicalLine.descriptor().canonicalParams.canonicalKey,
      "/tasks/task%3Atmp-1",
    );
  } finally {
    await runtime.cleanup();
  }
});

test("pending save response identity migration preserves stale target denial posture", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const deferred = createDeferred();
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()({ title: "title" }))
      .detail({
        load: ({ taskId }) => ({
          id: taskId,
          title: "Draft",
        }),
      });
    const draftLine = taskRead.line({ taskId: "tmp-1" });
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
        load: () => deferred.promise,
      });

    const createLine = createTask.line({
      body: {
        id: "tmp-1",
        title: "Draft",
      },
    });
    draftLine.patch(taskRead.patch.field({
      field: "title",
      value: "Local draft",
    }));
    deferred.resolve({
      id: "task:tmp-1",
      title: "Draft",
    });
    await deferred.promise;
    await Promise.resolve();
    const plan = createLine.mutationResponse();

    assert.equal(draftLine.value().title, "Local draft");
    assert.equal(plan.identityMigration.migrationNeeded, true);
    assert.equal(
      plan.identityMigration.targets[0].staleness.reason,
      "visibleValueVersionChanged",
    );
    assert.equal(
      plan.identityMigration.targets[0].submittedTarget.visibleValueVersion,
      1,
    );
    assert.equal(
      plan.identityMigration.targets[0].staleness.currentVisibleValueVersion,
      2,
    );
    assert.equal(plan.identityMigration.targets[0].outcome, "fallback");
    assert.equal(plan.confirmation.kind, "partialCanonicalTruth");
    assert.deepEqual(plan.confirmation.fallbackKinds, [
      "identityMigrationUnavailable",
    ]);
    assert.equal(plan.counters.identityMigrationStaleDenialBreadth, 1);
  } finally {
    await runtime.cleanup();
  }
});

test("identity migration does not fabricate a response identity when only canonical identity is declared", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .detail({
        load: ({ taskId }) => ({
          id: taskId,
          title: "Draft",
        }),
      });
    const createTask = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        identity: {
          submitted: ({ body }) => body.id,
          canonical: (value) => value.id,
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

    const plan = createTask.line({
      body: {
        id: "tmp-2",
        title: "Draft",
      },
    }).mutationResponse();

    assert.equal(plan.identityMigration.responseIdentity, null);
    assert.equal(
      plan.identityMigration.responseIdentityDigest,
      "mutation-response-identity-response|none",
    );
    assert.equal(plan.identityMigration.canonicalIdentity, "task:tmp-2");
    assert.equal(
      plan.identityMigration.counters.responseIdentityExtractionBreadth,
      0,
    );
    assert.equal(plan.counters.identityResponseExtractionBreadth, 0);
  } finally {
    await runtime.cleanup();
  }
});
