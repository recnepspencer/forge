import assert from "node:assert/strict";
import test from "node:test";

import { withSignals } from "../../action_execution_test_helpers.mjs";
import { createReadOnlyResourceLineFixture } from "../fixtures/resource_line_fixture.mjs";

test("signals.form carries resource-aware recovery hints directly on denied resource-line execution artifacts", async () => {
  await withSignals((signals) => {
    const staleLine = createReadOnlyResourceLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      status: Object.freeze({ kind: "fulfilled", operation: "initialLoad" }),
      freshness: Object.freeze({ kind: "stale", reason: "revalidationRequired" }),
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(staleLine, { id: "resource-execution-recovery" }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action }) => ({
        revalidateResourceSource: action("revalidateResourceSource", {
          resourceAction: { kind: "revalidate" },
        }),
      }),
    });

    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "denied");
    assert.equal(execution.effectStarted, false);
    assert.deepEqual(
      execution.recoveryActions.map((action) => action.kind),
      ["focusFirstActionableBlocker", "revalidateResourceSource", "replayExactResourceSource"],
    );
    assert.equal(
      execution.recoveryActions.find((action) => action.kind === "revalidateResourceSource")?.action,
      "revalidateResourceSource",
    );
  });
});

test("signals.form denied execution artifacts preserve combined semantic and resource recovery hints", async () => {
  await withSignals((signals) => {
    const driftBaseLine = createReadOnlyResourceLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      status: Object.freeze({ kind: "fulfilled", operation: "delivery" }),
      freshness: Object.freeze({ kind: "stale", reason: "deliveryInvalidate" }),
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
      source: signals.form.source.resourceLine(driftLine, { id: "resource-execution-delivery-drift" }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action }) => ({
        refreshResourceSource: action("refreshResourceSource", {
          resourceAction: { kind: "refresh" },
        }),
        revalidateResourceSource: action("revalidateResourceSource", {
          resourceAction: { kind: "revalidate" },
        }),
      }),
    });

    form.fields.title.set("Needs reconciliation");
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "denied");
    assert.deepEqual(
      execution.recoveryActions.map((action) => action.kind),
      [
        "focusFirstActionableBlocker",
        "acceptCanonicalValue",
        "revalidateResourceSource",
        "refreshResourceSource",
      ],
    );
    assert.equal(
      execution.recoveryActions.find((action) => action.kind === "refreshResourceSource")?.action,
      "refreshResourceSource",
    );
  });
});
