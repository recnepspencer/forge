import type {
  DemoRoleProfile,
  DemoThreeData,
  NavLinkDef,
  RouteAccessResolution,
} from "./demoThreeRouterModel";
import {
  adminActionButton,
  formatInventoryState,
  media,
  ownerLabel,
  permissionHint,
  productForRoute,
  statusTone,
} from "./DemoThreeContentHelpers";

interface RenderArgs {
  currentRouteId: NavLinkDef["id"];
  data: DemoThreeData;
  role: DemoRoleProfile;
  activeHref: string;
  lastResolution: RouteAccessResolution | null;
  navigateTo: (link: NavLinkDef) => Promise<void>;
  navLinkForRoute: (model: any, routeId: NavLinkDef["id"], productId?: string) => NavLinkDef;
  routerModel: any;
}

export function renderMainPanel({
  currentRouteId,
  data,
  role,
  activeHref,
  lastResolution,
  navigateTo,
  navLinkForRoute,
  routerModel,
}: RenderArgs) {
  const product = productForRoute(data, activeHref) ?? data.products[0];
  const canCreate = role.canAddProducts;
  const canEditCurrent =
    role.canEditAllProducts || (role.canEditOwnedProducts && role.id === product.ownerRole);

  if (currentRouteId === "sales") {
    return (
      <>
        <section className="storefront-summary-grid">
          {[
            ["Gross sales", data.sales.grossRevenue],
            ["Net sales", data.sales.netSales],
            ["Orders", data.sales.unitsSold],
            ["Average order value", data.sales.averageOrderValue],
          ].map(([label, value]) => (
            <article key={label} className="storefront-card storefront-metric">
              <span>{label}</span>
              <strong>{value}</strong>
            </article>
          ))}
        </section>

        <section className="storefront-detail-grid">
          <article className="storefront-card">
            <h3>Total sales over time</h3>
            <p>{data.reportingWindow} across the online store, retail, and wholesale channels.</p>
            <div className="storefront-note">
              <strong>{data.sales.topChannel}</strong>
              <span>Top channel · Conversion {data.sales.conversionRate} · Refunds {data.sales.refundRate}</span>
            </div>
          </article>

          <article className="storefront-card">
            <h3>Role-aware finance access</h3>
            <p>Analytics is visible to Admin and Accountant, and denied to Product Owner and Merchandiser.</p>
            <div className="storefront-note">
              <strong>Returning customers</strong>
              <span>{data.sales.returningCustomers} of sales came from repeat buyers.</span>
            </div>
          </article>
        </section>
      </>
    );
  }

  if (currentRouteId === "catalog" || currentRouteId === "addProduct") {
    return (
      <>
        <section className="storefront-summary-grid">
          {[
            ["Products", String(data.products.length)],
            ["Low stock", data.dashboard.lowStockCount],
            ["Needs review", data.dashboard.pendingReviewCount],
            ["Top collection", data.dashboard.topCollection],
          ].map(([label, value]) => (
            <article key={label} className="storefront-card storefront-metric">
              <span>{label}</span>
              <strong>{value}</strong>
            </article>
          ))}
        </section>

        <section className="storefront-card">
          <div className="storefront-filter-row">
            <div className="storefront-filter-search">
              <input readOnly value="Search products" />
            </div>
            <div className="storefront-filter-actions">
              <button className="storefront-filter-button" type="button">All</button>
              <button className="storefront-filter-button" type="button">Active</button>
              <button className="storefront-filter-button" type="button">Draft</button>
              <button className="storefront-filter-button" type="button">More filters</button>
            </div>
          </div>

          {permissionHint(
            canCreate,
            canCreate
              ? "This role can open the routed add-product sheet directly from the products index."
              : "Add product stays disabled before navigation because this role does not have product-creation access.",
          )}

          <div className="storefront-table-wrap">
            <table className="storefront-table">
              <thead>
                <tr>
                  <th>Product</th>
                  <th>Status</th>
                  <th>Inventory</th>
                  <th>Category</th>
                  <th>Markets</th>
                  <th>Sales</th>
                  <th>Updated</th>
                  <th />
                </tr>
              </thead>
              <tbody>
                {data.products.map((entry) => {
                  const detailLink = navLinkForRoute(routerModel, "product", entry.id);
                  const editLink = navLinkForRoute(routerModel, "editProduct", entry.id);
                  const canEdit =
                    role.canEditAllProducts || (role.canEditOwnedProducts && role.id === entry.ownerRole);

                  return (
                    <tr key={entry.id}>
                      <td>
                        <button className="storefront-product-link" onClick={() => void navigateTo(detailLink)} type="button">
                          {media(entry)}
                          <span>
                            <strong className="storefront-table-title">{entry.name}</strong>
                            <small>{entry.sku} · {entry.vendor}</small>
                          </span>
                        </button>
                      </td>
                      <td><span className={`storefront-status ${statusTone(entry.status)}`}>{entry.status}</span></td>
                      <td>{formatInventoryState(entry.inventory, entry.inventoryState)}</td>
                      <td>{entry.category}</td>
                      <td>{entry.market}</td>
                      <td>{entry.last30Sales}</td>
                      <td>{entry.updatedAt}</td>
                      <td>
                        <div className="storefront-row-actions">
                          <button className="storefront-button" onClick={() => void navigateTo(detailLink)} type="button">
                            Open
                          </button>
                          {adminActionButton(
                            "Edit",
                            () => {
                              void navigateTo(editLink);
                            },
                            canEdit,
                            canEdit ? null : `${role.label} cannot edit ${entry.name}.`,
                          )}
                        </div>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </section>
      </>
    );
  }

  if (currentRouteId === "product" || currentRouteId === "editProduct") {
    return (
      <section className="storefront-detail-grid">
        <div className="storefront-summary-grid" style={{ gridTemplateColumns: "1fr" }}>
          <article className="storefront-card">
            <div className="storefront-detail-head">
              <div className="storefront-inline-stat">
                <span>Status</span>
                <strong>{product.status}</strong>
              </div>
              <h2>{product.name}</h2>
              <p>{product.description}</p>
              <div className="storefront-detail-actions">
                {adminActionButton(
                  "Edit product",
                  () => {
                    void navigateTo(navLinkForRoute(routerModel, "editProduct", product.id));
                  },
                  canEditCurrent,
                  canEditCurrent ? null : `${role.label} cannot edit ${product.name}.`,
                )}
                <button className="storefront-button" type="button">Preview</button>
              </div>
            </div>
          </article>

          <article className="storefront-card">
            <div className="storefront-gallery-main">
              <img src={product.imageUrl} alt={product.name} />
            </div>
          </article>

          <article className="storefront-card">
            <h3>Media</h3>
            <p>Primary storefront image plus one supporting lifestyle slot.</p>
            <div className="storefront-detail-grid">
              <div className="storefront-gallery-secondary">
                <img src={product.imageUrl} alt={product.name} />
              </div>
              <div className="storefront-gallery-secondary">Lifestyle image placeholder</div>
            </div>
          </article>
        </div>

        <div className="storefront-summary-grid" style={{ gridTemplateColumns: "1fr" }}>
          <article className="storefront-card">
            <h3>Publishing</h3>
            {permissionHint(
              canEditCurrent,
              canEditCurrent
                ? "This role can open the routed edit sheet for this product."
                : "This role can view the product, but the routed edit sheet stays disabled.",
            )}
            <div className="storefront-note">
              <strong>{product.channels.join(", ")}</strong>
              <span>{product.market} · Updated by {product.updatedBy}</span>
            </div>
          </article>

          <article className="storefront-card">
            <h3>Product organization</h3>
            <div className="storefront-note">
              <strong>{product.productType}</strong>
              <span>{product.category} · Owned by {ownerLabel(data, product.ownerRole)}</span>
            </div>
          </article>

          <article className="storefront-card">
            <h3>Pricing</h3>
            <div className="storefront-note">
              <strong>{product.price}</strong>
              <span>Compare-at {product.compareAtPrice} · Margin {product.margin}</span>
            </div>
          </article>

          <article className="storefront-card">
            <h3>Inventory</h3>
            <div className="storefront-note">
              <strong>{product.inventory} units</strong>
              <span>{product.inventoryState} · {product.last30Sales} over the last 30 days</span>
            </div>
          </article>
        </div>
      </section>
    );
  }

  if (currentRouteId === "restricted") {
    return (
      <article className="storefront-card">
        <h3>Permission required</h3>
        <p>{lastResolution?.deniedReason ?? "That page is not available for the current role."}</p>
        <div className="storefront-note locked">
          <strong>{lastResolution?.requestedLabel ?? "Unknown page"}</strong>
          <span>The router redirected this role into the restricted route instead of opening the requested page.</span>
        </div>
        <div className="storefront-page-actions">
          <button
            className="storefront-button"
            onClick={() => void navigateTo(navLinkForRoute(routerModel, "catalog"))}
            type="button"
          >
            Return to products
          </button>
        </div>
      </article>
    );
  }

  return (
    <>
      <section className="storefront-summary-grid">
        {[
          ["Sessions", data.dashboard.onlineStoreSessions],
          ["Orders", data.dashboard.totalOrders],
          ["Gross sales", data.dashboard.grossSales],
          ["Returning customers", data.dashboard.returningCustomerRate],
        ].map(([label, value]) => (
          <article key={label} className="storefront-card storefront-metric">
            <span>{label}</span>
            <strong>{value}</strong>
          </article>
        ))}
      </section>

      <section className="storefront-detail-grid">
        <article className="storefront-card">
          <h3>Store activity</h3>
          <p>Low stock, pending reviews, and upcoming payout events for the storefront.</p>
          <div className="storefront-note">
            <strong>{data.dashboard.lowStockCount} products low on stock</strong>
            <span>{data.dashboard.pendingReviewCount} products are still waiting for merchandising review.</span>
          </div>
        </article>

        <article className="storefront-card">
          <h3>Top products</h3>
          <p>Quick access into the best-performing products for this reporting window.</p>
          {data.products.slice(0, 2).map((entry) => (
            <button
              key={entry.id}
              className="storefront-product-link"
              onClick={() => void navigateTo(navLinkForRoute(routerModel, "product", entry.id))}
              type="button"
            >
              {media(entry)}
              <span>
                <strong className="storefront-table-title">{entry.name}</strong>
                <small>{entry.last30Sales} in the last 30 days</small>
              </span>
            </button>
          ))}
        </article>
      </section>
    </>
  );
}
