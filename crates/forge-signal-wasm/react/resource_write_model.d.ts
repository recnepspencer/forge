import type {
  ResourceLineAwaitSettlementResult,
  ResourceLine,
  ResourceLineFreshness,
  ResourceLineStatus,
} from "../package/types/resource/resource_lifecycle.js";
import type { ResourceLineDiagnosticsSummary } from "../package/types/resource/resource_line_summary.js";
import type { ResourceMutationResponsePlan } from "../package/types/resource/resource_mutation_response.js";

type MaybePromise<TValue> = TValue | Promise<TValue>;

export interface ManagedResourceWriteLineLike<
  TParams = unknown,
  TValue = unknown,
> extends ResourceLine<TParams, TValue> {
  mutationResponse(): ResourceMutationResponsePlan | null;
  diagnosticsSummary(): ResourceLineDiagnosticsSummary;
}

export interface ManagedResourceRecoveryLineLike {
  refresh(): ResourceLineStatus;
  revalidate(): ResourceLineStatus;
}

export type ManagedResourceWriteRecoveryKind =
  | "refreshResourceLine"
  | "revalidateResourceLine";

interface ManagedResourceWriteRecoveryBase<
  TLine extends ManagedResourceRecoveryLineLike = ManagedResourceRecoveryLineLike,
> {
  readonly reason?: string;
  readonly line: TLine | (() => TLine);
}

export interface ManagedResourceWriteRefreshRecovery<
  TLine extends ManagedResourceRecoveryLineLike = ManagedResourceRecoveryLineLike,
> extends ManagedResourceWriteRecoveryBase<TLine> {
  readonly kind: "refreshResourceLine";
}

export interface ManagedResourceWriteRevalidateRecovery<
  TLine extends ManagedResourceRecoveryLineLike = ManagedResourceRecoveryLineLike,
> extends ManagedResourceWriteRecoveryBase<TLine> {
  readonly kind: "revalidateResourceLine";
}

export type ManagedResourceWriteRecoveryDeclaration<
  TLine extends ManagedResourceRecoveryLineLike = ManagedResourceRecoveryLineLike,
> =
  | ManagedResourceWriteRefreshRecovery<TLine>
  | ManagedResourceWriteRevalidateRecovery<TLine>;

export interface ManagedResourceWriteRecoveryExecutionSucceeded<
  TLine extends ManagedResourceRecoveryLineLike = ManagedResourceRecoveryLineLike,
> {
  readonly kind: ManagedResourceWriteRecoveryKind;
  readonly line: TLine;
  readonly reason: string | null;
  readonly status: ResourceLineStatus;
  readonly error: null;
}

export interface ManagedResourceWriteRecoveryExecutionFailed<
  TLine extends ManagedResourceRecoveryLineLike = ManagedResourceRecoveryLineLike,
> {
  readonly kind: ManagedResourceWriteRecoveryKind;
  readonly line: TLine | null;
  readonly reason: string | null;
  readonly status: null;
  readonly error: unknown;
}

export type ManagedResourceWriteRecoveryExecution<
  TLine extends ManagedResourceRecoveryLineLike = ManagedResourceRecoveryLineLike,
> =
  | ManagedResourceWriteRecoveryExecutionSucceeded<TLine>
  | ManagedResourceWriteRecoveryExecutionFailed<TLine>;

export type ManagedResourceWriteRecoverySeverity =
  | "none"
  | "info"
  | "warning"
  | "error";

export type ManagedResourceWriteRecoveryReason =
  | "exactCanonicalTruth"
  | "partialCanonicalTruth"
  | "refetchRequired"
  | "deliveryAwaited"
  | "rejected"
  | "timedOut";

export type ManagedResourceWriteRecoveryFollowup =
  | "none"
  | "refreshResidentTruth"
  | "waitForDelivery"
  | "inspectWriteFailure"
  | "retryWrite";

export interface ManagedResourceWriteRecoverySummary {
  readonly severity: ManagedResourceWriteRecoverySeverity;
  readonly reason: ManagedResourceWriteRecoveryReason;
  readonly recommendedFollowup: ManagedResourceWriteRecoveryFollowup;
  readonly requiresFollowup: boolean;
  readonly retryRecommended: boolean;
  readonly confirmationKind: ResourceMutationResponsePlan["confirmation"]["kind"] | null;
}

export interface ManagedResourceWriteRecoverySurface<
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> {
  readonly executions: readonly ManagedResourceWriteRecoveryExecution[];
  summary(): ManagedResourceWriteRecoverySummary;
  apply(
    policy?: ManagedResourceWriteRecoveryPolicy,
  ): Promise<ManagedResourceWriteResult<TLine>>;
}

export interface ManagedResourceWriteFeedbackMessages {
  readonly success?: string;
  readonly successDescription?: string;
  readonly partial?: string;
  readonly partialDescription?: string;
  readonly error?: string;
  readonly errorDescription?: string;
  readonly timedOut?: string;
  readonly timedOutDescription?: string;
}

export interface ManagedResourceWriteSuccessFeedback<
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> {
  readonly kind: "success" | "partial";
  readonly title: string;
  readonly description: string | null;
  readonly resultKind: "fulfilled" | "partial";
  readonly confirmationKind: ResourceMutationResponsePlan["confirmation"]["kind"] | null;
  readonly status: Extract<ManagedResourceWriteSuccessResult<TLine>["status"], { kind: "fulfilled" }>;
  readonly diagnosticsSummary: ResourceLineDiagnosticsSummary;
}

export interface ManagedResourceWriteFailureFeedback<
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> {
  readonly kind: "error";
  readonly title: string;
  readonly description: string | null;
  readonly resultKind: "rejected" | "timedOut";
  readonly confirmationKind: null;
  readonly status: ManagedResourceWriteFailureResult<TLine>["status"];
  readonly diagnosticsSummary: ResourceLineDiagnosticsSummary;
}

export type ManagedResourceWriteFeedback<
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> =
  | ManagedResourceWriteSuccessFeedback<TLine>
  | ManagedResourceWriteFailureFeedback<TLine>;

export interface ManagedResourceWriteExecution<
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> {
  readonly line: TLine;
  settled(): Promise<ManagedResourceWriteResult<TLine>>;
  feedback(
    messages?: ManagedResourceWriteFeedbackMessages,
  ): Promise<ManagedResourceWriteFeedback<TLine>>;
  free(): void;
  [Symbol.dispose](): void;
}

export interface ManagedResourceWriteRecoveryPolicy<
  TLine extends ManagedResourceRecoveryLineLike = ManagedResourceRecoveryLineLike,
> {
  readonly partial?: readonly ManagedResourceWriteRecoveryDeclaration<TLine>[];
  readonly rejected?: readonly ManagedResourceWriteRecoveryDeclaration<TLine>[];
  readonly timedOut?: readonly ManagedResourceWriteRecoveryDeclaration<TLine>[];
}

export interface ManagedResourceWriteSuccessResult<
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> {
  readonly resultKind: "fulfilled" | "partial";
  readonly status: Extract<ResourceLineStatus, { kind: "fulfilled" }>;
  readonly summary: ReturnType<TLine["summary"]>;
  readonly freshness: ResourceLineFreshness;
  readonly diagnosticsSummary: ResourceLineDiagnosticsSummary;
  readonly mutationResponse: ResourceMutationResponsePlan | null;
  readonly confirmationKind: ResourceMutationResponsePlan["confirmation"]["kind"] | null;
  readonly value: ReturnType<TLine["value"]>;
  readonly feedback: ManagedResourceWriteFeedback<TLine>;
  readonly recovery: ManagedResourceWriteRecoverySurface<TLine>;
}

export interface ManagedResourceWriteFailureResult<
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> {
  readonly resultKind: "rejected" | "timedOut";
  readonly status: Extract<ResourceLineStatus, { kind: "rejected" | "timedOut" }>;
  readonly summary: ReturnType<TLine["summary"]>;
  readonly freshness: ResourceLineFreshness;
  readonly diagnosticsSummary: ResourceLineDiagnosticsSummary;
  readonly mutationResponse: ResourceMutationResponsePlan | null;
  readonly confirmationKind: null;
  readonly feedback: ManagedResourceWriteFeedback<TLine>;
  readonly recovery: ManagedResourceWriteRecoverySurface<TLine>;
}

export type ManagedResourceWriteResult<
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> =
  | ManagedResourceWriteSuccessResult<TLine>
  | ManagedResourceWriteFailureResult<TLine>;

export interface ManagedResourceWriteOptions<
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> {
  timeoutMs?: number;
  freeOnSettle?: boolean;
  feedback?: ManagedResourceWriteFeedbackMessages;
  recovery?: ManagedResourceWriteRecoveryPolicy;
  recoveryPolicy?(
    context: {
      readonly line: TLine;
      readonly result: ResourceLineAwaitSettlementResult<unknown, ReturnType<TLine["value"]>>;
    },
  ): MaybePromise<ManagedResourceWriteRecoveryPolicy | undefined>;
  onPendingChange?(pending: boolean): MaybePromise<void>;
  onFeedback?(feedback: ManagedResourceWriteFeedback<TLine>): MaybePromise<void>;
  onFulfilled?(result: ManagedResourceWriteSuccessResult<TLine>): MaybePromise<void>;
  onPartial?(result: Extract<ManagedResourceWriteResult<TLine>, { resultKind: "partial" }>): MaybePromise<void>;
  onRejected?(result: ManagedResourceWriteFailureResult<TLine>): MaybePromise<void>;
  onSettled?(result: ManagedResourceWriteResult<TLine>): MaybePromise<void>;
}

export type ManagedResourceWriteHookOptions<
  TArgs,
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> =
  | ({
      line(args: TArgs): TLine;
    } & Omit<ManagedResourceWriteOptions<TLine>, "recoveryPolicy"> & {
      recoveryPolicy?(
        context: {
          readonly args: TArgs;
          readonly line: TLine;
          readonly result: ResourceLineAwaitSettlementResult<unknown, ReturnType<TLine["value"]>>;
        },
      ): MaybePromise<ManagedResourceWriteRecoveryPolicy | undefined>;
    })
  | ({
      createLine(args: TArgs): TLine;
    } & Omit<ManagedResourceWriteOptions<TLine>, "recoveryPolicy"> & {
      recoveryPolicy?(
        context: {
          readonly args: TArgs;
          readonly line: TLine;
          readonly result: ResourceLineAwaitSettlementResult<unknown, ReturnType<TLine["value"]>>;
        },
      ): MaybePromise<ManagedResourceWriteRecoveryPolicy | undefined>;
    });

export interface ManagedResourceWriteState<
  TArgs,
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> {
  readonly pending: boolean;
  readonly lastFeedback: ManagedResourceWriteFeedback<TLine> | null;
  readonly lastResult: ManagedResourceWriteResult<TLine> | null;
  readonly lastError: unknown;
  execute(args: TArgs): Promise<ManagedResourceWriteResult<TLine>>;
  reset(): void;
}
