import assert from "node:assert/strict";
import test from "node:test";

import { withSignals } from "../../action_execution_test_helpers.mjs";
import { createDetailPatchLineFixture } from "../fixtures/resource_line_fixture.mjs";

test("signals.form rollbackLastResourceEffect restores resource line source truth and records reset history", async () => {
  await withSignals((signals) => {
    const source = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        status: "draft",
      },
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-reset" }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
    });

    form.fields.title.set("Published docs");
    form.fields.status.set("review");
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "fulfilled");
    assert.deepEqual(form.source(), {
      title: "Published docs",
      status: "review",
    });

    const rollback = form.rollbackLastResourceEffect();
    assert.equal(rollback.mode, "resourceRollback");
    assert.equal(rollback.resultKind, "rolledBack");
    assert.equal(rollback.resourceRollback.kind, "rolledBack");
    assert.equal(rollback.resourceRollback.mode, "CompactInversePatch");
    assert.deepEqual(form.source(), {
      title: "Ship docs",
      status: "draft",
    });
    assert.deepEqual(form.draft(), {
      title: "Published docs",
      status: "review",
    });
    assert.deepEqual(form.effective(), {
      title: "Published docs",
      status: "review",
    });
    assert.equal(form.resetHistory().length, 1);
    assert.equal(form.resetHistory()[0].resetDigest, rollback.resetDigest);
    assert.equal(form.diagnostics().resetHistory.length, 1);
    assert.equal(form.verification().resetHistory.operations, 1);
    assert.equal(
      form.verification().digests.resetHistoryDigest,
      form.verification().resetHistory.digest,
    );
  });
});

test("signals.form reset clears local draft truth and rollbackLastResourceEffect names missing resource authority explicitly", async () => {
  await withSignals((signals) => {
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    form.fields.title.set("Client title");
    const reset = form.reset();
    assert.equal(reset.mode, "acceptCanonicalValue");
    assert.equal(reset.resultKind, "reset");
    assert.deepEqual(form.source(), { title: "Ship docs" });
    assert.deepEqual(form.draft(), {});
    assert.deepEqual(form.effective(), { title: "Ship docs" });

    const rollback = form.rollbackLastResourceEffect();
    assert.equal(rollback.mode, "resourceRollback");
    assert.equal(rollback.resultKind, "unavailable");
    assert.equal(rollback.resourceRollback.kind, "unavailable");
    assert.equal(rollback.resourceRollback.reason, "resourceSourceUnavailable");
    assert.equal(form.resetHistory().length, 2);

    const noOp = form.reset();
    assert.equal(noOp.resultKind, "noOp");
    assert.equal(form.resetHistory().length, 3);
  });
});
