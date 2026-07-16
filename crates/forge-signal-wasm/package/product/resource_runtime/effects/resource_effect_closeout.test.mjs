import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";
import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";
import { createEffectLine, titlePatch } from "./resource_effect_dag_fixture.mjs";

test("projections from separate lines remain simultaneously visible", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const canonical = createBranchHead(runtime.signals, "effect-dag-multiple-lines");
    const first = createEffectLine(runtime);
    const second = createEffectLine(runtime);
    const baselineBranchCount = runtime.signals.history().branches().length;
    await first.patch(titlePatch(runtime, 0, "first-line"));
    await second.patch(titlePatch(runtime, 1, "second-line"));
    assert.equal(first.value().items[0].title, "first-line");
    assert.equal(second.value().items[1].title, "second-line");
    assert.equal(first.effects().open()[0].nativeParentBranchId, Number(canonical.id));
    assert.equal(second.effects().open()[0].nativeParentBranchId, Number(canonical.id));
    assert.equal(
      first.diagnostics().visibleSelection.branchId,
      second.effects().projection().branch.id,
    );
    const firstEffect = first.effects().open()[0];
    const secondEffect = second.effects().open()[0];
    await second.effects().confirm(secondEffect.effectId);
    await first.effects().reject(firstEffect.effectId);
    assert.equal(first.value().items[0].title, "loaded-0");
    assert.equal(second.value().items[1].title, "second-line");
    assert.equal(runtime.signals.history().branches().length, baselineBranchCount);
  } finally {
    await runtime.cleanup();
  }
});

test("duplicate and contradictory settlement responses are explicit", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    createBranchHead(runtime.signals, "effect-dag-duplicate-settlement");
    const line = createEffectLine(runtime);
    await line.patch(titlePatch(runtime, 0, "confirmed-once"));
    const effect = line.effects().open()[0];
    const original = await line.effects().confirm(effect.effectId, {
      responseId: "response:one",
    });
    const duplicate = await line.effects().confirm(effect.effectId, {
      responseId: "response:one",
    });
    assert.equal(original.kind, "merged");
    assert.equal(duplicate.kind, "duplicateSettlement");
    assert.equal(duplicate.originalKind, "merged");
    assert.equal(line.value().items[0].title, "confirmed-once");
    await assert.rejects(
      line.effects().reject(effect.effectId, {
        responseId: "response:contradictory",
      }),
      (error) => error.code === "terminalOutcomeConflict",
    );
  } finally {
    await runtime.cleanup();
  }
});

test("closeout denial leaves canonical and effect lifecycle unmodified", async () => {
  const runtime = await createRealResourceTestRuntime({
    closeout_effect_branch(history, request) {
      return history.closeout_effect_branch({
        ...request,
        effectRetirement: {
          ...request.effectRetirement,
          expectedBasis: {
            ...request.effectRetirement.expectedBasis,
            nativeHeadGeneration:
              Number(request.effectRetirement.expectedBasis.nativeHeadGeneration) + 1,
          },
        },
      });
    },
  });
  try {
    createBranchHead(runtime.signals, "effect-dag-atomic-closeout");
    const line = createEffectLine(runtime);
    const baselineBranchCount = runtime.signals.history().branches().length;
    await line.patch(titlePatch(runtime, 0, "still-pending"));
    const effect = line.effects().open()[0];
    const openBranchCount = runtime.signals.history().branches().length;
    await assert.rejects(
      line.effects().confirm(effect.effectId, { responseId: "response:stale" }),
      (error) => error.code === "invalidInput"
        && /stale worker branch basis/.test(error.message),
    );
    assert.equal(line.effects().open().length, 1);
    assert.equal(line.value().items[0].title, "still-pending");
    assert.equal(runtime.signals.history().branches().length, openBranchCount);
    await line.effects().reject(effect.effectId, { responseId: "response:reject" });
    assert.equal(line.value().items[0].title, "loaded-0");
    assert.equal(runtime.signals.history().branches().length, baselineBranchCount);
  } finally {
    await runtime.cleanup();
  }
});

test("retry lineage admits one effect and denies divergent intent", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    createBranchHead(runtime.signals, "effect-dag-retry-lineage");
    const line = createEffectLine(runtime);
    const baselineBranchCount = runtime.signals.history().branches().length;
    const patch = titlePatch(runtime, 0, "idempotent");
    await line.patch(patch, {
      idempotencyKey: "request:retry",
      serverCorrelationId: "server:retry",
    });
    const effect = line.diagnostics().lastEffect;
    const openBranchCount = runtime.signals.history().branches().length;
    const duplicate = await line.patch(patch, {
      idempotencyKey: "request:retry",
      serverCorrelationId: "server:retry",
    });
    assert.equal(duplicate.kind, "duplicateEffectAdmission");
    assert.equal(duplicate.effectId, effect.effectId);
    assert.equal(line.effects().open().length, 1);
    assert.equal(runtime.signals.history().branches().length, openBranchCount);
    assert.equal(effect.idempotencyKey, "request:retry");
    assert.equal(effect.serverCorrelationId, "server:retry");
    assert.match(effect.plan.retryLineageId, /request:retry$/);
    await assert.rejects(
      line.patch(titlePatch(runtime, 0, "conflicting-retry"), {
        idempotencyKey: "request:retry",
      }),
      (error) => error.code === "retryIntentConflict",
    );
    await line.effects().confirm(effect.effectId, { responseId: "response:retry" });
    const terminalDuplicate = await line.patch(patch, {
      idempotencyKey: "request:retry",
    });
    assert.equal(terminalDuplicate.kind, "duplicateEffectAdmission");
    assert.equal(runtime.signals.history().branches().length, baselineBranchCount);
  } finally {
    await runtime.cleanup();
  }
});
