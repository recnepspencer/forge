import React from "react";
import type {
  DemoRoleProfile,
  DemoThreeData,
  NavLinkDef,
  RouteAccessResolution,
} from "./demoThreeRouterModel";
import { renderMainPanel } from "./DemoThreeContent";
import { DemoThreeModal } from "./DemoThreeModal";
import { describeSurface } from "./DemoThreeSurfaceMeta";
import { resolveRouteAccess } from "./demoThreeRouterModel";

interface NavRouteItem {
  kind: "route";
  label: string;
  link: NavLinkDef;
  activeRoutes: NavLinkDef["id"][];
}

interface NavStaticItem {
  kind: "static";
  label: string;
}

type NavItem = NavRouteItem | NavStaticItem;

interface DemoThreeSurfaceProps {
  data: DemoThreeData;
  role: DemoRoleProfile;
  activeHref: string;
  currentRouteId: NavLinkDef["id"];
  crumbLabels: string[];
  routerModel: any;
  lastResolution: RouteAccessResolution | null;
  navigateTo: (link: NavLinkDef) => Promise<void>;
  navLinkForRoute: (model: any, routeId: NavLinkDef["id"], productId?: string) => NavLinkDef;
}

export const DemoThreeSurface: React.FC<DemoThreeSurfaceProps> = ({
  data,
  role,
  activeHref,
  currentRouteId,
  crumbLabels,
  routerModel,
  lastResolution,
  navigateTo,
  navLinkForRoute,
}) => {
  const homeLink = navLinkForRoute(routerModel, "home");
  const catalogLink = navLinkForRoute(routerModel, "catalog");
  const salesLink = navLinkForRoute(routerModel, "sales");
  const addProductLink = navLinkForRoute(routerModel, "addProduct");
  const surface = describeSurface(currentRouteId, data, role, activeHref, lastResolution);
  const breadcrumbText = crumbLabels.length > 0 ? crumbLabels.join(" / ") : surface.breadcrumbs;
  const navSections: Array<{ heading: string; items: NavItem[] }> = [
    {
      heading: "Home",
      items: [{ kind: "route", label: "Home", link: homeLink, activeRoutes: ["home"] }],
    },
    {
      heading: "Orders",
      items: [{ kind: "static", label: "Orders" }],
    },
    {
      heading: "Products",
      items: [
        {
          kind: "route",
          label: "Products",
          link: catalogLink,
          activeRoutes: ["catalog", "product", "addProduct", "editProduct", "restricted"],
        },
        { kind: "static", label: "Collections" },
        { kind: "static", label: "Inventory" },
      ],
    },
    {
      heading: "Customers",
      items: [{ kind: "static", label: "Customers" }],
    },
    {
      heading: "Marketing",
      items: [{ kind: "static", label: "Discounts" }],
    },
    {
      heading: "Analytics",
      items: [{ kind: "route", label: "Analytics", link: salesLink, activeRoutes: ["sales"] }],
    },
  ];

  return (
    <div className="storefront-shell">
      <aside className="storefront-sidebar">
        <div className="storefront-brand">
          <div className="storefront-brand-badge">S</div>
          <div>
            <strong>Shopify-style admin</strong>
            <span>{data.storefrontName} Commerce</span>
            <small>Northstar outdoor goods</small>
          </div>
        </div>

        <div className="storefront-store-switch">
          <div>
            <strong>Northstar Commerce</strong>
            <small>Online Store</small>
          </div>
          <span className="storefront-status active">Live</span>
        </div>

        {navSections.map((section) => (
          <div key={section.heading} className="storefront-nav-group">
            <span className="storefront-nav-label">{section.heading}</span>
            {section.items.map((item) => {
              if (item.kind === "static") {
                return (
                  <button key={item.label} className="storefront-nav-item muted" disabled type="button">
                    <span>{item.label}</span>
                  </button>
                );
              }

              const access = resolveRouteAccess(data, role, item.link, routerModel.refs.restricted.href);
              const active = item.activeRoutes.includes(currentRouteId);
              return (
                <button
                  key={item.label}
                  className={`storefront-nav-item ${active ? "active" : ""}`}
                  disabled={!access.allowed}
                  onClick={() => void navigateTo(item.link)}
                  title={access.deniedReason ?? undefined}
                  type="button"
                >
                  <span>{item.label}</span>
                  {!access.allowed && <small className="storefront-nav-lock">Locked</small>}
                </button>
              );
            })}
          </div>
        ))}
      </aside>

      <div className="storefront-main">
        <header className="storefront-topbar">
          <div className="storefront-topbar-search">
            <span>Search</span>
            <input readOnly value="Search products, orders, customers, and analytics" />
          </div>

          <div className="storefront-topbar-actions">
            <button className="storefront-icon-button" type="button">Alerts</button>
            <button className="storefront-icon-button" type="button">Settings</button>
            <button className="storefront-user" type="button">
              <span className="storefront-avatar">{role.userInitial}</span>
              <span>
                <strong>{role.userName}</strong>
                <small>{role.userEmail}</small>
              </span>
            </button>
          </div>
        </header>

        <main className="storefront-content">
          <div className="storefront-breadcrumbs">{breadcrumbText}</div>

          <div className="storefront-page-header">
            <div>
              <h1>{surface.title}</h1>
              <p>{surface.subtitle}</p>
            </div>
            <div className="storefront-page-actions">
              {surface.showExport && <button className="storefront-button" type="button">Export</button>}
              {surface.showPrimaryAction && (
                <button
                  className="storefront-button primary"
                  disabled={!surface.primaryActionAllowed}
                  onClick={surface.primaryActionAllowed ? () => void navigateTo(addProductLink) : undefined}
                  title={surface.primaryActionReason ?? undefined}
                  type="button"
                >
                  {surface.primaryActionLabel}
                </button>
              )}
            </div>
          </div>

          {renderMainPanel({
            currentRouteId,
            data,
            role,
            activeHref,
            lastResolution,
            navigateTo,
            navLinkForRoute,
            routerModel,
          })}

          <DemoThreeModal
            currentRouteId={currentRouteId}
            data={data}
            role={role}
            activeHref={activeHref}
            resolution={lastResolution}
            navigateTo={navigateTo}
            navLinkForRoute={navLinkForRoute}
            routerModel={routerModel}
          />
        </main>
      </div>
    </div>
  );
};
