import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadStoreModule } from "../../../host_capabilities_certification/module_loading/load_store_module.mjs";
import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

/**
 * Stacked attack with API-shaped latency: delayed GET + in-flight save confirms
 * (setTimeout network tick, deferred HTTP body) racing importGraph while collab
 * form + React store + browser-history freeze are live.
 *
 * Honest supersession seam: binding-local line.value() may still show the open
 * projection, but React store / form submit / late HTTP confirms must fail closed.
 */

const SUPERSESSION_FAIL_CLOSED =
  /superseded|invalidated|not currently available|replaced the worker-owned runtime|cannot be used|unknown signal|unknown branch/u;
const FREEZE_MASK = /router admitted route authority|routeAuthority:frozen/u;
const TASK_ID = "collab-stack";
const LOAD_LATENCY_MS = 35;
const SAVE_NETWORK_TICK_MS = 20;

function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function createDeferred() {
  let resolve;
  const promise = new Promise((res) => {
    resolve = res;
  });
  return { promise, resolve };
}

/** Effect already open; RTT tick then wait for HTTP body before confirm(). */
function simulateInFlightApiSave(line, effectId, responseId) {
  const body = createDeferred();
  const flight = delay(SAVE_NETWORK_TICK_MS)
    .then(() => body.promise)
    .then(() => line.effects().confirm(effectId, { responseId }));
  return {
    flight,
    deliverServerResponse() {
      body.resolve();
    },
  };
}

async function flushMicrotasks() {
  await Promise.resolve();
  await Promise.resolve();
  await delay(0);
}

async function waitUntil(predicate, label, timeoutMs = 2_000) {
  const started = Date.now();
  while (Date.now() - started < timeoutMs) {
    if (await predicate()) {
      return;
    }
    await flushMicrotasks();
  }
  throw new Error(`timed out waiting for ${label}`);
}

async function assertFailsClosed(operation, pattern, { forbidPattern = null } = {}) {
  try {
    const result = operation();
    if (result && typeof result.then === "function") {
      await assert.rejects(() => result, pattern);
      return;
    }
    assert.fail("expected operation to fail closed");
  } catch (error) {
    if (error?.code === "ERR_ASSERTION" && /fail closed/u.test(error.message)) {
      throw error;
    }
    const message = String(error?.message ?? error);
    if (forbidPattern) {
      assert.doesNotMatch(
        message,
        forbidPattern,
        `fail-closed cause must not be masked by ${forbidPattern}`,
      );
    }
    assert.match(message, pattern);
  }
}

function createTaskLine(signals, taskId) {
  return signals.api({
    effects: signals.resource.effects.branchNative(),
  }).url("/tasks/:taskId").response(
    signals.resource.response.detail()({ title: "title", status: "status" }),
  ).detail({
    load: async ({ taskId: id }) => {
      await delay(LOAD_LATENCY_MS);
      return { id, title: "Draft", status: "editing" };
    },
  }).line({ taskId });
}

function defineTaskReviewRoutes(signals) {
  return signals.router.define({
    review: signals.router.route("/review/:taskId", {
      forms: signals.router.forms("task-review", {
        continuity: "freeze",
      }),
    }),
  });
}

test("ATTACK: worker-first collab form + open optimistic line + router freeze fails closed on importGraph", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const {
    createSignals,
    cleanup: cleanupSignals,
    resourcePatch,
  } = await loadSignalsModule({ rawSurface: "real" });
  const { createReactSignalsStore, cleanup: cleanupStore } = await loadStoreModule();

  let signals = null;
  let compatibility = null;
  let line = null;
  let store = null;
  let unsubscribeLine = null;
  let unsubscribeDiagnostics = null;
  const inFlightSaves = [];

  try {
    signals = await createSignals({ deployment: "workerFirst" });
    store = createReactSignalsStore(signals);
    const diagnosticsPulses = [];
    unsubscribeDiagnostics = store.subscribeDiagnostics(() => {
      diagnosticsPulses.push(store.getDiagnosticsSnapshot());
    });

    // --- Phase 1: delayed GET, then open branchNative effect ---
    line = createTaskLine(signals, TASK_ID);
    assert.equal(line.status().kind, "pending");
    assert.equal(line.status().operation, "initialLoad");
    const settled = await line.awaitSettlement({ timeoutMs: 5_000 });
    assert.equal(settled.resultKind, "fulfilled");
    assert.deepEqual(line.value(), {
      id: TASK_ID,
      title: "Draft",
      status: "editing",
    });

    await line.patch(resourcePatch.field({
      field: "title",
      value: "Optimistic",
    }));
    const firstOpenIds = line.effects().open().map((effect) => effect.effectId);
    assert.equal(firstOpenIds.length, 1);
    assert.equal(line.value().title, "Optimistic");

    // --- Phase 2: collaborative resource form + second optimistic submit ---
    const form = signals.form({
      source: signals.form.source.resourceLine(line, { id: "collab-stack-form" }),
      collaboration: {
        mode: "branchPerActor",
        actorId: "me",
        supportsPresence: true,
      },
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
      actions: ({ submit }) => ({
        submit: submit({
          resourceEffectProfile: signals.resource.effects.branchNative(),
        }),
      }),
    });

    const collab = form.collaboration();
    assert.equal(collab.posture, "active");
    assert.equal(collab.resourceProof.required, true);
    assert.equal(collab.resourceProof.admitted, true);
    assert.equal(typeof collab.branchId, "number");
    assert.equal(form.fieldWritePosture("title").canWrite, true);

    const lineSignal = line.signal();
    assert.equal(typeof form.summarySignal().id, "string");
    const linePulses = [];
    unsubscribeLine = store.subscribeSignal(lineSignal, () => {
      linePulses.push(store.getSignalSnapshot(lineSignal));
    });
    assert.deepEqual(store.getSignalSnapshot(lineSignal), line.value());

    await form.fields.title.set("Optimistic-2");
    assert.equal(form.effective().title, "Optimistic-2");
    const digestsBeforeSubmit = form.verification().digests;

    const execution = await form.executeAction("submit");
    assert.equal(execution.resultKind, "fulfilled");
    assert.equal(execution.effectStarted, true);
    assert.equal(
      execution.resourceSubmission?.effectProfile?.profile?.name,
      "branchNative",
    );

    await waitUntil(
      () => line.value().title === "Optimistic-2"
        && linePulses.some((value) => value?.title === "Optimistic-2"),
      "line + React store watch catch-up after collaborative form submit",
      5_000,
    );

    const openEffectIds = line.effects().open().map((effect) => effect.effectId);
    assert.equal(openEffectIds.length >= 2, true);
    assert.ok(
      firstOpenIds.every((effectId) => openEffectIds.includes(effectId)),
      "first patch effect must remain open after form submit",
    );
    const visibleSelection = form.resourceSource()?.visibleSelection;
    assert.equal(visibleSelection?.kind, "derivedEffectProjectionBranch");
    assert.equal(visibleSelection?.branchProof?.admitted, true);
    assert.ok(
      openEffectIds.includes(visibleSelection.effectId),
      "visible selection must name a currently open effect",
    );
    assert.equal(form.source().title, "Optimistic-2");
    assert.deepEqual(store.getSignalSnapshot(lineSignal), line.value());
    assert.equal(diagnosticsPulses.length > 0, true);
    assert.notEqual(store.getDiagnosticsSnapshot().latestObservation, null);

    // --- Phase 2b: both open effects enter simulated in-flight HTTP saves ---
    for (const [index, effectId] of openEffectIds.entries()) {
      inFlightSaves.push(
        simulateInFlightApiSave(line, effectId, `api:save:title-${index + 1}`),
      );
    }
    await delay(SAVE_NETWORK_TICK_MS + 5);
    assert.equal(
      line.effects().open().length,
      openEffectIds.length,
      "network ticks must not retire effects before HTTP bodies arrive",
    );

    // --- Phase 3: collaboration proof / presence must not forge merge ---
    const liveBranchId = form.collaboration().resourceProof.branchId;
    assert.equal(liveBranchId, visibleSelection.branchId);
    assert.throws(
      () => form.reportCollaboration({
        posture: "settling",
        branchId: "forged-branch",
        presence: [{ actorId: "peer-1", status: "active" }],
        remoteUpdateDigest: "remote:delta-1",
        reason: "forged branch must fail closed",
      }),
      /must match admitted resource branch proof/u,
    );
    assert.throws(
      () => form.reportCollaboration({
        posture: "settling",
        branchId: liveBranchId === 1 ? 999 : 1,
        presence: [{ actorId: "peer-1", status: "active" }],
        reason: "stale numeric branch must fail closed",
      }),
      /must match admitted resource branch proof/u,
    );

    const digestsBeforePresence = form.verification().digests;
    form.reportCollaboration({
      posture: "settling",
      branchId: liveBranchId,
      presence: [{ actorId: "peer-1", status: "active" }],
      remoteUpdateDigest: "remote:delta-1",
      reason: "peer presence while optimistic API saves are open",
    });
    assert.equal(form.collaboration().posture, "settling");
    assert.equal(form.collaboration().counters.presenceActors, 1);
    assert.notEqual(
      form.verification().digests.collaborationDigest,
      digestsBeforePresence.collaborationDigest,
    );
    assert.notEqual(
      form.verification().digests.semanticEqualityDigest,
      digestsBeforeSubmit.semanticEqualityDigest,
    );

    // --- Phase 4: browser-history router freeze coupled to the same task id ---
    const routes = defineTaskReviewRoutes(signals);
    const boundary = await routes.admitBrowserHistoryIngress(
      signals.router.browserHistory.push(`/review/${TASK_ID}`, {
        routeIdentity: `review:${TASK_ID}`,
        runtimeRouteSourceId: "routeIdentity",
        routeValue: `review:${TASK_ID}`,
      }),
    );
    assert.equal(boundary.outcome().kind, "admitted");
    assert.deepEqual(boundary.outcome().route().params, { taskId: TASK_ID });
    form.bindRouteAuthority(boundary.outcome().route());
    assert.equal(form.routeAuthority().summary.continuityApplied, "frozeDraft");
    assert.equal(
      form.fieldWritePosture("title").blockers[0]?.kind,
      "routeAuthority:frozen",
    );
    assert.throws(() => form.fields.title.set("After freeze"), FREEZE_MASK);
    assert.equal(line.value().title, "Optimistic-2");
    assert.deepEqual(store.getSignalSnapshot(lineSignal), line.value());
    assert.equal(line.effects().open().length, openEffectIds.length);

    // --- Phase 5 knife: supersede, then delayed API bodies arrive late ---
    assert.equal(line.effects().open().length >= 2, true);
    const confirmTargetEffectId = visibleSelection.effectId;

    compatibility = await createSignals({ deployment: "mainThreadCompatibility" });
    const document = compatibility.input(
      { title: "Foreign" },
      { debugName: "collab.stack.foreign" },
    );
    const graph = compatibility.graph("collabStackSupersede", {
      inputs: { document: compatibility.publicInput(document) },
      outputs: { document },
    });
    await signals.importGraph(
      graph.exportDefinition(),
      graph.exportSnapshot(),
    ).ready();

    await assertFailsClosed(
      () => form.executeAction("submit"),
      SUPERSESSION_FAIL_CLOSED,
      { forbidPattern: FREEZE_MASK },
    );

    assert.equal(line.value().title, "Optimistic-2");
    await assertFailsClosed(
      () => store.getSignalSnapshot(lineSignal),
      SUPERSESSION_FAIL_CLOSED,
    );

    for (const save of inFlightSaves) {
      save.deliverServerResponse();
    }
    const lateResults = await Promise.allSettled(
      inFlightSaves.map((save) => save.flight),
    );
    assert.equal(lateResults.length, openEffectIds.length);
    for (const result of lateResults) {
      assert.equal(result.status, "rejected");
      assert.match(String(result.reason?.message ?? result.reason), SUPERSESSION_FAIL_CLOSED);
    }

    const openAfter = line.effects().open().map((effect) => effect.effectId);
    assert.ok(
      openAfter.includes(confirmTargetEffectId),
      "late confirms must not silently retire open effects after supersession",
    );
  } finally {
    for (const save of inFlightSaves) {
      try {
        save.deliverServerResponse();
      } catch {
        // ignore
      }
    }
    await Promise.allSettled(inFlightSaves.map((save) => save.flight));
    try {
      unsubscribeLine?.();
    } catch {
      // ignore
    }
    try {
      unsubscribeDiagnostics?.();
    } catch {
      // ignore
    }
    try {
      store?.dispose();
    } catch {
      // ignore
    }
    try {
      line?.free();
    } catch {
      // ignore
    }
    compatibility?.free();
    if (signals) {
      await Promise.race([
        signals.terminate().catch(() => {}),
        delay(3_000),
      ]);
    }
    await cleanupStore();
    await cleanupSignals();
    globalThis.Worker = previousWorker;
  }
});
