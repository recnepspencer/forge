import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../../runtime_fixture/graph_operational_runtime.mjs";
import { createReadOnlyResourceLineFixture } from "../fixtures/resource_line_fixture.mjs";

test("signals.form exposes declared lifecycle recovery hints for delivery-basis drift submit posture", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
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
    const driftForm = signals.form({
      source: signals.form.source.resourceLine(driftLine, { id: "resource-delivery-drift-recovery" }),
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

    driftForm.fields.title.set("Needs reconciliation");
    const plan = driftForm.actionPlan("submit");
    assert.equal(plan.status, "denied");
    assert.deepEqual(
      plan.readiness.blockers.map((blocker) => blocker.kind),
      ["resource:actionUnavailable", "resource:deliveryBasisDrift"],
    );
    assert.deepEqual(
      plan.recoveryActions.map((action) => action.kind),
      [
        "focusFirstActionableBlocker",
        "acceptCanonicalValue",
        "revalidateResourceSource",
        "refreshResourceSource",
      ],
    );
    assert.equal(
      plan.recoveryActions.find((action) => action.kind === "revalidateResourceSource")?.action,
      "revalidateResourceSource",
    );
    assert.equal(
      plan.recoveryActions.find((action) => action.kind === "refreshResourceSource")?.action,
      "refreshResourceSource",
    );
    const attempt = driftForm.attemptAction("submit");
    assert.deepEqual(
      attempt.recoveryActions.map((action) => action.kind),
      [
        "focusFirstActionableBlocker",
        "acceptCanonicalValue",
        "revalidateResourceSource",
        "refreshResourceSource",
      ],
    );
  } finally {
    await cleanup();
  }
});
