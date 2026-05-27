import { describe, expect, it, vi } from "vitest";

import {
  executeManagedResourceWrite,
  managedResourceWriteRecovery,
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

type FakeAwaitSettlementResult = {
  resultKind: "partial";
  status: Extract<FakeStatus, { kind: "fulfilled" }>;
  value: { id: string };
  summary: { status: string };
  freshness: { kind: "fresh" };
  diagnosticsSummary: { latest: { errorMessage: string | null } };
  mutationResponse: { confirmation: { kind: string } };
  confirmationKind: string;
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

function createWriteLineFixture() {
  let currentStatus: FakeStatus = {
    kind: "pending",
    operation: "delivery",
    continuity: "preservedVisibleValue",
  };
  let confirmationKind: string | null = null;
  const listeners = new Set<() => void>();
  const free = vi.fn();

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
    diagnosticsSummary() {
      return { latest: { errorMessage: currentStatus.kind === "rejected" ? currentStatus.message : null } };
    },
    mutationResponse() {
      return confirmationKind === null ? null : { confirmation: { kind: confirmationKind } };
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
      if (currentStatus.kind === "fulfilled" && confirmationKind !== null) {
        return Promise.resolve({
          resultKind: "partial",
          status: currentStatus,
          value: { id: "product-7" },
          summary: { status: "ready" },
          freshness: { kind: "fresh" },
          diagnosticsSummary: { latest: { errorMessage: null } },
          mutationResponse: { confirmation: { kind: confirmationKind } },
          confirmationKind,
        });
      }
      return new Promise((resolve, reject) => {
        let timeoutHandle: ReturnType<typeof setTimeout> | null = null;
        const listener = () => {
          if (currentStatus.kind !== "fulfilled" || confirmationKind === null) {
            return;
          }
          listeners.delete(listener);
          if (timeoutHandle !== null) {
            clearTimeout(timeoutHandle);
          }
          resolve({
            resultKind: "partial",
            status: currentStatus,
            value: { id: "product-7" },
            summary: { status: "ready" },
            freshness: { kind: "fresh" },
            diagnosticsSummary: { latest: { errorMessage: null } },
            mutationResponse: { confirmation: { kind: confirmationKind } },
            confirmationKind,
          });
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
    settlePartial() {
      confirmationKind = "partialCanonicalTruth";
      currentStatus = { kind: "fulfilled", operation: "delivery" };
      notifySettlement();
    },
  };
}

function createResidentLineController() {
  const refresh = vi.fn(() => ({
    kind: "pending" as const,
    operation: "refresh" as const,
    continuity: "preservedVisibleValue" as const,
  }));
  const revalidate = vi.fn(() => ({
    kind: "pending" as const,
    operation: "revalidate" as const,
    continuity: "preservedVisibleValue" as const,
  }));

  return {
    line: {
      refresh,
      revalidate,
    },
    refresh,
    revalidate,
  };
}

describe("managed resource write recovery policy", () => {
  it("executes declarative partial recovery steps and returns recovery artifacts", async () => {
    const write = createWriteLineFixture();
    const listResident = createResidentLineController();
    const detailResident = createResidentLineController();

    const promise = executeManagedResourceWrite(write.line, {
      recovery: {
        partial: [
          managedResourceWriteRecovery.revalidate(
            () => listResident.line,
            "refresh the catalog list",
          ),
          managedResourceWriteRecovery.refresh(
            () => detailResident.line,
            "reload the current detail",
          ),
        ],
      },
    });

    write.settlePartial();
    const result = await promise;

    expect(result.resultKind).toBe("partial");
    expect(result.recovery.executions).toHaveLength(2);
    expect(result.recovery.executions[0]).toMatchObject({
      kind: "revalidateResourceLine",
      reason: "refresh the catalog list",
      status: {
        kind: "pending",
        operation: "revalidate",
      },
      error: null,
    });
    expect(result.recovery.executions[1]).toMatchObject({
      kind: "refreshResourceLine",
      reason: "reload the current detail",
      status: {
        kind: "pending",
        operation: "refresh",
      },
      error: null,
    });
    expect(listResident.revalidate).toHaveBeenCalledTimes(1);
    expect(detailResident.refresh).toHaveBeenCalledTimes(1);
    expect(write.free).toHaveBeenCalledTimes(1);
  });

  it("preserves the settled write result when declarative recovery itself fails", async () => {
    const write = createWriteLineFixture();
    const onSettled = vi.fn();

    const promise = executeManagedResourceWrite(write.line, {
      recovery: {
        partial: [
          managedResourceWriteRecovery.revalidate(
            () => {
              throw new Error("resident line missing");
            },
            "attempt the list refresh",
          ),
        ],
      },
      onSettled,
    });

    write.settlePartial();
    const result = await promise;

    expect(result.resultKind).toBe("partial");
    expect(result.recovery.executions).toHaveLength(1);
    expect(result.recovery.executions[0]).toMatchObject({
      kind: "revalidateResourceLine",
      line: null,
      reason: "attempt the list refresh",
      status: null,
    });
    expect(result.recovery.executions[0]?.error).toBeInstanceOf(Error);
    expect((result.recovery.executions[0]?.error as Error).message).toBe("resident line missing");
    expect(onSettled).toHaveBeenCalledTimes(1);
    expect(onSettled).toHaveBeenCalledWith(result);
    expect(write.free).toHaveBeenCalledTimes(1);
  });

  it("supports applying recovery policy after settlement and resolving it from execution context", async () => {
    const write = createWriteLineFixture();
    const listResident = createResidentLineController();

    write.settlePartial();
    const settled = await executeManagedResourceWrite(write.line, {
      freeOnSettle: false,
    });
    const recovered = await settled.recovery.apply({
        partial: [
          managedResourceWriteRecovery.revalidate(
            () => listResident.line,
            "refresh the list after a partial command",
          ),
        ],
      });

    expect(recovered.resultKind).toBe("partial");
    expect(recovered.recovery.executions).toHaveLength(1);
    expect(recovered.recovery.executions[0]).toMatchObject({
      kind: "revalidateResourceLine",
      reason: "refresh the list after a partial command",
      error: null,
    });
    expect(listResident.revalidate).toHaveBeenCalledTimes(1);
  });
});
