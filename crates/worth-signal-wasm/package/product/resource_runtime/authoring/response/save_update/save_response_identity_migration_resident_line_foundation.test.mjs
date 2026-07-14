import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("save response identity migration rewrites a resident collection line to canonical identity", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/task-lists/:listId").list({
      itemIdentity: (item) => item.id,
      load: ({ listId }) => [{ id: `${listId}:1`, title: "Draft item" }],
    });
    const draftLine = taskList.line({ listId: "tmp-list-1" });
    const firstRuntimeLineId = draftLine.descriptor().runtimeLineId;
    const createList = runtime.signals.api({}).url("/task-lists")
      .response(runtime.signals.resource.response.detail()())
      .create({
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          targets: [{
            family: taskList,
            params: ({ body }) => ({ listId: body.id }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              listId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }],
        },
        load: ({ body }) => ({
          id: `list:${body.id}`,
          title: body.title,
        }),
      });

    const plan = createList.line({
      body: { id: "tmp-list-1", title: "Draft list" },
    }).mutationResponse();
    const canonicalLine = taskList.line({ listId: "list:tmp-list-1" });

    assert.equal(plan.identityMigration.exactTargetCount, 1);
    assert.equal(plan.identityMigration.targets[0].line.familyKind, "collection");
    assert.equal(plan.identityMigration.targets[0].outcome, "exactResidentLine");
    assert.equal(plan.identityMigration.targets[0].execution.kind, "exactResidentLine");
    assert.equal(
      plan.identityMigration.targets[0].execution.previousCanonicalKey,
      "/task-lists/tmp-list-1",
    );
    assert.equal(
      plan.identityMigration.targets[0].execution.nextCanonicalKey,
      "/task-lists/list%3Atmp-list-1",
    );
    assert.equal(
      plan.identityMigration.targets[0].execution.requestPath,
      "/task-lists/list%3Atmp-list-1",
    );
    assert.equal(draftLine, canonicalLine);
    assert.equal(
      draftLine.descriptor().canonicalParams.canonicalKey,
      "/task-lists/list%3Atmp-list-1",
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

test("save response identity migration rewrites a resident paged line to canonical identity", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const feed = runtime.signals.api({}).url("/feeds/:feedId").paged({
      itemIdentity: (item) => item.id,
      accumulatePage: (existing, next) => [...existing, ...next],
      load: ({ feedId }) => [{ id: `${feedId}:1`, title: "Draft item" }],
    });
    const draftLine = feed.line({ feedId: "tmp-feed-1" });
    const firstRuntimeLineId = draftLine.descriptor().runtimeLineId;
    const createFeed = runtime.signals.api({}).url("/feeds")
      .response(runtime.signals.resource.response.detail()())
      .create({
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          targets: [{
            family: feed,
            params: ({ body }) => ({ feedId: body.id }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              feedId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }],
        },
        load: ({ body }) => ({
          id: `feed:${body.id}`,
          title: body.title,
        }),
      });

    const plan = createFeed.line({
      body: { id: "tmp-feed-1", title: "Draft feed" },
    }).mutationResponse();
    const canonicalLine = feed.line({ feedId: "feed:tmp-feed-1" });

    assert.equal(plan.identityMigration.exactTargetCount, 1);
    assert.equal(plan.identityMigration.targets[0].line.familyKind, "paged");
    assert.equal(plan.identityMigration.targets[0].outcome, "exactResidentLine");
    assert.equal(plan.identityMigration.targets[0].execution.kind, "exactResidentLine");
    assert.equal(
      plan.identityMigration.targets[0].execution.previousCanonicalKey,
      "/feeds/tmp-feed-1",
    );
    assert.equal(
      plan.identityMigration.targets[0].execution.nextCanonicalKey,
      "/feeds/feed%3Atmp-feed-1",
    );
    assert.equal(
      plan.identityMigration.targets[0].execution.requestPath,
      "/feeds/feed%3Atmp-feed-1",
    );
    assert.equal(draftLine, canonicalLine);
    assert.equal(
      draftLine.descriptor().canonicalParams.canonicalKey,
      "/feeds/feed%3Atmp-feed-1",
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

test("save response identity migration can rewrite multiple resident lines in one exact migration plan", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId").detail({
      load: ({ taskId }) => ({ id: taskId, title: "Draft" }),
    });
    const taskList = runtime.signals.api({}).url("/task-lists/:listId").list({
      itemIdentity: (item) => item.id,
      load: ({ listId }) => [{ id: `${listId}:1`, title: "Draft item" }],
    });
    const draftDetailLine = taskRead.line({ taskId: "tmp-batch-1" });
    const draftCollectionLine = taskList.line({ listId: "tmp-batch-1" });
    const firstDetailRuntimeLineId = draftDetailLine.descriptor().runtimeLineId;
    const firstCollectionRuntimeLineId = draftCollectionLine.descriptor().runtimeLineId;
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
            canonicalParams: (_params, _value, canonicalIdentity) => ({ taskId: canonicalIdentity }),
            fallback: "identityMigrationUnavailable",
          }, {
            family: taskList,
            params: ({ body }) => ({ listId: body.id }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({ listId: canonicalIdentity }),
            fallback: "identityMigrationUnavailable",
          }],
        },
        load: ({ body }) => ({ id: `task:${body.id}`, title: body.title }),
      });

    const plan = createTask.line({
      body: { id: "tmp-batch-1", title: "Draft task" },
    }).mutationResponse();

    assert.equal(plan.identityMigration.exactTargetCount, 2);
    assert.equal(plan.counters.identityMigrationExecutionBreadth, 2);
    assert.equal(
      plan.identityMigration.counters.requestDescriptorRewriteBreadth,
      2,
    );
    assert.deepEqual(
      plan.identityMigration.targets.map((target) => target.execution.kind),
      ["exactResidentLine", "exactResidentLine"],
    );
    assert.equal(
      draftDetailLine.descriptor().canonicalParams.canonicalKey,
      "/tasks/task%3Atmp-batch-1",
    );
    assert.equal(
      draftCollectionLine.descriptor().canonicalParams.canonicalKey,
      "/task-lists/task%3Atmp-batch-1",
    );
    assert.notEqual(draftDetailLine.descriptor().runtimeLineId, firstDetailRuntimeLineId);
    assert.notEqual(
      draftCollectionLine.descriptor().runtimeLineId,
      firstCollectionRuntimeLineId,
    );
  } finally {
    await runtime.cleanup();
  }
});
