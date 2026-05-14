import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form input capability report keeps unavailable posture explicit across controller diagnostics and verification", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: {
        title: "Ship docs",
        notes: "Plain text",
      },
      fields: ({ field }) => ({
        title: field("title", {
          adapter: {
            tier: "externalImperative",
            reportsRawInput: false,
            reportsCommitBoundary: false,
            reportsComposition: false,
            reportsFocus: false,
            supportsMessageTrack: false,
            supportsMinHeightSync: false,
            supportsResponsiveTokens: false,
          },
        }),
        notes: field("notes", {
          adapter: {
            tier: "signalNative",
          },
        }),
      }),
    });

    const inputCapabilities = form.inputCapabilities();
    assert.equal(inputCapabilities.summary.total, 2);
    assert.equal(inputCapabilities.summary.unavailableFields, 1);
    assert.equal(inputCapabilities.summary.rawInputUnavailableFields, 1);
    assert.equal(inputCapabilities.summary.commitBoundaryUnavailableFields, 1);
    assert.equal(inputCapabilities.summary.compositionUnavailableFields, 1);
    assert.equal(inputCapabilities.summary.focusUnavailableFields, 1);
    assert.equal(inputCapabilities.summary.messageTrackUnavailableFields, 1);
    assert.equal(inputCapabilities.summary.minHeightSyncUnavailableFields, 1);
    assert.equal(inputCapabilities.summary.responsiveTokenUnavailableFields, 1);
    assert.equal(inputCapabilities.counters.externalImperativeFields, 1);
    assert.equal(inputCapabilities.counters.signalNativeFields, 1);

    const titleField = inputCapabilities.fields.find((field) => field.field === "title");
    assert.equal(titleField?.posture, "unavailable");
    assert.match(titleField?.reason ?? "", /cannot honor/);
    assert.deepEqual(
      titleField?.unavailableCapabilities,
      [
        {
          capability: "reportsRawInput",
          reason: "externalImperative adapter did not declare reportsRawInput",
        },
        {
          capability: "reportsCommitBoundary",
          reason: "externalImperative adapter did not declare reportsCommitBoundary",
        },
        {
          capability: "reportsComposition",
          reason: "externalImperative adapter did not declare reportsComposition",
        },
        {
          capability: "reportsFocus",
          reason: "externalImperative adapter did not declare reportsFocus",
        },
        {
          capability: "supportsMessageTrack",
          reason: "externalImperative adapter did not declare supportsMessageTrack",
        },
        {
          capability: "supportsMinHeightSync",
          reason: "externalImperative adapter did not declare supportsMinHeightSync",
        },
        {
          capability: "supportsResponsiveTokens",
          reason: "externalImperative adapter did not declare supportsResponsiveTokens",
        },
      ],
    );

    assert.equal(form.diagnostics().inputCapabilities.digest, inputCapabilities.digest);
    assert.equal(form.verification().digests.inputCapabilityDigest, inputCapabilities.digest);
    assert.equal(
      form.verification().performanceEnvelope.inputCapabilities.unavailableCapabilities,
      7,
    );
  } finally {
    await cleanup();
  }
});
