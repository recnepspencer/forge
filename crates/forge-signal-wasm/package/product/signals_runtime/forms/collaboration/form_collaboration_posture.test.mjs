import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form collaboration posture blocks writes and submit without mutating semantic truth", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      collaboration: {
        mode: "singleWriterLock",
        actorId: "me",
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ submit }) => ({
        submit: submit(),
      }),
      presentation: {
        collaboration: { scope: "wholeForm" },
      },
    });

    const before = form.verification();
    const artifact = form.reportCollaboration({
      posture: "blocked",
      lockOwnerId: "reviewer-1",
      reason: "reviewer-1 currently owns the draft lock",
    });
    assert.equal(artifact.kind, "collaboration");
    assert.equal(form.collaboration().posture, "blocked");
    assert.equal(form.presentationLifecycle("collaboration").status, "busy");
    assert.equal(form.fieldWritePosture("title").canWrite, false);
    assert.equal(form.readiness().canSubmit, false);
    assert.equal(form.actionPlan("submit").status, "denied");
    assert.throws(() => form.fields.title.set("Blocked"), /reviewer-1 currently owns the draft lock/);
    assert.equal(form.effective().title, "Ship docs");

    const after = form.verification();
    assert.notEqual(after.digests.collaborationDigest, before.digests.collaborationDigest);
    assert.equal(after.digests.semanticEqualityDigest, before.digests.semanticEqualityDigest);
    assert.equal(after.digests.patchPlanDigest, before.digests.patchPlanDigest);
  } finally {
    await cleanup();
  }
});

test("signals.form collaboration field leases block only leased edited fields and preserve presentation settling", async () => {
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
      actions: ({ submit }) => ({
        submit: submit(),
      }),
      presentation: {
        collaboration: { scope: "wholeForm", settlementAcknowledgement: "required" },
      },
    });

    form.fields.notes.set("Local notes");
    form.reportCollaboration({
      posture: "settling",
      leasedFields: [{ field: "title", ownerId: "peer-1" }],
      remoteUpdateDigest: "remote:delta-1",
      presence: [{ actorId: "peer-1", status: "active" }],
      comments: [{ id: "comment-1", authorId: "peer-1", target: "title" }],
      reason: "remote title update is settling",
    });

    assert.equal(form.presentationLifecycle("collaboration").status, "settling");
    assert.equal(form.collaboration().counters.presenceActors, 1);
    assert.equal(form.collaboration().counters.commentArtifacts, 1);
    assert.equal(form.fieldWritePosture("notes").canWrite, true);
    assert.equal(form.fieldWritePosture("title").canWrite, false);
    assert.equal(form.actionPlan("submit").status, "accepted");

    form.fields.notes.set("Local notes updated");
    assert.throws(() => form.fields.title.set("Peer title"), /remote title update is settling/);

    const acknowledgement = form.acknowledgePresentation("collaboration");
    assert.equal(acknowledgement.resultKind, "acknowledged");
    assert.equal(form.presentationLifecycle("collaboration").status, "ready");
  } finally {
    await cleanup();
  }
});

test("signals.form collaboration unavailable posture stays explicit without blocking ordinary local editing", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      collaboration: {
        mode: "unavailable",
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      presentation: {
        collaboration: { unavailableAcknowledgement: "required" },
      },
    });

    assert.equal(form.collaboration().posture, "unavailable");
    assert.equal(form.presentationLifecycle("collaboration").status, "unavailable");
    form.fields.title.set("Local edit still allowed");
    assert.equal(form.effective().title, "Local edit still allowed");
  } finally {
    await cleanup();
  }
});

test("signals.form declared collaboration denies generic collaboration presentation lane updates", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      collaboration: {
        mode: "singleWriterLock",
        actorId: "me",
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    assert.throws(
      () =>
        form.reportPresentationLane("collaboration", {
          status: "busy",
          reason: "should use collaboration authority",
        }),
      /reportCollaboration\/clearCollaboration/,
    );
    assert.throws(
      () => form.clearPresentationLane("collaboration"),
      /reportCollaboration\/clearCollaboration/,
    );
  } finally {
    await cleanup();
  }
});

test("signals.form reviewer-comment-only collaboration blocks mutation with first-class posture", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      collaboration: {
        mode: "reviewerCommentOnly",
        actorId: "reviewer-1",
        supportsComments: true,
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ submit }) => ({
        submit: submit(),
      }),
    });

    const report = form.collaboration();
    assert.equal(report.readOnly, true);
    assert.equal(form.fieldWritePosture("title").canWrite, false);
    assert.equal(form.readiness().canSubmit, false);
    assert.equal(form.actionPlan("submit").status, "denied");
    assert.throws(() => form.fields.title.set("Blocked"), /collaboration posture is settled/);
  } finally {
    await cleanup();
  }
});

test("signals.form collaboration updates deny undeclared leased fields and unsupported presence status", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      collaboration: {
        mode: "fieldLease",
        actorId: "me",
        supportsPresence: true,
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    assert.throws(
      () =>
        form.reportCollaboration({
          leasedFields: [{ field: "notes", ownerId: "peer-1" }],
          reason: "bad lease target",
        }),
      /undeclared field/,
    );

    assert.throws(
      () =>
        form.reportCollaboration({
          presence: [{ actorId: "peer-1", status: "teleporting" }],
          reason: "bad presence status",
        }),
      /presence status is not supported/,
    );
  } finally {
    await cleanup();
  }
});

test("signals.form branch-per-actor collaboration consumes admitted resource branch proof", async () => {
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
      }), { id: "branch-collaboration" }),
      collaboration: {
        mode: "branchPerActor",
        actorId: "me",
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    const report = form.collaboration();
    assert.equal(report.posture, "active");
    assert.equal(report.branchId, 7);
    assert.equal(report.resourceProof.required, true);
    assert.equal(report.resourceProof.admitted, true);
    assert.equal(report.resourceProof.visibleSelectionKind, "speculative");
    assert.equal(report.counters.resourceProofRequired, 1);
    assert.equal(report.counters.resourceProofUnavailable, 0);
    assert.equal(form.fieldWritePosture("title").canWrite, true);
  } finally {
    await cleanup();
  }
});

test("signals.form branch-backed collaboration stays typed unavailable when no resource branch proof exists", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      collaboration: {
        mode: "optimisticMerge",
        actorId: "me",
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ submit }) => ({
        submit: submit(),
      }),
    });

    const report = form.collaboration();
    assert.equal(report.posture, "unavailable");
    assert.equal(report.resourceProof.required, true);
    assert.equal(report.resourceProof.admitted, false);
    assert.match(report.reason, /requires a resource line form source/);

    form.reportCollaboration({
      posture: "active",
      branchId: "peer-branch",
      reason: "should not override missing resource proof",
    });
    const afterReport = form.collaboration();
    assert.equal(afterReport.posture, "unavailable");
    assert.equal(afterReport.branchId, null);
    assert.match(afterReport.reason, /requires a resource line form source/);

    assert.equal(form.fieldWritePosture("title").canWrite, false);
    assert.equal(
      form.fieldWritePosture("title").blockers[0]?.kind,
      "collaboration:resourceProofUnavailable",
    );
    assert.equal(form.readiness().canSubmit, false);
    assert.equal(
      form.readiness().blockers[0]?.kind,
      "collaboration:resourceProofUnavailable",
    );
    assert.equal(form.actionPlan("submit").status, "denied");
    assert.throws(() => form.fields.title.set("Blocked"), /requires a resource line form source/);
  } finally {
    await cleanup();
  }
});
