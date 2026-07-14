import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form coalesces layout measurements by animation frame without mutating semantic truth", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    let validationReads = 0;
    const form = signals.form({
      source: { title: "", summary: "" },
      measurement: {
        maxRetainedSnapshots: 4,
      },
      fields: ({ field }) => ({
        title: field("title", {
          row: "hero",
          column: "left",
        }),
        summary: field("summary", {
          row: "hero",
          column: "right",
        }),
      }),
      validation: ({ field }) => ({
        titleRequired: field("title", (value) => {
          validationReads += 1;
          return value.trim().length === 0
            ? {
                kind: "invalid",
                message: {
                  code: "title.required",
                  severity: "error",
                  target: "title",
                  visibility: "visible",
                },
              }
            : true;
        }),
      }),
      actions: ({ submit }) => ({
        submit: submit(),
      }),
    });

    const before = form.verification();
    const validationReadsAfterVerification = validationReads;
    const resizeSnapshot = form.recordLayoutMeasurement([
      { row: "hero", labelHeight: 24, controlHeight: 32 },
    ], {
      cause: "resizeObserver",
      frameToken: "frame-1",
    });
    const fontSnapshot = form.recordLayoutMeasurement([
      { row: "hero", labelHeight: 26, controlHeight: 36, messageHeight: 18 },
    ], {
      cause: "fontLoad",
      frameToken: "frame-1",
    });
    const report = form.layoutMeasurement();
    assert.equal(validationReads, validationReadsAfterVerification);
    const after = form.verification();

    assert.equal(resizeSnapshot.snapshotId, fontSnapshot.snapshotId);
    assert.equal(report.snapshots.length, 1);
    assert.deepEqual(report.latestSnapshot.causes, ["resizeObserver", "fontLoad"]);
    assert.equal(report.latestSnapshot.rows[0].labelHeight, 26);
    assert.equal(report.latestSnapshot.rows[0].messageHeight, 18);
    assert.equal(report.counters.coalescedWrites, 1);
    assert.equal(before.digests.validationDigest, after.digests.validationDigest);
    assert.equal(before.digests.readinessDigest, after.digests.readinessDigest);
    assert.equal(before.digests.actionPlanDigestSetDigest, after.digests.actionPlanDigestSetDigest);
    assert.equal(after.digests.layoutMeasurementDigest, report.digest);
  } finally {
    await cleanup();
  }
});

test("signals.form records bounded layout measurement history across declared causes", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "" },
      measurement: {
        observe: ["viewport", "asyncMessage", "animationFrame"],
        maxRetainedSnapshots: 2,
      },
      fields: ({ field }) => ({
        title: field("title", {
          row: "hero",
          column: "full",
        }),
      }),
    });

    form.recordLayoutMeasurement([{ row: "hero", controlHeight: 40 }], {
      cause: "viewport",
      frameToken: "frame-1",
    });
    form.recordLayoutMeasurement([{ row: "hero", controlHeight: 44 }], {
      cause: "asyncMessage",
      frameToken: "frame-2",
    });
    form.recordLayoutMeasurement([{ row: "hero", controlHeight: 48 }], {
      cause: "animationFrame",
      frameToken: "frame-3",
    });

    const report = form.layoutMeasurement();
    assert.equal(report.snapshots.length, 2);
    assert.deepEqual(report.snapshots.map((snapshot) => snapshot.frameToken), ["frame-2", "frame-3"]);
    assert.equal(report.counters.retainedSnapshots, 2);
    assert.equal(report.counters.observedCauseCount, 3);
    assert.equal(report.counters.measuredRows, 2);
  } finally {
    await cleanup();
  }
});

test("signals.form denies undeclared or malformed layout measurement inputs", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "" },
      measurement: {
        observe: ["viewport"],
      },
      fields: ({ field }) => ({
        title: field("title", {
          row: "hero",
        }),
      }),
    });

    assert.throws(
      () => form.recordLayoutMeasurement([{ row: "hero", controlHeight: 30 }], {
        cause: "fontLoad",
      }),
      /cause is not declared/,
    );
    assert.throws(
      () => form.recordLayoutMeasurement([{ row: "hero", controlHeight: -1 }], {
        cause: "viewport",
      }),
      /must be a non-negative finite number/,
    );
    assert.throws(
      () => form.recordLayoutMeasurement([{ row: "missing", controlHeight: 30 }], {
        cause: "viewport",
      }),
      /must reference a declared layout row/,
    );
  } finally {
    await cleanup();
  }
});
