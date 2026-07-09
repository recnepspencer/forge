import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("identity migration certifies draft publication across detail, collection, summary, selection, and auxiliary targets", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const draftDetail = runtime.signals.api({}).url("/drafts/:draftId").detail({
      load: ({ draftId }) => ({ id: draftId, title: "Draft" }),
    });
    const draftIndex = runtime.signals.api({}).url("/draft-index/:draftId").list({
      itemIdentity: (item) => item.id,
      load: ({ draftId }) => [{ id: `${draftId}:1`, title: "Indexed" }],
    });
    const draftSummary = runtime.signals.api({}).url("/draft-summary/:draftId")
      .response(runtime.signals.resource.response.collection({
        itemId: (item) => item.id,
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: runtime.signalsMod.resourceValueSummaries({
          total: {
            read: (value) => value.total,
            write: (value, total) => ({ ...value, total }),
          },
        }),
      }))
      .list({
        load: ({ draftId }) => ({
          items: [{ id: `${draftId}:1`, title: "Summarized" }],
          total: 1,
        }),
      });
    const draftPresence = runtime.signals.api({}).url("/draft-presence/:draftId").detail({
      load: ({ draftId }) => ({ id: draftId, visible: true }),
    });
    const draftAudit = runtime.signals.api({}).url("/draft-audit/:draftId").detail({
      load: ({ draftId }) => ({ id: draftId, events: 0 }),
    });

    const detailLine = draftDetail.line({ draftId: "draft-7" });
    const indexLine = draftIndex.line({ draftId: "draft-7" });
    const summaryLine = draftSummary.line({ draftId: "draft-7" });
    const selectionLine = draftPresence.line({ draftId: "draft-7" });

    const publishDraft = runtime.signals.api({}).url("/publish-draft")
      .response(runtime.signals.resource.response.detail()())
      .create({
        identity: {
          submitted: ({ body }) => body.draftId,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          atomicity: "partialAllowed",
          targets: [{
            family: draftDetail,
            params: ({ body }) => ({ draftId: body.draftId }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              draftId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }, {
            family: draftIndex,
            params: ({ body }) => ({ draftId: body.draftId }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              draftId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }, {
            family: draftSummary,
            params: ({ body }) => ({ draftId: body.draftId }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              draftId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
            summary: {
              kind: "summary",
              summary: "total",
            },
          }, {
            family: draftPresence,
            params: ({ body }) => ({ draftId: body.draftId }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              draftId: canonicalIdentity,
            }),
            fallback: "deliveryAwaited",
            selection: {
              kind: "visibleSelection",
            },
          }, {
            family: draftAudit,
            params: ({ body }) => ({ draftId: body.draftId }),
            fallback: "partialReconciliation",
          }],
        },
        load: ({ body }) => ({
          id: `published:${body.draftId}`,
          title: body.title,
        }),
      });

    const plan = publishDraft.line({
      body: { draftId: "draft-7", title: "Published" },
    }).mutationResponse();

    assert.equal(plan.identityMigration.submittedIdentity, "draft-7");
    assert.equal(plan.identityMigration.canonicalIdentity, "published:draft-7");
    assert.equal(plan.identityMigration.partialAdmission, "admitted");
    assert.equal(plan.identityMigration.exactTargetCount, 4);
    assert.deepEqual(plan.identityMigration.fallbackKinds, ["partialReconciliation"]);
    assert.deepEqual(
      plan.identityMigration.targets.map((target) => target.scope.kind),
      [
        "residentLine",
        "residentLine",
        "summary",
        "visibleSelection",
        "residentLine",
      ],
    );
    assert.equal(plan.identityMigration.targets[2].scope.summary, "total");
    assert.equal(plan.identityMigration.targets[2].outcome, "exactResidentLine");
    assert.equal(plan.identityMigration.targets[3].outcome, "exactResidentLine");
    assert.equal(plan.identityMigration.targets[4].outcome, "fallback");
    assert.equal(plan.confirmation.kind, "partialCanonicalTruth");
    assert.equal(plan.counters.identityMigrationTargetFanoutBreadth, 5);
    assert.equal(
      plan.identityMigration.counters.requestDescriptorRewriteBreadth,
      4,
    );
    assert.equal(
      detailLine.descriptor().canonicalParams.canonicalKey,
      "/drafts/published%3Adraft-7",
    );
    assert.equal(
      indexLine.descriptor().canonicalParams.canonicalKey,
      "/draft-index/published%3Adraft-7",
    );
    assert.equal(
      summaryLine.descriptor().canonicalParams.canonicalKey,
      "/draft-summary/published%3Adraft-7",
    );
    assert.equal(
      selectionLine.descriptor().canonicalParams.canonicalKey,
      "/draft-presence/published%3Adraft-7",
    );
    assert.equal(selectionLine.diagnostics().visibleSelection.kind, "committed");
  } finally {
    await runtime.cleanup();
  }
});

test("identity migration certifies clone and import mapping through declared resident collection targets", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const importQueue = runtime.signals.api({}).url("/imports/:importId").list({
      itemIdentity: (item) => item.id,
      load: ({ importId }) => [{ id: `${importId}:queued`, title: "Queued" }],
    });
    const importSummary = runtime.signals.api({}).url("/import-summary/:importId")
      .response(runtime.signals.resource.response.collection({
        itemId: (item) => item.id,
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: runtime.signalsMod.resourceValueSummaries({
          imported: {
            read: (value) => value.imported,
            write: (value, imported) => ({ ...value, imported }),
          },
        }),
      }))
      .list({
        load: ({ importId }) => ({
          items: [{ id: `${importId}:queued`, title: "Queued" }],
          imported: 0,
        }),
      });

    const queueLine = importQueue.line({ importId: "clone-22" });
    const summaryLine = importSummary.line({ importId: "clone-22" });
    const importDraft = runtime.signals.api({}).url("/imports")
      .response(runtime.signals.resource.response.detail()())
      .create({
        identity: {
          submitted: ({ body }) => body.cloneId,
          canonical: (value) => value.id,
          targets: [{
            family: importQueue,
            params: ({ body }) => ({ importId: body.cloneId }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              importId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }, {
            family: importSummary,
            params: ({ body }) => ({ importId: body.cloneId }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              importId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
            summary: {
              kind: "summary",
              summary: "imported",
            },
          }],
        },
        load: ({ body }) => ({
          id: `import:${body.cloneId}`,
          status: "complete",
        }),
      });

    const plan = importDraft.line({
      body: { cloneId: "clone-22" },
    }).mutationResponse();

    assert.equal(plan.identityMigration.responseIdentity, null);
    assert.equal(plan.identityMigration.canonicalIdentity, "import:clone-22");
    assert.equal(plan.identityMigration.exactTargetCount, 2);
    assert.equal(plan.identityMigration.targets[1].scope.kind, "summary");
    assert.equal(plan.identityMigration.targets[1].scope.summary, "imported");
    assert.equal(
      queueLine.descriptor().canonicalParams.canonicalKey,
      "/imports/import%3Aclone-22",
    );
    assert.equal(
      summaryLine.descriptor().canonicalParams.canonicalKey,
      "/import-summary/import%3Aclone-22",
    );
  } finally {
    await runtime.cleanup();
  }
});

test("identity migration emits typed refetch and delivery-awaited posture when the route omits the exact detail-child region lens", async () => {
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
      "children.assignees": {
        read: (value) => value.children,
        write: (value, children) => ({ ...value, children }),
        identityBoundary: "inside",
        mergeGranularity: "child-assignees",
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
        title: "Draft",
        children: [{ id: `${taskId}:child`, title: "Child" }],
      }),
    });
    taskDetail.line({ taskId: "tmp-child-1" });
    const createTask = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          atomicity: "partialAllowed",
          targets: [{
            family: taskDetail,
            params: ({ body }) => ({ taskId: body.id }),
            fallback: "refetchRequired",
            detailChild: {
              kind: "detailChild",
              region: "children",
            },
          }, {
            family: taskDetail,
            params: ({ body }) => ({ taskId: body.id }),
            fallback: "deliveryAwaited",
            detailChild: {
              kind: "detailChild",
              region: "children.assignees",
            },
          }],
        },
        load: ({ body }) => ({
          id: `task:${body.id}`,
          title: body.title,
          children: [{ id: `server:${body.id}:child`, title: "Child" }],
        }),
      });

    const plan = createTask.line({
      body: { id: "tmp-child-1", title: "Draft" },
    }).mutationResponse();

    assert.equal(plan.identityMigration.exactTargetCount, 0);
    assert.deepEqual(plan.identityMigration.fallbackKinds, [
      "refetchRequired",
      "deliveryAwaited",
    ]);
    assert.deepEqual(
      plan.identityMigration.targets.map((target) => target.scope.kind),
      ["detailChild", "detailChild"],
    );
    assert.equal(plan.identityMigration.targets[0].scope.region, "children");
    assert.equal(
      plan.identityMigration.targets[1].scope.region,
      "children.assignees",
    );
    assert.match(
      plan.identityMigration.targets[0].detail,
      /resource\.response\.detailRegions<T>\(\) region "children"/,
    );
    assert.match(
      plan.identityMigration.targets[1].detail,
      /resource\.response\.detailRegions<T>\(\) region "children\.assignees"/,
    );
    assert.equal(plan.confirmation.kind, "partialCanonicalTruth");
  } finally {
    await runtime.cleanup();
  }
});

test("identity migration certifies exact detail-child region targets when the route declares a matching region lens", async () => {
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
          children: [{ id: "tmp-child-2", title: "Child" }],
        }),
      });
    const taskLine = taskDetail.line({ taskId: "task-2" });
    const createTask = runtime.signals.api({}).url("/tasks/:taskId/children")
      .response(runtime.signals.resource.response.detailRegions()(taskRegions))
      .create({
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.children[0]?.id ?? value.id,
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
      });

    const plan = createTask.line({
      taskId: "task-2",
      body: { id: "tmp-child-2", title: "Child" },
    }).mutationResponse();

    assert.deepEqual(taskLine.value().children, [{
      id: "child:tmp-child-2",
      title: "Child",
    }]);
    assert.equal(plan.identityMigration.exactTargetCount, 1);
    assert.deepEqual(plan.identityMigration.fallbackKinds, []);
    assert.equal(plan.identityMigration.targets[0].scope.kind, "detailChild");
    assert.equal(plan.identityMigration.targets[0].scope.region, "children");
    assert.equal(plan.identityMigration.targets[0].outcome, "exactDetailChildRegion");
    assert.equal(plan.identityMigration.targets[0].execution.kind, "exactDetailChildRegion");
    assert.equal(plan.identityMigration.targets[0].execution.region, "children");
    assert.equal(plan.identityMigration.targets[0].execution.outcomeKind, "applied");
    assert.ok(plan.identityMigration.targets[0].execution.effectProof);
    assert.equal(plan.identityMigration.counters.requestDescriptorRewriteBreadth, 0);
    assert.equal(taskLine.diagnostics().lastPatchedRegion, "children");
  } finally {
    await runtime.cleanup();
  }
});
