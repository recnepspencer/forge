import assert from "node:assert/strict";
import test from "node:test";

import { createDeferred } from "../runtime_fixture/async/deferred.mjs";
import { createRealLifecycleRuntime } from "../runtime_fixture/real_lifecycle_runtime.mjs";
import {
  snapshotLifecycleCore,
  snapshotLifecycleSupersession,
} from "../runtime_fixture/proof/lifecycle_history_entries.mjs";

test("resource line history carries initial async lifecycle truth through settlement", async () => {
  const runtime = await createRealLifecycleRuntime();
  try {
    const { mod, resource } = runtime;
    const deferred = createDeferred();
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: () => deferred.promise,
    });

    const line = detail.line({ productId: "p1" });

    assert.deepEqual(
      snapshotLifecycleCore(line.history().lifecycle),
      [{
        sequence: 1,
        event: "materialized",
        lastOutcome: "pending",
        status: {
          kind: "pending",
          operation: "initialLoad",
          continuity: "noVisibleValueYet",
        },
        freshness: { kind: "stale", reason: "initialLoadPending" },
        visibleValueVersion: 0,
      }],
    );

    deferred.resolve({ id: "p1", version: 1 });
    await deferred.promise;
    await Promise.resolve();

    assert.deepEqual(
      snapshotLifecycleCore(line.history().lifecycle),
      [{
        sequence: 1,
        event: "materialized",
        lastOutcome: "pending",
        status: {
          kind: "pending",
          operation: "initialLoad",
          continuity: "noVisibleValueYet",
        },
        freshness: { kind: "stale", reason: "initialLoadPending" },
        visibleValueVersion: 0,
      }, {
        sequence: 2,
        event: "fulfilled",
        lastOutcome: "fulfilled",
        status: {
          kind: "fulfilled",
          operation: "initialLoad",
        },
        freshness: { kind: "fresh" },
        visibleValueVersion: 1,
      }],
    );
  } finally {
    await runtime.cleanup();
  }
});

test("resource line history records rejection continuity and invalidation causes", async () => {
  const runtime = await createRealLifecycleRuntime();
  try {
    const { mod, resource } = runtime;
    let shouldFail = false;
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => {
        if (shouldFail) {
          throw new Error("refresh failed");
        }
        return { id: productId, version: 1 };
      },
    });

    const line = detail.line({ productId: "p2" });
    shouldFail = true;
    line.refresh();
    line.invalidate();

    assert.deepEqual(
      line.history().lifecycle.map((entry) => ({
        sequence: entry.sequence,
        event: entry.event,
        lastOutcome: entry.lastOutcome,
        status: entry.status,
        freshness: entry.freshness,
        lastErrorMessage: entry.lastErrorMessage,
        preservedVisibleValueOnLastRejection:
          entry.preservedVisibleValueOnLastRejection,
        lastInvalidationCause: entry.lastInvalidationCause,
      })),
      [{
        sequence: 1,
        event: "materialized",
        lastOutcome: "fulfilled",
        status: {
          kind: "fulfilled",
          operation: "initialLoad",
        },
        freshness: { kind: "fresh" },
        lastErrorMessage: null,
        preservedVisibleValueOnLastRejection: false,
        lastInvalidationCause: null,
      }, {
        sequence: 2,
        event: "rejected",
        lastOutcome: "rejected",
        status: {
          kind: "rejected",
          operation: "refresh",
          message: "refresh failed",
          continuity: "preservedVisibleValue",
        },
        freshness: { kind: "stale", reason: "refreshRejected" },
        lastErrorMessage: "refresh failed",
        preservedVisibleValueOnLastRejection: true,
        lastInvalidationCause: null,
      }, {
        sequence: 3,
        event: "invalidated",
        lastOutcome: "rejected",
        status: {
          kind: "rejected",
          operation: "refresh",
          message: "refresh failed",
          continuity: "preservedVisibleValue",
        },
        freshness: { kind: "stale", reason: "manualLineInvalidate" },
        lastErrorMessage: "refresh failed",
        preservedVisibleValueOnLastRejection: true,
        lastInvalidationCause: "manualLineInvalidate",
      }],
    );
  } finally {
    await runtime.cleanup();
  }
});

test("resource line history records superseded lifecycle explicitly", async () => {
  const runtime = await createRealLifecycleRuntime();
  try {
    const { mod, resource } = runtime;
    const initialDeferred = createDeferred();
    let callCount = 0;
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => {
        callCount += 1;
        if (callCount === 1) {
          return initialDeferred.promise;
        }
        return Promise.resolve({ id: productId, version: 2 });
      },
    });

    const line = detail.line({ productId: "p3" });
    line.refresh();
    await new Promise((resolve) => setTimeout(resolve, 0));

    assert.deepEqual(
      snapshotLifecycleSupersession(line.history().lifecycle),
      [{
        sequence: 1,
        event: "materialized",
        status: {
          kind: "pending",
          operation: "initialLoad",
          continuity: "noVisibleValueYet",
        },
        supersededOperation: null,
        lastSupersededOperation: null,
      }, {
        sequence: 2,
        event: "superseded",
        status: {
          kind: "pending",
          operation: "initialLoad",
          continuity: "noVisibleValueYet",
        },
        supersededOperation: "initialLoad",
        lastSupersededOperation: null,
      }, {
        sequence: 3,
        event: "pending",
        status: {
          kind: "pending",
          operation: "refresh",
          continuity: "noVisibleValueYet",
        },
        supersededOperation: null,
        lastSupersededOperation: "initialLoad",
      }, {
        sequence: 4,
        event: "fulfilled",
        status: {
          kind: "fulfilled",
          operation: "refresh",
        },
        supersededOperation: null,
        lastSupersededOperation: "initialLoad",
      }],
    );
  } finally {
    await runtime.cleanup();
  }
});
