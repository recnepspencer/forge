import assert from "node:assert/strict";
import test from "node:test";

import { withSignals } from "../../action_execution_test_helpers.mjs";
import {
  createDetailPatchLineFixture,
  createMutationResponsePlanFixture,
  createReadOnlyResourceLineFixture,
} from "../fixtures/resource_line_fixture.mjs";

test("signals.form carries confirmed resource settlement on fulfilled resource-line submit", async () => {
  await withSignals((signals) => {
    const form = signals.form({
      source: signals.form.source.resourceLine(
        createDetailPatchLineFixture({
          effectProfile: signals.resource.effects.branchNative(),
          initialValue: { title: "Ship docs", status: "draft" },
          mutationResponse: createMutationResponsePlanFixture({
            confirmationKind: "partialCanonicalTruth",
            fallbackKind: "partialReconciliation",
          }),
        }),
        { id: "resource-settlement-submit" },
      ),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
    });

    form.fields.title.set("Published docs");
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "fulfilled");
    assert.equal(execution.resourceSettlement.kind, "confirmed");
    assert.equal(execution.resourceSettlement.confirmationKind, "partialCanonicalTruth");
    assert.equal(
      execution.resourceSettlement.digest,
      form.resourceSource().settlement.digest,
    );
  });
});

test("signals.form carries pending resource settlement on fulfilled lifecycle resource actions", async () => {
  await withSignals((signals) => {
    const form = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: Object.freeze({ kind: "fulfilled", operation: "initialLoad" }),
          freshness: Object.freeze({ kind: "stale", reason: "revalidationRequired" }),
        }),
        { id: "resource-settlement-lifecycle" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
      actions: ({ action }) => ({
        revalidateResourceSource: action("revalidateResourceSource", {
          resourceAction: { kind: "revalidate" },
        }),
      }),
    });

    const execution = form.executeAction("revalidateResourceSource");
    assert.equal(execution.resultKind, "fulfilled");
    assert.equal(execution.resourceSettlement.kind, "pending");
    assert.equal(execution.resourceSettlement.operation, "revalidate");
    assert.equal(execution.resourceSettlement.continuity, "preserveVisibleValue");
    assert.equal(execution.resourceSettlement.digest, form.resourceSource().settlement.digest);
  });
});

test("signals.form carries failed resource settlement on denied resource-line submit", async () => {
  await withSignals((signals) => {
    const form = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: Object.freeze({
            kind: "timedOut",
            operation: "refresh",
            continuity: "preservedVisibleValue",
          }),
          freshness: Object.freeze({ kind: "stale", reason: "refreshTimedOut" }),
        }),
        { id: "resource-settlement-denied-submit" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
    });

    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "denied");
    assert.equal(execution.effectStarted, false);
    assert.equal(execution.resourceSettlement.kind, "failed");
    assert.equal(execution.resourceSettlement.failureKind, "timedOut");
    assert.equal(execution.resourceSettlement.operation, "refresh");
    assert.equal(execution.resourceSettlement.retryRecommended, true);
    assert.equal(execution.resourceSettlement.digest, form.resourceSource().settlement.digest);
  });
});

test("signals.form does not launder confirmed resource settlement into denied submit when the blocker is non-resource", async () => {
  await withSignals((signals) => {
    const form = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: Object.freeze({ kind: "fulfilled", operation: "delivery" }),
          freshness: Object.freeze({ kind: "fresh" }),
          mutationResponse: createMutationResponsePlanFixture({
            confirmationKind: "exactCanonicalTruth",
            fallbackKind: "none",
          }),
        }),
        { id: "resource-settlement-non-resource-denial" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
      validation: ({ field }) => ({
        titleRequired: field("title", (value) => (
          value
            ? { kind: "valid", field: "title", digest: value }
            : {
              kind: "invalid",
              field: "title",
              message: {
                code: "title.required",
                message: "title is required",
                severity: "error",
                target: "title",
                audience: "user",
                visibility: "visible",
              },
            }
        )),
      }),
    });

    form.fields.title.set("");
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "denied");
    assert.equal(execution.resourceSettlement, null);
    assert.equal(
      form.resourceSource().settlement.kind,
      "confirmed",
    );
    assert.equal(
      execution.attempt.blockers.some((blocker) => blocker.kind === "validation:invalid"),
      true,
    );
  });
});

test("signals.form does not launder confirmed resource settlement into denied submit when the blocker is stale freshness", async () => {
  await withSignals((signals) => {
    const form = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: Object.freeze({ kind: "fulfilled", operation: "delivery" }),
          freshness: Object.freeze({ kind: "stale", reason: "revalidationRequired" }),
          mutationResponse: createMutationResponsePlanFixture({
            confirmationKind: "exactCanonicalTruth",
            fallbackKind: "none",
          }),
        }),
        { id: "resource-settlement-stale-denial" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
    });

    form.fields.title.set("Edited locally");
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "denied");
    assert.equal(execution.resourceSettlement, null);
    assert.equal(form.resourceSource().settlement.kind, "confirmed");
    assert.equal(
      execution.attempt.blockers.some((blocker) => blocker.kind === "resource:stale"),
      true,
    );
  });
});

test("signals.form does not launder confirmed resource settlement into denied submit when the blocker is delivery-basis drift", async () => {
  await withSignals((signals) => {
    const driftBaseLine = createReadOnlyResourceLineFixture({
      status: Object.freeze({ kind: "fulfilled", operation: "delivery" }),
      freshness: Object.freeze({ kind: "stale", reason: "deliveryInvalidate" }),
      mutationResponse: createMutationResponsePlanFixture({
        confirmationKind: "exactCanonicalTruth",
        fallbackKind: "none",
      }),
    });
    const driftLine = Object.freeze({
      ...driftBaseLine,
      summary() {
        const summary = driftBaseLine.summary();
        return Object.freeze({
          ...summary,
          current: Object.freeze({
            ...summary.current,
            freshness: Object.freeze({ kind: "stale", reason: "deliveryInvalidate" }),
          }),
          diagnostics: Object.freeze({
            ...summary.diagnostics,
            current: Object.freeze({
              ...summary.diagnostics.current,
              freshness: Object.freeze({ kind: "stale", reason: "deliveryInvalidate" }),
            }),
            latest: Object.freeze({
              ...summary.diagnostics.latest,
              basisCurrentId: "basis-2",
              deliveryKind: "basisRefresh",
              deliveryScope: "basis",
              deliveryBasisId: "basis-2",
              invalidationCause: "deliveryInvalidate",
              invalidationScope: "line",
            }),
          }),
        });
      },
      freshness() {
        return Object.freeze({ kind: "stale", reason: "deliveryInvalidate" });
      },
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(driftLine, { id: "resource-settlement-delivery-drift-denial" }),
      fields: ({ field }) => ({ title: field("title") }),
    });

    form.fields.title.set("Edited locally");
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "denied");
    assert.equal(execution.resourceSettlement, null);
    assert.equal(form.resourceSource().settlement.kind, "confirmed");
    assert.equal(
      execution.attempt.blockers.some((blocker) => blocker.kind === "resource:deliveryBasisDrift"),
      true,
    );
  });
});
