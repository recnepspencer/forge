import assert from "node:assert/strict";
import test from "node:test";

import { withSignals } from "../../action_execution_test_helpers.mjs";
import { createDetailPatchLineFixture } from "../fixtures/resource_line_fixture.mjs";

test("signals.form submit inherits the backing resource line effect profile when no action profile is declared", async () => {
  await withSignals((signals) => {
    const form = signals.form({
      source: signals.form.source.resourceLine(
        createDetailPatchLineFixture({
          effectProfile: signals.resource.effects.branchNative(),
          initialValue: { title: "Ship docs" },
        }),
        { id: "resource-profile-inherited" },
      ),
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    form.fields.title.set("Publish docs");
    const plan = form.actionPlan("submit");
    const catalogEntry = form.actions().catalog.find((entry) => entry.id === "submit");
    assert.equal(plan.status, "accepted");
    assert.equal(plan.resourceEffectProfile.source, "inheritedFromResourceLine");
    assert.equal(plan.resourceEffectProfile.declared, null);
    assert.equal(plan.resourceEffectProfile.effective.name, "branchNative");
    assert.equal(typeof plan.resourceEffectProfile.closeoutMatrixDigest, "string");
    assert.equal(catalogEntry.resourceEffectProfile.source, "inheritedFromResourceLine");
  });
});

test("signals.form accepts a declared resource effect profile when it matches the backing resource line", async () => {
  await withSignals((signals) => {
    const form = signals.form({
      source: signals.form.source.resourceLine(
        createDetailPatchLineFixture({
          effectProfile: signals.resource.effects.branchNative(),
          initialValue: { title: "Ship docs" },
        }),
        { id: "resource-profile-match" },
      ),
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ submit }) => ({
        submit: submit({
          resourceEffectProfile: signals.resource.effects.branchNative(),
        }),
      }),
    });

    form.fields.title.set("Publish docs");
    const plan = form.actionPlan("submit");
    const execution = form.executeAction("submit");
    assert.equal(plan.status, "accepted");
    assert.equal(plan.resourceEffectProfile.source, "declaredMatchesResourceLine");
    assert.equal(plan.resourceEffectProfile.declared.name, "branchNative");
    assert.equal(plan.resourceEffectProfile.effective.name, "branchNative");
    assert.equal(execution.resultKind, "fulfilled");
    assert.equal(execution.effectStarted, true);
    assert.equal(execution.resourceSubmission.effectProfile.profile.name, "branchNative");
  });
});

test("signals.form denies a declared resource effect profile that mismatches the backing resource line before effects", async () => {
  await withSignals((signals) => {
    const source = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: { title: "Ship docs" },
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "resource-profile-mismatch" }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ submit }) => ({
        submit: submit({
          resourceEffectProfile: signals.resource.effects.serverCanonical(),
        }),
      }),
    });

    form.fields.title.set("Publish docs");
    const plan = form.actionPlan("submit");
    const readiness = form.readiness();
    const execution = form.executeAction("submit");
    assert.equal(plan.status, "denied");
    assert.equal(plan.resourceEffectProfile.source, "declaredMismatchedResourceLine");
    assert.equal(plan.readiness.blockers.some((blocker) => blocker.kind === "resource:profileMismatch"), true);
    assert.equal(readiness.canSubmit, false);
    assert.equal(readiness.blockers.some((blocker) => blocker.kind === "resource:profileMismatch"), true);
    assert.equal(execution.resultKind, "denied");
    assert.equal(execution.effectStarted, false);
    assert.equal(source.patchHistory().length, 0);
  });
});

test("signals.form denies declared resource effect profiles on non-resource-line forms", async () => {
  await withSignals((signals) => {
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ submit }) => ({
        submit: submit({
          resourceEffectProfile: signals.resource.effects.branchNative(),
        }),
      }),
    });

    form.fields.title.set("Publish docs");
    const plan = form.actionPlan("submit");
    const readiness = form.readiness();
    assert.equal(plan.status, "denied");
    assert.equal(plan.resourceEffectProfile.source, "declaredWithoutResourceLine");
    assert.equal(plan.resourceEffectProfile.declared.name, "branchNative");
    assert.equal(plan.resourceEffectProfile.effective, null);
    assert.equal(readiness.canSubmit, false);
    assert.equal(
      plan.readiness.blockers.some((blocker) => blocker.kind === "resource:profileUnavailable"),
      true,
    );
    assert.equal(
      readiness.blockers.some((blocker) => blocker.kind === "resource:profileUnavailable"),
      true,
    );
  });
});
