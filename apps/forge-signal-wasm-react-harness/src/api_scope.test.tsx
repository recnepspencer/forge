import { afterEach, describe, expect, it } from "vitest";

import { createSignals } from "@aust-group/forge-signal-wasm";

async function createMainThreadSignals() {
  return createSignals({ deployment: "mainThreadCompatibility" });
}

async function createWorkerFirstSignals() {
  return createSignals({ deployment: "workerFirst" });
}

afterEach(() => {
  // no-op; tests manage their own runtimes explicitly
});

describe("api scope authoring", () => {
  it("reuses the same runtime-scoped api identity for equivalent static config", async () => {
    const signals = await createMainThreadSignals();

    try {
      const first = signals.apiScope("workplace-admin", {
        baseUrl: "/api",
        headers: {
          "x-app-surface": "admin",
        },
      });
      const second = signals.apiScope("workplace-admin", {
        baseUrl: "/api",
        headers: {
          "x-app-surface": "admin",
        },
      });

      expect(first).toBe(second);
      expect(first.scopeId).toBe("workplace-admin");
    } finally {
      signals.free();
    }
  });

  it("rejects conflicting scoped defaults for the same runtime-scoped api identity", async () => {
    const signals = await createMainThreadSignals();

    try {
      signals.apiScope("workplace-admin", {
        baseUrl: "/api",
        headers: {
          "x-app-surface": "admin",
        },
      });

      expect(() =>
        signals.apiScope("workplace-admin", {
          baseUrl: "/api",
          headers: {
            "x-app-surface": "dashboard",
          },
        }),
      ).toThrow(
        'signals.apiScope("workplace-admin") was requested with conflicting scoped defaults for the same runtime',
      );
    } finally {
      signals.free();
    }
  });

  it("rejects param-dependent defaults on apiScope identities", async () => {
    const signals = await createMainThreadSignals();

    try {
      expect(() =>
        signals.apiScope("workplace-admin", {
          headers: ({ workspaceId }: { workspaceId: string }) => ({
            "x-workspace-id": workspaceId,
          }),
        }),
      ).toThrow(
        'signals.apiScope("workplace-admin") only admits static scoped defaults; use signals.api(...).scope({...}) when defaults depend on params',
      );
    } finally {
      signals.free();
    }
  });

  it("reuses nested api scopes and keeps parent-child scope identity explicit", async () => {
    const signals = await createMainThreadSignals();

    try {
      const rootApi = signals.apiScope("root", { baseUrl: "/api" });
      const first = rootApi.scope("workplace-admin", {
        headers: {
          "x-app-surface": "admin",
        },
      });
      const second = rootApi.scope("workplace-admin", {
        headers: {
          "x-app-surface": "admin",
        },
      });

      expect(first).toBe(second);
      expect(rootApi.scopeId).toBe("root");
      expect(first.scopeId).toBe("workplace-admin");
    } finally {
      signals.free();
    }
  });

  it("exposes apiScope on scoped namespaces", async () => {
    const signals = await createMainThreadSignals();

    try {
      const scoped = signals.scope("admin");
      const api = scoped.apiScope("workplace-admin", { baseUrl: "/api" });

      expect(typeof scoped.apiScope).toBe("function");
      expect(api.scopeId).toBe("workplace-admin");
      expect(api.url("/projects")).toBeDefined();
    } finally {
      signals.free();
    }
  });

  it("keeps apiScope available on worker-first callable and scoped surfaces", async () => {
    let signals;
    try {
      signals = await createWorkerFirstSignals();
    } catch (error) {
      expect(error).toBeInstanceOf(Error);
      expect((error as Error).message).toContain("Dedicated worker construction is unavailable");
      return;
    }

    try {
      const rootApi = signals.apiScope("workplace-admin", { baseUrl: "/api" });
      const scopedApi = signals.scope("admin").apiScope("backoffice", {
        baseUrl: "/api",
      });

      expect(rootApi.scopeId).toBe("workplace-admin");
      expect(scopedApi.scopeId).toBe("backoffice");
    } finally {
      await signals.terminate();
    }
  });
});
