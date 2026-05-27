import React from "react";
import { act, cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import {
  executeManagedResourceWrite,
  useManagedResourceWrite,
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

type FakeAwaitSettlementResult =
  | {
      resultKind: "fulfilled" | "partial";
      status: Extract<FakeStatus, { kind: "fulfilled" }>;
      value: { id: string };
      summary: { status: string };
      freshness: { kind: "fresh" };
      diagnosticsSummary: { latest: { errorMessage: string | null } };
      mutationResponse: { confirmation: { kind: string } } | null;
      confirmationKind: string | null;
    }
  | {
      resultKind: "rejected";
      status: Extract<FakeStatus, { kind: "rejected" }>;
      summary: { status: string };
      freshness: { kind: "fresh" };
      diagnosticsSummary: { latest: { errorMessage: string | null } };
      mutationResponse: null;
      confirmationKind: null;
    };

type FakeLine = {
  value(): { id: string };
  signal(): { id: string; get(): { id: string } };
  descriptor(): {};
  request(): {};
  summary(): { status: string };
  download(): {};
  history(): {};
  processing(): {};
  upload(): {};
  diagnostics(): {};
  diagnosticsSummary(): { latest: { errorMessage: string | null } };
  mutationResponse(): { confirmation: { kind: string } } | null;
  free(): void;
  invalidate(): { kind: "fresh" };
  refresh(): FakeStatus;
  revalidate(): FakeStatus;
  execute(options?: { freeOnSettle?: boolean }): {
    readonly line: FakeLine;
    settled(options?: { timeoutMs?: number }): Promise<FakeAwaitSettlementResult>;
    free(): void;
    [Symbol.dispose](): void;
  };
  awaitSettlement(options?: { timeoutMs?: number }): Promise<FakeAwaitSettlementResult>;
  status(): FakeStatus;
  freshness(): { kind: "fresh" };
  view<TValue>(project: (value: { id: string }) => TValue): { id: string; get(): TValue };
  [Symbol.dispose](): void;
};

type ManagedWriteController = {
  current: {
    pending: boolean;
    lastResult: { resultKind: string } | null;
    execute(args: { productId: string }): Promise<unknown>;
  } | null;
};

function createFakeLineFixture() {
  let currentStatus: FakeStatus = {
    kind: "pending",
    operation: "delivery",
    continuity: "preservedVisibleValue",
  };
  let confirmationKind: string | null = null;
  const listeners = new Set<() => void>();
  const free = vi.fn();
  const revalidateResident = vi.fn();

  function diagnosticsSummary() {
    return {
      latest: {
        errorMessage: currentStatus.kind === "rejected" ? currentStatus.message : null,
      },
    };
  }

  function currentSettlement(): FakeAwaitSettlementResult | null {
    if (currentStatus.kind === "pending") {
      return null;
    }
    if (currentStatus.kind === "fulfilled") {
      return {
        resultKind: confirmationKind === "partialCanonicalTruth"
          ? "partial"
          : "fulfilled",
        status: currentStatus,
        value: { id: "product-7" },
        summary: { status: "ready" },
        freshness: { kind: "fresh" },
        diagnosticsSummary: diagnosticsSummary(),
        mutationResponse: confirmationKind === null
          ? null
          : { confirmation: { kind: confirmationKind } },
        confirmationKind,
      };
    }
    return {
      resultKind: "rejected",
      status: currentStatus,
      summary: { status: "ready" },
      freshness: { kind: "fresh" },
      diagnosticsSummary: diagnosticsSummary(),
      mutationResponse: null,
      confirmationKind: null,
    };
  }

  function notifySettlement() {
    for (const listener of [...listeners]) {
      listener();
    }
  }

  const line: FakeLine = {
    value() {
      return { id: "product-7" };
    },
    signal() {
      return {
        id: "resource-line-value",
        get() {
          return { id: "product-7" };
        },
      };
    },
    descriptor() {
      return {};
    },
    request() {
      return {};
    },
    summary() {
      return { status: "ready" };
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
    diagnosticsSummary,
    mutationResponse() {
      return confirmationKind === null
        ? null
        : { confirmation: { kind: confirmationKind } };
    },
    free,
    [Symbol.dispose]() {
      free();
    },
    invalidate() {
      return { kind: "fresh" };
    },
    refresh() {
      return currentStatus;
    },
    revalidate() {
      return currentStatus;
    },
    execute(options) {
      let settlementPromise: Promise<FakeAwaitSettlementResult> | null = null;
      return {
        line,
        settled(settlementOptions) {
          if (settlementPromise !== null) {
            return settlementPromise;
          }
          settlementPromise = line.awaitSettlement(settlementOptions).finally(() => {
            if (options?.freeOnSettle ?? true) {
              free();
            }
          });
          return settlementPromise;
        },
        free() {
          free();
        },
        [Symbol.dispose]() {
          free();
        },
      };
    },
    awaitSettlement(options) {
      const settled = currentSettlement();
      if (settled !== null) {
        return Promise.resolve(settled);
      }
      return new Promise((resolve, reject) => {
        let timeoutHandle: ReturnType<typeof setTimeout> | null = null;
        const listener = () => {
          const next = currentSettlement();
          if (next === null) {
            return;
          }
          listeners.delete(listener);
          if (timeoutHandle !== null) {
            clearTimeout(timeoutHandle);
          }
          resolve(next);
        };
        listeners.add(listener);
        if (typeof options?.timeoutMs === "number" && options.timeoutMs >= 0) {
          timeoutHandle = setTimeout(() => {
            listeners.delete(listener);
            reject(new Error("Timed out waiting for resource line settlement."));
          }, options.timeoutMs);
        }
      });
    },
    status() {
      return currentStatus;
    },
    freshness() {
      return { kind: "fresh" };
    },
    view(project) {
      return {
        id: "resource-line-view",
        get() {
          return project({ id: "product-7" });
        },
      };
    },
  };

  return {
    line,
    free,
    revalidateResident,
    settlePartial() {
      confirmationKind = "partialCanonicalTruth";
      currentStatus = { kind: "fulfilled", operation: "delivery" };
      notifySettlement();
    },
    settleFulfilled() {
      confirmationKind = "consumedCanonicalTruth";
      currentStatus = { kind: "fulfilled", operation: "delivery" };
      notifySettlement();
    },
    reject(message = "write failed") {
      confirmationKind = null;
      currentStatus = {
        kind: "rejected",
        operation: "delivery",
        message,
        continuity: "preservedVisibleValue",
      };
      notifySettlement();
    },
  };
}

function ManagedWriteProbe({
  controller,
  createLine,
}: {
  controller: ManagedWriteController;
  createLine(args: { productId: string }): FakeLine;
}): JSX.Element {
  const managed = useManagedResourceWrite({
    createLine,
  });
  controller.current = managed;
  return (
    <div data-testid="managed-write-state">
      {managed.pending ? "pending" : managed.lastResult?.resultKind ?? "idle"}
    </div>
  );
}

afterEach(() => {
  cleanup();
});

describe("managed resource write execution", () => {
  it("classifies partial settlement, runs partial follow-up, and frees the transient line", async () => {
    const fixture = createFakeLineFixture();
    const onPartial = vi.fn(() => {
      fixture.revalidateResident();
    });

    const promise = executeManagedResourceWrite(fixture.line, {
      onPartial,
    });

    fixture.settlePartial();
    const result = await promise;

    expect(result.resultKind).toBe("partial");
    expect(result.confirmationKind).toBe("partialCanonicalTruth");
    expect(onPartial).toHaveBeenCalledTimes(1);
    expect(fixture.revalidateResident).toHaveBeenCalledTimes(1);
    expect(fixture.free).toHaveBeenCalledTimes(1);
  });

  it("exposes pending state and last result through the React hook", async () => {
    const fixture = createFakeLineFixture();
    const controller: ManagedWriteController = { current: null };

    render(
      <ManagedWriteProbe
        controller={controller}
        createLine={() => fixture.line}
      />,
    );

    expect(screen.getByTestId("managed-write-state").textContent).toBe("idle");

    let execution: Promise<unknown> | null = null;
    await act(async () => {
      execution = controller.current?.execute({ productId: "product-7" }) ?? null;
    });
    expect(screen.getByTestId("managed-write-state").textContent).toBe("pending");

    fixture.reject("permission denied");
    await act(async () => {
      await execution;
    });

    expect(screen.getByTestId("managed-write-state").textContent).toBe("rejected");
    expect(controller.current?.lastResult?.resultKind).toBe("rejected");
    expect(controller.current?.pending).toBe(false);
    expect(fixture.free).toHaveBeenCalledTimes(1);
  });

  it("supports the cleaner line(args) hook authoring lane", async () => {
    const fixture = createFakeLineFixture();
    const controller: ManagedWriteController = { current: null };

    function LineOptionProbe({
      line,
      controller,
    }: {
      line(args: { productId: string }): FakeLine;
      controller: ManagedWriteController;
    }): JSX.Element {
      const managed = useManagedResourceWrite({
        line,
      });
      controller.current = managed;
      return <div data-testid="managed-write-state">{managed.pending ? "pending" : managed.lastResult?.resultKind ?? "idle"}</div>;
    }

    render(<LineOptionProbe controller={controller} line={() => fixture.line} />);

    let execution: Promise<unknown> | null = null;
    await act(async () => {
      execution = controller.current?.execute({ productId: "product-7" }) ?? null;
    });

    expect(screen.getByTestId("managed-write-state").textContent).toBe("pending");

    fixture.settleFulfilled();
    await act(async () => {
      await execution;
    });

    expect(screen.getByTestId("managed-write-state").textContent).toBe("fulfilled");
    expect(fixture.free).toHaveBeenCalledTimes(1);
  });

  it("still frees the transient line when a managed follow-up callback throws", async () => {
    const fixture = createFakeLineFixture();

    const promise = executeManagedResourceWrite(fixture.line, {
      onPartial() {
        throw new Error("follow-up failed");
      },
    });
    const handled = promise.catch((error) => error);

    fixture.settlePartial();

    const error = await handled;
    expect(error).toBeInstanceOf(Error);
    expect((error as Error).message).toBe("follow-up failed");
    expect(fixture.free).toHaveBeenCalledTimes(1);
  });

  it("keeps pending true until every overlapping execution settles", async () => {
    const first = createFakeLineFixture();
    const second = createFakeLineFixture();
    const controller: ManagedWriteController = { current: null };
    let executionCount = 0;

    render(
      <ManagedWriteProbe
        controller={controller}
        createLine={() => {
          executionCount += 1;
          return executionCount === 1 ? first.line : second.line;
        }}
      />,
    );

    let firstExecution: Promise<unknown> | null = null;
    let secondExecution: Promise<unknown> | null = null;
    await act(async () => {
      firstExecution = controller.current?.execute({ productId: "product-7" }) ?? null;
      secondExecution = controller.current?.execute({ productId: "product-7" }) ?? null;
      await Promise.resolve();
    });

    expect(screen.getByTestId("managed-write-state").textContent).toBe("pending");

    first.reject("first failed");
    await act(async () => {
      try {
        await firstExecution;
      } catch {
        // expected for this regression
      }
    });

    expect(screen.getByTestId("managed-write-state").textContent).toBe("pending");

    second.settleFulfilled();
    await act(async () => {
      await secondExecution;
    });

    expect(screen.getByTestId("managed-write-state").textContent).toBe("fulfilled");
    expect(first.free).toHaveBeenCalledTimes(1);
    expect(second.free).toHaveBeenCalledTimes(1);
  });
});
