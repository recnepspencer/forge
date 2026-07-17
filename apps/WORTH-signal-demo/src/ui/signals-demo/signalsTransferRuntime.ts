import { createSignals } from "worth-signals-wasm";

type SignalsRuntime = Awaited<ReturnType<typeof createSignals>>;

interface ReadableSignal<T> {
  (): T;
  id: string;
  free: () => void;
  get: () => T;
}

interface WritableSignal<T> extends ReadableSignal<T> {
  set: (value: T) => unknown;
}

export interface WhySummary {
  id: string;
  apiFamily?: string | null;
  state: string;
  upstream: readonly string[];
  outputChange?: string | null;
  propagationSuppressed?: boolean;
  callback?: { currentReads?: readonly string[] } | null;
}

interface DiagnosticsSurface {
  why: (id: string) => WhySummary;
  latestFlow: () => unknown;
  free: () => void;
}

export interface TransferGraph {
  signals: SignalsRuntime;
  diagnostics: DiagnosticsSurface;
  requestedAmount: WritableSignal<number>;
  processingFee: ReadableSignal<number>;
  reviewLane: ReadableSignal<string>;
  friendlyNames: Record<string, string>;
}

export type NodeKey = "amount" | "fee" | "lane";

export interface AuditEntry {
  revision: number;
  kind: "created" | "commit";
  recordedAt: string;
  amountFrom: number;
  amountTo: number;
  feeFrom: number;
  feeTo: number;
  laneFrom: string;
  laneTo: string;
  feeOutcome: string | null;
  laneOutcome: string | null;
  recomputedCount: number | null;
  stageCount: number | null;
  payload: unknown;
}

export const REVIEW_THRESHOLD = 10_000;
const PROCESSING_RATE = 0.004;
export const INITIAL_AMOUNT = 8_000;
export const AMOUNT_MAX = 25_000;
export const VISIBLE_AUDIT_ROWS = 7;

export const PRESET_SCENARIOS = [
  { amount: 2_400, label: "Vendor invoice" },
  { amount: 9_800, label: "Payroll batch" },
  { amount: 14_500, label: "Wire transfer" },
] as const;

export const currency = new Intl.NumberFormat("en-US", {
  currency: "USD",
  maximumFractionDigits: 2,
  style: "currency",
});

export const wholeCurrency = new Intl.NumberFormat("en-US", {
  currency: "USD",
  maximumFractionDigits: 0,
  style: "currency",
});

export function createTransferGraph(
  signals: SignalsRuntime,
  diagnostics: DiagnosticsSurface,
): TransferGraph {
  const requestedAmount = signals.input(INITIAL_AMOUNT, {
    debugName: "transfer.requestedAmount",
  }) as unknown as WritableSignal<number>;
  const processingFee = signals.computed(
    () => Math.round(requestedAmount() * PROCESSING_RATE * 100) / 100,
    { debugName: "transfer.processingFee" },
  ) as unknown as ReadableSignal<number>;
  const reviewLane = signals.computed(
    () => requestedAmount() >= REVIEW_THRESHOLD ? "Manual review" : "Automatic",
    { debugName: "transfer.reviewLane" },
  ) as unknown as ReadableSignal<string>;

  processingFee();
  reviewLane();

  return {
    signals,
    diagnostics,
    requestedAmount,
    processingFee,
    reviewLane,
    friendlyNames: {
      [requestedAmount.id]: "amount",
      [processingFee.id]: "fee",
      [reviewLane.id]: "reviewLane",
    },
  };
}

export function disposeTransferGraph(graph: TransferGraph): void {
  graph.reviewLane.free();
  graph.processingFee.free();
  graph.requestedAmount.free();
}

export function safeWhy(graph: TransferGraph, id: string): WhySummary | null {
  try {
    return graph.diagnostics.why(id);
  } catch {
    return null;
  }
}

export function safeLatestFlow(graph: TransferGraph): unknown {
  try {
    return graph.diagnostics.latestFlow();
  } catch {
    return null;
  }
}

export function readFlowStats(flow: unknown): { recomputed: number | null; stages: number | null } {
  const report = (flow as {
    flow?: {
      apply?: {
        report?: {
          stage_count?: number;
          task_outcome_counts?: Record<string, number>;
        };
      };
    };
  })?.flow?.apply?.report;
  if (!report) return { recomputed: null, stages: null };
  return {
    recomputed: report.task_outcome_counts?.Recomputed ?? 0,
    stages: report.stage_count ?? null,
  };
}

export function parseUpstreamVersions(upstream: readonly string[]) {
  for (const cause of upstream) {
    const versions = /cached_version:\s*(\d+),\s*current_version:\s*(\d+)/.exec(cause);
    if (versions) return { cached: versions[1], current: versions[2] };
  }
  return null;
}

export function buildInitialEntry(graph: TransferGraph): AuditEntry {
  return {
    revision: 1,
    kind: "created",
    recordedAt: new Date().toISOString(),
    amountFrom: INITIAL_AMOUNT,
    amountTo: INITIAL_AMOUNT,
    feeFrom: graph.processingFee(),
    feeTo: graph.processingFee(),
    laneFrom: graph.reviewLane(),
    laneTo: graph.reviewLane(),
    feeOutcome: null,
    laneOutcome: null,
    recomputedCount: null,
    stageCount: null,
    payload: {
      latestFlow: safeLatestFlow(graph),
      whyFee: safeWhy(graph, graph.processingFee.id),
      whyReviewLane: safeWhy(graph, graph.reviewLane.id),
    },
  };
}

export function downloadDecisionTrail(graph: TransferGraph, entries: AuditEntry[]): void {
  let replay: unknown = null;
  try {
    const history = (graph.signals as {
      history: () => { replay_for: (id: string) => unknown };
    }).history();
    replay = {
      amount: history.replay_for(graph.requestedAmount.id),
      fee: history.replay_for(graph.processingFee.id),
      reviewLane: history.replay_for(graph.reviewLane.id),
    };
  } catch {
    replay = "history replay unavailable in this deployment";
  }

  const artifact = {
    decisionTrail: entries,
    exportedAt: new Date().toISOString(),
    policy: { processingRate: PROCESSING_RATE, reviewThreshold: REVIEW_THRESHOLD },
    replay,
    source: {
      capturedEvidence: "UI-owned list of runtime diagnostics snapshots",
      replay: "signals.history() runtime replay when available",
    },
  };
  const blob = new Blob([JSON.stringify(artifact, null, 2)], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = "worth-decision-trail.json";
  anchor.click();
  URL.revokeObjectURL(url);
}
