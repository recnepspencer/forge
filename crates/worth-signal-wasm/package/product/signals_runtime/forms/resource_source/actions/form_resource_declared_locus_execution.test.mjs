import assert from "node:assert/strict";
import test from "node:test";

import { withSignals } from "../../action_execution_test_helpers.mjs";
import { createDeclaredLocusDetailLineFixture } from "../fixtures/resource_declared_locus_line_fixture.mjs";

test("signals.form lowers declared jsonPath and region resource loci plus attachment attach detach operations", async () => {
  await withSignals((signals) => {
    const source = createDeclaredLocusDetailLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        profile: { displayName: "Ship docs" },
        evidenceRegion: { digest: "file-0", name: "draft.pdf" },
      },
      jsonPathNames: ["$.profile.displayName"],
      regionNames: ["evidenceRegion"],
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "declared-locus-submit" }),
      fields: ({ field, evidence }) => ({
        displayName: field("profile.displayName", {
          resourceLocus: { kind: "jsonPath", path: "$.profile.displayName" },
        }),
        evidence: evidence("evidenceRegion", {
          attachmentIdentity: "digest",
          resourceLocus: { kind: "region", region: "evidenceRegion" },
        }),
      }),
    });

    form.fields.displayName.set("Published docs");
    form.fields.evidence.set({ digest: "file-1", name: "audit.pdf" });
    const attachExecution = form.executeAction("submit");
    assert.equal(attachExecution.resultKind, "fulfilled");
    assert.deepEqual(
      attachExecution.resourceSubmission.patches.map((patch) => ({
        operationKind: patch.operationKind,
        patchKind: patch.patchKind,
        locusKind: patch.locusKind,
        locus: patch.locus,
      })),
      [
        {
          operationKind: "set",
          patchKind: "jsonPath",
          locusKind: "jsonPath",
          locus: "$.profile.displayName",
        },
        {
          operationKind: "attach",
          patchKind: "region",
          locusKind: "region",
          locus: "evidenceRegion",
        },
      ],
    );
    assert.deepEqual(form.source(), {
      profile: { displayName: "Published docs" },
      evidenceRegion: { digest: "file-1", name: "audit.pdf" },
    });

    form.fields.evidence.set(null);
    const detachTransferReport = form.attachmentTransfers();
    assert.equal(detachTransferReport.fields[0].bindingKind, "noAttachment");
    assert.equal(detachTransferReport.fields[0].attachmentDigest, null);
    assert.equal(detachTransferReport.fields[0].attachmentPresent, false);
    const detachExecution = form.executeAction("submit");
    assert.equal(detachExecution.resultKind, "fulfilled");
    assert.deepEqual(
      detachExecution.resourceSubmission.patches.map((patch) => ({
        operationKind: patch.operationKind,
        patchKind: patch.patchKind,
        locusKind: patch.locusKind,
        locus: patch.locus,
      })),
      [
        {
          operationKind: "detach",
          patchKind: "region",
          locusKind: "region",
          locus: "evidenceRegion",
        },
      ],
    );
    assert.deepEqual(form.source(), {
      profile: { displayName: "Published docs" },
      evidenceRegion: null,
    });
  });
});

test("signals.form lowers declared itemAspect and summary resource loci without broad replace", async () => {
  await withSignals((signals) => {
    const source = createDeclaredLocusDetailLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        taskViews: { "task-1": { statusBadge: "draft" } },
        totals: 1,
      },
      aspectNames: ["taskViews.task-1.statusBadge"],
      summaryNames: ["totals"],
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "declared-locus-aspect-summary" }),
      fields: ({ field }) => ({
        statusBadge: field("taskViews.task-1.statusBadge", {
          resourceLocus: { kind: "itemAspect", itemId: "task-1", aspect: "taskViews.task-1.statusBadge" },
        }),
        totals: field("totals", {
          resourceLocus: { kind: "summary", summary: "totals" },
        }),
      }),
    });

    form.fields.statusBadge.set("published");
    form.fields.totals.set(2);
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "fulfilled");
    assert.deepEqual(
      execution.resourceSubmission.patches.map((patch) => ({
        patchKind: patch.patchKind,
        locusKind: patch.locusKind,
        locus: patch.locus,
      })),
      [
        {
          patchKind: "itemAspect",
          locusKind: "aspect",
          locus: "taskViews.task-1.statusBadge",
        },
        {
          patchKind: "summary",
          locusKind: "summary",
          locus: "totals",
        },
      ],
    );
    assert.deepEqual(form.source(), {
      taskViews: { "task-1": { statusBadge: "draft" } },
      totals: 2,
      "taskViews.task-1.statusBadge": "published",
    });
  });
});

test("signals.form denies itemAspect resource loci without declared item identity", async () => {
  await withSignals((signals) => {
    assert.throws(
      () =>
        signals.form({
          source: signals.form.source.resourceLine(createDeclaredLocusDetailLineFixture({
            effectProfile: signals.resource.effects.branchNative(),
            initialValue: { taskViews: { "task-1": { statusBadge: "draft" } } },
            aspectNames: ["taskViews.task-1.statusBadge"],
          }), { id: "declared-locus-missing-item-id" }),
          fields: ({ field }) => ({
            statusBadge: field("taskViews.task-1.statusBadge", {
              resourceLocus: { kind: "itemAspect", aspect: "taskViews.task-1.statusBadge" },
            }),
          }),
        }),
      /resourceLocus\.itemId must be a non-empty string/,
    );
  });
});
