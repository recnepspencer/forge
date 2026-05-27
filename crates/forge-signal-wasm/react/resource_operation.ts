import { useMemo } from "react";

import { useReactSignalsStore } from "./context.js";
import { useSignalValue } from "./hooks.js";

import type {
  ReactSignalsStore,
  ResourceLineReactLike,
  ResourceOperationExecutionReactLike,
  ResourceOperationResultKind,
  ResourceOperationView,
} from "./model.js";

function readResourceOperationResultKind(
  status: ResourceLineReactLike["status"] extends () => infer TStatus ? TStatus : never,
  mutationResponse: ResourceLineReactLike["mutationResponse"] extends () => infer TPlan ? TPlan : never,
): ResourceOperationResultKind {
  if (status.kind === "pending") {
    return "pending";
  }
  if (status.kind === "rejected") {
    return "rejected";
  }
  if (status.kind === "timedOut") {
    return "timedOut";
  }
  const confirmationKind = mutationResponse?.confirmation.kind;
  if (
    confirmationKind === "partialCanonicalTruth"
    || confirmationKind === "refetchRequired"
    || confirmationKind === "deliveryAwaited"
  ) {
    return "partial";
  }
  return "fulfilled";
}

function readResourceOperationMessage(
  status: ResourceLineReactLike["status"] extends () => infer TStatus ? TStatus : never,
  diagnosticsSummary: ResourceLineReactLike["diagnosticsSummary"] extends () => infer TSummary ? TSummary : never,
): string | null {
  if ((status.kind === "rejected" || status.kind === "timedOut") && "message" in status) {
    return status.message;
  }
  return diagnosticsSummary.latest.errorMessage ?? null;
}

export function useResourceOperation<
  TValue = unknown,
  TParams = unknown,
  TLine extends ResourceLineReactLike<TValue, TParams> = ResourceLineReactLike<TValue, TParams>,
  TExecution extends ResourceOperationExecutionReactLike<TValue, TParams, TLine> = ResourceOperationExecutionReactLike<TValue, TParams, TLine>,
>(
  execution: TExecution,
  store?: ReactSignalsStore,
): ResourceOperationView<TLine, TValue, TParams> {
  const resolvedStore = store ?? useReactSignalsStore();
  const summary = useSignalValue<ReturnType<TLine["summary"]>>(
    execution.line.summarySignal(),
    resolvedStore,
  );

  return useMemo(() => {
    const line = execution.line;
    const mutationResponse = line.mutationResponse();
    const status = summary.current.status as ReturnType<TLine["status"]>;
    const freshness = summary.current.freshness as ReturnType<TLine["freshness"]>;
    const diagnosticsSummary = summary.diagnostics as ReturnType<TLine["diagnosticsSummary"]>;
    const resultKind = readResourceOperationResultKind(status, mutationResponse);
    return Object.freeze({
      line,
      summary,
      status,
      freshness,
      diagnosticsSummary,
      mutationResponse,
      confirmationKind: mutationResponse?.confirmation.kind ?? null,
      resultKind,
      pending: resultKind === "pending",
      settled: resultKind !== "pending",
      message: readResourceOperationMessage(status, diagnosticsSummary),
    });
  }, [execution, summary]);
}
