import React from "react";
import { act, cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { createSignals } from "@aust-group/forge-signal-wasm";
import {
  createManagedResourceWriteExecution,
  createReactSignalsStore,
  useResourceOperation,
} from "@aust-group/forge-signal-wasm/react";

type FakeStatus =
  | {
      kind: "pending";
      operation: "delivery";
      continuity: "preservedVisibleValue";
    }
  | {
      kind: "fulfilled";
      operation: "delivery";
    }
  | {
      kind: "rejected";
      operation: "delivery";
      message: string;
      continuity: "preservedVisibleValue";
    };

type FakeLine = ReturnType<typeof createFixture> extends Promise<infer TFixture>
  ? TFixture["line"]
  : never;

async function createFixture() {
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const store = createReactSignalsStore(signals);
  const valueSignal = signals.input({ id: "product-7" });
  const summarySignal = signals.input({
    current: {
      status: {
        kind: "pending",
        operation: "delivery",
        continuity: "preservedVisibleValue",
      } as FakeStatus,
      freshness: { kind: "fresh" as const },
      hasVisibleValue: true,
      visibleValueVersion: 1,
      visibleSelection: null,
    },
    diagnostics: {
      current: {
        status: {
          kind: "pending",
          operation: "delivery",
          continuity: "preservedVisibleValue",
        } as FakeStatus,
        freshness: { kind: "fresh" as const },
        hasVisibleValue: true,
        visibleValueVersion: 1,
        visibleSelection: null,
      },
      latest: {
        errorMessage: null as string | null,
      },
    },
  });

  let status: FakeStatus = {
    kind: "pending",
    operation: "delivery",
    continuity: "preservedVisibleValue",
  };
  let confirmationKind: string | null = null;
  let errorMessage: string | null = null;

  function publishSummary() {
    summarySignal.set({
      current: {
        status,
        freshness: { kind: "fresh" as const },
        hasVisibleValue: true,
        visibleValueVersion: 1,
        visibleSelection: null,
      },
      diagnostics: {
        current: {
          status,
          freshness: { kind: "fresh" as const },
          hasVisibleValue: true,
          visibleValueVersion: 1,
          visibleSelection: null,
        },
        latest: {
          errorMessage,
        },
      },
    });
  }

  const line = {
    value() {
      return valueSignal.get();
    },
    signal() {
      return valueSignal;
    },
    summarySignal() {
      return summarySignal;
    },
    summary() {
      return summarySignal.get();
    },
    status() {
      return status;
    },
    freshness() {
      return { kind: "fresh" as const };
    },
    diagnosticsSummary() {
      return summarySignal.get().diagnostics;
    },
    mutationResponse() {
      return confirmationKind === null
        ? null
        : { confirmation: { kind: confirmationKind } };
    },
    free() {},
    execute() {
      return {
        line,
        settled() {
          throw new Error("not needed in this test");
        },
        free() {},
        [Symbol.dispose]() {},
      };
    },
    awaitSettlement() {
      throw new Error("not needed in this test");
    },
    invalidate() {
      return { kind: "fresh" as const };
    },
    refresh() {
      return status;
    },
    revalidate() {
      return status;
    },
    descriptor() {
      return {};
    },
    request() {
      return {};
    },
    download() {
      return {};
    },
    history() {
      return {};
    },
    processing() {
      return {};
    },
    upload() {
      return {};
    },
    diagnostics() {
      return {};
    },
    view<TValue>(project: (value: { id: string }) => TValue) {
      return {
        id: "resource-view",
        get() {
          return project(valueSignal.get());
        },
      };
    },
    [Symbol.dispose]() {},
  };

  return {
    store,
    line,
    dispose() {
      store.dispose();
      signals.free();
    },
    setPartial() {
      confirmationKind = "partialCanonicalTruth";
      errorMessage = null;
      status = { kind: "fulfilled", operation: "delivery" };
      publishSummary();
    },
    setFulfilled() {
      confirmationKind = "consumedCanonicalTruth";
      errorMessage = null;
      status = { kind: "fulfilled", operation: "delivery" };
      publishSummary();
    },
    setRejected(message: string) {
      confirmationKind = null;
      errorMessage = message;
      status = {
        kind: "rejected",
        operation: "delivery",
        message,
        continuity: "preservedVisibleValue",
      };
      publishSummary();
    },
  };
}

function ResourceOperationProbe({
  execution,
  store,
}: {
  execution: { line: FakeLine };
  store: ReturnType<typeof createReactSignalsStore>;
}): JSX.Element {
  const operation = useResourceOperation(execution, store);
  return (
    <div data-testid="resource-operation">
      {[
        operation.resultKind,
        operation.status.kind,
        operation.confirmationKind ?? "none",
        operation.pending ? "pending" : "steady",
        operation.message ?? "no-message",
      ].join(":")}
    </div>
  );
}

afterEach(() => {
  cleanup();
});

describe("resource operation React hook", () => {
  it("projects pending, partial, and rejected truth from a line execution", async () => {
    const fixture = await createFixture();
    const execution = fixture.line.execute();
    const rendered = render(
      <ResourceOperationProbe execution={execution} store={fixture.store} />,
    );

    try {
      expect(screen.getByTestId("resource-operation").textContent).toBe(
        "pending:pending:none:pending:no-message",
      );

      await act(async () => {
        fixture.setPartial();
        await Promise.resolve();
      });
      expect(screen.getByTestId("resource-operation").textContent).toBe(
        "partial:fulfilled:partialCanonicalTruth:steady:no-message",
      );

      await act(async () => {
        fixture.setRejected("permission denied");
        await Promise.resolve();
      });
      expect(screen.getByTestId("resource-operation").textContent).toBe(
        "rejected:rejected:none:steady:permission denied",
      );
    } finally {
      rendered.unmount();
      fixture.dispose();
    }
  });

  it("accepts the managed write execution surface too", async () => {
    const fixture = await createFixture();
    const execution = createManagedResourceWriteExecution(fixture.line);
    const rendered = render(
      <ResourceOperationProbe execution={execution} store={fixture.store} />,
    );

    try {
      expect(screen.getByTestId("resource-operation").textContent).toBe(
        "pending:pending:none:pending:no-message",
      );

      await act(async () => {
        fixture.setFulfilled();
        await Promise.resolve();
      });
      expect(screen.getByTestId("resource-operation").textContent).toBe(
        "fulfilled:fulfilled:consumedCanonicalTruth:steady:no-message",
      );
    } finally {
      rendered.unmount();
      fixture.dispose();
    }
  });
});
