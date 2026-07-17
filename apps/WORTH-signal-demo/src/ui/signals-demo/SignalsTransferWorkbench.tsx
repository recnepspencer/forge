import React from "react";
import { createReactSignalsStore, useSignalValue } from "worth-signals-wasm/react";

import { SignalsCodeSample } from "../SignalsSectionCodeSample";
import { AmountCard, AuditRow, DecisionCard } from "./SignalsTransferCards";
import {
  AMOUNT_MAX,
  type AuditEntry,
  buildInitialEntry,
  currency,
  downloadDecisionTrail,
  INITIAL_AMOUNT,
  type NodeKey,
  readFlowStats,
  safeLatestFlow,
  safeWhy,
  type TransferGraph,
  VISIBLE_AUDIT_ROWS,
} from "./signalsTransferRuntime";
import { SignalsWhyPanel } from "./SignalsWhyPanel";

interface SignalsTransferWorkbenchProps {
  graph: TransferGraph;
  store: ReturnType<typeof createReactSignalsStore>;
}

export function SignalsTransferWorkbench({ graph, store }: SignalsTransferWorkbenchProps): React.ReactElement {
  const amount = useSignalValue<number>(graph.requestedAmount, store);
  const processingFee = useSignalValue<number>(graph.processingFee, store);
  const reviewLane = useSignalValue<string>(graph.reviewLane, store);

  const [stagedAmount, setStagedAmount] = React.useState<number>(INITIAL_AMOUNT);
  const initialEntriesRef = React.useRef<AuditEntry[] | null>(null);
  const explanationPanelRef = React.useRef<HTMLElement | null>(null);
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
    }).transaction((tx) => tx.set(graph.requestedAmount, nextAmount));

    const feeAfter = graph.processingFee();
    const laneAfter = graph.reviewLane();
    const latestFlow = safeLatestFlow(graph);
    const whyFee = safeWhy(graph, graph.processingFee.id);
    const whyReviewLane = safeWhy(graph, graph.reviewLane.id);
    const flowStats = readFlowStats(latestFlow);

    setEntries((current) => [{
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
    }, ...current]);
  };

  const showRuntimeExplanation = (node: NodeKey): void => {
    setSelectedNode(node);
    window.requestAnimationFrame(() => {
      const panel = explanationPanelRef.current;
      if (!panel) return;
      const reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
      panel.scrollIntoView({ behavior: reduceMotion ? "auto" : "smooth", block: "center" });
      panel.focus({ preventScroll: true });
    });
  };

  const manualReview = reviewLane === "Manual review";
  const latestEntry = entries[0];
  const liveWhyLine = latestEntry && latestEntry.kind === "commit"
    ? `// → { state: "Clean", outputChange: "${latestEntry.laneOutcome ?? "Unchanged"}", reads: ["amount"] }`
    : "// → { state: \"Clean\", outputChange: —, reads: [\"amount\"] }";

  return (
    <>
      <header className="signals-demo-part-intro">
        <span>Runtime diagnostics no other state system exposes</span>
        <h2>Change one input. Inspect the exact causal record.</h2>
        <p>
          Every commit produces runtime-native evidence for what was touched, what recomputed, which
          dependencies were read, and whether each output actually changed. Choose <strong>Payroll batch</strong>
          to change the fee without changing the review decision. Then choose <strong>Wire transfer</strong>
          to cross $10,000 and flip the lane to Manual review. Select any value and ask why. No other
          state management system can answer with evidence from the same runtime that made the decision.
        </p>
      </header>

      <section className="signals-decision-grid" aria-label="Transfer decision">
        <AmountCard
          committedAmount={amount}
          onCommit={commitAmount}
          onSelect={() => showRuntimeExplanation("amount")}
          onStage={setStagedAmount}
          selected={selectedNode === "amount"}
          stagedAmount={stagedAmount}
        />
        <DecisionCard
          caption="0.4% of the requested amount"
          label="Processing fee"
          onSelect={() => showRuntimeExplanation("fee")}
          selected={selectedNode === "fee"}
          value={currency.format(processingFee)}
        />
        <DecisionCard
          caption={manualReview ? "Reviewer approval required" : "No manual review required"}
          className={manualReview ? "signals-lane-review" : "signals-lane-auto"}
          label="Review lane"
          onSelect={() => showRuntimeExplanation("lane")}
          selected={selectedNode === "lane"}
          value={reviewLane}
        />
      </section>

      <section className="signals-evidence-grid">
        <section className="signals-audit-panel" aria-label="Captured runtime evidence">
          <header className="signals-panel-head">
            <h3>Runtime-native propagation evidence</h3>
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
              Showing the latest {VISIBLE_AUDIT_ROWS} of {entries.length} records — the export carries all of them.
            </p>
          ) : null}
          <p className="signals-audit-footnote">
            Worth supplies every payload. React only pins them here for comparison—like a corkboard,
            but with fewer red strings.
          </p>
        </section>

        <SignalsWhyPanel graph={graph} panelRef={explanationPanelRef} selected={selectedNode} />
      </section>

      <section className="signals-code-section" aria-labelledby="signals-code-title">
        <h2 id="signals-code-title">The DAG computes the answer—and preserves the evidence.</h2>
        <SignalsCodeSample liveWhyLine={liveWhyLine} />
      </section>
    </>
  );
}
