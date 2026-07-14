import assert from "node:assert/strict";
import test from "node:test";

import { withSignals } from "../../action_execution_test_helpers.mjs";
import { createDetailPatchLineFixture } from "../fixtures/resource_line_fixture.mjs";

test("signals.form resource-line submit emits the same resource effect posture as an equivalent direct resource patch", async () => {
  await withSignals((signals) => {
    const effectProfile = signals.resource.effects.branchNative();
    const initialValue = {
      title: "Ship docs",
      status: "draft",
    };
    const formSource = createDetailPatchLineFixture({
      effectProfile,
      initialValue,
    });
    const directSource = createDetailPatchLineFixture({
      effectProfile,
      initialValue,
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(formSource, { id: "task-resource-effect-parity" }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
    });

    form.fields.title.set("Published docs");
    const execution = form.executeAction("submit");
    const directPatchResult = directSource.patch({
      kind: "field",
      field: "title",
      value: "Published docs",
    });
    const directSummary = directSource.summary();
    const directEffect = directSummary.diagnostics.latest.effect;
    const directVerification = directSource.history().verificationPackage();

    assert.equal(execution.resultKind, "fulfilled");
    assert.equal(directPatchResult.kind, "narrowed");
    assert.deepEqual(directSource.patchHistory(), [
      {
        kind: "field",
        field: "title",
        value: "Published docs",
      },
    ]);
    assert.equal(execution.resourceSubmission?.patchCount, 1);
    assert.equal(
      execution.resourceSubmission?.effectProfile.profile?.name,
      directVerification.requestPosture.effects.name,
    );
    assert.equal(execution.resourceSubmission?.rollback?.kind, directEffect.optimistic.rollback.kind);
    assert.equal(execution.resourceSubmission?.rollback?.mode, directEffect.optimistic.rollback.mode);
    assert.equal(execution.resourceSubmission?.rollback?.branchId, directEffect.optimistic.rollback.branchId);
    assert.equal(execution.resourceSubmission?.rollback?.snapshotId, directEffect.optimistic.rollback.snapshotId);
    assert.equal(
      execution.resourceSubmission?.visibleSelection.kind,
      directSummary.current.visibleSelection.kind,
    );
    assert.equal(
      execution.resourceSubmission?.visibleSelection.branchId,
      directSummary.current.visibleSelection.branchId,
    );
    assert.equal(
      execution.resourceSubmission?.visibleSelection.snapshotId,
      directSummary.current.visibleSelection.snapshotId,
    );
    assert.equal(
      execution.resourceSubmission?.visibleSelection.basisId,
      directSummary.current.visibleSelection.basisId,
    );
    assert.deepEqual(execution.resourceSubmission?.patches[0], {
      field: directEffect.locus.field,
      path: directEffect.patch.field,
      locusKind: "field",
      locus: directEffect.locus.field,
      operationKind: "set",
      patchKind: directEffect.patch.kind,
      patchResultKind: directPatchResult.kind,
      patchScope: directPatchResult.scope,
      effectDigest: execution.resourceSubmission?.patches[0].effectDigest,
      basisId: directSummary.current.visibleSelection.basisId,
    });
  });
});
