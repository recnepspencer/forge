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

  it("accepts ordinary object-shaped state values without cast-only store wrappers", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });

    try {
      type LayoutConfig = {
        density: "compact" | "comfortable";
        visibleColumns: readonly string[];
        pinned: {
          left: readonly string[];
          right: readonly string[];
        };
      };

      type QueryValues = {
        search: string;
        severities: readonly ("info" | "warning" | "error")[];
        includeResolved: boolean;
      };

      const initialLayoutConfig: LayoutConfig = {
        density: "comfortable",
        visibleColumns: ["event", "actor", "severity"],
        pinned: {
          left: ["event"],
          right: [],
        },
      };

      const initialQueryValues: QueryValues = {
        search: "",
        severities: ["warning"],
        includeResolved: false,
      };

      const store = signals.featureStore({
        id: "workplace-audit-logs-admin",
        state: {
          search: "",
          page: 1,
          layoutConfig: initialLayoutConfig,
          queryValues: initialQueryValues,
          quickReportId: null as string | null,
        },
        actions: ({ set, read }) => ({
          setQueryValues(next: QueryValues) {
            return set("queryValues", next);
          },
          setLayoutConfig(next: LayoutConfig) {
            return set("layoutConfig", next);
          },
          snapshot() {
            return read();
          },
        }),
      });

      expect(store.read().layoutConfig).toEqual(initialLayoutConfig);
      expect(store.read().queryValues).toEqual(initialQueryValues);

      const nextQueryValues: QueryValues = {
        search: "billing",
        severities: ["error"],
        includeResolved: true,
      };
      const nextLayoutConfig: LayoutConfig = {
        density: "compact",
        visibleColumns: ["event", "severity"],
        pinned: {
          left: ["severity"],
          right: [],
        },
      };

      store.actions.setQueryValues(nextQueryValues);
      store.actions.setLayoutConfig(nextLayoutConfig);

      expect(store.actions.snapshot().queryValues).toEqual(nextQueryValues);
      expect(store.actions.snapshot().layoutConfig).toEqual(nextLayoutConfig);
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
