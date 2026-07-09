import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form binds regulated admission evidence to source patch and schema digests", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);
    const source = signals.input({
      title: "Ship docs",
      actor: "reviewer",
      policy: "qms-submit",
    });
    let approvalEvidence = null;

    const form = signals.form({
      source,
      fields: ({ field }) => ({
        title: field("title"),
        actor: field("actor"),
        policy: field("policy"),
      }),
      admission: ({ action }) => ({
        approval: action("submit", "approval", ["actor", "policy"], (values, context) => ({
          posture: "requiresApproval",
          reason: "submit requires approval",
          actorDigest: approvalEvidence?.actorDigest ?? `actor:${values.actor}`,
          policyDigest: approvalEvidence?.policyDigest ?? `policy:${values.policy}`,
          currentActorDigest: `actor:${values.actor}`,
          currentPolicyDigest: `policy:${values.policy}`,
          sourceDigest: approvalEvidence?.sourceDigest ?? context.binding.sourceDigest,
          patchDigest: approvalEvidence?.patchDigest ?? context.binding.patchDigest,
          schemaDigest: approvalEvidence?.schemaDigest ?? context.binding.schemaDigest,
        })),
      }),
    });

    const initialApproval = form.admission().artifacts[0];
    assert.deepEqual(form.admission().counters, {
      costBasis: "derivedFullReportScan",
      incrementalStatus: "notIncremental",
      declarations: 1,
      dependencyReads: 2,
      fieldScopes: 0,
      actionScopes: 1,
      regulatedArtifacts: 1,
      staleRegulatedArtifacts: 0,
    });
    assert.equal(initialApproval.stale.isStale, false);
    assert.deepEqual(initialApproval.stale.reasons, []);
    approvalEvidence = initialApproval.binding.current;
    const initialBindingDigest = initialApproval.binding.bindingDigest;

    form.fields.title.set("Ready for review");
    const stalePatchApproval = form.admission().artifacts[0];
    assert.equal(form.admission().counters.staleRegulatedArtifacts, 1);
    assert.equal(stalePatchApproval.stale.isStale, true);
    assert.deepEqual(stalePatchApproval.stale.reasons, ["patchDigest changed"]);
    assert.equal(stalePatchApproval.binding.expected.patchDigest, approvalEvidence.patchDigest);
    assert.equal(stalePatchApproval.binding.current.patchDigest, form.patchPlan().equivalenceDigest);
    assert.deepEqual(
      form.actionReadiness("submit").blockers.map((blocker) => blocker.kind),
      ["admission:requiresApproval"],
    );
    assert.match(
      form.actionReadiness("submit").blockers[0].reason,
      /patchDigest changed/,
    );

    approvalEvidence = stalePatchApproval.binding.current;
    source.set({
      title: "Server canonical title",
      actor: "reviewer",
      policy: "qms-submit",
    });
    const staleSourceApproval = form.admission().artifacts[0];
    assert.equal(staleSourceApproval.stale.isStale, true);
    assert.equal(staleSourceApproval.stale.reasons.includes("sourceDigest changed"), true);

    approvalEvidence = staleSourceApproval.binding.current;
    form.fields.actor.set("approver");
    const staleActorApproval = form.admission().artifacts[0];
    assert.equal(staleActorApproval.stale.isStale, true);
    assert.equal(staleActorApproval.stale.reasons.includes("actorDigest changed"), true);
    assert.notEqual(staleActorApproval.binding.bindingDigest, initialBindingDigest);

    approvalEvidence = staleActorApproval.binding.current;
    form.fields.policy.set("expedited");
    const stalePolicyApproval = form.admission().artifacts[0];
    assert.equal(stalePolicyApproval.stale.isStale, true);
    assert.equal(stalePolicyApproval.stale.reasons.includes("policyDigest changed"), true);

    approvalEvidence = {
      ...stalePolicyApproval.binding.current,
      schemaDigest: "schema:previous",
    };
    const staleSchemaApproval = form.admission().artifacts[0];
    assert.equal(staleSchemaApproval.stale.isStale, true);
    assert.equal(staleSchemaApproval.stale.reasons.includes("schemaDigest changed"), true);
    assert.equal(form.diagnostics().admission.artifacts[0].binding.bindingDigest.length > 0, true);
  } finally {
    await cleanup();
  }
});

test("signals.form denies regulated admission artifacts without actor and policy evidence", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);

    const missingActor = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      admission: ({ action }) => ({
        approval: action("submit", "approval", ["title"], () => ({
          posture: "requiresApproval",
          policyDigest: "policy:qms-submit",
        })),
      }),
    });
    assert.throws(
      () => missingActor.admission(),
      /requiresApproval actorDigest must be a non-empty string/,
    );

    const missingPolicy = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      admission: ({ action }) => ({
        signature: action("submit", "signature", ["title"], () => ({
          posture: "requiresSignature",
          actorDigest: "actor:signer",
        })),
      }),
    });
    assert.throws(
      () => missingPolicy.admission(),
      /requiresSignature policyDigest must be a non-empty string/,
    );
  } finally {
    await cleanup();
  }
});
