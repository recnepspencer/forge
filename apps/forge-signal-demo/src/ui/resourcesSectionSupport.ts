export interface ListItem {
  id: string;
  label: string;
  sync: "saved" | "syncing";
}

export interface SaveItemBody {
  id: string;
  outcome?: "confirm" | "reject";
}

export type SaveItemResponse = ListItem;

export interface PanelEvent {
  id: string;
  tone: "info" | "success" | "error";
  title: string;
  detail: string;
}

export interface ResourceDemoDiagnosticBlock {
  label: string;
  value: unknown;
  mode?: "json" | "rows";
}

export interface ResourceDemoDiagnostics {
  schema: "resource-demo-diagnostics.v1";
  runtime: "tanstack" | "forge";
  blocks: readonly ResourceDemoDiagnosticBlock[];
}

export const RESOURCE_DEMO_DIAGNOSTICS_JSON_SCHEMA = Object.freeze({
  $id: "resource-demo-diagnostics.v1",
  type: "object",
  required: ["schema", "runtime", "blocks"],
  properties: {
    schema: { const: "resource-demo-diagnostics.v1" },
    runtime: { enum: ["tanstack", "forge"] },
    blocks: {
      type: "array",
      items: {
        type: "object",
        required: ["label", "value"],
        properties: {
          label: { type: "string" },
          mode: { enum: ["json", "rows"] },
          value: {},
        },
      },
    },
  },
});

const API_BASE = "/api/resource-comparison";
const API_URLS = {
  items: `${API_BASE}/items.json`,
  saveItem: `${API_BASE}/items/:itemId`,
} as const;

const NETWORK_DELAY_MS = 300;
const NEXT_ITEM_ID = "item-packing-cubes";
const NEXT_ITEM_LABEL = "Packing cubes";
const SEED_ITEMS: readonly ListItem[] = Object.freeze([
  { id: "item-carry-on", label: "Northstar Carry-On", sync: "saved" },
  { id: "item-garment-sleeve", label: "Garment sleeve", sync: "saved" },
]);

function wait(ms: number): Promise<void> {
  return new Promise((resolve) => window.setTimeout(resolve, ms));
}

function cloneItem(item: ListItem): ListItem {
  return { ...item };
}

function cloneItems(items: readonly ListItem[]): ListItem[] {
  return items.map(cloneItem);
}

export function createComparisonStore() {
  let items = cloneItems(SEED_ITEMS);

  return {
    async fetchItems(): Promise<ListItem[]> {
      await wait(NETWORK_DELAY_MS);
      return cloneItems(items);
    },
    async confirmItem(id: string, outcome: SaveItemBody["outcome"] = "confirm"): Promise<SaveItemResponse> {
      await wait(NETWORK_DELAY_MS);
      if (outcome === "reject") {
        throw new Error(`Server rejected item "${id}".`);
      }
      const existing = items.find((item) => item.id === id);
      const saved = existing
        ? { ...existing, sync: "saved" as const }
        : { id, label: NEXT_ITEM_LABEL, sync: "saved" as const };
      items = existing
        ? items.map((item) => (item.id === id ? saved : item))
        : [...items, saved];
      return cloneItem(saved);
    },
    reset(): void {
      items = cloneItems(SEED_ITEMS);
    },
  };
}

export function createPanelEvent(
  tone: PanelEvent["tone"],
  title: string,
  detail: string,
): PanelEvent {
  return {
    id: `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
    tone,
    title,
    detail,
  };
}

function formatJsonBlock(label: string, value: unknown): string[] {
  return [`> ${label}`, ...JSON.stringify(value, null, 2).split("\n")];
}

export function formatItemRows(items: readonly ListItem[] | null): string[] {
  return items ? items.map((item) => `${item.id}: ${item.label} (${item.sync})`) : ["null"];
}

function formatTextBlock(label: string, lines: readonly string[]): string[] {
  return [`> ${label}`, ...lines];
}

export function renderResourceDemoDiagnostics(diagnostics: ResourceDemoDiagnostics | null): string[] {
  if (!diagnostics) return [];
  return diagnostics.blocks.flatMap((block, index) => [
    ...(index === 0 ? [] : [""]),
    ...(block.mode === "rows"
      ? formatTextBlock(block.label, block.value as readonly string[])
      : formatJsonBlock(block.label, block.value)),
  ]);
}

export function latestMutationSummary(latest: unknown, unavailableReason = "no mutation response admitted"): Record<string, unknown> {
  if (!latest || typeof latest !== "object") {
    return {
      status: "notAvailable",
      reason: unavailableReason,
    };
  }
  const record = latest as Record<string, unknown>;
  const outcomes = record.mutationResponseTargetOutcomes;
  const targets = Array.isArray(outcomes) ? outcomes.map((target) => {
    const row = target as Record<string, unknown>;
    return `${row.outcomeKind ?? "unknown"} ${row.executionKind ?? "target"} -> ${row.locus ?? row.canonicalKey ?? "unknown"}`;
  }) : [];
  return {
    status: record.mutationResponseConfirmationKind ? "available" : "notAvailable",
    confirmation: record.mutationResponseConfirmationKind ?? null,
    targets: targets.length ? targets : null,
    fallback: record.mutationResponseFallbackReasonDigest ?? null,
    declaredTargets: record.mutationResponseNoHiddenMutationDigest ? "all declared targets accounted for" : null,
    reason: record.mutationResponseConfirmationKind ? null : unavailableReason,
  };
}

export function summarizeRecovery(result: {
  recovery: { summary(): { severity: unknown; reason: unknown; recommendedFollowup: unknown } };
}): string {
  const { severity, reason, recommendedFollowup } = result.recovery.summary();
  return `${severity}: ${reason}; followup=${recommendedFollowup}`;
}

export const comparisonApiUrls = API_URLS;
export const optimisticItem = Object.freeze({
  id: NEXT_ITEM_ID,
  label: NEXT_ITEM_LABEL,
  sync: "syncing",
}) satisfies ListItem;
