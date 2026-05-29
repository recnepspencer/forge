import React from "react";
import { act, cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { createSignals } from "@aust-group/forge-signal-wasm";
import {
  createReactSignalsStore,
  optionalResourceLine,
  ReactSignalsStoreProvider,
  useOptionalResourceLineValue,
  useResourceLine,
  useOptionalResourceLine,
  useOptionalSignalValue,
} from "@aust-group/forge-signal-wasm/react";

type FakeLine = {
  signal(): ReturnType<ReturnType<typeof createSignals>["input"]>;
  summarySignal(): ReturnType<ReturnType<typeof createSignals>["input"]>;
  summary(): {
    current: { status: { kind: string }; freshness: { kind: string }; hasVisibleValue: boolean };
    diagnostics: { latest: { errorMessage: string | null } };
  };
  status(): { kind: string };
  freshness(): { kind: string };
  diagnosticsSummary(): { latest: { errorMessage: string | null } };
  mutationResponse(): null;
};

function createFixture() {
  return createSignals({ deployment: "mainThreadCompatibility" }).then((signals) => {
    const store = createReactSignalsStore(signals);
    const value = signals.input("Trailblazer Jacket");
    const summary = signals.input({
      current: {
        status: { kind: "fulfilled" },
        freshness: { kind: "fresh" },
        hasVisibleValue: true,
        visibleValueVersion: 1,
        visibleSelection: null,
      },
      diagnostics: {
        current: {
          status: { kind: "fulfilled" },
          freshness: { kind: "fresh" },
          hasVisibleValue: true,
          visibleValueVersion: 1,
          visibleSelection: null,
        },
        latest: { errorMessage: null },
      },
    });
    const counters = {
      summaryReads: 0,
      statusReads: 0,
      freshnessReads: 0,
      diagnosticsReads: 0,
    };
    const line: FakeLine = {
      signal() {
        return value;
      },
      summarySignal() {
        return summary;
      },
      summary() {
        counters.summaryReads += 1;
        return {
          current: {
            status: { kind: "fulfilled" },
            freshness: { kind: "fresh" },
            hasVisibleValue: true,
          },
          diagnostics: { latest: { errorMessage: null } },
        };
      },
      status() {
        counters.statusReads += 1;
        return { kind: "fulfilled" };
      },
      freshness() {
        counters.freshnessReads += 1;
        return { kind: "fresh" };
      },
      diagnosticsSummary() {
        counters.diagnosticsReads += 1;
        return { latest: { errorMessage: null } };
      },
      mutationResponse() {
        return null;
      },
    };

    const familyCounters = {
      lineCalls: 0,
      optionalLineCalls: 0,
    };
    const family = {
      line(params: { productId: string }) {
        familyCounters.lineCalls += 1;
        return line;
      },
      optionalLine(selection: { productId: string } | { enabled: false } | null | undefined) {
        familyCounters.optionalLineCalls += 1;
        if (
          selection == null
          || ("enabled" in selection && selection.enabled === false)
        ) {
          return null;
        }
        return line;
      },
    };

    function dispose() {
      store.dispose();
      signals.free();
    }

    return { signals, store, value, line, family, familyCounters, counters, dispose };
  });
}

function OptionalSignalProbe({
  signal,
  store,
}: {
  signal: ReturnType<ReturnType<typeof createSignals>["input"]> | null;
  store: ReturnType<typeof createReactSignalsStore>;
}): JSX.Element {
  const result = useOptionalSignalValue<string, string>(signal, store, {
    inactiveValue: "nothing selected",
  });
  return (
    <div data-testid="optional-signal">
      {result.kind}:{result.value}
    </div>
  );
}

function OptionalResourceProbe({
  line,
  store,
}: {
  line: FakeLine | null;
  store: ReturnType<typeof createReactSignalsStore>;
}): JSX.Element {
  const result = useOptionalResourceLine<string, string, FakeLine>(line, store, {
    inactiveValue: "no selection",
  });
  return (
    <div data-testid="optional-line">
      {result.kind === "inactive"
        ? `${result.kind}:${result.value}`
        : `${result.kind}:${result.value}:${result.summary.current.status.kind}:${result.status.kind}:${result.freshness.kind}`}
    </div>
  );
}

function ResourceFamilyProbe({
  family,
  selection,
  store,
}: {
  family: { line(params: { productId: string }): FakeLine };
  selection: { productId: string } | { enabled: false };
  store?: ReturnType<typeof createReactSignalsStore>;
}): JSX.Element {
  const result = useResourceLine<string, string, { productId: string }, FakeLine>(
    family,
    selection,
    store,
    {
      inactiveValue: "nothing selected",
    },
  );
  return (
    <div data-testid="resource-family-line">
      {result.kind === "inactive"
        ? `${result.kind}:${result.value}`
        : `${result.kind}:${result.value}`}
    </div>
  );
}

function OptionalResourceLineValueProbe({
  line,
  store,
}: {
  line: FakeLine | null;
  store?: ReturnType<typeof createReactSignalsStore>;
}): JSX.Element {
  const result = useOptionalResourceLineValue<string, string, { productId: string }, FakeLine>(
    line,
    store,
    {
      inactiveValue: "no selection",
    },
  );
  return (
    <div data-testid="optional-line-value">
      {result.kind}:{result.value}
    </div>
  );
}

afterEach(() => {
  cleanup();
});

describe("optional resource line React hooks", () => {
  it("keeps hook order stable with an explicit inactive signal posture", async () => {
    const fixture = await createFixture();
    const rendered = render(
      <OptionalSignalProbe signal={null} store={fixture.store} />,
    );

    try {
      expect(screen.getByTestId("optional-signal").textContent).toBe("inactive:nothing selected");
      expect(fixture.store.performanceSummary()).toEqual({
        activeSignalSubscriptionCount: 0,
        activeReactSubscriberCount: 0,
        activeRuntimeWatchHandleCount: 0,
        diagnosticsSubscriberCount: 0,
        sharedFanoutRatio: 0,
      });

      rendered.rerender(<OptionalSignalProbe signal={fixture.value} store={fixture.store} />);
      expect(screen.getByTestId("optional-signal").textContent).toBe(
        "active:Trailblazer Jacket",
      );
      expect(fixture.store.performanceSummary().activeRuntimeWatchHandleCount).toBe(1);

      act(() => {
        fixture.value.set("Aurora Fleece");
      });
      await act(async () => {
        await Promise.resolve();
      });

      expect(screen.getByTestId("optional-signal").textContent).toBe("active:Aurora Fleece");

      rendered.rerender(<OptionalSignalProbe signal={null} store={fixture.store} />);
      expect(screen.getByTestId("optional-signal").textContent).toBe("inactive:nothing selected");
      expect(fixture.store.performanceSummary().activeRuntimeWatchHandleCount).toBe(0);
    } finally {
      rendered.unmount();
      fixture.dispose();
    }
  });

  it("exposes inactive resource-line posture without synthetic fallback signals", async () => {
    const fixture = await createFixture();
    const rendered = render(
      <OptionalResourceProbe line={null} store={fixture.store} />,
    );

    try {
      expect(screen.getByTestId("optional-line").textContent).toBe("inactive:no selection");
      expect(fixture.counters.summaryReads).toBe(0);
      expect(fixture.counters.statusReads).toBe(0);
      expect(fixture.counters.freshnessReads).toBe(0);
      expect(fixture.counters.diagnosticsReads).toBe(0);
      expect(fixture.store.performanceSummary().activeRuntimeWatchHandleCount).toBe(0);

      rendered.rerender(<OptionalResourceProbe line={fixture.line} store={fixture.store} />);
      expect(screen.getByTestId("optional-line").textContent).toBe(
        "active:Trailblazer Jacket:fulfilled:fulfilled:fresh",
      );
      expect(fixture.counters.summaryReads).toBe(0);
      expect(fixture.counters.statusReads).toBe(0);
      expect(fixture.counters.freshnessReads).toBe(0);
      expect(fixture.counters.diagnosticsReads).toBe(0);
      expect(fixture.store.performanceSummary().activeRuntimeWatchHandleCount).toBe(2);

      act(() => {
        fixture.value.set("Wayfinder Tee");
      });
      await act(async () => {
        await Promise.resolve();
      });

      expect(screen.getByTestId("optional-line").textContent).toBe(
        "active:Wayfinder Tee:fulfilled:fulfilled:fresh",
      );

      rendered.rerender(<OptionalResourceProbe line={null} store={fixture.store} />);
      expect(screen.getByTestId("optional-line").textContent).toBe("inactive:no selection");
      expect(fixture.store.performanceSummary().activeRuntimeWatchHandleCount).toBe(0);
    } finally {
      rendered.unmount();
      fixture.dispose();
    }
  });

  it("supports family-first resource selection with an explicit enabled false posture", async () => {
    const fixture = await createFixture();
    const rendered = render(
      <ResourceFamilyProbe
        family={fixture.family}
        selection={{ enabled: false }}
        store={fixture.store}
      />,
    );

    try {
      expect(screen.getByTestId("resource-family-line").textContent).toBe("inactive:nothing selected");
      expect(fixture.familyCounters.lineCalls).toBe(0);

      rendered.rerender(
        <ResourceFamilyProbe
          family={fixture.family}
          selection={{ productId: "product-7" }}
          store={fixture.store}
        />,
      );
      expect(screen.getByTestId("resource-family-line").textContent).toBe("active:Trailblazer Jacket");
      expect(fixture.familyCounters.lineCalls).toBe(0);
      expect(fixture.familyCounters.optionalLineCalls).toBe(2);

      rendered.rerender(
        <ResourceFamilyProbe
          family={fixture.family}
          selection={{ enabled: false }}
          store={fixture.store}
        />,
      );
      expect(screen.getByTestId("resource-family-line").textContent).toBe("inactive:nothing selected");
      expect(fixture.familyCounters.lineCalls).toBe(0);
      expect(fixture.familyCounters.optionalLineCalls).toBe(3);
      expect(optionalResourceLine(fixture.family, { enabled: false })).toBeNull();
      expect(optionalResourceLine(fixture.family, { productId: "product-7" })).toBe(fixture.line);
    } finally {
      rendered.unmount();
      fixture.dispose();
    }
  });

  it("supports the final-form family hook through ReactSignalsStoreProvider", async () => {
    const fixture = await createFixture();
    const rendered = render(
      <ReactSignalsStoreProvider store={fixture.store}>
        <ResourceFamilyProbe
          family={fixture.family}
          selection={{ enabled: false }}
        />
      </ReactSignalsStoreProvider>,
    );

    try {
      expect(screen.getByTestId("resource-family-line").textContent).toBe("inactive:nothing selected");
      rendered.rerender(
        <ReactSignalsStoreProvider store={fixture.store}>
          <ResourceFamilyProbe
            family={fixture.family}
            selection={{ productId: "product-7" }}
          />
        </ReactSignalsStoreProvider>,
      );
      expect(screen.getByTestId("resource-family-line").textContent).toBe("active:Trailblazer Jacket");
    } finally {
      rendered.unmount();
      fixture.dispose();
    }
  });

  it("supports the final-form optionalLine plus useOptionalResourceLineValue lane", async () => {
    const fixture = await createFixture();
    const rendered = render(
      <ReactSignalsStoreProvider store={fixture.store}>
        <OptionalResourceLineValueProbe
          line={fixture.family.optionalLine({ enabled: false })}
        />
      </ReactSignalsStoreProvider>,
    );

    try {
      expect(screen.getByTestId("optional-line-value").textContent).toBe("inactive:no selection");
      rendered.rerender(
        <ReactSignalsStoreProvider store={fixture.store}>
          <OptionalResourceLineValueProbe
            line={fixture.family.optionalLine({ productId: "product-7" })}
          />
        </ReactSignalsStoreProvider>,
      );
      expect(screen.getByTestId("optional-line-value").textContent).toBe("active:Trailblazer Jacket");
    } finally {
      rendered.unmount();
      fixture.dispose();
    }
  });
});
