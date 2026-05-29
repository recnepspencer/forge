import { describe, expect, it } from "vitest";

import { executeManagedResourceWrite } from "@aust-group/forge-signal-wasm/react";

type FakeStatus =
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
  execute(): {
    readonly line: FakeLine;
    settled(): Promise<{
      resultKind: "fulfilled" | "partial" | "rejected";
      status: FakeStatus;
      value?: { id: string };
      summary: { status: string };
      freshness: { kind: "fresh" };
      diagnosticsSummary: { latest: { errorMessage: string | null } };
      mutationResponse: { confirmation: { kind: string } } | null;
      confirmationKind: string | null;
    }>;
    free(): void;
    [Symbol.dispose](): void;
  };
  awaitSettlement(): Promise<{
    resultKind: "fulfilled" | "partial" | "rejected";
    status: FakeStatus;
    value?: { id: string };
    summary: { status: string };
    freshness: { kind: "fresh" };
    diagnosticsSummary: { latest: { errorMessage: string | null } };
    mutationResponse: { confirmation: { kind: string } } | null;
    confirmationKind: string | null;
  }>;
  status(): FakeStatus;
  freshness(): { kind: "fresh" };
  view<TValue>(project: (value: { id: string }) => TValue): { id: string; get(): TValue };
  [Symbol.dispose](): void;
};

function createSummaryFixture(resultKind: "partialCanonicalTruth" | "refetchRequired" | "deliveryAwaited" | "rejected") {
  const status: FakeStatus = resultKind === "rejected"
    ? {
        kind: "rejected",
        operation: "delivery",
        message: "permission denied",
        continuity: "preservedVisibleValue",
      }
    : {
        kind: "fulfilled",
        operation: "delivery",
      };

  const settled = {
    resultKind: resultKind === "rejected"
      ? "rejected"
      : resultKind === "partialCanonicalTruth" || resultKind === "refetchRequired" || resultKind === "deliveryAwaited"
        ? "partial"
        : "fulfilled",
    status,
    value: { id: "product-7" },
    summary: { status: "ready" },
    freshness: { kind: "fresh" as const },
    diagnosticsSummary: {
      latest: {
        errorMessage: resultKind === "rejected" ? "permission denied" : null,
      },
    },
    mutationResponse: resultKind === "rejected"
      ? null
      : { confirmation: { kind: resultKind } },
    confirmationKind: resultKind === "rejected" ? null : resultKind,
  };

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
      return settled.diagnosticsSummary;
    },
    mutationResponse() {
      return settled.mutationResponse;
    },
    free() {},
    [Symbol.dispose]() {},
    invalidate() {
      return { kind: "fresh" };
    },
    refresh() {
      return status;
    },
    revalidate() {
      return status;
    },
    execute() {
      return {
        line,
        settled: () => Promise.resolve(settled),
        free() {},
        [Symbol.dispose]() {},
      };
    },
    awaitSettlement() {
      return Promise.resolve(settled);
    },
    status() {
      return status;
    },
    freshness() {
      return { kind: "fresh" };
    },
    view(project) {
      return { id: "resource-line-view", get: () => project({ id: "product-7" }) };
    },
  };

  return line;
}

describe("managed resource write recovery summary", () => {
  it("classifies safe delayed fallback as info", async () => {
    const result = await executeManagedResourceWrite(createSummaryFixture("deliveryAwaited"));
    expect(result.recovery.summary()).toMatchObject({
      severity: "info",
      reason: "deliveryAwaited",
      recommendedFollowup: "waitForDelivery",
    });
  });

  it("classifies explicit refetch fallback as warning", async () => {
    const result = await executeManagedResourceWrite(createSummaryFixture("refetchRequired"));
    expect(result.recovery.summary()).toMatchObject({
      severity: "warning",
      reason: "refetchRequired",
      recommendedFollowup: "refreshResidentTruth",
    });
  });

  it("classifies rejected writes without app-authored severity heuristics", async () => {
    const result = await executeManagedResourceWrite(createSummaryFixture("rejected"));
    expect(result.recovery.summary()).toMatchObject({
      severity: "error",
      reason: "rejected",
      recommendedFollowup: "inspectWriteFailure",
    });
  });
});
