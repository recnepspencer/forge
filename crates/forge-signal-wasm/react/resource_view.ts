import { useMemo } from "react";

import { useOptionalResourceLine } from "./hooks.js";

import type {
  ReactSignalsStore,
  ResourceLineReactLike,
  ResourceViewResult,
} from "./model.js";

function resolveResourceViewMessage(
  status: ReturnType<ResourceLineReactLike["status"]>,
  diagnosticsSummary: ReturnType<ResourceLineReactLike["diagnosticsSummary"]>,
  errorMessage?: string,
): string | null {
  if (errorMessage) {
    return errorMessage;
  }
  if (status.kind === "rejected") {
    return status.message ?? diagnosticsSummary.latest.errorMessage ?? null;
  }
  if (status.kind === "timedOut") {
    return diagnosticsSummary.latest.errorMessage ?? "Resource request timed out.";
  }
  return null;
}

export function useResourceView<
  TValue = unknown,
  TInactive = undefined,
  TLine extends ResourceLineReactLike<TValue> = ResourceLineReactLike<TValue>,
>(
  line: TLine | null | undefined,
  store: ReactSignalsStore,
  options?: {
    inactiveValue?: TInactive;
    emptyWhen?(value: TValue): boolean;
    errorMessage?: string;
  },
): ResourceViewResult<TLine, TValue, TInactive> {
  const resourceLine = useOptionalResourceLine<TValue, TInactive, unknown, TLine>(
    line,
    store,
    options,
  );

  return useMemo(() => {
    if (resourceLine.kind === "inactive") {
      return Object.freeze({
        kind: "inactive",
        reason: "authorInactive",
        contentState: null,
        line: null,
        value: resourceLine.value,
        summary: null,
        status: null,
        freshness: null,
        diagnosticsSummary: null,
        message: null,
        hasVisibleValue: false,
        isRefreshing: false,
        isEmpty: false,
      }) as ResourceViewResult<TLine, TValue, TInactive>;
    }

    const { diagnosticsSummary, status, value, summary, freshness } = resourceLine;
    const hasVisibleValue = diagnosticsSummary.current.hasVisibleValue;
    const isEmpty = hasVisibleValue && (options?.emptyWhen?.(value) ?? false);
    const isRefreshing = status.kind === "pending" && hasVisibleValue;

    let contentState: "loading" | "refreshing" | "ready" | "empty" | "error";
    if (status.kind === "pending" && !hasVisibleValue) {
      contentState = "loading";
    } else if (isRefreshing) {
      contentState = "refreshing";
    } else if (status.kind === "rejected" || status.kind === "timedOut") {
      contentState = "error";
    } else if (isEmpty) {
      contentState = "empty";
    } else {
      contentState = "ready";
    }

    return Object.freeze({
      kind: "active",
      contentState,
      line: resourceLine.line as TLine,
      value,
      summary,
      status,
      freshness,
      diagnosticsSummary,
      message: resolveResourceViewMessage(
        status,
        diagnosticsSummary,
        options?.errorMessage,
      ),
      hasVisibleValue,
      isRefreshing,
      isEmpty,
    }) as ResourceViewResult<TLine, TValue, TInactive>;
  }, [options?.emptyWhen, options?.errorMessage, resourceLine]);
}
