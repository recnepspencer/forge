import assert from "node:assert/strict";
import test from "node:test";

import { createDeferred } from "../runtime_fixture/async/deferred.mjs";
import { createPhase2FamilyCases } from "../runtime_fixture/family_cases/phase2_family_cases.mjs";
import { createRealLifecycleRuntime } from "../runtime_fixture/real_lifecycle_runtime.mjs";
import {
  snapshotLifecycleSupersession,
} from "../runtime_fixture/proof/lifecycle_history_entries.mjs";

test("async-first no-visible-value-yet histories stay coherent across family kinds", async () => {
  const runtime = await createRealLifecycleRuntime();
  try {
    const { mod, resource } = runtime;
    const familyCases = createPhase2FamilyCases(resource, mod);

    for (const familyCase of familyCases) {
      const initialDeferred = createDeferred();
      let callCount = 0;
      const family = familyCase.build({
        load: ({ productId }) => {
          callCount += 1;
          if (callCount === 1) {
            return initialDeferred.promise;
          }
          return Promise.resolve(familyCase.changedValue(productId, 2));
        },
      });

      const line = family.line({ productId: "p1" });

      assert.equal(line.value(), null);
      assert.deepEqual(line.status(), {
        kind: "pending",
        operation: "initialLoad",
        continuity: "noVisibleValueYet",
      });
      assert.deepEqual(line.freshness(), {
        kind: "stale",
        reason: "initialLoadPending",
      });

      line.refresh();
      await new Promise((resolve) => setTimeout(resolve, 0));

      const lifecycle = snapshotLifecycleSupersession(line.history().lifecycle)
        .map((entry, index) => ({
          event: entry.event,
          status: entry.status,
          freshness: line.history().lifecycle[index].freshness,
          visibleValueVersion: line.history().lifecycle[index].visibleValueVersion,
          supersededOperation: entry.supersededOperation,
          lastSupersededOperation: entry.lastSupersededOperation,
        }));

      assert.deepEqual(lifecycle, [{
        event: "materialized",
        status: {
          kind: "pending",
          operation: "initialLoad",
          continuity: "noVisibleValueYet",
        },
        freshness: { kind: "stale", reason: "initialLoadPending" },
        visibleValueVersion: 0,
        supersededOperation: null,
        lastSupersededOperation: null,
      }, {
        event: "superseded",
        status: {
          kind: "pending",
          operation: "initialLoad",
          continuity: "noVisibleValueYet",
        },
        freshness: { kind: "stale", reason: "initialLoadPending" },
        visibleValueVersion: 0,
        supersededOperation: "initialLoad",
        lastSupersededOperation: null,
      }, {
        event: "pending",
        status: {
          kind: "pending",
          operation: "refresh",
          continuity: "noVisibleValueYet",
        },
        freshness: { kind: "stale", reason: "refreshPending" },
        visibleValueVersion: 0,
        supersededOperation: null,
        lastSupersededOperation: "initialLoad",
      }, {
        event: "fulfilled",
        status: {
          kind: "fulfilled",
          operation: "refresh",
        },
        freshness: { kind: "fresh" },
        visibleValueVersion: 1,
        supersededOperation: null,
        lastSupersededOperation: "initialLoad",
      }]);

      initialDeferred.resolve(familyCase.value("p1", 1));
      await initialDeferred.promise;
      await new Promise((resolve) => setTimeout(resolve, 0));

      assert.deepEqual(line.value(), familyCase.changedValue("p1", 2));
      assert.deepEqual(line.status(), {
        kind: "fulfilled",
        operation: "refresh",
      });
    }
  } finally {
    await runtime.cleanup();
  }
});
