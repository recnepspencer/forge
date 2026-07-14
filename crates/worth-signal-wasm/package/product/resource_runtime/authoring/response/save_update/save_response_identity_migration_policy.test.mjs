import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("identity migration keeps mixed exact and fallback targets unavailable by default all-or-none policy", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .detail({ load: ({ taskId }) => ({ id: taskId, title: "Draft" }) });
    const auditRead = runtime.signals.api({}).url("/task-audit/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .detail({ load: ({ taskId }) => ({ id: taskId, events: 0 }) });
    taskRead.line({ taskId: "tmp-3" });
    const plan = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          targets: [{
            family: taskRead,
            params: ({ body }) => ({ taskId: body.id }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({ taskId: canonicalIdentity }),
            fallback: "identityMigrationUnavailable",
          }, {
            family: auditRead,
            params: ({ body }) => ({ taskId: body.id }),
            fallback: "partialReconciliation",
          }],
        },
        load: ({ body }) => ({ id: `task:${body.id}`, title: body.title }),
      })
      .line({ body: { id: "tmp-3", title: "Draft" } })
      .mutationResponse();

    assert.equal(plan.identityMigration.atomicity, "allOrNone");
    assert.equal(plan.identityMigration.partialAdmission, "denied");
    assert.equal(plan.identityMigration.exactTargetCount, 0);
    assert.equal(plan.identityMigration.targets[0].outcome, "fallback");
    assert.match(plan.identityMigration.targets[0].detail, /identity\.atomicity=allOrNone/);
    assert.deepEqual(plan.identityMigration.fallbackKinds, [
      "identityMigrationUnavailable",
      "partialReconciliation",
    ]);
    assert.equal(plan.counters.identityMigrationExecutionBreadth, 0);
  } finally {
    await runtime.cleanup();
  }
});

test("identity migration admits mixed exact and fallback targets only when partial policy is declared", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .detail({ load: ({ taskId }) => ({ id: taskId, title: "Draft" }) });
    const auditRead = runtime.signals.api({}).url("/task-audit/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .detail({ load: ({ taskId }) => ({ id: taskId, events: 0 }) });
    taskRead.line({ taskId: "tmp-4" });
    const plan = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          atomicity: "partialAllowed",
          targets: [{
            family: taskRead,
            params: ({ body }) => ({ taskId: body.id }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({ taskId: canonicalIdentity }),
            fallback: "identityMigrationUnavailable",
          }, {
            family: auditRead,
            params: ({ body }) => ({ taskId: body.id }),
            fallback: "partialReconciliation",
          }],
        },
        load: ({ body }) => ({ id: `task:${body.id}`, title: body.title }),
      })
      .line({ body: { id: "tmp-4", title: "Draft" } })
      .mutationResponse();

    assert.equal(plan.identityMigration.atomicity, "partialAllowed");
    assert.equal(plan.identityMigration.partialAdmission, "admitted");
    assert.equal(plan.identityMigration.exactTargetCount, 1);
    assert.equal(plan.identityMigration.targets[0].outcome, "exactResidentLine");
    assert.equal(plan.identityMigration.targets[1].outcome, "fallback");
    assert.deepEqual(plan.identityMigration.fallbackKinds, ["partialReconciliation"]);
    assert.equal(plan.confirmation.kind, "partialCanonicalTruth");
    assert.equal(plan.counters.identityMigrationExecutionBreadth, 1);
  } finally {
    await runtime.cleanup();
  }
});

test("response-owned write planning denies malformed identity migration fallback declarations", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId").detail({
      load: ({ taskId }) => ({ id: taskId, title: "Draft" }),
    });
    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks")
          .response(runtime.signals.resource.response.detail()())
          .create({
            identity: {
              submitted: ({ body }) => body.id,
              canonical: (value) => value.id,
              targets: [{
                family: taskRead,
                params: ({ body }) => ({ taskId: body.id }),
                fallback: "unsupportedTarget",
              }],
            },
            load: ({ body }) => body,
          }),
      /identity\.targets\[0\] fallback must be one of identityMigrationUnavailable, refetchRequired, deliveryAwaited, partialReconciliation/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("response-owned write planning denies malformed identity migration atomicity declarations", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks")
          .response(runtime.signals.resource.response.detail()())
          .create({
            identity: {
              submitted: ({ body }) => body.id,
              canonical: (value) => value.id,
              atomicity: "sometimes",
            },
            load: ({ body }) => body,
          }),
      /identity\.atomicity must be one of allOrNone, partialAllowed/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("response-owned write planning denies summary identity targets on detail families", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId").detail({
      load: ({ taskId }) => ({ id: taskId, title: "Draft" }),
    });
    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks")
          .response(runtime.signals.resource.response.detail()())
          .create({
            identity: {
              submitted: ({ body }) => body.id,
              canonical: (value) => value.id,
              targets: [{
                family: taskRead,
                params: ({ body }) => ({ taskId: body.id }),
                fallback: "identityMigrationUnavailable",
                summary: {
                  kind: "summary",
                  summary: "total",
                },
              }],
            },
            load: ({ body }) => body,
          }),
      /summary targets require a collection or paged family/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("response-owned write planning denies canonicalParams on detail-child identity targets before exact child rewrite support lands", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId").detail({
      load: ({ taskId }) => ({ id: taskId, title: "Draft" }),
    });
    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks")
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
                fallback: "refetchRequired",
                detailChild: {
                  kind: "detailChild",
                  region: "children",
                },
              }],
            },
            load: ({ body }) => body,
          }),
      /detailChild targets do not admit canonicalParams/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("response-owned write planning admits combining exact reconciliation with exact identity migration", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()({ title: "title" }))
      .detail({ load: ({ taskId }) => ({ id: taskId, title: "Draft" }) });
    taskRead.line({ taskId: "tmp-1" });
    const saveTask = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .update({
        reconciles: [{
          family: taskRead,
          params: ({ taskId }) => ({ taskId }),
          fallback: "refetchRequired",
          detail: { kind: "replace" },
        }],
        identity: {
          submitted: ({ taskId }) => taskId,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          targets: [{
            family: taskRead,
            params: ({ taskId }) => ({ taskId }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({ taskId: canonicalIdentity }),
            fallback: "identityMigrationUnavailable",
          }],
        },
        load: ({ taskId, body }) => ({ id: body.id, title: `${taskId}:${body.title}` }),
      });

    const plan = saveTask.line({
        taskId: "tmp-1",
        body: { id: "task:tmp-1", title: "Updated" },
      }).mutationResponse();

    assert.equal(plan.executionArtifacts[0].kind, "exactDetail");
    assert.equal(plan.identityMigration.exactTargetCount, 1);
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.equal(plan.confirmation.exactTargetCount, 2);
    assert.equal(plan.counters.executionBreadth, 1);
    assert.equal(plan.counters.identityMigrationExecutionBreadth, 1);
  } finally {
    await runtime.cleanup();
  }
});

test("identity migration denies sibling targets that claim the same canonical destination before any resident line mutates", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId").detail({
      load: ({ taskId }) => ({ id: taskId, title: "Draft" }),
    });
    const firstDraftLine = taskRead.line({ taskId: "tmp-conflict-1" });
    const secondDraftLine = taskRead.line({ taskId: "alias-conflict-1" });
    const plan = runtime.signals.api({}).url("/tasks/merge")
      .response(runtime.signals.resource.response.detail()())
      .create({
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          targets: [{
            family: taskRead,
            params: ({ body }) => ({ taskId: body.id }),
            canonicalParams: () => ({ taskId: "task:shared-conflict" }),
            fallback: "identityMigrationUnavailable",
          }, {
            family: taskRead,
            params: ({ body }) => ({ taskId: body.aliasId }),
            canonicalParams: () => ({ taskId: "task:shared-conflict" }),
            fallback: "identityMigrationUnavailable",
          }],
        },
        load: ({ body }) => ({ id: `task:${body.id}`, title: body.title }),
      })
      .line({
        body: {
          id: "tmp-conflict-1",
          aliasId: "alias-conflict-1",
          title: "Draft",
        },
      })
      .mutationResponse();

    assert.equal(plan.identityMigration.exactTargetCount, 0);
    assert.deepEqual(plan.identityMigration.fallbackKinds, [
      "identityMigrationUnavailable",
      "identityMigrationUnavailable",
    ]);
    assert.equal(plan.identityMigration.partialAdmission, "notNeeded");
    assert.match(
      plan.identityMigration.targets[0].detail,
      /claim canonical destination/,
    );
    assert.equal(
      firstDraftLine.descriptor().canonicalParams.canonicalKey,
      "/tasks/tmp-conflict-1",
    );
    assert.equal(
      secondDraftLine.descriptor().canonicalParams.canonicalKey,
      "/tasks/alias-conflict-1",
    );
  } finally {
    await runtime.cleanup();
  }
});
