import React from "react";

import {
  type NodeKey,
  parseUpstreamVersions,
  safeWhy,
  type TransferGraph,
} from "./signalsTransferRuntime";

export function SignalsWhyPanel({
  graph,
  panelRef,
  selected,
}: {
  graph: TransferGraph;
  panelRef: React.RefObject<HTMLElement | null>;
  selected: NodeKey;
}): React.ReactElement {
  const handle = selected === "amount"
    ? graph.requestedAmount
    : selected === "fee"
      ? graph.processingFee
      : graph.reviewLane;
  const friendlyName = graph.friendlyNames[handle.id] ?? selected;
  const displayName = selected === "amount"
    ? "the requested amount"
    : selected === "fee"
      ? "the processing fee"
      : "the review lane";

  const why = safeWhy(graph, handle.id);
  const versions = why ? parseUpstreamVersions(why.upstream ?? []) : null;
  const reads = why?.callback?.currentReads?.map((id) => graph.friendlyNames[id] ?? id) ?? [];

  return (
    <aside
      aria-label={`Runtime explanation for ${friendlyName}`}
      className="signals-why-panel"
      id="signals-why-panel"
      ref={panelRef}
      tabIndex={-1}
    >
      <header className="signals-panel-head">
        <h3>Why did {displayName} do that?</h3>
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
            <dd>{reads.length > 0 ? reads.join(", ") : "nothing — this is the source value"}</dd>
          </div>
          <div>
            <dt>state</dt>
            <dd>{why.state}</dd>
          </div>
          <div>
            <dt>last outcome</dt>
            <dd className={why.outputChange === "Refreshed" ? "is-changed" : ""}>
              {why.outputChange === "Refreshed"
                ? "ran · changed its answer"
                : why.outputChange === "Unchanged"
                  ? "ran · kept the same answer"
                  : why.outputChange
                    ? `ran · ${why.outputChange.toLowerCase()}`
                    : "has not needed to run again"}
            </dd>
          </div>
          {versions ? (
            <div>
              <dt>dependency versions</dt>
              <dd>
                cached v{versions.cached} · current v{versions.current}
                {versions.cached === versions.current ? " — in sync" : " — stale"}
              </dd>
            </div>
          ) : null}
        </dl>
      ) : (
        <p className="signals-why-empty">Worth has not needed to explain this value yet.</p>
      )}
      {why ? (
        <details className="signals-audit-payload">
          <summary>Show the raw receipt</summary>
          <pre>{JSON.stringify(why, null, 2)}</pre>
        </details>
      ) : null}
    </aside>
  );
}
