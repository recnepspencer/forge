export type DemoRoleId = "accountant" | "admin" | "owner" | "merchandiser";
export type ReplayMode = "boundary" | "breadcrumbs" | "history";
export type DemoRouteId =
  | "home"
  | "sales"
  | "catalog"
  | "product"
  | "addProduct"
  | "editProduct"
  | "restricted";

export interface DemoProduct {
  id: string;
  name: string;
  handle: string;
  sku: string;
  status: string;
  ownerRole: DemoRoleId;
  price: string;
  compareAtPrice: string;
  inventory: number;
  inventoryState: string;
  category: string;
  vendor: string;
  margin: string;
  updatedAt: string;
  updatedBy: string;
  channels: string[];
  description: string;
  imageUrl: string;
  productType: string;
  market: string;
  last30Sales: string;
}

export interface DemoRoleProfile {
  id: DemoRoleId;
  label: string;
  userName: string;
  userEmail: string;
  userInitial: string;
  canViewSales: boolean;
  canAddProducts: boolean;
  canEditAllProducts: boolean;
  canViewProducts: boolean;
  canEditOwnedProducts: boolean;
  description: string;
}

export interface DemoThreeData {
  storefrontName: string;
  reportingWindow: string;
  dashboard: {
    onlineStoreSessions: string;
    totalOrders: string;
    grossSales: string;
    returningCustomerRate: string;
    lowStockCount: string;
    pendingReviewCount: string;
    nextPayout: string;
    topCollection: string;
  };
  sales: {
    grossRevenue: string;
    netSales: string;
    unitsSold: string;
    refundRate: string;
    conversionRate: string;
    averageOrderValue: string;
    topChannel: string;
    returningCustomers: string;
  };
  products: DemoProduct[];
  roles: Record<DemoRoleId, DemoRoleProfile>;
}

export interface NavLinkDef {
  id: DemoRouteId;
  label: string;
  href: string;
  productId?: string;
}

export interface RouteAccessResolution {
  allowed: boolean;
  deniedReason: string | null;
  finalHref: string;
  finalLabel: string;
  finalRouteId: DemoRouteId;
  requestedHref: string;
  requestedLabel: string;
  requestedRouteId: DemoRouteId;
  productId?: string;
}

function prettifyHandle(value: string) {
  return value
    .split("-")
    .filter(Boolean)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ");
}

export function buildDemoThreeModel(signals: any) {
  const productParent = signals.router.breadcrumbEntry({
    id: "catalog-parent",
    label: "Products",
    target: "/products",
  });

  const routes = signals.router.define({
    home: signals.router.route("/", {
      breadcrumb: signals.router.breadcrumb({ id: "home", label: "Home" }),
    }),
    sales: signals.router.route("/analytics", {
      breadcrumb: signals.router.breadcrumb({ id: "sales", label: "Analytics" }),
    }),
    catalog: signals.router.route("/products", {
      breadcrumb: signals.router.breadcrumb({ id: "catalog", label: "Products" }),
    }),
    product: signals.router.route("/products/:productId", {
      breadcrumb: signals.router.breadcrumb({
        id: "product",
        label: ({ params }: any) => prettifyHandle(params.productId),
        parent: signals.router.breadcrumbParent({ fallback: productParent }),
      }),
    }),
    addProduct: signals.router.route("/products/new", {
      breadcrumb: signals.router.breadcrumb({
        id: "add-product",
        label: "Add product",
        parent: signals.router.breadcrumbParent({ fallback: productParent }),
      }),
    }),
    editProduct: signals.router.route("/products/:productId/edit", {
      breadcrumb: signals.router.breadcrumb({
        id: "edit-product",
        label: "Edit product",
        parent: signals.router.breadcrumbParent({
          carry: true,
          fallback: signals.router.breadcrumbTrail([
            productParent,
            signals.router.breadcrumbEntry({
              id: "product-parent",
              label: ({ href }: any) => prettifyHandle(href.split("/").slice(-2, -1)[0]),
              target: ({ href }: any) => href.replace(/\/edit$/, ""),
            }),
          ]),
        }),
      }),
    }),
    restricted: signals.router.route("/permission-denied", {
      breadcrumb: signals.router.breadcrumb({ id: "restricted", label: "Permission required" }),
    }),
  });

  return {
    routes,
    refs: {
      home: routes.home.to({}),
      sales: routes.sales.to({}),
      catalog: routes.catalog.to({}),
      addProduct: routes.addProduct.to({}),
      restricted: routes.restricted.to({}),
      product: (productId: string) => routes.product.to({ params: { productId } }),
      editProduct: (productId: string) => routes.editProduct.to({ params: { productId } }),
    },
  };
}

export function navLinkForRoute(
  model: ReturnType<typeof buildDemoThreeModel>,
  routeId: DemoRouteId,
  productId?: string,
): NavLinkDef {
  if (routeId === "product" && productId) {
    const ref = model.refs.product(productId);
    return { id: "product", label: "Product detail", href: ref.href, productId };
  }
  if (routeId === "editProduct" && productId) {
    const ref = model.refs.editProduct(productId);
    return { id: "editProduct", label: "Edit product", href: ref.href, productId };
  }

  const labels: Record<Exclude<DemoRouteId, "product" | "editProduct">, string> = {
    home: "Home",
    sales: "Analytics",
    catalog: "Products",
    addProduct: "Add product",
    restricted: "Permission required",
  };
  const ref =
    routeId === "home"
      ? model.refs.home
      : routeId === "sales"
        ? model.refs.sales
        : routeId === "catalog"
          ? model.refs.catalog
          : routeId === "addProduct"
            ? model.refs.addProduct
            : model.refs.restricted;

  return {
    id: routeId,
    label: labels[routeId as keyof typeof labels],
    href: ref.href,
  };
}

export function readProductIdFromHref(href: string): string | null {
  const match = href.match(/\/products\/([^/]+)/);
  return match?.[1] ?? null;
}

export function findProductByHref(data: DemoThreeData, href: string): DemoProduct | null {
  const productId = readProductIdFromHref(href);
  return data.products.find((entry) => entry.id === productId) ?? null;
}

export function deriveCurrentLink(
  model: ReturnType<typeof buildDemoThreeModel>,
  data: DemoThreeData,
  routeId: DemoRouteId,
  href: string,
  priorResolution: RouteAccessResolution | null,
): NavLinkDef | null {
  if (routeId === "restricted" && priorResolution) {
    return navLinkForRoute(model, priorResolution.requestedRouteId, priorResolution.productId);
  }

  if (routeId === "product" || routeId === "editProduct") {
    const product = findProductByHref(data, href);
    return product ? navLinkForRoute(model, routeId, product.id) : null;
  }

  if (routeId === "home" || routeId === "sales" || routeId === "catalog" || routeId === "addProduct") {
    return navLinkForRoute(model, routeId);
  }

  return null;
}

export function resolveRouteAccess(
  data: DemoThreeData,
  role: DemoRoleProfile,
  link: NavLinkDef,
  restrictedHref: string,
): RouteAccessResolution {
  if (link.id === "sales" && !role.canViewSales) {
    return denied(link, restrictedHref, `${role.label} can't open finance and sales reporting.`);
  }

  if (link.id === "addProduct" && !role.canAddProducts) {
    return denied(link, restrictedHref, `${role.label} can't add products.`);
  }

  if (link.id === "editProduct" || link.id === "product") {
    const product = data.products.find((entry) => entry.id === link.productId);
    if (!product) {
      return denied(link, restrictedHref, "That product could not be found.");
    }

    if (link.id === "product" && !role.canViewProducts) {
      return denied(link, restrictedHref, `${role.label} can't browse catalog details.`);
    }

    if (link.id === "editProduct") {
      const ownsProduct = product.ownerRole === role.id;
      const canEdit = role.canEditAllProducts || (role.canEditOwnedProducts && ownsProduct);
      if (!canEdit) {
        return denied(link, restrictedHref, `${role.label} can't edit ${product.name}.`);
      }
    }
  }

  return {
    allowed: true,
    deniedReason: null,
    finalHref: link.href,
    finalLabel: link.label,
    finalRouteId: link.id,
    requestedHref: link.href,
    requestedLabel: link.label,
    requestedRouteId: link.id,
    productId: link.productId,
  };
}

function denied(link: NavLinkDef, restrictedHref: string, deniedReason: string): RouteAccessResolution {
  return {
    allowed: false,
    deniedReason,
    finalHref: restrictedHref,
    finalLabel: "Permission required",
    finalRouteId: "restricted",
    requestedHref: link.href,
    requestedLabel: link.label,
    requestedRouteId: link.id,
    productId: link.productId,
  };
}
