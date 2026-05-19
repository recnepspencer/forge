import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../../runtime_fixture/graph_operational_runtime.mjs";
import {
  createMutationResponsePlanFixture,
  createReadOnlyResourceLineFixture,
} from "../fixtures/resource_line_fixture.mjs";

test("signals.form exposes resource line mutation-response completion readback and verification digests", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: { kind: "pending", operation: "initialLoad", continuity: "noVisibleValueYet" },
          freshness: { kind: "stale", reason: "initialLoadPending" },
          mutationResponse: createMutationResponsePlanFixture({
            confirmationKind: "deliveryAwaited",
            fallbackKind: "deliveryAwaited",
            planCount: 3,
          }),
        }),
        { id: "resource-task-mutation-response-digest" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
    });

    const report = form.resourceSource().mutationResponse;
    assert.equal(report.confirmationKind, "deliveryAwaited");
    assert.equal(report.planCount, 3);
    assert.equal(report.outOfContractTargetDigest, "mutation-response-unsupportedTarget-targets|none");
    assert.equal(
      report.contract.outOfContractTargetDigest,
      report.outOfContractTargetDigest,
    );
    assert.equal(report.completion.multiFamily, false);
    assert.equal(report.completion.familyCounts.detail, 1);
    assert.equal(
      form.verification().digests.resourceMutationResponseContractDigest,
      report.contract.digest,
    );
    assert.equal(
      form.verification().digests.resourceMutationResponseCompletionDigest,
      report.completion.digest,
    );
    assert.equal(
      form.verification().digests.resourceMutationResponseTargetOutcomeDigest,
      report.targetOutcomeDigest,
    );
  } finally {
    await cleanup();
  }
});

test("signals.form summarizes create placement delete tombstone and multi-family mutation-response outcomes", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: { kind: "fulfilled", operation: "delivery" },
          freshness: { kind: "fresh" },
          familyKind: "collection",
          familyId: "task-list",
          runtimeLineId: "task:list",
          canonicalKey: "workspace=current",
          mutationResponse: createMutationResponsePlanFixture({
            confirmationKind: "consumedCanonicalTruth",
            targets: [
              Object.freeze({
                targetId: "target-insert",
                family: Object.freeze({ kind: "collection", familyId: "task-list" }),
                line: Object.freeze({ canonicalKey: "workspace=current", residency: "resident" }),
                execution: Object.freeze({
                  kind: "exactCollectionInsert",
                  scope: "item",
                  itemId: "task-2",
                  placement: "append",
                  field: null,
                  region: null,
                  path: null,
                  summary: null,
                }),
                targetDigest: "target:digest:insert",
              }),
              Object.freeze({
                targetId: "target-tombstone",
                family: Object.freeze({ kind: "collection", familyId: "task-list" }),
                line: Object.freeze({ canonicalKey: "workspace=current", residency: "resident" }),
                execution: Object.freeze({
                  kind: "exactCollectionTombstone",
                  scope: "item",
                  itemId: "task-1",
                  placement: null,
                  field: null,
                  region: null,
                  path: null,
                  summary: null,
                }),
                targetDigest: "target:digest:tombstone",
              }),
              Object.freeze({
                targetId: "target-summary",
                family: Object.freeze({ kind: "paged", familyId: "task-search" }),
                line: Object.freeze({ canonicalKey: "query=task&page=1", residency: "resident" }),
                execution: Object.freeze({
                  kind: "exactSummary",
                  scope: "summary",
                  itemId: null,
                  placement: null,
                  field: null,
                  region: null,
                  path: null,
                  summary: "totals",
                  summaryScope: "pageWindow",
                }),
                targetDigest: "target:digest:summary",
              }),
            ],
          }),
        }),
        { id: "resource-task-reconciliation-outcomes" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
    });

    const completion = form.resourceSource().mutationResponse.completion;
    const contract = form.resourceSource().mutationResponse.contract;
    assert.equal(completion.multiFamily, true);
    assert.deepEqual(completion.familyKinds, ["collection", "paged"]);
    assert.equal(completion.placement.kind, "appendOnly");
    assert.equal(completion.placement.appendCount, 1);
    assert.equal(completion.deletion.kind, "tombstoneOnly");
    assert.equal(completion.deletion.tombstoneCount, 1);
    assert.equal(completion.summaryTargetCount, 1);
    assert.equal(
      form.verification().digests.resourceMutationResponseContractDigest,
      contract.digest,
    );
    assert.equal(
      form.verification().digests.resourceMutationResponseCompletionDigest,
      completion.digest,
    );
  } finally {
    await cleanup();
  }
});
