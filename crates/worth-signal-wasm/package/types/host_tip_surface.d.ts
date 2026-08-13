/**
 * Host tip ingress surfaces for worker-first.
 * Paint follows tip notify; settleAuthoredWork remains the handoff drain.
 */

export interface HostTipWrite {
  readonly id: string;
  readonly value: unknown;
  readonly epochAtWrite?: number;
}

export interface HostTipCommitResult {
  readonly changedIds: readonly string[];
  readonly projectedReadableIds: readonly string[];
  readonly epochById: ReadonlyMap<string, number>;
  rollback(): void;
}

declare module "./callable_surface.js" {
  interface CallableSignals {
    /**
     * Advance host tips, project dependents, and notify observers once.
     * Does not enqueue worker apply — pair with applyCommittedTipWorkerBatch.
     */
    commitHostTipAndNotify(tipWrites: readonly HostTipWrite[]): HostTipCommitResult;
    /**
     * Queue one worker apply for tips already advanced via commitHostTipAndNotify.
     * Must not re-tip or re-notify.
     */
    applyCommittedTipWorkerBatch(tipWrites: readonly HostTipWrite[]): Promise<unknown>;
    /**
     * Notify-only dependent tip projection when ids already hold tip values.
     * Not a substitute for commitHostTipAndNotify on product worker-first scopes.
     */
    publishAuthoredTipProjection(changedIds: readonly string[]): void;
    /**
     * Diagnostic: settleAuthoredWork invocation count on this root.
     * For proofs that default awaitSettlement does not drain; not a product handoff API.
     */
    authoredSettleInvocationCount(): number;
  }

  interface ScopedSignalNamespace {
    settleAuthoredWork(): Promise<void>;
    commitHostTipAndNotify(tipWrites: readonly HostTipWrite[]): HostTipCommitResult;
    applyCommittedTipWorkerBatch(tipWrites: readonly HostTipWrite[]): Promise<unknown>;
    publishAuthoredTipProjection(changedIds: readonly string[]): void;
    authoredSettleInvocationCount(): number;
  }
}

export {};
