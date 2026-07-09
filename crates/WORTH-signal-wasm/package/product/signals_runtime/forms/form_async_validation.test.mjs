import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form runs async validation through stale-safe lifecycle artifacts", async () => {
  await withSignals((signals) => {
    const form = createSlugForm(signals);

    const pending = form.startAsyncValidation("slugUnique");
    assertAsyncValidation(pending, {
      resultKind: "pending",
      validationId: "slugUnique",
      field: "slug",
      stale: false,
    });
    assert.equal(form.validation().summary.pending, 1);
    assert.equal(form.readiness().canSubmit, false);

    form.fields.slug.set("already-taken");
    const stale = form.fulfillAsyncValidation(pending.operationId);
    assertAsyncValidation(stale, {
      resultKind: "staleCompletion",
      targetOperationId: pending.operationId,
      targetValidationId: "slugUnique",
      stale: true,
      reason: "async validation completion targeted a superseded form truth snapshot",
    });
    assert.equal(form.validation().summary.blocked, 1);
    assert.deepEqual(
      form.readiness().blockers.filter((blocker) => blocker.kind === "validation:blocked"),
      [{
        kind: "validation:blocked",
        field: "slug",
        reason: "async validation requires a fresh run",
      }],
    );

    const replacement = form.startAsyncValidation("slugUnique");
    const rejected = form.rejectAsyncValidation(replacement.operationId, {
      reason: "Slug is already taken",
      code: "slug.not_unique",
    });
    assertAsyncValidation(rejected, {
      resultKind: "rejected",
      validationId: "slugUnique",
      field: "slug",
    });
    assert.equal(form.validation().summary.invalid, 1);
    assert.deepEqual(form.visibleMessages().map((message) => message.code), ["slug.not_unique"]);
    assert.equal(form.verification().asyncValidationHistory.operations, 4);
    assert.deepEqual(
      form.asyncValidationHistory().map((entry) => entry.resultKind),
      ["pending", "staleCompletion", "pending", "rejected"],
    );
  });
});

test("signals.form records async validation cancellation and timeout as terminal lifecycle facts", async () => {
  await withSignals((signals) => {
    const form = createSlugForm(signals);

    const cancelledPending = form.startAsyncValidation("slugUnique");
    const cancelled = form.cancelAsyncValidation(cancelledPending.operationId, {
      reason: "blur validation superseded by explicit check",
    });
    const staleCancel = form.fulfillAsyncValidation(cancelledPending.operationId);
    assertAsyncValidation(cancelled, {
      resultKind: "cancelled",
      validationId: "slugUnique",
      field: "slug",
    });
    assertAsyncValidation(staleCancel, {
      resultKind: "staleCompletion",
      targetOperationId: cancelledPending.operationId,
      stale: true,
    });

    const timeoutPending = form.startAsyncValidation("slugUnique");
    const timedOut = form.timeoutAsyncValidation(timeoutPending.operationId);
    assertAsyncValidation(timedOut, {
      resultKind: "timedOut",
      validationId: "slugUnique",
      field: "slug",
    });
    assert.deepEqual(
      form.asyncValidationHistory().map((entry) => entry.resultKind),
      ["pending", "cancelled", "staleCompletion", "pending", "timedOut"],
    );
    assert.equal(form.validation().summary.blocked, 1);
  });
});

test("signals.form supersedes older async validation operations for the same validation id", async () => {
  await withSignals((signals) => {
    const form = createSlugForm(signals);

    const first = form.startAsyncValidation("slugUnique");
    const second = form.startAsyncValidation("slugUnique");
    assert.equal(form.validation().artifacts[0].operationId, second.operationId);
    assertAsyncValidation(form.asyncValidationHistory()[1], {
      resultKind: "superseded",
      validationId: "slugUnique",
      supersededByOperationId: second.operationId,
    });

    const staleFirst = form.fulfillAsyncValidation(first.operationId);
    assertAsyncValidation(staleFirst, {
      resultKind: "staleCompletion",
      targetOperationId: first.operationId,
      stale: true,
    });
    assert.equal(form.validation().summary.pending, 1);
    assert.equal(form.validation().artifacts[0].operationId, second.operationId);

    const fulfilledSecond = form.fulfillAsyncValidation(second.operationId);
    assertAsyncValidation(fulfilledSecond, {
      resultKind: "fulfilled",
      validationId: "slugUnique",
    });
    assert.equal(form.validation().summary.valid, 1);
    assert.deepEqual(
      form.asyncValidationHistory().map((entry) => entry.resultKind),
      ["pending", "superseded", "pending", "staleCompletion", "fulfilled"],
    );
  });
});

test("signals.form denies malformed async validation declarations and undeclared starts", async () => {
  await withSignals((signals) => {
    assert.throws(
      () => signals.form({
        source: { slug: "ship-docs" },
        fields: ({ field }) => ({
          slug: field("slug"),
        }),
        validation: ({ asyncField }) => ({
          slugUnique: asyncField("slug", {
            id: "slugUnique",
            triggers: [],
          }),
        }),
      }),
      /async validation triggers must be a non-empty array/,
    );

    const form = createSlugForm(signals);
    assert.throws(
      () => form.startAsyncValidation("missingValidation"),
      /async validation is not declared/,
    );
    assert.deepEqual(form.asyncValidationHistory(), []);
  });
});

async function withSignals(assertion) {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    await assertion(wrapSignals(createGraphOperationalRuntime()));
  } finally {
    await cleanup();
  }
}

function createSlugForm(signals) {
  return signals.form({
    source: { slug: "ship-docs" },
    fields: ({ field }) => ({
      slug: field("slug"),
    }),
    validation: ({ asyncField }) => ({
      slugUnique: asyncField("slug", {
        id: "slugUnique",
        triggers: ["input", "blur", "explicit"],
        debounceMs: 250,
      }),
    }),
  });
}

function assertAsyncValidation(artifact, expected) {
  for (const [key, value] of Object.entries(expected)) {
    assert.deepEqual(artifact[key], value, `async validation ${key}`);
  }
  assert.equal(typeof artifact.lifecycleDigest, "string");
  assert.ok(artifact.lifecycleDigest.length > 0);
}
