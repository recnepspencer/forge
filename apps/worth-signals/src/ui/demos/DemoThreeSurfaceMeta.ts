import type {
  DemoRoleProfile,
  DemoThreeData,
  NavLinkDef,
  RouteAccessResolution,
} from "./demoThreeRouterModel";
import { productForRoute } from "./DemoThreeContentHelpers";

export function describeSurface(
  currentRouteId: NavLinkDef["id"],
  data: DemoThreeData,
  role: DemoRoleProfile,
  activeHref: string,
  lastResolution: RouteAccessResolution | null,
) {
  const currentProduct = productForRoute(data, activeHref);

  if (currentRouteId === "sales") {
    return {
      title: "Analytics",
      subtitle: `${data.reportingWindow} across gross sales, sessions, and repeat customer behavior.`,
      breadcrumbs: "Home / Analytics",
      showPrimaryAction: false,
      showExport: true,
    };
  }

  if (currentRouteId === "catalog" || currentRouteId === "addProduct") {
    return {
      title: "Products",
      subtitle: "Manage active catalog items, drafts, inventory, and publishing from one index.",
      breadcrumbs: "Home / Products",
      showPrimaryAction: true,
      primaryActionLabel: "Add product",
      primaryActionAllowed: role.canAddProducts,
      primaryActionReason: role.canAddProducts ? null : `${role.label} cannot add products.`,
      showExport: true,
    };
  }

  if (currentRouteId === "product" || currentRouteId === "editProduct") {
    return {
      title: currentProduct?.name ?? "Product detail",
      subtitle: "Description, media, pricing, inventory, and publishing details for the selected product.",
      breadcrumbs: `Home / Products / ${currentProduct?.name ?? "Product detail"}`,
      showPrimaryAction: false,
      showExport: false,
    };
  }

  if (currentRouteId === "restricted") {
    return {
      title: "Permission required",
      subtitle: lastResolution?.deniedReason ?? "This page is not available for the current role.",
      breadcrumbs: "Home / Permission required",
      showPrimaryAction: false,
      showExport: false,
    };
  }

  return {
    title: "Home",
    subtitle: "Performance, tasks, and catalog health for the current storefront.",
    breadcrumbs: "Home",
    showPrimaryAction: false,
    showExport: false,
  };
}
