import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createDeferred } from "../runtime_fixture/deferred.mjs";
import { createPhase2FamilyCases } from "../runtime_fixture/phase2_family_cases.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("refresh and revalidate preserve one lifecycle story across detail, collection, and paged lines", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const familyCases = createPhase2FamilyCases(resource, mod);

    for (const familyCase of familyCases) {
      let version = 0;
      const deferred = createDeferred();
      let loadCount = 0;
      const family = familyCase.build({
        load: ({ productId }) => {
          loadCount += 1;
          if (loadCount === 1) {
            version += 1;
            return familyCase.value(productId, version);
          }
          return deferred.promise;
        },
      });

      const line = family.line({ productId: "p1" });
      assert.deepEqual(line.value(), familyCase.value("p1", 1));

      const refreshStatus = line.refresh();
      assert.deepEqual(refreshStatus, {
        kind: "pending",
        operation: "refresh",
        continuity: "preservedVisibleValue",
      });
      assert.deepEqual(line.freshness(), {
        kind: "stale",
        reason: "refreshPending",
      });
      assert.equal(line.diagnostics().pendingOperation, "refresh");

      const revalidateStatus = line.revalidate();
      assert.deepEqual(revalidateStatus, {
        kind: "pending",
        operation: "revalidate",
        continuity: "preservedVisibleValue",
      });
      assert.equal(line.diagnostics().supersessionCount, 1);
      assert.equal(line.diagnostics().lastSupersededOperation, "refresh");
      assert.equal(line.diagnostics().pendingOperation, "revalidate");
      assert.deepEqual(line.value(), familyCase.value("p1", 1));

      deferred.resolve(familyCase.changedValue("p1", 2));
      await deferred.promise;
      await new Promise((resolve) => setTimeout(resolve, 0));

      assert.deepEqual(line.value(), familyCase.changedValue("p1", 2));
      assert.deepEqual(line.status(), {
        kind: "fulfilled",
        operation: "revalidate",
      });
      assert.equal(line.diagnostics().pendingOperation, null);
    }
  } finally {
    await mod.cleanup();
  }
});

test("repeated retry-bearing failures settle identically across family kinds", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const familyCases = createPhase2FamilyCases(resource, mod);

    for (const familyCase of familyCases) {
      let callCount = 0;
      const firstDeferred = createDeferred();
      const secondDeferred = createDeferred();
      const family = familyCase.build({
        policy: mod.resourcePolicyProfiles.retryOnce(),
        load: ({ productId }) => {
          callCount += 1;
          if (callCount === 1) {
            return familyCase.value(productId, 1);
          }
          return callCount === 2 ? firstDeferred.promise : secondDeferred.promise;
        },
      });

      const line = family.line({ productId: "p1" });
      line.refresh();

      firstDeferred.reject(new Error(`${familyCase.kind} temporary failure`));
      await firstDeferred.promise.catch(() => {});
      await Promise.resolve();
      secondDeferred.reject(new Error(`${familyCase.kind} terminal failure`));
      await secondDeferred.promise.catch(() => {});
      await new Promise((resolve) => setTimeout(resolve, 0));

      assert.deepEqual(line.value(), familyCase.value("p1", 1));
      assert.deepEqual(line.status(), {
        kind: "rejected",
        operation: "refresh",
        message: `${familyCase.kind} terminal failure`,
        continuity: "preservedVisibleValue",
      });
      assert.equal(line.diagnostics().retryAttemptCount, 1);
      assert.equal(line.diagnostics().rejectionCount, 1);
      assert.equal(line.diagnostics().pendingOperation, null);
      assert.equal(line.history().lifecycle.at(-1)?.event, "rejected");
    }
  } finally {
    await mod.cleanup();
  }
});
