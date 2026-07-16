import React from "react";

import {
  formatOffset,
  PO_NUMBER,
  PO_URL,
  type Agreement,
  type PanelEvent,
  type PanelVariant,
  type PoLine,
  type ServerTruth,
  type ServerTruthRecord,
} from "./resourcesSectionSupport";

function useVisibleToasts(events: readonly PanelEvent[]): readonly PanelEvent[] {
  const [visible, setVisible] = React.useState<readonly PanelEvent[]>([]);
  const timeoutByEventId = React.useRef(new Map<string, number>());
  const latestId = events[0]?.id ?? null;

  React.useEffect(() => {
    if (!latestId) {
      for (const timeout of timeoutByEventId.current.values()) {
        window.clearTimeout(timeout);
      }
      timeoutByEventId.current.clear();
      setVisible([]);
      return;
    }
    const next = events[0];
    setVisible((current) => [next, ...current.filter((entry) => entry.id !== next.id)].slice(0, 2));
    if (timeoutByEventId.current.has(next.id)) return;
    const timeout = window.setTimeout(() => {
      setVisible((current) => current.filter((entry) => entry.id !== next.id));
      timeoutByEventId.current.delete(next.id);
    }, 2600);
    timeoutByEventId.current.set(next.id, timeout);
  }, [events, latestId]);

  React.useEffect(() => () => {
    for (const timeout of timeoutByEventId.current.values()) {
      window.clearTimeout(timeout);
    }
  }, []);

  return visible;
}

function agreementLabel(agreement: Agreement): string {
  if (agreement.kind === "matches") return "matches server";
  if (agreement.kind === "speculating") {
    return `speculating · ${agreement.pendingCount} pending`;
  }
  const parts: string[] = [];
  if (agreement.missingLabels.length > 0) {
    parts.push(`${agreement.missingLabels.length} confirmed record${agreement.missingLabels.length === 1 ? "" : "s"} missing`);
  }
  if (agreement.phantomLabels.length > 0) {
    parts.push(`${agreement.phantomLabels.length} rejected record${agreement.phantomLabels.length === 1 ? "" : "s"} on screen`);
  }
  return parts.join(" · ");
}

export function AgreementBadge({ agreement }: { agreement: Agreement | null }): React.ReactElement | null {
  if (!agreement) return null;
  return (
    <span className={`po-agreement po-agreement-${agreement.kind}`} role="status">
      {agreementLabel(agreement)}
    </span>
  );
}

const TRUTH_CHIP_LABEL: Record<ServerTruthRecord["status"], string> = {
  pending: "deciding…",
  confirmed: "confirmed",
  rejected: "rejected",
  cancelled: "cancelled",
};

export function PlatformOwner({
  description,
  title,
  variant,
}: {
  description: string;
  title: string;
  variant: PanelVariant;
}): React.ReactElement {
  return (
    <header className={`po-platform-owner po-platform-owner-${variant}`}>
      <span>{variant === "tanstack" ? "Left column · shared cache" : "Right column · branch runtime"}</span>
      <h2>{title}</h2>
      <p>{description}</p>
    </header>
  );
}

export function PoPanel({
  agreement,
  caption,
  error,
  events,
  highlightId,
  lines,
  loading,
  refetching,
  serverTruth = [],
  title,
  variant,
}: {
  agreement: Agreement | null;
  caption: string;
  error?: string | null;
  events: readonly PanelEvent[];
  highlightId: string | null;
  lines: readonly PoLine[] | null;
  loading: boolean;
  refetching?: boolean;
  serverTruth: ServerTruth;
  title: string;
  variant: PanelVariant;
}): React.ReactElement {
  const toasts = useVisibleToasts(events);
  const serverStatusById = new Map(serverTruth.map((record) => [record.line.id, record.status]));
  const diverged = agreement?.kind === "wrong";

  return (
    <article className={`po-window po-window-${variant}`}>
      <header className="po-window-chrome">
        <span className="signals-code-dots" aria-hidden="true"><i /><i /><i /></span>
        <code className="po-window-url">{PO_URL}</code>
        <span className={`po-window-badge po-window-badge-${variant}`}>{title}</span>
      </header>

      <div className="po-window-body">
        <div className="po-order-head">
          <strong>{PO_NUMBER}</strong>
          <span>Purchase order · line items</span>
          {refetching ? <em className="po-refetching">refetching entire list…</em> : null}
          <span className="po-order-agreement"><AgreementBadge agreement={agreement} /></span>
        </div>

        {error ? (
          <div className="po-window-empty is-error">{error}</div>
        ) : loading || !lines ? (
          <div className="po-window-empty">
            <span className="po-spinner" aria-hidden="true" />
            <span>loading lines…</span>
          </div>
        ) : (
          <div className="po-current-value">
            <div className="po-current-value-head">
              <span>Current visible value</span>
              <code>{lines.length} record{lines.length === 1 ? "" : "s"}</code>
              {diverged ? <em>diverged here</em> : null}
            </div>
            <ul className="po-lines">
              {lines.map((line) => {
                const serverStatus = serverStatusById.get(line.id);
                return (
                  <li
                    className={`po-line${line.id === highlightId ? " is-highlighted" : ""}${serverStatus ? ` is-server-${serverStatus}` : ""}`}
                    key={line.id}
                  >
                    <div className="po-line-main">
                      <strong>{line.label}</strong>
                      <span>{line.qty}</span>
                    </div>
                    <div className="po-line-statuses">
                      <span className={`po-sync po-sync-${line.sync}`}>
                        {line.sync === "synced" ? "synced" : "saving…"}
                      </span>
                      {serverStatus ? (
                        <span className={`po-server-state po-server-state-${serverStatus}`}>
                          server · {TRUTH_CHIP_LABEL[serverStatus]}
                        </span>
                      ) : null}
                    </div>
                  </li>
                );
              })}
            </ul>
            {agreement && agreement.missingLabels.length > 0 ? (
              <div className="po-value-missing" role="status">
                <span>Missing from this current value</span>
                <strong>{agreement.missingLabels.join(" · ")}</strong>
                <em>server confirmed</em>
              </div>
            ) : null}
          </div>
        )}

        <div className="po-toast-stack" aria-live="polite">
          {toasts.map((toast) => (
            <div className={`po-toast po-toast-${toast.tone}`} key={toast.id}>
              <strong>{toast.title}</strong>
              <span>{toast.detail}</span>
            </div>
          ))}
        </div>
      </div>

      <footer className="po-window-caption">
        <code>{caption}</code>
      </footer>
    </article>
  );
}

export function ServerTruthStrip({
  baseMs,
  truth,
}: {
  baseMs: number | null;
  truth: ServerTruth;
}): React.ReactElement {
  return (
    <section className="po-truth" aria-label="Server truth">
      <header className="po-truth-head">
        <span className="po-truth-title">Server truth · {PO_NUMBER}</span>
        <code>the referee — both screens are judged against this</code>
      </header>
      <div className="po-truth-chips">
        {truth.map((record) => (
          <span className={`po-truth-chip po-truth-chip-${record.status}`} key={record.line.id}>
            <strong>{record.line.label}</strong>
            <em>
              {TRUTH_CHIP_LABEL[record.status]}
              {record.status !== "pending" && record.atMs > 0 && baseMs !== null
                ? ` ${formatOffset(record.atMs, baseMs)}`
                : ""}
            </em>
          </span>
        ))}
      </div>
    </section>
  );
}

export function CliffhangerCard({
  onDeliver,
}: {
  onDeliver: () => void;
}): React.ReactElement {
  return (
    <aside className="po-cliffhanger" role="status">
      <p className="po-cliffhanger-question">
        The vendor check on the calibration kit is about to fail. The server has already
        confirmed the sterile tubing. What should happen to it?
      </p>
      <button className="po-control-button po-cliffhanger-button" onClick={onDeliver} type="button">
        Deliver the rejection
      </button>
    </aside>
  );
}

export function VerdictLine({
  healed,
}: {
  healed: boolean;
}): React.ReactElement {
  return (
    <p className="po-verdict" role="status">
      One rejection was delivered to both screens. Only one still agrees with the server.
      {healed
        ? " The left screen healed itself by refetching the entire list — and no record remains that it was ever wrong. The right screen was never wrong, and keeps the retired branches as evidence."
        : " The left column marks the exact wrong row in red under Current visible value."}
    </p>
  );
}

export interface ConvergenceFacts {
  readonly matchesServer: boolean;
  readonly openEffectCount: number;
  readonly mergedCount: number;
  readonly rejectedCount: number;
  readonly cancelledCount: number;
}

export function ConvergenceReceipt({ facts }: { facts: ConvergenceFacts }): React.ReactElement {
  return (
    <div className="po-convergence" role="status">
      <span className={`po-convergence-item ${facts.matchesServer ? "is-good" : "is-bad"}`}>
        {facts.matchesServer ? "✓" : "✗"} visible records match scripted server truth
      </span>
      <span className={`po-convergence-item ${facts.openEffectCount === 0 ? "is-good" : "is-bad"}`}>
        {facts.openEffectCount === 0 ? "✓" : "✗"} open effects: {facts.openEffectCount}
      </span>
      <span className="po-convergence-item is-good">
        {facts.mergedCount} merged · {facts.rejectedCount} rejected · {facts.cancelledCount} dependency-cancelled
      </span>
      <code>line.effects().counters() · terminal receipts</code>
    </div>
  );
}
