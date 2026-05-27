import React from "react";
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { createSignals } from "@aust-group/forge-signal-wasm";
import {
  createReactSignalsStore,
  createResourceCatalog,
  getResourceCatalog,
  ReactSignalsStoreProvider,
  useResourceCatalog,
} from "@aust-group/forge-signal-wasm/react";

const projectsCatalog = createResourceCatalog({
  id: "workplace-admin.projects",
  scope(signals: Awaited<ReturnType<typeof createSignals>>) {
    return signals.apiScope("workplace-admin", {
      baseUrl: "/api",
      headers: {
        "x-surface": "admin",
      },
    });
  },
  domains: {
    projects(api: ReturnType<Awaited<ReturnType<typeof createSignals>>["api"]>) {
      return Object.freeze({
        list: api.url("/projects"),
      });
    },
    auditLogs(api: ReturnType<Awaited<ReturnType<typeof createSignals>>["api"]>) {
      return Object.freeze({
        list: api.url("/audit-logs"),
      });
    },
  },
});

const buildCatalog = createResourceCatalog({
  id: "workplace-admin.legacy",
  build(signals: Awaited<ReturnType<typeof createSignals>>) {
    const api = signals.apiScope("workplace-admin.legacy", {
      baseUrl: "/api",
      headers: {
        "x-surface": "admin",
      },
    });
    return Object.freeze({
      scope: api,
      domains: {
        projects: {
          list: api.url("/projects"),
        },
      },
      projects: {
        list: api.url("/projects"),
      },
    });
  },
});

const conflictingProjectsCatalog = createResourceCatalog({
  id: "workplace-admin.projects",
  build() {
    return Object.freeze({
      domains: {
        unexpected: true,
      },
    });
  },
});

function CatalogProbe(): JSX.Element {
  const catalog = useResourceCatalog(projectsCatalog);
  return (
    <div data-testid="catalog-proof">
      {catalog.scope && catalog.projects === catalog.domains.projects ? "stable" : "rebuilt"}
    </div>
  );
}

function CatalogBuildProbe({
  store,
}: {
  store: ReturnType<typeof createReactSignalsStore>;
}): JSX.Element {
  const catalog = useResourceCatalog(store, buildCatalog);
  return (
    <div data-testid="build-catalog-proof">
      {catalog === getResourceCatalog(store.signals, buildCatalog) ? "stable" : "rebuilt"}
    </div>
  );
}

function SwitchingCatalogProbe({
  store,
  explicitStore,
}: {
  store: ReturnType<typeof createReactSignalsStore>;
  explicitStore: boolean;
}): JSX.Element {
  const catalog = useResourceCatalog(
    projectsCatalog,
    explicitStore ? store : undefined,
  );
  return (
    <div data-testid="switching-catalog-proof">
      {catalog.scope && catalog.projects === catalog.domains.projects ? "stable" : "rebuilt"}
    </div>
  );
}

afterEach(() => {
  cleanup();
});

describe("resource catalog helpers", () => {
  it("caches one catalog instance per signals runtime", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const store = createReactSignalsStore(signals);

    try {
      const first = getResourceCatalog(signals, projectsCatalog);
      const second = getResourceCatalog(signals, projectsCatalog);

      expect(first).toBe(second);
      expect(first.scope).toBeDefined();
      expect(first.projects.list).toBeDefined();
      expect(first.projects).toBe(first.domains.projects);
    } finally {
      store.dispose();
      signals.free();
    }
  });

  it("keeps the same catalog instance through React rerenders and separates runtimes", async () => {
    const firstSignals = await createSignals({ deployment: "mainThreadCompatibility" });
    const secondSignals = await createSignals({ deployment: "mainThreadCompatibility" });
    const firstStore = createReactSignalsStore(firstSignals);
    const secondStore = createReactSignalsStore(secondSignals);

    const rendered = render(
      <ReactSignalsStoreProvider store={firstStore}>
        <CatalogProbe />
      </ReactSignalsStoreProvider>,
    );

    try {
      expect(screen.getByTestId("catalog-proof").textContent).toBe("stable");

      rendered.rerender(
        <ReactSignalsStoreProvider store={firstStore}>
          <CatalogProbe />
        </ReactSignalsStoreProvider>,
      );
      expect(screen.getByTestId("catalog-proof").textContent).toBe("stable");

      const firstCatalog = getResourceCatalog(firstSignals, projectsCatalog);
      const secondCatalog = getResourceCatalog(secondSignals, projectsCatalog);
      expect(firstCatalog).not.toBe(secondCatalog);
    } finally {
      rendered.unmount();
      firstStore.dispose();
      secondStore.dispose();
      firstSignals.free();
      secondSignals.free();
    }
  });

  it("keeps the legacy build(signals) lane working", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const store = createReactSignalsStore(signals);
    const rendered = render(<CatalogBuildProbe store={store} />);

    try {
      expect(screen.getByTestId("build-catalog-proof").textContent).toBe("stable");
    } finally {
      rendered.unmount();
      store.dispose();
      signals.free();
    }
  });

  it("supports switching between provider-backed and explicit-store catalog consumption without hook-order hazards", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const store = createReactSignalsStore(signals);
    const rendered = render(
      <ReactSignalsStoreProvider store={store}>
        <SwitchingCatalogProbe store={store} explicitStore={false} />
      </ReactSignalsStoreProvider>,
    );

    try {
      expect(screen.getByTestId("switching-catalog-proof").textContent).toBe("stable");

      rendered.rerender(
        <ReactSignalsStoreProvider store={store}>
          <SwitchingCatalogProbe store={store} explicitStore />
        </ReactSignalsStoreProvider>,
      );
      expect(screen.getByTestId("switching-catalog-proof").textContent).toBe("stable");
    } finally {
      rendered.unmount();
      store.dispose();
      signals.free();
    }
  });

  it("rejects conflicting catalog definitions that reuse the same runtime-scoped id", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const store = createReactSignalsStore(signals);

    try {
      getResourceCatalog(signals, projectsCatalog);
      expect(() => getResourceCatalog(signals, conflictingProjectsCatalog)).toThrow(
        'resource catalog id "workplace-admin.projects" was registered with more than one definition for the same signals runtime',
      );
    } finally {
      store.dispose();
      signals.free();
    }
  });
});
