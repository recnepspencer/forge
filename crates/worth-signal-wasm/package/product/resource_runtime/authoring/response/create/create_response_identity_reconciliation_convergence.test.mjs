import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("create responses can insert a resident collection item while migrating a resident draft detail line", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [{ id: "task:existing", title: "Existing" }],
      });
    const draftDetail = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .detail({
        load: ({ taskId }) => ({ id: taskId, title: "Draft" }),
      });
    const draftLine = draftDetail.line({ taskId: "tmp-5" });
    const firstRuntimeLineId = draftLine.descriptor().runtimeLineId;
    const listLine = taskList.line({});

    const plan = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [{
          family: taskList,
          params: () => ({}),
          fallback: "placementUnavailable",
          collection: { kind: "insert", placement: "append" },
        }],
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          targets: [{
            family: draftDetail,
            params: ({ body }) => ({ taskId: body.id }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              taskId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }],
        },
        load: ({ body }) => ({ id: `task:${body.id}`, title: body.title }),
      })
      .line({
        body: { id: "tmp-5", title: "Created" },
      })
      .mutationResponse();
    const canonicalLine = draftDetail.line({ taskId: "task:tmp-5" });

    assert.deepEqual(listLine.value(), [
      { id: "task:existing", title: "Existing" },
      { id: "task:tmp-5", title: "Created" },
    ]);
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionInsert");
    assert.equal(plan.executionArtifacts[0].itemId, "task:tmp-5");
    assert.equal(plan.identityMigration.exactTargetCount, 1);
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.equal(plan.confirmation.exactTargetCount, 2);
    assert.equal(plan.counters.executionBreadth, 1);
    assert.equal(plan.counters.identityMigrationExecutionBreadth, 1);
    assert.equal(canonicalLine, draftLine);
    assert.equal(
      draftLine.descriptor().canonicalParams.canonicalKey,
      "/tasks/task%3Atmp-5",
    );
    assert.notEqual(draftLine.descriptor().runtimeLineId, firstRuntimeLineId);
    assert.equal(draftLine.history().lifecycle.at(-2)?.event, "materialized");
    assert.equal(draftLine.history().lifecycle.at(-1)?.event, "identityMigrated");
  } finally {
    await runtime.cleanup();
  }
});

test("create responses can replace a resident draft detail line before migrating it to canonical identity", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskDetail = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .detail({
        load: ({ taskId }) => ({ id: taskId, title: "Draft" }),
      });
    const draftLine = taskDetail.line({ taskId: "tmp-6" });

    const plan = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [{
          family: taskDetail,
          params: ({ body }) => ({ taskId: body.id }),
          fallback: "refetchRequired",
          detail: { kind: "replace" },
        }],
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          targets: [{
            family: taskDetail,
            params: ({ body }) => ({ taskId: body.id }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              taskId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }],
        },
        load: ({ body }) => ({ id: `task:${body.id}`, title: `${body.title}:canonical` }),
      })
      .line({
        body: { id: "tmp-6", title: "Created" },
      })
      .mutationResponse();

    assert.deepEqual(draftLine.value(), {
      id: "task:tmp-6",
      title: "Created:canonical",
    });
    assert.equal(plan.executionArtifacts[0].kind, "exactDetail");
    assert.equal(plan.executionArtifacts[0].deliveryKind, "replace");
    assert.equal(plan.identityMigration.exactTargetCount, 1);
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.equal(plan.confirmation.exactTargetCount, 2);
    assert.equal(draftLine.history().lifecycle.at(-2)?.event, "delivered");
    assert.equal(draftLine.history().lifecycle.at(-1)?.event, "identityMigrated");
  } finally {
    await runtime.cleanup();
  }
});

test("create responses preserve exact reconciliation when identity migration falls back", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [],
      });
    const taskDetail = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .detail({
        load: ({ taskId }) => ({ id: taskId, title: "Draft" }),
      });
    const draftLine = taskDetail.line({ taskId: "tmp-7" });
    taskDetail.line({ taskId: "task:tmp-7" });
    const listLine = taskList.line({});

    const plan = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [{
          family: taskList,
          params: () => ({}),
          fallback: "placementUnavailable",
          collection: { kind: "insert", placement: "append" },
        }],
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          atomicity: "partialAllowed",
          targets: [{
            family: taskDetail,
            params: ({ body }) => ({ taskId: body.id }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              taskId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }],
        },
        load: ({ body }) => ({ id: `task:${body.id}`, title: body.title }),
      })
      .line({
        body: { id: "tmp-7", title: "Created" },
      })
      .mutationResponse();

    assert.deepEqual(listLine.value(), [{ id: "task:tmp-7", title: "Created" }]);
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionInsert");
    assert.equal(plan.identityMigration.targets[0].outcome, "fallback");
    assert.equal(plan.identityMigration.targets[0].execution.kind, "fallback");
    assert.deepEqual(plan.identityMigration.fallbackKinds, [
      "identityMigrationUnavailable",
    ]);
    assert.equal(plan.confirmation.kind, "partialCanonicalTruth");
    assert.equal(plan.confirmation.exactTargetCount, 1);
    assert.deepEqual(plan.confirmation.fallbackKinds, [
      "identityMigrationUnavailable",
    ]);
    assert.equal(
      draftLine.descriptor().canonicalParams.canonicalKey,
      "/tasks/tmp-7",
    );
  } finally {
    await runtime.cleanup();
  }
});

test("create responses can rewrite a resident detail child region from canonical response truth", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRegions = runtime.signals.resource.detailRegions({
      children: {
        read: (value) => value.children,
        write: (value, children) => ({ ...value, children }),
        identityBoundary: "inside",
        mergeGranularity: "child-list",
        cost: {
          traversalBreadth: 1,
          reconstructionBreadth: 1,
        },
      },
    });
    const taskDetail = runtime.signals.api({}).url("/tasks/:taskId").detail({
        reconcile: taskRegions,
        load: ({ taskId }) => ({
          id: taskId,
          title: "Task",
          children: [{ id: "tmp-child-8", title: "Draft child" }],
        }),
      });
    const taskLine = taskDetail.line({ taskId: "task-8" });

    const plan = runtime.signals.api({}).url("/tasks/:taskId/children")
      .response(runtime.signals.resource.response.detailRegions()(taskRegions))
      .create({
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.children.at(-1)?.id ?? value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          targets: [{
            family: taskDetail,
            params: ({ taskId }) => ({ taskId }),
            fallback: "identityMigrationUnavailable",
            detailChild: {
              kind: "detailChild",
              region: "children",
            },
          }],
        },
        load: ({ taskId, body }) => ({
          id: taskId,
          title: "Task",
          children: [{ id: `child:${body.id}`, title: body.title }],
        }),
      })
      .line({
        taskId: "task-8",
        body: { id: "tmp-child-8", title: "Draft child" },
      })
      .mutationResponse();

    assert.deepEqual(taskLine.value().children, [{
      id: "child:tmp-child-8",
      title: "Draft child",
    }]);
    assert.equal(plan.identityMigration.exactTargetCount, 1);
    assert.equal(plan.identityMigration.targets[0].outcome, "exactDetailChildRegion");
    assert.equal(plan.identityMigration.targets[0].execution.kind, "exactDetailChildRegion");
    assert.equal(plan.identityMigration.targets[0].execution.region, "children");
    assert.equal(plan.identityMigration.targets[0].execution.outcomeKind, "applied");
    assert.ok(plan.identityMigration.targets[0].execution.effectId);
    assert.ok(plan.identityMigration.targets[0].execution.effectProof);
    assert.equal(plan.lifecycleProof.entries[0].entryKind, "identityMigration");
    assert.equal(plan.lifecycleProof.entries[0].rollback.kind, "notApplicable");
    assert.equal(plan.lifecycleProof.entries[0].mergeRebase.kind, "unavailable");
    assert.equal(plan.identityMigration.counters.requestDescriptorRewriteBreadth, 0);
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.equal(taskLine.diagnostics().lastPatchedRegion, "children");
    assert.equal(taskLine.history().lifecycle.at(-1)?.event, "delivered");
  } finally {
    await runtime.cleanup();
  }
});
