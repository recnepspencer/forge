import type {
  AdminProductsPageJson,
  CatalogPageJson,
  OrderPageJson,
  RevenueReportPageJson,
} from "./routerSectionSupport";

interface RouterSectionBrowserSurfaceProps {
  outcome: any;
  statusKind: string | null;
  pageData: unknown;
  isNavigating: boolean;
}

export function RouterSectionBrowserSurface({
  outcome,
  statusKind,
  pageData,
  isNavigating,
}: RouterSectionBrowserSurfaceProps) {
  if (!outcome || isNavigating || (outcome.kind === "admitted" && statusKind !== "fulfilled")) {
    return (
      <div className="router-browser-stage is-loading">
        <div className="router-page-header">
          <strong>{outcome?.kind === "admitted" ? outcome.routeId : "Route transition"}</strong>
          <span>Loading route resources</span>
        </div>
        <div className="router-spinner" aria-hidden="true" />
        <div className="router-report-skeleton">
          <span />
          <span />
          <span />
        </div>
      </div>
    );
  }

  if (outcome.kind === "redirect") {
    return (
      <div className="router-browser-stage is-redirected">
        <div className="router-page-header">
          <strong>Sign in</strong>
          <span>Continue to the requested route</span>
        </div>
        <div className="router-auth-card">
          <span>Email</span>
          <div />
          <span>Password</span>
          <div />
        </div>
      </div>
    );
  }

  if (outcome.kind !== "admitted") {
    return (
      <div className="router-browser-stage is-denied">
        <div className="router-page-header">
          <strong>Permission required</strong>
          <span>Upgrade session access</span>
        </div>
        <div className="router-denied-banner">Admin permission required.</div>
      </div>
    );
  }

  if (outcome.routeId === "catalog") {
    const catalog = pageData as CatalogPageJson;
    return (
      <div className="router-browser-stage is-catalog">
        <div className="router-page-header">
          <strong>Catalog</strong>
          <span>{catalog.summary}</span>
        </div>
        <div className="router-catalog-grid">
          {catalog.items.map((item) => (
            <div className="router-product-tile" key={item.name}>
              <strong>{item.name}</strong>
              <span>{item.price}</span>
            </div>
          ))}
        </div>
      </div>
    );
  }

  if (outcome.routeId === "orderDetails") {
    const order = pageData as OrderPageJson;
    return (
      <div className="router-browser-stage is-order">
        <div className="router-page-header">
          <strong>Order #{order.orderId}</strong>
          <span>{order.payment}</span>
        </div>
        <div className="router-order-card">
          <span>Customer</span>
          <strong>{order.customer}</strong>
        </div>
        <div className="router-order-card">
          <span>Shipment</span>
          <strong>{order.shipment}</strong>
        </div>
      </div>
    );
  }

  if (outcome.routeId === "adminProducts") {
    const adminProducts = pageData as AdminProductsPageJson;
    return (
      <div className="router-browser-stage is-admin">
        <div className="router-page-header">
          <strong>Admin products</strong>
          <span>{adminProducts.updatedAt}</span>
        </div>
        {adminProducts.rows.map((row) => (
          <div className="router-admin-row" key={row.name}>
            <span>{row.name}</span>
            <strong>{row.status}</strong>
          </div>
        ))}
      </div>
    );
  }

  const report = pageData as RevenueReportPageJson;
  return (
    <div className="router-browser-stage is-report">
      <div className="router-page-header">
        <strong>Revenue report</strong>
        <span>{report.updatedAt}</span>
      </div>
      {report.totals.map((entry) => (
        <div className="router-admin-row" key={entry.label}>
          <span>{entry.label}</span>
          <strong>{entry.value}</strong>
        </div>
      ))}
    </div>
  );
}
