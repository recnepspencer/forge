import React from "react";
import { act, cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { createSignals } from "@aust-group/forge-signal-wasm";
import { createReactSignalsStore, useResourceView } from "@aust-group/forge-signal-wasm/react";

type FakeStatus =
  | {
      kind: "pending";
      operation: "initialLoad" | "refresh";
      continuity: "preservedVisibleValue" | "noVisibleValueYet";
    }
  | {
      kind: "fulfilled";
      operation: "initialLoad" | "refresh";
    }
  | {
      kind: "rejected";
      operation: "refresh";
      message: string;
      continuity: "preservedVisibleValue";
    };

type FakeLine = {
  signal(): ReturnType<ReturnType<typeof createSignals>["input"]>;
  summarySignal(): ReturnType<ReturnType<typeof createSignals>["input"]>;
  summary(): {
    current: { hasVisibleValue: boolean };
    diagnostics: { latest: { errorMessage: string | null } };
  };
  status(): FakeStatus;
  freshness(): { kind: "fresh" | "stale" };
  diagnosticsSummary(): {
    current: { hasVisibleValue: boolean };
    latest: { errorMessage: string | null };
  };
  mutationResponse(): null;
};

function createFixture() {
  return createSignals({ deployment: "mainThreadCompatibility" }).then((signals) => {
    const store = createReactSignalsStore(signals);
    const value = signals.input<string[]>([]);
    const summarySignal = signals.input({
      current: {
        status: {
          kind: "pending",
          operation: "initialLoad",
          continuity: "noVisibleValueYet",
        } as FakeStatus,
        freshness: { kind: "fresh" as const },
        hasVisibleValue: false,
        visibleValueVersion: 0,
        visibleSelection: null,
      },
      diagnostics: {
        current: {
          status: {
            kind: "pending",
            operation: "initialLoad",
            continuity: "noVisibleValueYet",
          } as FakeStatus,
          freshness: { kind: "fresh" as const },
          hasVisibleValue: false,
          visibleValueVersion: 0,
          visibleSelection: null,
        },
        latest: { errorMessage: null as string | null },
      },
    });
    let hasVisibleValue = false;
    let errorMessage: string | null = null;
    let status: FakeStatus = {
      kind: "pending",
      operation: "initialLoad",
      continuity: "noVisibleValueYet",
    };

    const line: FakeLine = {
      signal() {
        return value;
      },
      summarySignal() {
        return summarySignal;
      },
      summary() {
        return {
          current: { hasVisibleValue },
          diagnostics: { latest: { errorMessage } },
        };
      },
      status() {
        return status;
      },
      freshness() {
        return { kind: "fresh" };
      },
      diagnosticsSummary() {
        return {
          current: { hasVisibleValue },
          latest: { errorMessage },
        };
      },
      mutationResponse() {
        return null;
      },
    };

    function publishSummary() {
      const nextSummary = {
        current: {
          status,
          freshness: { kind: "fresh" as const },
          hasVisibleValue,
          visibleValueVersion: hasVisibleValue ? 1 : 0,
          visibleSelection: null,
        },
        diagnostics: {
          current: {
            status,
            freshness: { kind: "fresh" as const },
            hasVisibleValue,
            visibleValueVersion: hasVisibleValue ? 1 : 0,
            visibleSelection: null,
          },
          latest: { errorMessage },
        },
      };
      summarySignal.set(nextSummary);
    }

    function setLoading() {
      hasVisibleValue = false;
      errorMessage = null;
      value.set([]);
      status = {
        kind: "pending",
        operation: "initialLoad",
        continuity: "noVisibleValueYet",
      };
      publishSummary();
    }

    function setRefreshing(rows: string[]) {
      hasVisibleValue = true;
      errorMessage = null;
      value.set(rows);
      status = {
        kind: "pending",
        operation: "refresh",
        continuity: "preservedVisibleValue",
      };
      publishSummary();
    }

    function setReady(rows: string[]) {
      hasVisibleValue = true;
      errorMessage = null;
      value.set(rows);
      status = {
        kind: "fulfilled",
        operation: "refresh",
      };
      publishSummary();
    }

    function setError(rows: string[], message: string) {
      hasVisibleValue = true;
      errorMessage = message;
      value.set(rows);
      status = {
        kind: "rejected",
        operation: "refresh",
        message,
        continuity: "preservedVisibleValue",
      };
      publishSummary();
    }

    function setErrorWithoutValueChange(message: string) {
      hasVisibleValue = true;
      errorMessage = message;
      status = {
        kind: "rejected",
        operation: "refresh",
        message,
        continuity: "preservedVisibleValue",
      };
      publishSummary();
    }

    function dispose() {
      store.dispose();
      signals.free();
    }

    return {
      store,
      line,
      setLoading,
      setRefreshing,
      setReady,
      setError,
      setErrorWithoutValueChange,
      dispose,
    };
  });
}

function ResourceViewProbe({
  line,
  store,
  errorMessage,
}: {
  line: FakeLine | null;
  store: ReturnType<typeof createReactSignalsStore>;
  errorMessage?: string;
}): JSX.Element {
  const result = useResourceView<string[], string, FakeLine>(line, store, {
    inactiveValue: ["inactive"],
    emptyWhen(value) {
      return value.length === 0;
    },
    errorMessage,
  });

  if (result.kind === "inactive") {
    return <div data-testid="resource-view">inactive</div>;
  }

  return (
    <div data-testid="resource-view">
      {[
        result.contentState,
        result.value.join("|") || "none",
        result.message ?? "no-message",
        result.hasVisibleValue ? "visible" : "not-visible",
        result.isRefreshing ? "refreshing" : "steady",
        result.isEmpty ? "empty" : "not-empty",
      ].join(":")}
    </div>
  );
}

afterEach(() => {
  cleanup();
});

describe("resource view React hook", () => {
  it("projects inactive, loading, refreshing, ready, empty, and error view states", async () => {
    const fixture = await createFixture();
    const rendered = render(
      <ResourceViewProbe line={null} store={fixture.store} />,
    );

    try {
      expect(screen.getByTestId("resource-view").textContent).toBe("inactive");

      fixture.setLoading();
      rendered.rerender(<ResourceViewProbe line={fixture.line} store={fixture.store} />);
      expect(screen.getByTestId("resource-view").textContent).toBe(
        "loading:none:no-message:not-visible:steady:not-empty",
      );

      await act(async () => {
        fixture.setRefreshing(["sku-1", "sku-2"]);
        await Promise.resolve();
      });
      expect(screen.getByTestId("resource-view").textContent).toBe(
        "refreshing:sku-1|sku-2:no-message:visible:refreshing:not-empty",
      );

      await act(async () => {
        fixture.setReady(["sku-1"]);
        await Promise.resolve();
      });
      expect(screen.getByTestId("resource-view").textContent).toBe(
        "ready:sku-1:no-message:visible:steady:not-empty",
      );

      await act(async () => {
        fixture.setReady([]);
        await Promise.resolve();
      });
      expect(screen.getByTestId("resource-view").textContent).toBe(
        "empty:none:no-message:visible:steady:empty",
      );

      await act(async () => {
        fixture.setError(["sku-9"], "Unable to refresh catalog.");
        await Promise.resolve();
      });
      expect(screen.getByTestId("resource-view").textContent).toBe(
        "error:sku-9:Unable to refresh catalog.:visible:steady:not-empty",
      );
    } finally {
      rendered.unmount();
      fixture.dispose();
    }
  });

  it("lets apps override the default error message copy", async () => {
    const fixture = await createFixture();
    fixture.setError(["sku-4"], "transport failure");
    const rendered = render(
      <ResourceViewProbe
        line={fixture.line}
        store={fixture.store}
        errorMessage="Unable to load products."
      />,
    );

    try {
      expect(screen.getByTestId("resource-view").textContent).toBe(
        "error:sku-4:Unable to load products.:visible:steady:not-empty",
      );
    } finally {
      rendered.unmount();
      fixture.dispose();
    }
  });

  it("updates content state when lifecycle truth changes without a visible-value change", async () => {
    const fixture = await createFixture();
    fixture.setReady(["sku-1"]);
    const rendered = render(
      <ResourceViewProbe line={fixture.line} store={fixture.store} />,
    );

    try {
      expect(screen.getByTestId("resource-view").textContent).toBe(
        "ready:sku-1:no-message:visible:steady:not-empty",
      );

      await act(async () => {
        fixture.setErrorWithoutValueChange("Permission changed.");
        await Promise.resolve();
      });

      expect(screen.getByTestId("resource-view").textContent).toBe(
        "error:sku-1:Permission changed.:visible:steady:not-empty",
      );
    } finally {
      rendered.unmount();
      fixture.dispose();
    }
  });
});
