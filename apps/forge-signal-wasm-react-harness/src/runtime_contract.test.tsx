import { describe, expect, it } from "vitest";

import { createSignals } from "@aust-group/forge-signal-wasm";

async function createMainThreadSignals() {
  return createSignals({ deployment: "mainThreadCompatibility" });
}

async function createWorkerFirstSignals() {
  return createSignals({ deployment: "workerFirst" });
}

describe("runtime contract introspection", () => {
  it("describes the main-thread callable surface honestly", async () => {
    const signals = await createMainThreadSignals();

    try {
      const contract = signals.contract();

      expect(contract.surfaceFamily).toBe("mainThreadCompatibilityCallable");
      expect(contract.surfaceVersion).toBe("1");
      expect(contract.scopeId).toBeNull();
      expect(contract.capabilities).toEqual({
        callableSurface: true,
        scopedAuthoring: true,
        specNamespace: true,
        workerRuntime: false,
      });
      expect(
        signals.assertCompatibility({
          requires: ["callableSurface", "scopedAuthoring", "specNamespace"],
        }),
      ).toBe(contract);
    } finally {
      signals.free();
    }
  });

  it("describes scoped namespaces as scoped authoring surfaces, not callable roots", async () => {
    const signals = await createMainThreadSignals();

    try {
      const scoped = signals.scope("admin");
      const contract = scoped.contract();

      expect(contract.surfaceFamily).toBe("mainThreadCompatibilityScoped");
      expect(contract.scopeId).toBe("admin");
      expect(contract.capabilities.callableSurface).toBe(false);
      expect(contract.capabilities.scopedAuthoring).toBe(true);
      expect(contract.capabilities.specNamespace).toBe(true);
      let thrown: unknown = null;
      try {
        scoped.assertCompatibility({
          requires: ["callableSurface"],
        });
      } catch (error) {
        thrown = error;
      }

      expect(thrown).toBeInstanceOf(Error);
      expect(thrown).toMatchObject({
        name: "SignalsCompatibilityAssertionError",
        code: "signalsCompatibilityAssertionFailed",
      });
    } finally {
      signals.free();
    }
  });

  it("reports machine-readable missing capabilities", async () => {
    const signals = await createMainThreadSignals();

    try {
      let thrown: unknown = null;
      try {
        signals.assertCompatibility({
          requires: ["workerRuntime"],
        });
      } catch (error) {
        thrown = error;
      }

      expect(thrown).toBeInstanceOf(Error);
      expect(thrown).toMatchObject({
        name: "SignalsCompatibilityAssertionError",
        code: "signalsCompatibilityAssertionFailed",
        missingCapabilities: ["workerRuntime"],
      });
    } finally {
      signals.free();
    }
  });

  it("keeps the worker-first contract explicit when that deployment is available", async () => {
    let signals;
    try {
      signals = await createWorkerFirstSignals();
    } catch (error) {
      expect(error).toBeInstanceOf(Error);
      expect((error as Error).message).toContain(
        "Dedicated worker construction is unavailable",
      );
      return;
    }

    try {
      const contract = signals.contract();
      const scopedContract = signals.scope("workspace").contract();

      expect(contract.surfaceFamily).toBe("workerFirstCallable");
      expect(contract.capabilities.workerRuntime).toBe(true);
      expect(scopedContract.surfaceFamily).toBe("workerFirstScoped");
      expect(scopedContract.scopeId).toBe("workspace");
      expect(scopedContract.capabilities.workerRuntime).toBe(true);
      expect(scopedContract.capabilities.callableSurface).toBe(false);
    } finally {
      await signals.terminate();
    }
  });

  it("rejects unknown capability names immediately", async () => {
    const signals = await createMainThreadSignals();

    try {
      expect(() =>
        signals.assertCompatibility({
          requires: ["bogusCapability" as never],
        }),
      ).toThrow(
        "signals.assertCompatibility received unknown runtime capability `bogusCapability`",
      );
    } finally {
      signals.free();
    }
  });
});
