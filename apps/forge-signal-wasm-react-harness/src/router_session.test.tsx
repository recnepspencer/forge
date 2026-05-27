import React from "react";
import { act, cleanup, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import {
  ReactSignalsStoreProvider,
  useRouterSession,
} from "@aust-group/forge-signal-wasm/react";

type FakeBreadcrumbEntry = {
  label: string;
};

type FakeBreadcrumbTrail = {
  entries: readonly FakeBreadcrumbEntry[];
};

type FakeIngress = {
  method: string;
  location: string;
  options: Record<string, unknown>;
};

type FakeReport = {
  routeId: string;
  location: string;
  breadcrumbTrail: FakeBreadcrumbTrail;
  method: string;
};

function createFakeStory() {
  const listeners = new Set<() => void>();
  const entries: FakeReport[] = [];
  const events: Array<{ routeId: string; method: string }> = [];

  function latestEntry(): FakeReport | null {
    return entries.at(-1) ?? null;
  }

  function emit(): void {
    listeners.forEach((listener) => listener());
  }

  return {
    record(report: FakeReport) {
      entries.push(report);
      events.push({
        routeId: report.routeId,
        method: report.method,
      });
      emit();
    },
    subscribe(listener: () => void) {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
    current() {
      return latestEntry();
    },
    admittedEntries() {
      return [...entries];
    },
    breadcrumbTrail() {
      return latestEntry()?.breadcrumbTrail ?? { entries: [] };
    },
    backProvenance() {
      return {
        available: entries.length > 1,
      };
    },
    events() {
      return [...events];
    },
    latestBoundaryEvent() {
      return events.at(-1) ?? null;
    },
  };
}

function createFakeStore() {
  const story = createFakeStory();
  const ingressCalls: FakeIngress[] = [];

  const signals = {
    router: {
      browserHistory: {
        load(location: string, options?: Record<string, unknown>) {
          const ingress = {
            method: "load",
            location,
            options: options ?? {},
          };
          ingressCalls.push(ingress);
          return ingress;
        },
        push(location: string, options?: Record<string, unknown>) {
          const ingress = {
            method: "push",
            location,
            options: options ?? {},
          };
          ingressCalls.push(ingress);
          return ingress;
        },
        replace(location: string, options?: Record<string, unknown>) {
          const ingress = {
            method: "replace",
            location,
            options: options ?? {},
          };
          ingressCalls.push(ingress);
          return ingress;
        },
        pop(location: string, options?: Record<string, unknown>) {
          const ingress = {
            method: "pop",
            location,
            options: options ?? {},
          };
          ingressCalls.push(ingress);
          return ingress;
        },
        manual(location: string, options?: Record<string, unknown>) {
          const ingress = {
            method: "manual",
            location,
            options: options ?? {},
          };
          ingressCalls.push(ingress);
          return ingress;
        },
        external(location: string, options?: Record<string, unknown>) {
          const ingress = {
            method: "external",
            location,
            options: options ?? {},
          };
          ingressCalls.push(ingress);
          return ingress;
        },
        story() {
          return story;
        },
      },
      carryBreadcrumbs(trail: FakeBreadcrumbTrail) {
        return trail.entries.map((entry) => entry.label);
      },
    },
  };

  return {
    story,
    ingressCalls,
    signals,
    subscribeSignal() {
      return () => {};
    },
    getSignalSnapshot() {
      return null;
    },
    subscribeDiagnostics() {
      return () => {};
    },
    getDiagnosticsSnapshot() {
      return {
        latestObservation: null,
        latestFlow: null,
        performanceSummary: {
          version: 0,
        },
      };
    },
    transaction(callback: (tx: unknown) => void) {
      callback({});
    },
    batch(callback: (tx: unknown) => void) {
      callback({});
    },
    refreshDiagnostics() {
      return this.getDiagnosticsSnapshot();
    },
    performanceSummary() {
      return {
        activeSignalSubscriptionCount: 0,
        activeReactSubscriberCount: 0,
        activeRuntimeWatchHandleCount: 0,
        diagnosticsSubscriberCount: 0,
        sharedFanoutRatio: 0,
      };
    },
    dispose() {},
  };
}

function createFakeRoutes() {
  return {
    async admitBrowserHistoryIngress(ingress: FakeIngress) {
      return {
        routeId: ingress.location === "/" ? "home" : ingress.location.replaceAll("/", ".").slice(1),
        location: ingress.location,
        method: ingress.method,
        breadcrumbTrail: {
          entries: [
            ...(Array.isArray(ingress.options.carriedBreadcrumbs)
              ? (ingress.options.carriedBreadcrumbs as string[]).map((label) => ({ label }))
              : []),
            {
              label: ingress.location === "/" ? "Home" : ingress.location.split("/").at(-1) ?? "Unknown",
            },
          ],
        },
      } satisfies FakeReport;
    },
  };
}

function RouterSessionProbe({
  initialLocation,
  routes,
}: {
  initialLocation: string;
  routes: ReturnType<typeof createFakeRoutes>;
}): JSX.Element {
  const session = useRouterSession(routes, {
    history: "browser",
    initialLocation,
  });

  return (
    <section>
      <div data-testid="router-session-proof">
        {JSON.stringify({
          currentRouteId: session.currentRoute?.routeId ?? null,
          entryCount: session.story.entries.length,
          breadcrumbLabels: session.breadcrumbs.entries.map((entry) => entry.label),
        })}
      </div>
      <button
        type="button"
        onClick={() => {
          void session.navigate("/products/sku-42");
        }}
      >
        Navigate
      </button>
    </section>
  );
}

afterEach(() => {
  cleanup();
});

describe("router session react adapter", () => {
  it("creates one retained browser session authority from routes, ingress, and story truth", async () => {
    const store = createFakeStore();
    const routes = createFakeRoutes();
    const rendered = render(
      <ReactSignalsStoreProvider store={store as never}>
        <RouterSessionProbe initialLocation="/products" routes={routes} />
      </ReactSignalsStoreProvider>,
    );

    try {
      await waitFor(() => {
        expect(screen.getByTestId("router-session-proof").textContent).toContain(
          '"currentRouteId":"products"',
        );
      });

      expect(store.ingressCalls).toHaveLength(1);
      expect(store.ingressCalls[0]).toMatchObject({
        method: "load",
        location: "/products",
      });

      await act(async () => {
        screen.getByRole("button", { name: "Navigate" }).click();
      });

      await waitFor(() => {
        const text = screen.getByTestId("router-session-proof").textContent ?? "";
        expect(text).toContain('"currentRouteId":"products.sku-42"');
        expect(text).toContain('"entryCount":2');
        expect(text).toContain('"breadcrumbLabels":["products","sku-42"]');
      });

      expect(store.ingressCalls).toHaveLength(2);
      expect(store.ingressCalls[1]).toMatchObject({
        method: "push",
        location: "/products/sku-42",
        options: {
          carriedBreadcrumbs: ["products"],
        },
      });
    } finally {
      rendered.unmount();
      store.dispose();
    }
  });

  it("retains one session across remounts instead of replaying the initial load choreography", async () => {
    const store = createFakeStore();
    const routes = createFakeRoutes();
    const firstRender = render(
      <ReactSignalsStoreProvider store={store as never}>
        <RouterSessionProbe initialLocation="/products" routes={routes} />
      </ReactSignalsStoreProvider>,
    );

    await waitFor(() => {
      expect(screen.getByTestId("router-session-proof").textContent).toContain(
        '"currentRouteId":"products"',
      );
    });
    expect(store.ingressCalls).toHaveLength(1);

    firstRender.unmount();

    const secondRender = render(
      <ReactSignalsStoreProvider store={store as never}>
        <RouterSessionProbe initialLocation="/products" routes={routes} />
      </ReactSignalsStoreProvider>,
    );

    try {
      await waitFor(() => {
        expect(screen.getByTestId("router-session-proof").textContent).toContain(
          '"entryCount":1',
        );
      });
      expect(store.ingressCalls).toHaveLength(1);
    } finally {
      secondRender.unmount();
      store.dispose();
    }
  });
});
