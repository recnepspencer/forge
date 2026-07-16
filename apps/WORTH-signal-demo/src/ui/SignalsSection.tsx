import React from "react";
import { createSignals } from "worth-signals-wasm";
import { useSignal } from "./Demos";
import { SignalsCodeSample } from "./SignalsSectionCodeSample";
import "./signalsSection.css";

interface SignalsSectionProps {
  onNavigate: (path: string) => void;
}

type SignalsRuntime = Awaited<ReturnType<typeof createSignals>>;

interface ReadableSignal<T> {
  (): T;
  id: string;
  free: () => void;
}

interface WritableSignal<T> extends ReadableSignal<T> {
  set: (value: T) => unknown;
}

interface WhySummary {
  id: string;
  apiFamily?: string | null;
  state: string;
  upstream: string[];
  outputChange?: string | null;
  propagationSuppressed?: boolean;
  callback?: { currentReads?: string[] } | null;
}

interface DiagnosticsSurface {
  why: (id: string) => WhySummary;
  latestFlow: () => unknown;
  free: () => void;
}

interface TransferGraph {
  signals: SignalsRuntime;
  diagnostics: DiagnosticsSurface;
  requestedAmount: WritableSignal<number>;
  processingFee: ReadableSignal<number>;
  reviewLane: ReadableSignal<string>;
  friendlyNames: Record<string, string>;
}

type NodeKey = "amount" | "fee" | "lane";

interface AuditEntry {
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

const REVIEW_THRESHOLD = 10_000;
const PROCESSING_RATE = 0.004;
const INITIAL_AMOUNT = 8_000;
const AMOUNT_MAX = 25_000;
const VISIBLE_AUDIT_ROWS = 7;

const PRESET_SCENARIOS = [
  { amount: 2_400, label: "Vendor invoice" },
  { amount: 9_800, label: "Payroll batch" },
  { amount: 14_500, label: "Wire transfer" },
] as const;

const currency = new Intl.NumberFormat("en-US", {
  currency: "USD",
  maximumFractionDigits: 2,
  style: "currency",
});

const wholeCurrency = new Intl.NumberFormat("en-US", {
  currency: "USD",
  maximumFractionDigits: 0,
  style: "currency",
});

function createTransferGraph(signals: SignalsRuntime): TransferGraph {
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

  const diagnostics = (signals as unknown as { diagnostics: () => DiagnosticsSurface }).diagnostics();

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

function disposeTransferGraph(graph: TransferGraph): void {
  graph.diagnostics.free();
  graph.reviewLane.free();
  graph.processingFee.free();
  graph.requestedAmount.free();
}

function safeWhy(graph: TransferGraph, id: string): WhySummary | null {
  try {
    return graph.diagnostics.why(id);
  } catch {
    return null;
  }
}

function safeLatestFlow(graph: TransferGraph): unknown {
  try {
    return graph.diagnostics.latestFlow();
  } catch {
    return null;
  }
}

function readFlowStats(flow: unknown): { recomputed: number | null; stages: number | null } {
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

function parseUpstreamVersions(upstream: string[]): { cached: string; current: string } | null {
  for (const cause of upstream) {
    const versions = /cached_version:\s*(\d+),\s*current_version:\s*(\d+)/.exec(cause);
    if (versions) return { cached: versions[1], current: versions[2] };
  }
  return null;
}

function buildInitialEntry(graph: TransferGraph): AuditEntry {
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

function downloadDecisionTrail(graph: TransferGraph, entries: AuditEntry[]): void {
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
    policy: {
      processingRate: PROCESSING_RATE,
      reviewThreshold: REVIEW_THRESHOLD,
    },
    replay,
    source: "signals.diagnostics() + signals.history(), read from the Worth runtime",
  };

  const blob = new Blob([JSON.stringify(artifact, null, 2)], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = "worth-decision-trail.json";
  anchor.click();
  URL.revokeObjectURL(url);
}

function AmountCard({
  committedAmount,
  onCommit,
  onStage,
  selected,
  onSelect,
  stagedAmount,
}: {
  committedAmount: number;
  onCommit: (value: number) => void;
  onStage: (value: number) => void;
  selected: boolean;
  onSelect: () => void;
  stagedAmount: number;
}): React.ReactElement {
  const hasDraft = stagedAmount !== committedAmount;
  const fillPercent = Math.max(0, Math.min(100, (stagedAmount / AMOUNT_MAX) * 100));
  const thresholdPercent = (REVIEW_THRESHOLD / AMOUNT_MAX) * 100;

  return (
    <article
      className={`signals-card signals-amount-card${selected ? " is-selected" : ""}`}
      onClick={onSelect}
    >
      <header className="signals-card-head">
        <span>Requested amount</span>
        <code>input</code>
      </header>
      <div className="signals-currency-input">
        <b aria-hidden="true">$</b>
        <input
          aria-describedby="transfer-policy-threshold"
          aria-label="Requested amount in dollars"
          max={AMOUNT_MAX}
          min="0"
          onBlur={(event) => onCommit(event.currentTarget.valueAsNumber)}
          onChange={(event) => onStage(event.currentTarget.valueAsNumber)}
          onClick={(event) => event.stopPropagation()}
          onKeyDown={(event) => {
            if (event.key === "Enter") onCommit(event.currentTarget.valueAsNumber);
          }}
          step="100"
          type="number"
          value={Number.isFinite(stagedAmount) ? stagedAmount : ""}
        />
      </div>
      <div className="signals-slider-zone">
        <div className="signals-slider-track-wrap">
          <input
            aria-label="Requested amount slider"
            className="signals-slider"
            max={AMOUNT_MAX}
            min="0"
            onChange={(event) => onStage(event.currentTarget.valueAsNumber)}
            onClick={(event) => event.stopPropagation()}
            onKeyUp={(event) => onCommit(event.currentTarget.valueAsNumber)}
            onPointerUp={(event) => onCommit(event.currentTarget.valueAsNumber)}
            step="100"
            style={{ "--fill": `${fillPercent}%` } as React.CSSProperties}
            type="range"
            value={stagedAmount}
          />
          <span
            aria-hidden="true"
            className="signals-threshold-tick"
            style={{ left: `${thresholdPercent}%` }}
          >
            <i />
            <em>{wholeCurrency.format(REVIEW_THRESHOLD)}</em>
          </span>
        </div>
        <small id="transfer-policy-threshold">
          {hasDraft
            ? `Draft ${wholeCurrency.format(stagedAmount)} â€” release to commit the transaction`
            : `Transfers at or above ${wholeCurrency.format(REVIEW_THRESHOLD)} route to manual review.`}
        </small>
      </div>
      <div className="signals-preset-row" role="group" aria-label="Preset transfers">
        {PRESET_SCENARIOS.map((preset) => (
          <button
            key={preset.label}
            onClick={(event) => {
              event.stopPropagation();
              onCommit(preset.amount);
            }}
            type="button"
          >
            {preset.label}
            <b>{wholeCurrency.format(preset.amount)}</b>
          </button>
        ))}
      </div>
    </article>
  );
}

function DecisionCard({
  caption,
  className,
  label,
  onSelect,
  selected,
  value,
}: {
  caption: string;
  className?: string;
  label: string;
  onSelect: () => void;
  selected: boolean;
  value: string;
}): React.ReactElement {
  return (
    <article
      className={`signals-card signals-decision-card${selected ? " is-selected" : ""}${className ? ` ${className}` : ""}`}
      onClick={onSelect}
    >
      <header className="signals-card-head">
        <span>{label}</span>
        <code>computed</code>
      </header>
      <strong aria-live="polite">{value}</strong>
      <small>{caption}</small>
      <footer className="signals-card-why-hint">why did this change? â†’</footer>
    </article>
  );
}

function AuditRow({ entry }: { entry: AuditEntry }): React.ReactElement {
  // the runtime reports "Refreshed" when a recompute produced a new output
  const laneChanged = entry.laneOutcome === "Refreshed";

  return (
    <li className="signals-audit-row">
      <span className="signals-audit-tx">{entry.kind === "created" ? "init" : `tx ${String(entry.revision).padStart(2, "0")}`}</span>
      <div className="signals-audit-body">
        {entry.kind === "created" ? (
          <p className="signals-audit-main">
            graph created â€” amount {wholeCurrency.format(entry.amountTo)} Â· fee {currency.format(entry.feeTo)} Â· reviewLane {entry.laneTo}
          </p>
        ) : (
          <>
            <p className="signals-audit-main">
              amount {wholeCurrency.format(entry.amountFrom)} â†’ <strong>{wholeCurrency.format(entry.amountTo)}</strong>
            </p>
            <p className="signals-audit-chips">
              <span className="signals-chip signals-chip-fee">
                fee recomputed {currency.format(entry.feeFrom)} â†’ {currency.format(entry.feeTo)}
              </span>
              {laneChanged ? (
                <span className="signals-chip signals-chip-flipped">
                  reviewLane changed {entry.laneFrom} â†’ {entry.laneTo}
                </span>
              ) : (
                <span className="signals-chip signals-chip-unchanged">
                  reviewLane recomputed â€” output unchanged ({entry.laneTo})
                </span>
              )}
            </p>
          </>
        )}
        <details className="signals-audit-payload">
          <summary>runtime payload</summary>
          <pre>{JSON.stringify(entry.payload, null, 2)}</pre>
        </details>
      </div>
      <span className="signals-audit-meta">
        {entry.recomputedCount !== null
          ? `${entry.recomputedCount} recomputed${entry.stageCount !== null ? ` Â· ${entry.stageCount} stages` : ""}`
          : "initial"}
      </span>
    </li>
  );
}

function WhyPanel({
  graph,
  revision,
  selected,
}: {
  graph: TransferGraph;
  revision: number;
  selected: NodeKey;
}): React.ReactElement {
  const handle =
    selected === "amount" ? graph.requestedAmount :
    selected === "fee" ? graph.processingFee :
    graph.reviewLane;
  const friendlyName = graph.friendlyNames[handle.id] ?? selected;

  const why = React.useMemo(
    () => safeWhy(graph, handle.id),
    // revision retriggers the explanation after each committed transaction
    [graph, handle, revision],
  );

  const versions = why ? parseUpstreamVersions(why.upstream ?? []) : null;
  const reads = why?.callback?.currentReads?.map((id) => graph.friendlyNames[id] ?? id) ?? [];

  return (
    <aside className="signals-why-panel" aria-label={`Runtime explanation for ${friendlyName}`}>
      <header className="signals-panel-head">
        <h3>Why: {friendlyName}</h3>
        <code>{`diagnostics().why(${friendlyName}.id)`}</code>
      </header>
      {why ? (
        <dl className="signals-why-grid">
          <div>
            <dt>node</dt>
            <dd><code className="signals-why-id">{why.id}</code></dd>
          </div>
          <div>
            <dt>family</dt>
            <dd>{why.apiFamily ?? "unknown"}</dd>
          </div>
          <div>
            <dt>reads</dt>
            <dd>{reads.length > 0 ? reads.join(", ") : "none â€” source input"}</dd>
          </div>
          <div>
            <dt>state</dt>
            <dd>{why.state}</dd>
          </div>
          <div>
            <dt>last outcome</dt>
            <dd className={why.outputChange === "Refreshed" ? "is-changed" : ""}>
              {why.outputChange === "Refreshed"
                ? "recomputed Â· output changed"
                : why.outputChange === "Unchanged"
                  ? "recomputed Â· output unchanged"
                  : why.outputChange
                    ? `recomputed Â· ${why.outputChange.toLowerCase()}`
                    : "no recompute recorded"}
            </dd>
          </div>
          {versions ? (
            <div>
              <dt>dependency versions</dt>
              <dd>
                cached v{versions.cached} Â· current v{versions.current}
                {versions.cached === versions.current ? " â€” in sync" : " â€” stale"}
              </dd>
            </div>
          ) : null}
        </dl>
      ) : (
        <p className="signals-why-empty">The runtime has no explanation for this node yet.</p>
      )}
      {why ? (
        <details className="signals-audit-payload">
          <summary>raw why() payload</summary>
          <pre>{JSON.stringify(why, null, 2)}</pre>
        </details>
      ) : null}
    </aside>
  );
}

function TransferWorkbench({ graph }: { graph: TransferGraph }): React.ReactElement {
  const amount = useSignal<number>(graph.signals, graph.requestedAmount);
  const processingFee = useSignal<number>(graph.signals, graph.processingFee);
  const reviewLane = useSignal<string>(graph.signals, graph.reviewLane);

  const [stagedAmount, setStagedAmount] = React.useState<number>(INITIAL_AMOUNT);
  const initialEntriesRef = React.useRef<AuditEntry[] | null>(null);
  initialEntriesRef.current ??= [buildInitialEntry(graph)];
  const [entries, setEntries] = React.useState<AuditEntry[]>(initialEntriesRef.current);
  const [selectedNode, setSelectedNode] = React.useState<NodeKey>("lane");

  const commitAmount = (nextValue: number): void => {
    if (!Number.isFinite(nextValue)) {
      setStagedAmount(amount);
      return;
    }
    const nextAmount = Math.max(0, Math.min(AMOUNT_MAX, Math.round(nextValue)));
    setStagedAmount(nextAmount);
    if (nextAmount === amount) return;

    const before = {
      amount: graph.requestedAmount(),
      fee: graph.processingFee(),
      lane: graph.reviewLane(),
    };

    (graph.signals as {
      transaction: (callback: (tx: { set: (input: unknown, value: unknown) => void }) => void) => void;
    }).transaction((tx) => {
      tx.set(graph.requestedAmount, nextAmount);
    });

    const feeAfter = graph.processingFee();
    const laneAfter = graph.reviewLane();
    const latestFlow = safeLatestFlow(graph);
    const whyFee = safeWhy(graph, graph.processingFee.id);
    const whyReviewLane = safeWhy(graph, graph.reviewLane.id);
    const flowStats = readFlowStats(latestFlow);

    setEntries((current) => [
      {
        revision: current.length + 1,
        kind: "commit",
        recordedAt: new Date().toISOString(),
        amountFrom: before.amount,
        amountTo: nextAmount,
        feeFrom: before.fee,
        feeTo: feeAfter,
        laneFrom: before.lane,
        laneTo: laneAfter,
        feeOutcome: whyFee?.outputChange ?? null,
        laneOutcome: whyReviewLane?.outputChange ?? null,
        recomputedCount: flowStats.recomputed,
        stageCount: flowStats.stages,
        payload: { latestFlow, whyFee, whyReviewLane },
      },
      ...current,
    ]);
  };

  const manualReview = reviewLane === "Manual review";
  const latestEntry = entries[0];
  const liveWhyLine = latestEntry && latestEntry.kind === "commit"
    ? `// â†’ { state: "Clean", outputChange: "${latestEntry.laneOutcome ?? "Unchanged"}", reads: ["amount"] }`
    : `// â†’ { state: "Clean", outputChange: â€”, reads: ["amount"] }`;

  return (
    <>
      <section className="signals-decision-grid" aria-label="Transfer decision">
        <AmountCard
          committedAmount={amount}
          onCommit={commitAmount}
          onSelect={() => setSelectedNode("amount")}
          onStage={setStagedAmount}
          selected={selectedNode === "amount"}
          stagedAmount={stagedAmount}
        />
        <DecisionCard
          caption="0.4% of the requested amount"
          label="Processing fee"
          onSelect={() => setSelectedNode("fee")}
          selected={selectedNode === "fee"}
          value={currency.format(processingFee)}
        />
        <DecisionCard
          caption={manualReview ? "Reviewer approval required" : "No manual review required"}
          className={manualReview ? "signals-lane-review" : "signals-lane-auto"}
          label="Review lane"
          onSelect={() => setSelectedNode("lane")}
          selected={selectedNode === "lane"}
          value={reviewLane}
        />
      </section>

      <section className="signals-evidence-grid">
        <section className="signals-audit-panel" aria-label="Audit trail">
          <header className="signals-panel-head">
            <h3>Audit trail</h3>
            <code>diagnostics().latestFlow()</code>
            <button
              className="signals-export-button"
              onClick={() => downloadDecisionTrail(graph, entries)}
              type="button"
            >
              Export decision trail (JSON)
            </button>
          </header>
          <ul className="signals-audit-list">
            {entries.slice(0, VISIBLE_AUDIT_ROWS).map((entry) => (
              <AuditRow entry={entry} key={entry.revision} />
            ))}
          </ul>
          {entries.length > VISIBLE_AUDIT_ROWS ? (
            <p className="signals-audit-more">
              Showing the latest {VISIBLE_AUDIT_ROWS} of {entries.length} records â€” the export carries all of them.
            </p>
          ) : null}
          <p className="signals-audit-footnote">
            Every row is read back from the runtime after the transaction commits. Nothing here is kept by the UI.
          </p>
        </section>

        <WhyPanel graph={graph} revision={entries.length} selected={selectedNode} />
      </section>

      <section className="signals-code-section" aria-labelledby="signals-code-title">
        <h2 id="signals-code-title">The whole graph, and the question you get to ask it</h2>
        <SignalsCodeSample liveWhyLine={liveWhyLine} />
      </section>
    </>
  );
}

export function SignalsSection({ onNavigate }: SignalsSectionProps): React.ReactElement {
  const [graph, setGraph] = React.useState<TransferGraph | null>(null);
  const [bootError, setBootError] = React.useState<string | null>(null);

  React.useEffect(() => {
    let active = true;
    let createdGraph: TransferGraph | null = null;

    createSignals({ deployment: "mainThreadCompatibility" })
      .then((signals) => {
        createdGraph = createTransferGraph(signals);
        if (!active) {
          disposeTransferGraph(createdGraph);
          return;
        }
        setGraph(createdGraph);
      })
      .catch((error: unknown) => {
        if (active) setBootError(error instanceof Error ? error.message : "Could not start the Worth runtime.");
      });

    return () => {
      active = false;
      if (createdGraph) disposeTransferGraph(createdGraph);
    };
  }, []);

  return (
    <div className="accent-signals signals-section">
      {bootError ? <div className="signals-runtime-message">{bootError}</div> : null}
      {!graph && !bootError ? <div className="signals-runtime-message">Connecting to the Worth runtimeâ€¦</div> : null}
      {graph ? <TransferWorkbench graph={graph} /> : null}

      <div className="signals-docs-row">
        <button onClick={() => onNavigate("#/docs/learn/feature-index")} type="button">
          Explore signals in the documentation <span aria-hidden="true">â†’</span>
        </button>
      </div>
    </div>
  );
}
