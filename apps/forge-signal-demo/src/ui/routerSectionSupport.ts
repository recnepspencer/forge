import { resourcePolicyProfiles } from "forge-signal-wasm";

export type SessionRole = "loggedOut" | "user" | "admin";

export interface RouterSectionModel {
  readonly routes: any;
  readonly initialTarget: string;
  readonly routeOptions: ReadonlyArray<{ path: string; label: string }>;
}

export interface CatalogPageJson {
  readonly summary: string;
  readonly items: ReadonlyArray<{ name: string; price: string }>;
}

export interface OrderPageJson {
  readonly orderId: string;
  readonly customer: string;
  readonly shipment: string;
  readonly payment: string;
}

export interface AdminProductsPageJson {
  readonly updatedAt: string;
  readonly rows: ReadonlyArray<{ name: string; status: string }>;
}

export interface RevenueReportPageJson {
  readonly updatedAt: string;
  readonly totals: ReadonlyArray<{ label: string; value: string }>;
}

export const roleLabels: Record<SessionRole, string> = {
  loggedOut: "Logged out",
  user: "User",
  admin: "Admin",
};

const routeOptions = [
  { path: "/catalog", label: "Catalog" },
  { path: "/orders/123", label: "Order details" },
  { path: "/admin/products", label: "Admin products" },
  { path: "/reports/revenue", label: "Revenue report" },
] as const;

const fakeApiJson = {
  catalog: {
    summary: "3 products loaded",
    items: [
      { name: "Carry-On", price: "$120" },
      { name: "Weekender", price: "$168" },
      { name: "Travel Kit", price: "$42" },
    ],
  } satisfies CatalogPageJson,
  orders: {
    "123": {
      orderId: "123",
      customer: "Maya Chen",
      shipment: "US priority",
      payment: "Paid by card",
    } satisfies OrderPageJson,
  },
  adminProducts: {
    updatedAt: "Synced 2m ago",
    rows: [
      { name: "Carry-On", status: "Price edit enabled" },
      { name: "Shipping policy", status: "Route-coupled form available" },
    ],
  } satisfies AdminProductsPageJson,
  revenueReport: {
    updatedAt: "Latest warehouse close",
    totals: [
      { label: "Gross revenue", value: "$182,400" },
      { label: "Net sales", value: "$164,090" },
      { label: "Return rate", value: "1.8%" },
    ],
  } satisfies RevenueReportPageJson,
} as const;

function cloneJson<T>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}

async function loadJson<T>(value: T): Promise<T> {
  await new Promise((resolve) => window.setTimeout(resolve, 300));
  return cloneJson(value);
}

async function waitForPolicy(): Promise<void> {
  await new Promise((resolve) => window.setTimeout(resolve, 300));
}

export function buildRouterSectionModel(signals: any): RouterSectionModel {
  const api = signals.api({
    baseUrl: "/router-demo-api",
    effects: signals.resource.effects.branchNative(),
  });
  const defaultPolicy = resourcePolicyProfiles.stable();

  const catalogFamily = api.url("/catalog")
    .items((item: { name: string }) => item.name)
    .reconcile(
      (value: CatalogPageJson) => value.items,
      (value: CatalogPageJson, nextItems: CatalogPageJson["items"]) => ({
        ...value,
        items: [...nextItems],
      }),
    )
    .summary(
      "summary",
      (value: CatalogPageJson) => value.summary,
      (value: CatalogPageJson, summary: string) => ({ ...value, summary }),
    )
    .list({
      policy: defaultPolicy,
      load: async () => loadJson(fakeApiJson.catalog),
    });

  const orderFamily = api.url("/orders/:orderId").detail({
    policy: defaultPolicy,
    load: async ({ orderId }: { orderId: string }) =>
      loadJson(fakeApiJson.orders[orderId as keyof typeof fakeApiJson.orders]),
  });

  const adminProductsFamily = api.url("/admin/products").detail({
    policy: defaultPolicy,
    load: async () => loadJson(fakeApiJson.adminProducts),
  });

  const revenueReportFamily = api.url("/reports/revenue").detail({
    policy: defaultPolicy,
    load: async () => loadJson(fakeApiJson.revenueReport),
  });

  const requiresSession = signals.router.prerequisite(
    "requiresSession",
    async ({ facts, allow, redirect }: any) => {
      await waitForPolicy();
      return facts.role === "loggedOut"
        ? redirect({
            href: "/sign-in",
            reason: "signInRequired",
            detail: "Route requires an authenticated session.",
          })
        : allow({ reason: "sessionPresent" });
    },
  );

  const adminOnly = signals.router.prerequisite(
    "adminOnly",
    async ({ facts, allow, forbidden }: any) => {
      await waitForPolicy();
      return facts.role === "admin"
        ? allow({ reason: "adminRolePresent" })
        : forbidden({
            reason: "adminRoleRequired",
            detail: "Route requires the admin role.",
          });
    },
  );

  const routes = signals.router.define({
    catalog: signals.router.route("/catalog", {
      resources: {
        page: signals.router.resourceLine(catalogFamily, {
          params: () => ({}),
          prefetch: "intent",
        }),
      },
    }),
    orderDetails: signals.router.route("/orders/:orderId", {
      admission: [requiresSession],
      resources: {
        page: signals.router.resourceLine(orderFamily, {
          params: ({ params }: any) => ({ orderId: params.orderId }),
          prefetch: "intent",
        }),
      },
    }),
    adminProducts: signals.router.route("/admin/products", {
      admission: [requiresSession, adminOnly],
      resources: {
        page: signals.router.resourceLine(adminProductsFamily, {
          params: () => ({}),
          prefetch: "intent",
        }),
      },
    }),
    revenueReport: signals.router.route("/reports/revenue", {
      admission: [requiresSession, adminOnly],
      resources: {
        page: signals.router.resourceLine(revenueReportFamily, {
          params: () => ({}),
          prefetch: "intent",
        }),
      },
    }),
    signIn: signals.router.route("/sign-in"),
  });

  return {
    routes,
    initialTarget: routeOptions[0].path,
    routeOptions,
  };
}

export function formatBrowserResult(
  role: SessionRole,
  report: any,
  story: any,
  pageLine: any,
): string {
  const outcome = report?.outcome?.();
  const inspection = story?.inspection?.() ?? null;
  const auditability = story?.auditability?.() ?? null;
  const admittedHistory = story?.admittedEntries?.() ?? [];
  const boundaryEvents = story?.events?.() ?? [];
  const inspectionSummary = inspection?.summary?.() ?? null;
  const auditabilitySummary = auditability?.summary?.() ?? null;

  return JSON.stringify(
    {
      role,
      current: {
        attemptedHref: report?.rawLocationHref ?? null,
        visibleHref: story?.current?.()?.href ?? null,
        outcomeKind: outcome?.kind ?? null,
        routeId: story?.current?.()?.routeId ?? null,
      },
      page:
        pageLine && outcome?.kind === "admitted"
          ? {
              status: pageLine.status().kind,
              freshness: pageLine.freshness().kind,
            }
          : null,
      history: {
        entries: admittedHistory.length,
        events: boundaryEvents.length,
        visibleSource: auditabilitySummary?.currentVisibleRouteSource ?? null,
        latestBoundary: auditabilitySummary?.latestBoundaryArtifact ?? null,
        chain: admittedHistory.map((entry: any) => ({
          href: entry.href,
          routeId: entry.routeId,
        })),
        boundaryEvents: boundaryEvents.map((event: any) => ({
          targetHref: event.targetHref,
          outcomeKind: event.outcomeKind,
          boundaryArtifact: event.boundaryArtifact,
        })),
      },
      inspection: inspectionSummary
        ? {
            currentEntryAvailable: inspectionSummary.currentEntryAvailable,
            backProvenanceAvailable: inspectionSummary.backProvenanceAvailable,
            convergedBoundaryEventCount: inspectionSummary.convergedBoundaryEventCount,
            notAdmittedBoundaryEventCount: inspectionSummary.notAdmittedBoundaryEventCount,
          }
        : null,
    },
    null,
    2,
  );
}

export function formatSequenceResult(
  role: SessionRole,
  result: any,
): string {
  return JSON.stringify(
    {
      role,
      steps:
        result?.steps?.map((step: any) => ({
          targetHref: step.targetHref,
          outcomeKind: step.report.outcome().kind,
          routeId: step.current?.routeId ?? null,
          href: step.current?.href ?? null,
          boundaryArtifact: step.event.boundaryArtifact,
        })) ?? [],
      current: result?.story?.current?.()
        ? {
            routeId: result.story.current().routeId,
            href: result.story.current().href,
          }
        : null,
      history:
        result?.story?.admittedEntries?.().map((entry: any) => ({
          routeId: entry.routeId,
          href: entry.href,
        })) ?? [],
    },
    null,
    2,
  );
}
