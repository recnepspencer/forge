export interface StorefrontProduct {
  id: string;
  name: string;
  status: "active" | "review";
  price: number;
  inventory: number;
}

export interface ServerPushPacket {
  packetId: string;
  basisId: string | null;
  nextBasisId: string;
  field: "price" | "inventory";
  value: number;
  previousValue: number;
}

const NETWORK_DELAY_MS = 650;

const SEED_PRODUCTS: readonly StorefrontProduct[] = Object.freeze([
  { id: "p-204", name: "Northstar Carry-On", status: "active", price: 184, inventory: 18 },
  { id: "p-381", name: "Garment sleeve", status: "active", price: 72, inventory: 41 },
  { id: "p-672", name: "Packing cubes", status: "review", price: 38, inventory: 7 },
]);

function wait(ms: number): Promise<void> {
  return new Promise((resolve) => window.setTimeout(resolve, ms));
}

export function createStorefrontServer() {
  const products = new Map(SEED_PRODUCTS.map((product) => [product.id, { ...product }]));
  let version = 1;
  let packetCounter = 0;

  function basisFor(revision: number): string {
    return `srv-v${revision}`;
  }

  return {
    async fetchProducts(): Promise<StorefrontProduct[]> {
      await wait(NETWORK_DELAY_MS);
      return [...products.values()].map((product) => ({ ...product }));
    },

    async fetchProduct(productId: string): Promise<StorefrontProduct> {
      await wait(NETWORK_DELAY_MS);
      const product = products.get(productId);
      if (!product) throw new Error(`Unknown product "${productId}".`);
      return { ...product };
    },

    /** Mutates server truth and returns the delivery packet describing the push. */
    pushChange(productId: string, field: "price" | "inventory"): ServerPushPacket {
      const product = products.get(productId);
      if (!product) throw new Error(`Unknown product "${productId}".`);
      const previousValue = product[field];
      const value = field === "price"
        ? previousValue + 4
        : Math.max(0, previousValue - 3);
      product[field] = value;
      const basisId = basisFor(version);
      version += 1;
      packetCounter += 1;
      return {
        packetId: `pkt-${String(packetCounter).padStart(2, "0")}`,
        basisId,
        nextBasisId: basisFor(version),
        field,
        value,
        previousValue,
      };
    },

    get version(): number {
      return version;
    },
  };
}

export type StorefrontServer = ReturnType<typeof createStorefrontServer>;

export interface LifecycleEntryView {
  sequence: number;
  event: string;
  statusKind: string;
  freshnessKind: string;
  headline: string;
  detail: string | null;
  raw: unknown;
}

interface RawLifecycleEntry {
  sequence?: number;
  event?: string;
  status?: { kind?: string };
  freshness?: { kind?: string };
  lastPatchedField?: string | null;
  lastDeliveryPacketId?: string | null;
  lastInvalidationCause?: string | null;
  lastOperation?: string | null;
}

export function describeLifecycleEntry(entry: unknown): LifecycleEntryView {
  const raw = (entry ?? {}) as RawLifecycleEntry;
  const event = raw.event ?? "unknown";
  const statusKind = raw.status?.kind ?? "unknown";
  const freshnessKind = raw.freshness?.kind ?? "unknown";

  let headline: string;
  let detail: string | null = null;

  switch (event) {
    case "materialized":
      headline = "line materialized";
      detail = "the recorder starts here — one line, one identity";
      break;
    case "pending":
      headline = "request in flight";
      detail = raw.lastOperation ? `operation: ${raw.lastOperation}` : "waiting on the server";
      break;
    case "requested":
      headline = "load requested";
      detail = "the line asked the server for its value";
      break;
    case "fulfilled":
      headline = "value settled";
      detail = "server truth landed and the line went fresh";
      break;
    case "delivered":
      headline = "server delivery landed";
      detail = raw.lastPatchedField
        ? `backend pushed a patch to “${raw.lastPatchedField}” (packet ${raw.lastDeliveryPacketId ?? "?"}) — no user action involved`
        : `backend delivery ${raw.lastDeliveryPacketId ?? ""}`.trim();
      break;
    case "invalidated":
      headline = "marked stale";
      detail = raw.lastInvalidationCause
        ? `cause: ${raw.lastInvalidationCause}`
        : "freshness dropped without rewriting the value";
      break;
    case "superseded":
      headline = "request superseded";
      detail = "a newer operation replaced the one in flight";
      break;
    default:
      headline = event;
      detail = raw.lastOperation ? `operation: ${raw.lastOperation}` : null;
  }

  return {
    sequence: raw.sequence ?? 0,
    event,
    statusKind,
    freshnessKind,
    headline,
    detail,
    raw: entry,
  };
}
