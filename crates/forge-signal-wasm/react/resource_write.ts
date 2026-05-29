import { useCallback, useRef, useState } from "react";

import type {
  ManagedResourceWriteExecution,
  ManagedResourceRecoveryLineLike,
  ManagedResourceWriteFeedback,
  ManagedResourceWriteFeedbackMessages,
  ManagedResourceWriteRecoveryDeclaration,
  ManagedResourceWriteRecoveryExecution,
  ManagedResourceWriteRecoveryPolicy,
  ManagedResourceWriteRecoverySurface,
  ManagedResourceWriteHookOptions,
  ManagedResourceWriteLineLike,
  ManagedResourceWriteOptions,
  ManagedResourceWriteResult,
  ManagedResourceWriteState,
} from "./model.js";

type ManagedResourceWriteSettledResult<
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> = Omit<ManagedResourceWriteResult<TLine>, "recovery">;

function readManagedWriteErrorMessage(result: ManagedResourceWriteSettledResult): string | null {
  if (result.resultKind === "rejected" && "message" in result.status) {
    return result.status.message;
  }
  const diagnosticsSummary = result.diagnosticsSummary as {
    latest?: {
      errorMessage?: string | null;
    };
  };
  return diagnosticsSummary.latest?.errorMessage ?? null;
}

function createManagedWriteFeedback<TLine extends ManagedResourceWriteLineLike>(
  result: ManagedResourceWriteSettledResult<TLine>,
  messages?: ManagedResourceWriteFeedbackMessages,
): ManagedResourceWriteFeedback<TLine> {
  if (result.resultKind === "fulfilled") {
    return Object.freeze({
      kind: "success",
      title: messages?.success ?? "Saved",
      description: messages?.successDescription ?? null,
      resultKind: "fulfilled",
      confirmationKind: result.confirmationKind,
      status: result.status,
      diagnosticsSummary: result.diagnosticsSummary,
    }) as ManagedResourceWriteFeedback<TLine>;
  }
  if (result.resultKind === "partial") {
    return Object.freeze({
      kind: "partial",
      title: messages?.partial ?? "Saved with follow-up refresh",
      description: messages?.partialDescription ?? "The latest server truth is being refreshed.",
      resultKind: "partial",
      confirmationKind: result.confirmationKind,
      status: result.status,
      diagnosticsSummary: result.diagnosticsSummary,
    }) as ManagedResourceWriteFeedback<TLine>;
  }
  if (result.resultKind === "timedOut") {
    return Object.freeze({
      kind: "error",
      title: messages?.timedOut ?? messages?.error ?? "Request timed out",
      description: messages?.timedOutDescription ?? messages?.errorDescription ?? readManagedWriteErrorMessage(result),
      resultKind: "timedOut",
      confirmationKind: null,
      status: result.status,
      diagnosticsSummary: result.diagnosticsSummary,
    }) as ManagedResourceWriteFeedback<TLine>;
  }
  return Object.freeze({
    kind: "error",
    title: messages?.error ?? "Unable to save",
    description: messages?.errorDescription ?? readManagedWriteErrorMessage(result),
    resultKind: "rejected",
    confirmationKind: null,
    status: result.status,
    diagnosticsSummary: result.diagnosticsSummary,
  }) as ManagedResourceWriteFeedback<TLine>;
}

function createManagedWriteRecoverySummary(
  result: ManagedResourceWriteSettledResult,
) {
  if (result.resultKind === "fulfilled") {
    return Object.freeze({
      severity: "none",
      reason: "exactCanonicalTruth",
      recommendedFollowup: "none",
      requiresFollowup: false,
      retryRecommended: false,
      confirmationKind: result.confirmationKind,
    } as const);
  }
  if (result.resultKind === "partial") {
    if (result.confirmationKind === "refetchRequired") {
      return Object.freeze({
        severity: "warning",
        reason: "refetchRequired",
        recommendedFollowup: "refreshResidentTruth",
        requiresFollowup: true,
        retryRecommended: false,
        confirmationKind: result.confirmationKind,
      } as const);
    }
    if (result.confirmationKind === "deliveryAwaited") {
      return Object.freeze({
        severity: "info",
        reason: "deliveryAwaited",
        recommendedFollowup: "waitForDelivery",
        requiresFollowup: true,
        retryRecommended: false,
        confirmationKind: result.confirmationKind,
      } as const);
    }
    return Object.freeze({
      severity: "info",
      reason: "partialCanonicalTruth",
      recommendedFollowup: "refreshResidentTruth",
      requiresFollowup: true,
      retryRecommended: false,
      confirmationKind: result.confirmationKind,
    } as const);
  }
  if (result.resultKind === "timedOut") {
    return Object.freeze({
      severity: "error",
      reason: "timedOut",
      recommendedFollowup: "retryWrite",
      requiresFollowup: true,
      retryRecommended: true,
      confirmationKind: null,
    } as const);
  }
  return Object.freeze({
    severity: "error",
    reason: "rejected",
    recommendedFollowup: "inspectWriteFailure",
    requiresFollowup: true,
    retryRecommended: false,
    confirmationKind: null,
  } as const);
}

function createManagedWriteExecution<TLine extends ManagedResourceWriteLineLike>(
  line: TLine,
  options: ManagedResourceWriteOptions<TLine> = {},
): ManagedResourceWriteExecution<TLine> {
  let settlementPromise: Promise<ManagedResourceWriteResult<TLine>> | null = null;
  return Object.freeze({
    line,
    settled() {
      if (settlementPromise === null) {
        settlementPromise = executeManagedResourceWrite(line, options);
      }
      return settlementPromise;
    },
    async feedback(messages?: ManagedResourceWriteFeedbackMessages) {
      const result = await this.settled();
      return messages
        ? createManagedWriteFeedback(result, messages)
        : result.feedback;
    },
    free() {
      line.free();
    },
    [Symbol.dispose]() {
      line[Symbol.dispose]();
    },
  });
}

function resolveManagedRecoveryLine<TLine extends ManagedResourceRecoveryLineLike>(
  lineOrFactory: TLine | (() => TLine),
): TLine {
  return typeof lineOrFactory === "function"
    ? (lineOrFactory as () => TLine)()
    : lineOrFactory;
}

async function executeManagedWriteRecovery<TLine extends ManagedResourceWriteLineLike>(
  result: ManagedResourceWriteSettledResult<TLine>,
  policy?: ManagedResourceWriteRecoveryPolicy,
  feedbackMessages?: ManagedResourceWriteFeedbackMessages,
): Promise<ManagedResourceWriteResult<TLine>> {
  const declarations =
    result.resultKind === "partial"
      ? policy?.partial
      : result.resultKind === "rejected"
        ? policy?.rejected
        : result.resultKind === "timedOut"
          ? policy?.timedOut
          : undefined;

  if (!declarations || declarations.length === 0) {
    return createManagedWriteResult(result, Object.freeze([]), feedbackMessages);
  }

  const recovery = declarations.map((declaration) => {
    try {
      const line = resolveManagedRecoveryLine(declaration.line);
      const status = declaration.kind === "refreshResourceLine"
        ? line.refresh()
        : line.revalidate();
      return Object.freeze({
        kind: declaration.kind,
        line,
        reason: declaration.reason ?? null,
        status,
        error: null,
      }) as ManagedResourceWriteRecoveryExecution;
    } catch (error) {
      return Object.freeze({
        kind: declaration.kind,
        line: null,
        reason: declaration.reason ?? null,
        status: null,
        error,
      }) as ManagedResourceWriteRecoveryExecution;
    }
  });

  return createManagedWriteResult(result, Object.freeze(recovery), feedbackMessages);
}

async function dispatchManagedWriteCallbacks<TLine extends ManagedResourceWriteLineLike>(
  result: ManagedResourceWriteResult<TLine>,
  options: ManagedResourceWriteOptions<TLine>,
): Promise<void> {
  await options.onFeedback?.(result.feedback);
  if (result.resultKind === "partial") {
    await options.onPartial?.(result as Extract<ManagedResourceWriteResult<TLine>, { resultKind: "partial" }>);
    await options.onFulfilled?.(result);
  } else if (result.resultKind === "fulfilled") {
    await options.onFulfilled?.(result);
  } else {
    await options.onRejected?.(result as Extract<ManagedResourceWriteResult<TLine>, { resultKind: "rejected" | "timedOut" }>);
  }
  await options.onSettled?.(result);
}

function createManagedWriteResult<TLine extends ManagedResourceWriteLineLike>(
  result: ManagedResourceWriteSettledResult<TLine>,
  executions: readonly ManagedResourceWriteRecoveryExecution[],
  feedbackMessages?: ManagedResourceWriteFeedbackMessages,
): ManagedResourceWriteResult<TLine> {
  return Object.freeze({
    ...result,
    feedback: createManagedWriteFeedback(result, feedbackMessages),
    recovery: createManagedWriteRecoverySurface(result, executions, feedbackMessages),
  }) as ManagedResourceWriteResult<TLine>;
}

function createManagedWriteRecoverySurface<TLine extends ManagedResourceWriteLineLike>(
  result: ManagedResourceWriteSettledResult<TLine>,
  executions: readonly ManagedResourceWriteRecoveryExecution[],
  feedbackMessages?: ManagedResourceWriteFeedbackMessages,
): ManagedResourceWriteRecoverySurface<TLine> {
  return Object.freeze({
    executions,
    summary() {
      return createManagedWriteRecoverySummary(result);
    },
    apply(policy?: ManagedResourceWriteRecoveryPolicy) {
      return executeManagedWriteRecovery(result, policy, feedbackMessages);
    },
  });
}

export const managedResourceWriteFeedback = Object.freeze({
  create<TLine extends ManagedResourceWriteLineLike>(
    result: ManagedResourceWriteResult<TLine>,
    messages?: ManagedResourceWriteFeedbackMessages,
  ): ManagedResourceWriteFeedback<TLine> {
    return createManagedWriteFeedback(result, messages);
  },
});

export const managedResourceWriteRecovery = Object.freeze({
  refresh<TLine extends ManagedResourceRecoveryLineLike>(
    line: TLine | (() => TLine),
    reason?: string,
  ): ManagedResourceWriteRecoveryDeclaration<TLine> {
    return Object.freeze({
      kind: "refreshResourceLine",
      line,
      reason,
    });
  },
  revalidate<TLine extends ManagedResourceRecoveryLineLike>(
    line: TLine | (() => TLine),
    reason?: string,
  ): ManagedResourceWriteRecoveryDeclaration<TLine> {
    return Object.freeze({
      kind: "revalidateResourceLine",
      line,
      reason,
    });
  },
  async apply<TLine extends ManagedResourceWriteLineLike>(
    result: ManagedResourceWriteResult<TLine>,
    policy?: ManagedResourceWriteRecoveryPolicy,
  ): Promise<ManagedResourceWriteResult<TLine>> {
    return result.recovery.apply(policy);
  },
});

export function createManagedResourceWriteExecution<TLine extends ManagedResourceWriteLineLike>(
  line: TLine,
  options: ManagedResourceWriteOptions<TLine> = {},
): ManagedResourceWriteExecution<TLine> {
  return createManagedWriteExecution(line, options);
}

export async function executeManagedResourceWrite<TLine extends ManagedResourceWriteLineLike>(
  line: TLine,
  options: ManagedResourceWriteOptions<TLine> = {},
): Promise<ManagedResourceWriteResult<TLine>> {
  const settled = await line.execute({
    freeOnSettle: options.freeOnSettle ?? true,
  }).settled({
      timeoutMs: options.timeoutMs ?? 15_000,
  }) as ManagedResourceWriteSettledResult<TLine>;
  const result = await executeManagedWriteRecovery(
    settled,
    {
      ...options.recovery,
      ...(await options.recoveryPolicy?.({
        line,
        result: settled as never,
      })),
    },
    options.feedback,
  );
  await dispatchManagedWriteCallbacks(result, options);
  return result;
}

function resolveManagedWriteLineFactory<TArgs, TLine extends ManagedResourceWriteLineLike>(
  options: ManagedResourceWriteHookOptions<TArgs, TLine>,
): (args: TArgs) => TLine {
  if ("line" in options && typeof options.line === "function") {
    return options.line;
  }
  return "createLine" in options ? options.createLine : options.line;
}

export function useManagedResourceWrite<
  TArgs,
  TLine extends ManagedResourceWriteLineLike,
>(
  options: ManagedResourceWriteHookOptions<TArgs, TLine>,
): ManagedResourceWriteState<TArgs, TLine> {
  const [pending, setPending] = useState(false);
  const [lastFeedback, setLastFeedback] = useState<ManagedResourceWriteFeedback<TLine> | null>(null);
  const [lastResult, setLastResult] = useState<ManagedResourceWriteResult<TLine> | null>(null);
  const [lastError, setLastError] = useState<unknown>(null);
  const activeExecutionCountRef = useRef(0);
  const createLine = resolveManagedWriteLineFactory(options);

  const beginPending = useCallback(() => {
    activeExecutionCountRef.current += 1;
    if (activeExecutionCountRef.current !== 1) {
      return;
    }
    setPending(true);
    options.onPendingChange?.(true);
  }, [options]);

  const endPending = useCallback(() => {
    activeExecutionCountRef.current = Math.max(0, activeExecutionCountRef.current - 1);
    if (activeExecutionCountRef.current !== 0) {
      return;
    }
    setPending(false);
    options.onPendingChange?.(false);
  }, [options]);

  const reset = useCallback(() => {
    setLastFeedback(null);
    setLastResult(null);
    setLastError(null);
  }, []);

  const execute = useCallback(async (args: TArgs) => {
    beginPending();
    setLastError(null);
    try {
      const line = createLine(args);
      const result = await createManagedWriteExecution(line, {
        ...options,
        recoveryPolicy: async ({ result }) => options.recoveryPolicy?.({
          args,
          line,
          result,
        }),
      }).settled();
      setLastFeedback(result.feedback);
      setLastResult(result);
      return result;
    } catch (error) {
      setLastError(error);
      throw error;
    } finally {
      endPending();
    }
  }, [beginPending, createLine, endPending, options]);

  return Object.freeze({
    pending,
    lastFeedback,
    lastResult,
    lastError,
    execute,
    reset,
  });
}
