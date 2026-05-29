import React from "react";
import { act, cleanup, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import {
  createManagedResourceWriteExecution,
  executeManagedResourceWrite,
  managedResourceWriteFeedback,
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

function createFeedbackFixture() {
  let currentStatus: FakeStatus = {
    kind: "pending",
    operation: "delivery",
    continuity: "preservedVisibleValue",
  };
  let confirmationKind: string | null = null;
  const listeners = new Set<() => void>();

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
      return { id: "resource-line-value", get: () => ({ id: "product-7" }) };
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
      return {
        latest: {
          errorMessage: currentStatus.kind === "rejected" ? currentStatus.message : null,
        },
      };
    },
    mutationResponse() {
      return confirmationKind === null ? null : { confirmation: { kind: confirmationKind } };
    },
    free() {},
    [Symbol.dispose]() {},
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
              line.free();
            }
          });
          return settlementPromise;
        },
        free() {
          line.free();
        },
        [Symbol.dispose]() {
          line.free();
        },
      };
    },
    awaitSettlement(options) {
      if (currentStatus.kind === "fulfilled") {
        return Promise.resolve({
          resultKind: confirmationKind === "partialCanonicalTruth" ? "partial" : "fulfilled",
          status: currentStatus,
          value: { id: "product-7" },
          summary: { status: "ready" },
          freshness: { kind: "fresh" },
          diagnosticsSummary: line.diagnosticsSummary(),
          mutationResponse: line.mutationResponse(),
          confirmationKind,
        });
      }
      if (currentStatus.kind === "rejected") {
        return Promise.resolve({
          resultKind: "rejected",
          status: currentStatus,
          summary: { status: "ready" },
          freshness: { kind: "fresh" },
          diagnosticsSummary: line.diagnosticsSummary(),
          mutationResponse: null,
          confirmationKind: null,
        });
      }
      return new Promise((resolve, reject) => {
        let timeoutHandle: ReturnType<typeof setTimeout> | null = null;
        const listener = () => {
          listeners.delete(listener);
          if (timeoutHandle !== null) {
            clearTimeout(timeoutHandle);
          }
          line.awaitSettlement().then(resolve, reject);
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
      return { id: "resource-line-view", get: () => project({ id: "product-7" }) };
    },
  };

  return {
    line,
    settlePartial() {
      confirmationKind = "partialCanonicalTruth";
      currentStatus = { kind: "fulfilled", operation: "delivery" };
      notifySettlement();
    },
    reject(message = "permission denied") {
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

afterEach(() => {
  cleanup();
});

describe("managed resource write feedback", () => {
  it("derives standardized feedback with only message customization", async () => {
    const fixture = createFeedbackFixture();

    const pending = executeManagedResourceWrite(fixture.line, {
      feedback: {
        partial: "Project created, refreshing list",
      },
    });

    fixture.settlePartial();
    const result = await pending;

    expect(result.feedback).toMatchObject({
      kind: "partial",
      title: "Project created, refreshing list",
      resultKind: "partial",
      confirmationKind: "partialCanonicalTruth",
    });
    expect(managedResourceWriteFeedback.create(result, {
      error: "Unable to create project",
    })).toMatchObject({
      kind: "partial",
      title: "Saved with follow-up refresh",
    });
  });

  it("supports a lower-level execution.feedback() lane without app-authored classification", async () => {
    const fixture = createFeedbackFixture();
    const execution = createManagedResourceWriteExecution(fixture.line, {
      feedback: {
        partial: "Project created, refreshing list",
      },
    });

    fixture.settlePartial();
    const feedback = await execution.feedback();

    expect(feedback).toMatchObject({
      kind: "partial",
      title: "Project created, refreshing list",
      resultKind: "partial",
    });
  });

  it("surfaces standardized rejection feedback from the managed hook and callback bridge", async () => {
    const fixture = createFeedbackFixture();
    const onFeedback = vi.fn();
    const controller: {
      current: ReturnType<typeof useManagedResourceWrite<{ productId: string }, FakeLine>> | null;
    } = { current: null };

    function Probe(): JSX.Element {
      const managed = useManagedResourceWrite({
        line: () => fixture.line,
        feedback: {
          error: "Unable to create project",
        },
        onFeedback,
      });
      controller.current = managed;
      return <div>{managed.lastFeedback?.title ?? "idle"}</div>;
    }

    render(<Probe />);

    let execution: Promise<unknown> | null = null;
    await act(async () => {
      execution = controller.current?.execute({ productId: "product-7" }) ?? null;
    });

    fixture.reject("permission denied");
    await act(async () => {
      try {
        await execution;
      } catch {
        // handled below through managed state
      }
    });

    expect(controller.current?.lastFeedback).toMatchObject({
      kind: "error",
      title: "Unable to create project",
      description: "permission denied",
      resultKind: "rejected",
    });
    expect(onFeedback).toHaveBeenCalledTimes(1);
    expect(onFeedback).toHaveBeenCalledWith(
      expect.objectContaining({
        kind: "error",
        title: "Unable to create project",
      }),
    );
  });
});
