import assert from "node:assert/strict";
import test from "node:test";

import { withSignals } from "../../action_execution_test_helpers.mjs";
import {
  createDetailPatchLineFixture,
  createMutationResponsePlanFixture,
} from "../fixtures/resource_line_fixture.mjs";

test("signals.form carries create placement delete tombstone and multi-family mutation-response completion through execution history", async () => {
  await withSignals((signals) => {
    const source = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        status: "draft",
      },
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
              placement: "prepend",
              field: null,
              region: null,
              path: null,
              summary: null,
            }),
            targetDigest: "target:digest:insert",
          }),
          Object.freeze({
            targetId: "target-delete",
            family: Object.freeze({ kind: "collection", familyId: "task-list" }),
            line: Object.freeze({ canonicalKey: "workspace=current", residency: "resident" }),
            execution: Object.freeze({
              kind: "exactCollectionDelete",
              scope: "item",
              itemId: "task-1",
              placement: null,
              field: null,
              region: null,
              path: null,
              summary: null,
            }),
            targetDigest: "target:digest:delete",
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
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-submit-outcomes" }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
    });

    form.fields.title.set("Published docs");
    const execution = form.executeAction("submit");
    const completion = execution.resourceSubmission.mutationResponse.completion;
    const contract = execution.resourceSubmission.mutationResponse.contract;
    assert.equal(completion.multiFamily, true);
    assert.deepEqual(completion.familyKinds, ["collection", "paged"]);
    assert.equal(completion.placement.kind, "prependOnly");
    assert.equal(completion.deletion.kind, "deleteOnly");
    assert.equal(completion.summaryTargetCount, 1);
    assert.equal(
      form.canonicalizationHistory()[0].resourceLine.mutationResponse.contract.digest,
      contract.digest,
    );
    assert.equal(
      form.canonicalizationHistory()[0].resourceLine.mutationResponse.completion.digest,
      completion.digest,
    );
  });
});
