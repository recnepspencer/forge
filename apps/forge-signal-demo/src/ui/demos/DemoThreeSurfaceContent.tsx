import React from "react";
import type { DemoRoleProfile, DemoThreeData, RouteAccessResolution } from "./demoThreeRouterModel";

export function pageCopy(routeId: string, data: DemoThreeData) {
  if (routeId === "sales") {
    return {
      title: "Sales performance",
      subtitle: "Revenue and conversion reporting for the current storefront.",
      body: "This page is intentionally sensitive. The router demo uses it to prove that two users can replay the same session and land on different outcomes depending on role.",
    };
  }
  if (routeId === "catalog") {
    return {
      title: "Product catalog",
      subtitle: "Search, filter, and manage active SKUs.",
      body: "This is the parent route that makes breadcrumb context feel real. It is the page users come from before opening or editing a product.",
    };
  }
  if (routeId === "product") {
    return {
      title: data.products[0].name,
      subtitle: "Product overview with inventory, pricing, and ownership.",
      body: "This detail route anchors the product itself so the router can preserve or collapse context when the user moves into edit mode.",
    };
  }
  if (routeId === "addProduct") {
    return {
      title: "Add product",
      subtitle: "Create a new SKU and assign merchandising ownership.",
      body: "This route is useful for the permission demo because some users can create products even if they cannot view financial reporting.",
    };
  }
  if (routeId === "editProduct") {
    return {
      title: "Edit product",
      subtitle: "Modify pricing, content, inventory, and ownership.",
      body: "This is the breadcrumb-policy page. The route can either stand alone or preserve the full catalog and product hierarchy around it.",
    };
  }
  if (routeId === "restricted") {
    return {
      title: "Permission required",
      subtitle: "The requested page exists, but the current role cannot enter it.",
      body: "The denied state is part of the router product story. It shows that route outcomes can stay structured and replayable even when access changes by user type.",
    };
  }
  return {
    title: data.storefrontName,
    subtitle: "Commerce operations and merchandising workspace.",
    body: "Use the role selector and route controls to see how the router owns access outcomes, breadcrumb truth, and retained session history.",
  };
}

function statusClass(status: string) {
  const value = status.toLowerCase();
  if (value === "live") return "live";
  if (value === "draft") return "draft";
  return "restricted";
}

function actionButtonStyle(allowed: boolean, primary = false): React.CSSProperties {
  return {
    padding: "0.68rem 0.9rem",
    borderRadius: "10px",
    border: primary ? "none" : "1px solid var(--border-light)",
    background: primary
      ? allowed
        ? "linear-gradient(135deg, #8b5cf6, #7c3aed)"
        : "rgba(255,255,255,0.06)"
      : "rgba(255,255,255,0.03)",
    color: allowed ? "var(--text-primary)" : "var(--text-muted)",
    fontWeight: 700,
    fontSize: "0.82rem",
    cursor: allowed ? "pointer" : "not-allowed",
    opacity: allowed ? 1 : 0.55,
  };
}

export function renderMainPanel(
  currentRouteId: string,
  data: DemoThreeData,
  role: DemoRoleProfile,
  resolution: RouteAccessResolution | null,
  navigate: (linkId: string) => Promise<void>,
) {
  const displayRouteId =
    currentRouteId === "addProduct"
      ? "catalog"
      : currentRouteId === "editProduct"
        ? "product"
        : currentRouteId === "restricted"
          ? resolution?.requestedLabel === "Sales Stats"
            ? "home"
            : resolution?.requestedLabel === "Add Product"
              ? "catalog"
              : "product"
          : currentRouteId;

  if (displayRouteId === "sales") {
    const metrics = [
      { label: "Gross revenue", value: data.sales.grossRevenue },
      { label: "Units sold", value: data.sales.unitsSold },
      { label: "Refund rate", value: data.sales.refundRate },
      { label: "Conversion", value: data.sales.conversionRate },
    ];
    return (
      <div className="admin-grid metrics">
        {metrics.map((metric) => (
          <div key={metric.label} className="admin-card">
            <div className="admin-card-label">{metric.label}</div>
            <div className="admin-card-value">{metric.value}</div>
          </div>
        ))}
      </div>
    );
  }

  if (displayRouteId === "catalog") {
    const canAdd = role.canAddProducts;
    return (
      <div className="admin-card" style={{ padding: 0, overflow: "hidden" }}>
        <div className="admin-toolbar" style={{ borderRadius: 0, border: "none", borderBottom: "1px solid var(--border-light)" }}>
          <div className="admin-segmented">
            <button className="active">All products</button>
            <button>Live</button>
            <button>Draft</button>
          </div>
          <div style={{ display: "flex", alignItems: "center", gap: "0.75rem", flexWrap: "wrap" }}>
            <div style={{ color: "var(--text-muted)", fontSize: "0.84rem" }}>3 SKUs visible</div>
            <button
              onClick={() => { if (canAdd) void navigate("addProduct"); }}
              disabled={!canAdd}
              title={canAdd ? "Open add-product route" : `${role.label} cannot create products.`}
              style={actionButtonStyle(canAdd, true)}
            >
              Add product
            </button>
          </div>
        </div>
        <table className="admin-table">
          <thead>
            <tr>
              <th>Product</th>
              <th>Status</th>
              <th>Price</th>
              <th>Inventory</th>
              <th>Owner</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {data.products.map((product) => {
              const canEdit = role.canEditAllProducts || (role.canEditOwnedProducts && role.id === product.ownerRole);
              return (
                <tr key={product.id}>
                  <td>
                    <div style={{ color: "var(--text-primary)", fontWeight: 600 }}>{product.name}</div>
                    <div style={{ color: "var(--text-muted)", fontSize: "0.78rem", marginTop: "0.2rem" }}>{product.sku}</div>
                  </td>
                  <td><span className={`admin-status-pill ${statusClass(product.status)}`}>{product.status}</span></td>
                  <td>{product.price}</td>
                  <td>{product.inventory}</td>
                  <td>{data.roles[product.ownerRole].label}</td>
                  <td>
                    <button
                      onClick={() => { if (canEdit) void navigate("editProduct"); }}
                      disabled={!canEdit}
                      title={canEdit ? "Open edit-product route" : `${role.label} cannot edit ${product.name}.`}
                      style={actionButtonStyle(canEdit)}
                    >
                      Edit
                    </button>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    );
  }

  if (displayRouteId === "product") {
    const product = data.products[0];
    const canEdit = role.canEditAllProducts || (role.canEditOwnedProducts && role.id === product.ownerRole);
    return (
      <div className="admin-grid two-up">
        <div className="admin-card">
          <div className="admin-card-label">Product summary</div>
          <div style={{ display: "flex", gap: "0.65rem", flexWrap: "wrap", marginTop: "0.9rem" }}>
            <button onClick={() => { if (canEdit) void navigate("editProduct"); }} disabled={!canEdit} title={canEdit ? "Open edit-product route" : `${role.label} cannot edit this product.`} style={actionButtonStyle(canEdit, true)}>
              Edit product
            </button>
            <button onClick={() => { if (role.canAddProducts) void navigate("addProduct"); }} disabled={!role.canAddProducts} title={role.canAddProducts ? "Open add-product route" : `${role.label} cannot create products.`} style={actionButtonStyle(role.canAddProducts)}>
              Duplicate as new
            </button>
          </div>
          <div className="admin-mini-list" style={{ marginTop: "0.9rem" }}>
            {[
              ["SKU", product.sku],
              ["Retail price", product.price],
              ["Inventory", String(product.inventory)],
              ["Owner", data.roles[product.ownerRole].label],
            ].map(([label, value]) => (
              <div key={label} className="admin-mini-row">
                <span style={{ color: "var(--text-muted)" }}>{label}</span>
                <span style={{ color: "var(--text-primary)", fontWeight: 600 }}>{value}</span>
              </div>
            ))}
          </div>
        </div>
        <div className="admin-card">
          <div className="admin-card-label">Publishing state</div>
          <div style={{ marginTop: "0.9rem", color: "var(--text-secondary)", lineHeight: "1.65" }}>
            The Trailblazer Jacket is live across the main storefront. Pricing is stable, but content updates still require edit permission from a role with merchandising authority.
          </div>
          <div style={{ marginTop: "1rem", display: "flex", gap: "0.6rem", flexWrap: "wrap" }}>
            <span className={`admin-status-pill ${canEdit ? "live" : "restricted"}`}>
              {canEdit ? "edit access granted" : "edit access restricted"}
            </span>
            <span className={`admin-status-pill ${role.canAddProducts ? "live" : "restricted"}`}>
              {role.canAddProducts ? "create access granted" : "create access restricted"}
            </span>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="admin-grid metrics">
      {[
        { label: "Storefront", value: data.storefrontName },
        { label: "Role", value: role.label },
        { label: "Accessible sales", value: role.canViewSales ? "Yes" : "No" },
        { label: "Can create", value: role.canAddProducts ? "Yes" : "No" },
      ].map((item) => (
        <div key={item.label} className="admin-card">
          <div className="admin-card-label">{item.label}</div>
          <div className="admin-card-value">{item.value}</div>
        </div>
      ))}
    </div>
  );
}
