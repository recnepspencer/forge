import React from "react";
import {
  buildDemoThreeModel,
  deriveCurrentLink,
  navLinkForRoute,
  resolveRouteAccess,
  type DemoRoleId,
  type DemoThreeData,
  type NavLinkDef,
  type RouteAccessResolution,
} from "./demoThreeRouterModel";
import { DemoThreeSurface } from "./DemoThreeSurface";
import "./demoThreeSettings.css";
import "./demoThree.css";
import "./demoThreeSheet.css";

interface DemoThreeProps {
  signals: any;
  demo?: any;
  onNavigate: (path: string) => void;
}

export const DemoThree: React.FC<DemoThreeProps> = ({ signals, onNavigate }) => {
  const [data, setData] = React.useState<DemoThreeData | null>(null);
  const [roleId, setRoleId] = React.useState<DemoRoleId>("admin");
  const [activeHref, setActiveHref] = React.useState("/products");
  const [currentRouteId, setCurrentRouteId] = React.useState<NavLinkDef["id"]>("catalog");
  const [crumbLabels, setCrumbLabels] = React.useState<string[]>([]);
  const [lastResolution, setLastResolution] = React.useState<RouteAccessResolution | null>(null);
  const previousRoleRef = React.useRef(roleId);
  const routerModel = React.useMemo(() => buildDemoThreeModel(signals), [signals]);
  const story = React.useMemo(() => signals.router.browserHistory.story(), [signals]);

  React.useEffect(() => {
    let cancelled = false;

    const loadData = async () => {
      const response = await fetch("/api/storefront-router-demo.json");
      const json = (await response.json()) as DemoThreeData;
      if (!cancelled) {
        setData(json);
      }
    };

    void loadData();
    return () => {
      cancelled = true;
    };
  }, []);

  const syncRouteState = React.useCallback((href: string, report: any) => {
    setActiveHref(href);
    setCurrentRouteId(report.diagnostics().routeId);
    setCrumbLabels(story.breadcrumbTrail().entries.map((entry: any) => entry.label));
  }, [story]);

  React.useEffect(() => {
    let cancelled = false;

    const boot = async () => {
      const ingress = signals.router.browserHistory.load(routerModel.refs.catalog.href);
      const report = await routerModel.routes.admitBrowserHistoryIngress(ingress);
      if (cancelled) {
        return;
      }
      story.record(report);
      syncRouteState(routerModel.refs.catalog.href, report);
    };

    void boot();
    return () => {
      cancelled = true;
    };
  }, [routerModel, signals, story, syncRouteState]);

  const navigateTo = React.useCallback(async (link: NavLinkDef) => {
    if (!data) {
      return;
    }

    const resolution = resolveRouteAccess(data, data.roles[roleId], link, routerModel.refs.restricted.href);
    setLastResolution(resolution);

    const ingress = signals.router.browserHistory.push(resolution.finalHref, {
      carriedBreadcrumbs: signals.router.carryBreadcrumbs(story.breadcrumbTrail()),
    });
    const report = await routerModel.routes.admitBrowserHistoryIngress(ingress);
    story.record(report);
    syncRouteState(resolution.finalHref, report);
  }, [data, roleId, routerModel, signals, story, syncRouteState]);

  React.useEffect(() => {
    if (!data) {
      return;
    }
    if (previousRoleRef.current === roleId) {
      return;
    }
    previousRoleRef.current = roleId;

    const currentLink = deriveCurrentLink(routerModel, data, currentRouteId, activeHref, lastResolution);
    if (!currentLink) {
      return;
    }

    const nextResolution = resolveRouteAccess(data, data.roles[roleId], currentLink, routerModel.refs.restricted.href);
    const shouldRedirect =
      (!nextResolution.allowed && currentRouteId !== "restricted") ||
      (currentRouteId === "restricted" && nextResolution.allowed);

    if (shouldRedirect) {
      void navigateTo(currentLink);
    }
  }, [activeHref, currentRouteId, data, lastResolution, navigateTo, roleId, routerModel]);

  if (!data) {
    return <div className="shop-demo-loading">Loading products admin...</div>;
  }

  return (
    <div className="storefront-demo-page">
      <div className="storefront-demo-settings">
        <div className="storefront-demo-settings-head">
          <div className="storefront-demo-settings-copy">
            <span>Demo settings</span>
          <strong>Scenario role</strong>
          </div>

          <button className="storefront-demo-settings-back" onClick={() => onNavigate("#/demos")} type="button">
            Back to ladder
          </button>
        </div>

        <div className="storefront-demo-role-grid">
          {Object.values(data.roles).map((role) => (
            <button
              key={role.id}
              className={`storefront-demo-role-chip ${roleId === role.id ? "active" : ""}`}
              onClick={() => setRoleId(role.id)}
              type="button"
            >
              <strong>{role.label}</strong>
              <span>{role.userName}</span>
            </button>
          ))}
        </div>
      </div>

      <DemoThreeSurface
        data={data}
        role={data.roles[roleId]}
        activeHref={activeHref}
        currentRouteId={currentRouteId}
        crumbLabels={crumbLabels}
        routerModel={routerModel}
        lastResolution={lastResolution}
        navigateTo={navigateTo}
        navLinkForRoute={navLinkForRoute}
      />
    </div>
  );
};
