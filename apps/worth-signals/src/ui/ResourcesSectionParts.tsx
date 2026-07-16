import React from "react";

import {
  formatOffset,
  PO_NUMBER,
  PO_URL,
  type ClaimEntry,
  type LedgerRow,
  type PanelEvent,
  type PanelVariant,
  type PoLine,
  type ScenarioPhase,
} from "./resourcesSectionSupport";

function useVisibleToasts(events: readonly PanelEvent[]): readonly PanelEvent[] {
  const [visible, setVisible] = React.useState<readonly PanelEvent[]>([]);
  const latestId = events[0]?.id ?? null;

  React.useEffect(() => {
    if (!latestId) {
      setVisible([]);
      return;
    }
    const next = events[0];
    setVisible((current) => [next, ...current.filter((entry) => entry.id !== next.id)].slice(0, 2));
    const timeout = window.setTimeout(() => {
      setVisible((current) => current.filter((entry) => entry.id !== next.id));
    }, 2600);
    return () => window.clearTimeout(timeout);
  }, [events, latestId]);

  return visible;
}

export function PoPanel({
  caption,
  error,
  events,
  highlightId,
  lines,
  loading,
  refetching,
  title,
  variant,
}: {
  caption: string;
  error?: string | null;
  events: readonly PanelEvent[];
  highlightId: string | null;
  lines: readonly PoLine[] | null;
  loading: boolean;
  refetching?: boolean;
  title: string;
  variant: PanelVariant;
}): React.ReactElement {
  const toasts = useVisibleToasts(events);

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
        </div>

        {error ? (
          <div className="po-window-empty is-error">{error}</div>
        ) : loading || !lines ? (
          <div className="po-window-empty">
            <span className="po-spinner" aria-hidden="true" />
            <span>loading lines…</span>
          </div>
        ) : (
          <ul className="po-lines">
            {lines.map((line) => (
              <li
                className={`po-line${line.id === highlightId ? " is-highlighted" : ""}`}
                key={line.id}
              >
                <div className="po-line-main">
                  <strong>{line.label}</strong>
                  <span>{line.qty}</span>
                </div>
                <span className={`po-sync po-sync-${line.sync}`}>
                  {line.sync === "synced" ? "synced" : "saving…"}
                </span>
              </li>
            ))}
          </ul>
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

const CLAIM_LABEL: Record<string, string> = {
  absent: "not on screen",
  saving: "saving…",
  synced: "synced",
};

export function ClaimTimeline({
  baseMs,
  claims,
  confirmedAtMs,
}: {
  baseMs: number | null;
  claims: Record<PanelVariant, readonly ClaimEntry[]>;
  confirmedAtMs: number | null;
}): React.ReactElement {
  const rows: Array<{ variant: PanelVariant; label: string }> = [
    { variant: "tanstack", label: "TanStack screen" },
    { variant: "worth", label: "Worth screen" },
  ];

  return (
    <div className="po-claims">
      <header className="signals-panel-head">
        <h3>What each screen claimed about “Sterile tubing”</h3>
        <code>observed from the rendered list, both panels</code>
      </header>
      {confirmedAtMs !== null && baseMs !== null ? (
        <p className="po-claims-note">
          The server confirmed the line at <strong>{formatOffset(confirmedAtMs, baseMs)}</strong>.
          Everything after that moment should say “synced”.
        </p>
      ) : null}
      <div className="po-claims-rows">
        {rows.map(({ variant, label }) => (
          <div className="po-claims-row" key={variant}>
            <span className="po-claims-owner">{label}</span>
            <div className="po-claims-chips">
              {claims[variant].length === 0 ? (
                <span className="po-claims-empty">nothing yet</span>
              ) : (
                claims[variant].map((entry, index) => (
                  <span className={`po-claim po-claim-${entry.sync}`} key={`${variant}-${index}`}>
                    <em>{formatOffset(entry.atMs, baseMs)}</em>
                    {CLAIM_LABEL[entry.sync]}
                  </span>
                ))
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export function RevealBanner({
  baseMs,
  confirmedAtMs,
  healedAtMs,
  phase,
}: {
  baseMs: number | null;
  confirmedAtMs: number | null;
  healedAtMs: number | null;
  phase: ScenarioPhase;
}): React.ReactElement | null {
  if (phase !== "diverged" && phase !== "healed") return null;

  return (
    <aside className="po-reveal" role="status">
      <p className="po-reveal-question">Same clicks. Which screen is telling the truth?</p>
      <p className="po-reveal-body">
        When the calibration kit failed, the left screen rolled back to a cache snapshot taken{" "}
        <strong>before the sterile tubing existed</strong>
        {confirmedAtMs !== null && baseMs !== null
          ? ` — a line the server confirmed at ${formatOffset(confirmedAtMs, baseMs)} just vanished from the screen.`
          : "."}{" "}
        The right screen removed one row: the kit. The tubing it had already reconciled was never
        touched.
      </p>
      {phase === "healed" && healedAtMs !== null && baseMs !== null ? (
        <p className="po-reveal-heal">
          The left screen healed at {formatOffset(healedAtMs, baseMs)} — by refetching the entire
          list. No record remains that it was ever wrong. The right side has the whole incident in
          its history.
        </p>
      ) : null}
    </aside>
  );
}

export function EffectLedger({
  baseMs,
  onExport,
  rows,
}: {
  baseMs: number | null;
  onExport: () => void;
  rows: readonly LedgerRow[];
}): React.ReactElement {
  return (
    <section className="po-ledger" aria-label="Effect ledger">
      <header className="signals-panel-head">
        <h3>The effect ledger</h3>
        <code>line.diagnostics().lastEffect · line.history().lifecycle</code>
        <button className="signals-export-button" onClick={onExport} type="button">
          Export ledger (JSON)
        </button>
      </header>
      <ul className="po-ledger-rows">
        {rows.length === 0 ? (
          <li className="po-ledger-row"><span className="po-ledger-detail">no effects admitted yet</span></li>
        ) : (
          rows.map((row) => (
            <li className="po-ledger-row" key={row.id}>
              <span className="po-ledger-time">{formatOffset(row.atMs, baseMs)}</span>
              <div className="po-ledger-body">
                <p className="po-ledger-title">{row.title}</p>
                <p className="po-ledger-detail">{row.detail}</p>
                {row.payload ? (
                  <details className="signals-audit-payload">
                    <summary>raw effect envelope</summary>
                    <pre>{JSON.stringify(row.payload, null, 2)}</pre>
                  </details>
                ) : null}
              </div>
            </li>
          ))
        )}
      </ul>
      <p className="po-ledger-footnote">
        Every row is read back from the Worth runtime — provenance, confirmation, and rollback
        posture live on the effect, not in a toast that already disappeared.
      </p>
    </section>
  );
}

export function CallbackAftermath({
  cacheLines,
  mutationStatus,
}: {
  cacheLines: readonly PoLine[] | null;
  mutationStatus: string;
}): React.ReactElement {
  return (
    <section className="po-aftermath" aria-label="What the callback model keeps">
      <header className="signals-panel-head">
        <h3>What the callback model keeps</h3>
        <code>queryClient.getQueryData([…]) · mutation.status</code>
      </header>
      <div className="po-aftermath-blocks">
        <div>
          <span>cache value</span>
          <pre>{cacheLines ? JSON.stringify(cacheLines.map((line) => `${line.label} (${line.sync})`), null, 2) : "null"}</pre>
        </div>
        <div>
          <span>mutation.status</span>
          <pre>{JSON.stringify(mutationStatus)}</pre>
        </div>
      </div>
      <p className="po-aftermath-footnote">
        This is the whole inspectable surface. The snapshot lived in a callback closure and is
        gone; nothing records that the screen changed its mind.
      </p>
    </section>
  );
}
