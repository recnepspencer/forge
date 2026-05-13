import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("exact identity migration contributes typed lifecycle proof when no resource effect envelope exists", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .detail({ load: ({ taskId }) => ({ id: taskId, title: "Draft" }) });
    taskRead.line({ taskId: "tmp-lifecycle-1" });
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
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              taskId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }],
        },
        load: ({ body }) => ({
          id: `task:${body.id}`,
          title: body.title,
        }),
      })
      .line({ body: { id: "tmp-lifecycle-1", title: "Draft" } })
      .mutationResponse();

    const proof = plan.lifecycleProof.entries[0];

    assert.equal(plan.lifecycleProof.count, 1);
    assert.equal(proof.entryKind, "identityMigration");
    assert.equal(proof.targetId, "migrationTarget1");
    assert.equal(proof.effectId, null);
    assert.equal(proof.authorityDigest, plan.identityMigration.declarationDigest);
    assert.equal(proof.rollback.kind, "identityMigrationUnavailable");
    assert.match(proof.rollback.detail, /no resource-effect rollback envelope exists/);
    assert.equal(proof.mergeRebase.kind, "identityMigrationUnavailable");
    assert.match(proof.mergeRebase.granularity, /identityMigration:\/tasks\/tmp-lifecycle-1:/);
    assert.match(proof.mergeRebase.detail, /without issuing a resource effect locus/);
    assert.equal(plan.counters.identityMigrationLifecycleProofBreadth, 1);
    assert.equal(plan.counters.lifecycleProofBreadth, 1);
    assert.equal(plan.identityMigration.counters.lifecycleProofBreadth, 1);
    assert.match(plan.lifecycleProof.rollbackDigest, /identityMigration:migrationTarget1:none:identityMigrationUnavailable/);
    assert.match(plan.lifecycleProof.mergeRebaseDigest, /identityMigration:migrationTarget1:none:identityMigrationUnavailable/);
  } finally {
    await runtime.cleanup();
  }
});

test("fallback identity migration contributes fallback lifecycle proof instead of disappearing from the envelope", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRead = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .detail({ load: ({ taskId }) => ({ id: taskId, title: "Draft" }) });
    taskRead.line({ taskId: "tmp-lifecycle-2" });
    taskRead.line({ taskId: "task:tmp-lifecycle-2" });
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
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              taskId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }],
        },
        load: ({ body }) => ({
          id: `task:${body.id}`,
          title: body.title,
        }),
      })
      .line({ body: { id: "tmp-lifecycle-2", title: "Draft" } })
      .mutationResponse();

    const proof = plan.lifecycleProof.entries[0];

    assert.equal(plan.lifecycleProof.count, 1);
    assert.equal(proof.entryKind, "identityMigration");
    assert.equal(proof.rollback.kind, "fallbackUnavailable");
    assert.equal(proof.mergeRebase.kind, "fallbackUnavailable");
    assert.equal(proof.mergeRebase.granularity, "identityMigrationUnavailable");
    assert.match(proof.rollback.detail, /canonical destination is already resident/);
    assert.equal(plan.counters.identityMigrationLifecycleProofBreadth, 1);
    assert.equal(plan.identityMigration.counters.lifecycleProofBreadth, 1);
  } finally {
    await runtime.cleanup();
  }
});
