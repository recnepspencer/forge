import React from "react";

import {
  PO_NUMBER,
  type Agreement,
  type PanelEvent,
  type PoLine,
  type ServerTruth,
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
  if (agreement.kind === "matches") return "Inventory current";
  if (agreement.kind === "speculating") {
    return `${agreement.pendingCount} approval check${agreement.pendingCount === 1 ? "" : "s"}`;
  }
  const parts: string[] = [];
  if (agreement.missingLabels.length > 0) {
    parts.push(`${agreement.missingLabels.length} confirmed record${agreement.missingLabels.length === 1 ? "" : "s"} missing`);
  }
  if (agreement.phantomLabels.length > 0) {
    parts.push(`${agreement.phantomLabels.length} rejected record${agreement.phantomLabels.length === 1 ? "" : "s"} on screen`);
  }
  return parts.join(" / ");
}

export function AgreementBadge({ agreement }: { agreement: Agreement | null }): React.ReactElement | null {
  if (!agreement) return null;
  return (
    <span className={`po-agreement po-agreement-${agreement.kind}`} role="status">
      {agreementLabel(agreement)}
    </span>
  );
}

const SERVER_STATUS_LABEL = {
  pending: "Approval pending",
  confirmed: "Approved",
  rejected: "Rejected",
  cancelled: "Cancelled",
} as const;

const PRODUCT_CATALOG: Record<string, {
  image: string;
  sku: string;
  category: string;
  location: string;
  bin: string;
  unitPrice: string;
}> = {
  "line-071": {
    image: "/products/nitrile-gloves.jpg",
    sku: "PPE-NG-440",
    category: "Examination PPE",
    location: "Central Supply",
    bin: "A-14",
    unitPrice: "$112.50",
  },
  "line-072": {
    image: "/products/controlled-solvent.jpg",
    sku: "LAB-CS-210",
    category: "Controlled laboratory material",
    location: "Hazard Storage",
    bin: "H-03",
    unitPrice: "$86.25",
  },
  "line-073": {
    image: "/products/safety-goggles.jpg",
    sku: "PPE-SG-118",
    category: "Protective eyewear",
    location: "Central Supply",
    bin: "B-07",
    unitPrice: "$74.40",
  },
  "line-074": {
    image: "/products/solvent-handling-kit.jpg",
    sku: "LAB-SHK-032",
    category: "Hazard response kit",
    location: "Hazard Storage",
    bin: "H-08",
    unitPrice: "$129.00",
  },
};

export function MedicalInventoryHeader(): React.ReactElement {
  return (
    <header className="po-app-topbar">
      <div className="po-app-brand">
        <span aria-hidden="true">+</span>
        <div><strong>Northstar</strong><small>Supply Operations</small></div>
      </div>
      <div className="po-app-facility">Northstar Medical Center</div>
      <div className="po-app-account"><span>Central Supply</span><b>ES</b></div>
    </header>
  );
}

export function MedicalInventorySidebar(): React.ReactElement {
  return (
    <aside className="po-app-sidebar">
      <span>Supply chain</span>
      <nav aria-label="Supply chain navigation">
        <span>Overview</span>
        <span>Inventory</span>
        <span className="is-active">Purchase orders</span>
        <span>Suppliers</span>
        <span>Receiving</span>
      </nav>
      <div className="po-app-sidebar-note">
        <strong>Central Supply</strong>
        <small>Building A / Level 1</small>
      </div>
    </aside>
  );
}

export function PoPanel({
  agreement,
  error,
  events,
  highlightId,
  lines,
  loading,
  serverTruth = [],
}: {
  agreement: Agreement | null;
  error?: string | null;
  events: readonly PanelEvent[];
  highlightId: string | null;
  lines: readonly PoLine[] | null;
  loading: boolean;
  serverTruth: ServerTruth;
}): React.ReactElement {
  const toasts = useVisibleToasts(events);
  const serverStatusById = new Map(serverTruth.map((record) => [record.line.id, record.status]));
  const diverged = agreement?.kind === "wrong";

  return (
    <article className="po-window">
      <div className="po-window-body">
        <div className="po-breadcrumbs">Supply chain <span>/</span> Purchase orders <span>/</span> {PO_NUMBER}</div>
        <div className="po-inventory-title">
          <div>
            <span>Purchase order</span>
            <h3>{PO_NUMBER}</h3>
            <small>Created July 18, 2026</small>
          </div>
          <div className="po-order-heading-actions">
            <span className="po-order-agreement"><AgreementBadge agreement={agreement} /></span>
          </div>
        </div>

        <dl className="po-order-metadata">
          <div><dt>Supplier</dt><dd>Meridian Clinical Supply</dd></div>
          <div><dt>Requested by</dt><dd>ICU Operations</dd></div>
          <div><dt>Facility</dt><dd>Northstar Medical Center</dd></div>
          <div><dt>Need by</dt><dd>Jul 22</dd></div>
        </dl>

        {error ? (
          <div className="po-window-empty is-error">{error}</div>
        ) : loading || !lines ? (
          <div className="po-window-empty">
            <span className="po-spinner" aria-hidden="true" />
            <span>loading lines...</span>
          </div>
        ) : (
          <div className="po-current-value">
            <div className="po-current-value-head">
              <div><strong>Order items</strong><span>{lines.length} line item{lines.length === 1 ? "" : "s"}</span></div>
              {diverged ? <em>Needs reconciliation</em> : null}
            </div>
            <div className="po-table-head" aria-hidden="true">
              <span>Item</span><span>SKU</span><span>Qty</span><span>Unit price</span><span>Location</span><span>Approval status</span>
            </div>
            <ul className="po-lines">
              {lines.map((line) => {
                const serverStatus = serverStatusById.get(line.id);
                const product = PRODUCT_CATALOG[line.id];
                return (
                  <li
                    className={`po-line${line.id === highlightId ? " is-highlighted" : ""}${serverStatus ? ` is-server-${serverStatus}` : ""}`}
                    key={line.id}
                  >
                    <img
                      alt=""
                      className="po-line-image"
                      height="96"
                      src={product?.image}
                      width="96"
                    />
                    <div className="po-line-main">
                      <strong>{line.label}</strong>
                      <span>{product?.category}</span>
                    </div>
                    <code className="po-line-sku">{product?.sku}</code>
                    <div className="po-line-stock">
                      <strong>{line.qty}</strong>
                    </div>
                    <div className="po-line-price">{product?.unitPrice}</div>
                    <div className="po-line-location"><span>{product?.location}</span><small>{product?.bin}</small></div>
                    <div className="po-line-statuses">
                      {serverStatus ? (
                        <span className={`po-server-state po-server-state-${serverStatus}`}>
                          {SERVER_STATUS_LABEL[serverStatus]}
                        </span>
                      ) : null}
                      <span className={`po-sync po-sync-${line.sync}`}>
                        {line.sync === "synced" ? "Synced" : "Submitting..."}
                      </span>
                    </div>
                  </li>
                );
              })}
            </ul>
            {agreement && agreement.missingLabels.length > 0 ? (
              <div className="po-value-missing" role="status">
                <span>Missing from this current value</span>
                <strong>{agreement.missingLabels.join(" / ")}</strong>
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
    </article>
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
        {facts.matchesServer ? "Correct:" : "Mismatch:"} inventory matches supplier approvals
      </span>
      <span className={`po-convergence-item ${facts.openEffectCount === 0 ? "is-good" : "is-bad"}`}>
        {facts.openEffectCount === 0 ? "Closed:" : "Open:"} pending writes: {facts.openEffectCount}
      </span>
      <span className="po-convergence-item is-good">
        {facts.mergedCount} accepted / {facts.rejectedCount} rejected / {facts.cancelledCount} related cancellation
      </span>
      <code>verified from runtime counters and audit receipts</code>
    </div>
  );
}
