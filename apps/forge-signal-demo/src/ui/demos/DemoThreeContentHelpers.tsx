import type { DemoProduct, DemoThreeData } from "./demoThreeRouterModel";

export function ownerLabel(data: DemoThreeData, roleId: string) {
  return data.roles[roleId as keyof typeof data.roles]?.label ?? "Unknown";
}

export function media(product: DemoProduct) {
  return (
    <div className="storefront-thumb">
      <img src={product.imageUrl} alt={product.name} />
    </div>
  );
}

export function statusTone(status: string) {
  return status.toLowerCase() === "active" ? "active" : "draft";
}

export function formatInventoryState(inventory: number, state: string) {
  return `${inventory} · ${state}`;
}

export function permissionHint(allowed: boolean, message: string) {
  return (
    <div className={`storefront-note ${allowed ? "" : "locked"}`}>
      <strong>{allowed ? "Allowed" : "Locked"}</strong>
      <span>{message}</span>
    </div>
  );
}

export function adminActionButton(
  label: string,
  onClick: () => void,
  allowed: boolean,
  title?: string | null,
) {
  return (
    <button
      className="storefront-button primary"
      disabled={!allowed}
      onClick={allowed ? onClick : undefined}
      title={title ?? undefined}
      type="button"
    >
      {label}
    </button>
  );
}

export function productForRoute(data: DemoThreeData, href: string) {
  const productId = href.match(/\/products\/([^/]+)/)?.[1];
  return data.products.find((entry) => entry.id === productId) ?? null;
}
