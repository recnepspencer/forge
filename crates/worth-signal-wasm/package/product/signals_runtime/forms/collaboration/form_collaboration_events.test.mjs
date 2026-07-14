import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form collaboration exposes lock lease presence comment and branch changes as first-class events", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs", notes: "Ready" },
      collaboration: {
        mode: "fieldLease",
        actorId: "me",
        supportsPresence: true,
        supportsComments: true,
      },
      fields: ({ field }) => ({
        title: field("title"),
        notes: field("notes"),
      }),
    });

    form.reportCollaboration({
      posture: "settling",
      leasedFields: [{ field: "title", ownerId: "peer-1" }],
      branchId: "branch-a",
      remoteUpdateDigest: "remote:delta-1",
      presence: [{ actorId: "peer-1", status: "active" }],
      comments: [{ id: "comment-1", authorId: "peer-1", target: "title" }],
      reason: "peer title update is settling",
    });
    form.reportCollaboration({
      posture: "blocked",
      lockOwnerId: "peer-2",
      leasedFields: [{ field: "notes", ownerId: "peer-2" }],
      branchId: "branch-b",
      remoteUpdateDigest: "remote:delta-2",
      presence: [
        { actorId: "peer-1", status: "idle" },
        { actorId: "peer-2", status: "viewing" },
      ],
      comments: [
        { id: "comment-1", authorId: "peer-1", target: "title" },
        { id: "comment-2", authorId: "peer-2", target: "notes" },
      ],
      reason: "lease and review advanced",
    });
    form.clearCollaboration({ reason: "collaboration cleared after review" });

    const report = form.collaboration();
    const eventKinds = report.events.map((event) => event.kind);
    assert.ok(eventKinds.includes("postureChange"));
    assert.ok(eventKinds.includes("lockChange"));
    assert.ok(eventKinds.includes("leaseChange"));
    assert.ok(eventKinds.includes("branchChange"));
    assert.ok(eventKinds.includes("presenceChange"));
    assert.ok(eventKinds.includes("commentChange"));
    assert.ok(eventKinds.includes("remoteUpdateChange"));
    assert.equal(report.counters.eventArtifacts, report.events.length);
    assert.equal(
      report.counters.lockChanges,
      report.events.filter((event) => event.kind === "lockChange").length,
    );
    assert.equal(
      report.counters.commentChanges,
      report.events.filter((event) => event.kind === "commentChange").length,
    );
    assert.equal(report.events.at(-1)?.source, "clear");
    assert.equal(report.events.at(-1)?.reason, "collaboration cleared after review");
    assert.equal(typeof report.events[0]?.digest, "string");
    assert.equal(form.diagnostics().collaboration.eventsDigest, report.eventsDigest);
    assert.equal(form.verification().digests.collaborationEventDigest, report.eventsDigest);
  } finally {
    await cleanup();
  }
});

test("signals.form collaboration denies advisory presence and comments unless explicitly declared", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      collaboration: {
        mode: "fieldLease",
        actorId: "me",
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    assert.throws(
      () =>
        form.reportCollaboration({
          presence: [{ actorId: "peer-1", status: "active" }],
          reason: "presence should require declared support",
        }),
      /supportsPresence: true/,
    );
    assert.throws(
      () =>
        form.reportCollaboration({
          comments: [{ id: "comment-1", authorId: "peer-1", target: "title" }],
          reason: "comments should require declared support",
        }),
      /supportsComments: true/,
    );
  } finally {
    await cleanup();
  }
});

test("signals.form collaboration preserves native numeric branch identity and does not fabricate branch changes from string coercion", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const { createReadOnlyResourceLineFixture } = await import("../resource_source/fixtures/resource_line_fixture.mjs");
    const form = signals.form({
      source: signals.form.source.resourceLine(createReadOnlyResourceLineFixture({
        status: Object.freeze({ kind: "fulfilled", operation: "initialLoad" }),
        freshness: Object.freeze({ kind: "fresh" }),
        visibleSelection: Object.freeze({
          kind: "speculative",
          source: "localPatch",
          effectId: "effect-1",
          branchId: 7,
          snapshotId: 11,
          basisId: "basis-1",
          rollbackKind: "compactInverseAvailable",
          detail: "resource line is showing speculative branch truth",
        }),
      }), { id: "branch-collaboration-events" }),
      collaboration: {
        mode: "branchPerActor",
        actorId: "me",
        supportsPresence: true,
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    form.reportCollaboration({
      posture: "settling",
      branchId: 7,
      presence: [{ actorId: "peer-1", status: "active" }],
      reason: "branch-backed collaboration update is settling",
    });

    const report = form.collaboration();
    assert.equal(report.branchId, 7);
    assert.equal(typeof report.branchId, "number");
    assert.equal(
      report.events.filter((event) => event.kind === "branchChange").length,
      1,
    );
    assert.equal(report.events.find((event) => event.kind === "branchChange")?.branchId, 7);
  } finally {
    await cleanup();
  }
});

test("signals.form collaboration defaults report posture and reason without requiring explicit proof-bearing inputs", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      collaboration: {
        mode: "fieldLease",
        actorId: "me",
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    const artifact = form.reportCollaboration({
      leasedFields: [{ field: "title", ownerId: "peer-1" }],
    });

    assert.equal(artifact.posture, "active");
    assert.equal(artifact.reason, "collaboration posture is settled");
    assert.equal(form.collaboration().history.at(-1)?.reason, "collaboration posture is settled");
  } finally {
    await cleanup();
  }
});
