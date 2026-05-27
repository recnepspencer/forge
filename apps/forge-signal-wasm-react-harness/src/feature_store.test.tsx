import { describe, expect, it } from "vitest";

import { createSignals } from "@aust-group/forge-signal-wasm";

describe("feature store authoring", () => {
  it("creates a scoped feature store with named state and authored actions", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });

    try {
      const store = signals.featureStore({
        id: "workplace-user-groups-admin",
        state: {
          selectedGroupId: null as string | null,
          selectedCandidateId: "",
          view: "users" as "users" | "groups",
        },
        actions: ({ set, reset, read }) => ({
          setSelectedGroupId(next: string | null) {
            return set("selectedGroupId", next);
          },
          showGroups() {
            return set("view", "groups");
          },
          clearSelection() {
            return reset("selectedGroupId");
          },
          snapshot() {
            return read();
          },
        }),
      });

      expect(store.scopeId).toBe("workplace-user-groups-admin");
      expect(store.state.selectedGroupId.signalIdentity?.().localId).toBe("selectedGroupId");
      expect(store.state.view.signalIdentity?.().canonicalId).toBe(
        "workplace-user-groups-admin.view",
      );
      expect(store.snapshot.signalIdentity?.().localId).toBe("snapshot");
      expect(store.read()).toEqual({
        selectedGroupId: null,
        selectedCandidateId: "",
        view: "users",
      });

      store.actions.setSelectedGroupId("group-17");
      store.actions.showGroups();
      expect(store.actions.snapshot()).toEqual({
        selectedGroupId: "group-17",
        selectedCandidateId: "",
        view: "groups",
      });

      store.actions.clearSelection();
      expect(store.read().selectedGroupId).toBeNull();
      expect(typeof store.free).toBe("function");
      store.free();
    } finally {
      signals.free();
    }
  });

  it("stays available on scoped namespaces and prefixes signal identity honestly", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });

    try {
      const store = signals.scope("admin").featureStore({
        id: "catalog",
        state: {
          selectedProductId: null as string | null,
        },
        actions: ({ set }) => ({
          setSelectedProductId(next: string | null) {
            return set("selectedProductId", next);
          },
        }),
      });

      expect(store.scopeId).toBe("admin.catalog");
      expect(store.state.selectedProductId.signalIdentity?.().canonicalId).toBe(
        "admin.catalog.selectedProductId",
      );
      store.actions.setSelectedProductId("prod-9");
      expect(store.read().selectedProductId).toBe("prod-9");
    } finally {
      signals.free();
    }
  });

  it("rejects non-object action surfaces instead of exporting a fake store contract", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });

    try {
      expect(() =>
        signals.featureStore({
          id: "broken-store",
          state: {
            selectedId: null as string | null,
          },
          actions: () => [] as never,
        }),
      ).toThrow(
        'signals.featureStore(...) actions(...) for "broken-store" must return a plain object',
      );
    } finally {
      signals.free();
    }
  });

  it("rejects unknown state-key writes through the feature-store contract", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });

    try {
      const store = signals.featureStore({
        id: "users-store",
        state: {
          selectedId: null as string | null,
        },
        actions: ({ set }) => ({
          breakIt() {
            return (set as (key: string, value: unknown) => unknown)("missingKey", "x");
          },
        }),
      });

      expect(() => store.actions.breakIt()).toThrow(
        'signals.featureStore(...) action contract referenced unknown state key "missingKey" in store "users-store"',
      );
    } finally {
      signals.free();
    }
  });
});
