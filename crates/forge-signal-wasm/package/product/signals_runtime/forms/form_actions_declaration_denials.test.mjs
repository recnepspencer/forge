import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form denies malformed action declarations before planning", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());

    assert.throws(
      () => signals.form({
        source: { title: "Ship docs" },
        fields: ({ field }) => ({ title: field("title") }),
        actions: ({ action }) => ({ invalid: action("save", { patchPolicy: "sometimes" }) }),
      }),
      /action patch policy is not supported/,
    );

    assert.throws(
      () => signals.form({
        source: { title: "Ship docs" },
        fields: ({ field }) => ({ title: field("title") }),
        actions: ({ submit, action }) => ({ submit: submit(), duplicate: action("submit") }),
      }),
      /action declaration ids must be unique/,
    );

    assert.throws(
      () => signals.form({
        source: { title: "Ship docs" },
        fields: ({ field }) => ({ title: field("title") }),
        actions: ({ action }) => ({ impersonator: action("impersonator", { kind: "step" }) }),
      }),
      /custom actions cannot impersonate built-in action kinds/,
    );

    assert.throws(
      () => signals.form({
        source: { title: "Ship docs" },
        fields: ({ field }) => ({ title: field("title") }),
        steps: ({ step }) => ({ details: step("details", ["title"]) }),
        actions: ({ step }) => ({ missing: step("missing", "review", "next") }),
      }),
      /step action references an undeclared step/,
    );

    assert.throws(
      () => signals.form({
        source: { title: "Ship docs" },
        fields: ({ field }) => ({ title: field("title") }),
        actions: ({ action }) => ({ malformedRoute: action("malformedRoute", { routeCoupled: true }) }),
      }),
      /only step actions may declare route-coupled posture/,
    );

    assert.throws(
      () => signals.form({
        source: { title: "Ship docs" },
        fields: ({ field }) => ({ title: field("title") }),
        steps: ({ step }) => ({ details: step("details", ["title"]) }),
        actions: ({ step }) => ({ malformedRoute: step("malformedRoute", "details", "next", { routeCoupled: "yes" }) }),
      }),
      /action routeCoupled posture must be a boolean/,
    );

    assert.throws(
      () => signals.form({
        source: { title: "Ship docs" },
        fields: ({ field }) => ({ title: field("title") }),
        actions: ({ action }) => ({
          invalidResourceLifecycle: action("invalidResourceLifecycle", {
            resourceAction: { kind: "refresh" },
            patchPolicy: "allowEmpty",
          }),
        }),
      }),
      /resource-line lifecycle actions require ignore patch policy/,
    );

    assert.throws(
      () => signals.form({
        source: { title: "Ship docs" },
        fields: ({ field }) => ({ title: field("title") }),
        actions: ({ action }) => ({
          invalidLifecycleProfile: action("invalidLifecycleProfile", {
            resourceAction: { kind: "revalidate" },
            resourceEffectProfile: signals.resource.effects.branchNative(),
          }),
        }),
      }),
      /resource-line lifecycle actions cannot declare resourceEffectProfile/,
    );

    assert.throws(
      () => signals.form({
        source: { title: "Ship docs" },
        fields: ({ field }) => ({ title: field("title") }),
        actions: ({ action }) => ({
          invalidRecoveryPolicy: action("invalidRecoveryPolicy", {
            resourceAction: { kind: "restoreExact" },
            patchPolicy: "allowEmpty",
          }),
        }),
      }),
      /resource-line recovery actions require ignore patch policy/,
    );

    assert.throws(
      () => signals.form({
        source: { title: "Ship docs" },
        fields: ({ field }) => ({ title: field("title") }),
        actions: ({ action }) => ({
          invalidRecoveryProfile: action("invalidRecoveryProfile", {
            resourceAction: { kind: "rollbackLastEffect" },
            resourceEffectProfile: signals.resource.effects.branchNative(),
          }),
        }),
      }),
      /resource-line recovery actions cannot declare resourceEffectProfile/,
    );

    assert.throws(
      () => signals.form({
        source: { title: "Ship docs" },
        fields: ({ field, evidence }) => ({
          title: field("title"),
          evidence: evidence("evidence", { attachmentIdentity: "digest" }),
        }),
        actions: ({ action }) => ({
          addEvidence: action("addEvidence", {
            resourceAction: { kind: "patchPlan", fields: ["missingField"] },
          }),
        }),
      }),
      /cannot reference undeclared fields/,
    );

    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({ title: field("title") }),
    });
    assert.throws(() => form.actionPlan("missing"), /form action is not declared/);
  } finally {
    await cleanup();
  }
});
