export interface PoLine {
  id: string;
  label: string;
  qty: string;
  sync: "syncing" | "synced";
}

/** Write payload: each submission attempt is a distinct request. */
export interface PoWriteBody extends PoLine {
  attempt: number;
}

export type PanelVariant = "tanstack" | "worth";

export type ScenarioPhase =
  | "idle"
  | "firstInFlight"
  | "overlap"
  | "confirmed"
  | "diverged"
  | "healed"
  | "settledSolo";

export type ClaimSync = "absent" | "saving" | "synced";

export interface ClaimEntry {
  atMs: number;
  sync: ClaimSync;
}

export interface PanelEvent {
  id: string;
  tone: "info" | "success" | "error";
  title: string;
  detail: string;
}

export interface LedgerRow {
  id: string;
  atMs: number;
  title: string;
  detail: string;
  payload: unknown;
}

export const PO_NUMBER = "PO-1142";
export const PO_URL = "procure.worth.example/orders/PO-1142";

export const EXISTING_LINE: PoLine = Object.freeze({
  id: "line-071",
  label: "Nitrile gloves",
  qty: "40 cases",
  sync: "synced",
});

/** Added first; its vendor-qualification check is slow — and fails. */
export const LINE_RISKY: PoLine = Object.freeze({
  id: "line-072",
  label: "Calibration kit",
  qty: "3 units",
  sync: "syncing",
});

/** Added second, while the first is still saving; confirms quickly. */
export const LINE_SAFE: PoLine = Object.freeze({
  id: "line-073",
  label: "Sterile tubing",
  qty: "12 cases",
  sync: "syncing",
});

export const REJECT_MESSAGE = "Vendor is not qualified for calibrated equipment.";

export const FETCH_DELAY_MS = 1_200;
export const CONFIRM_SAFE_AFTER_MS = 1_400;
export const REJECT_RISKY_AFTER_MS = 3_000;
export const SOLO_REJECT_AFTER_MS = 5_500;

interface PendingSave {
  resolve: (line: PoLine) => void;
  reject: (reason: Error) => void;
  line: PoLine;
}

export interface PoServer {
  fetchLines(): Promise<PoLine[]>;
  save(line: PoLine): Promise<PoLine>;
  settle(lineId: string, accepted: boolean): void;
  reset(): void;
}

export function createPoServer(): PoServer {
  let lines: PoLine[] = [{ ...EXISTING_LINE }];
  const pending = new Map<string, PendingSave>();

  return {
    fetchLines() {
      return new Promise((resolve) => {
        window.setTimeout(() => resolve(lines.map((line) => ({ ...line }))), FETCH_DELAY_MS);
      });
    },
    save(line) {
      return new Promise((resolve, reject) => {
        pending.set(line.id, { resolve, reject, line });
      });
    },
    settle(lineId, accepted) {
      const entry = pending.get(lineId);
      pending.delete(lineId);
      if (!entry) return;
      if (accepted) {
        const saved: PoLine = {
          id: entry.line.id,
          label: entry.line.label,
          qty: entry.line.qty,
          sync: "synced",
        };
        lines = [...lines.filter((line) => line.id !== saved.id), saved];
        entry.resolve(saved);
      } else {
        entry.reject(new Error(REJECT_MESSAGE));
      }
    },
    reset() {
      lines = [{ ...EXISTING_LINE }];
      pending.clear();
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

export function claimSyncOf(lines: readonly PoLine[] | null | undefined, lineId: string): ClaimSync {
  const row = lines?.find((line) => line.id === lineId);
  if (!row) return "absent";
  return row.sync === "synced" ? "synced" : "saving";
}

export function formatOffset(atMs: number, baseMs: number | null): string {
  if (baseMs === null) return "—";
  return `t+${((atMs - baseMs) / 1000).toFixed(1)}s`;
}

interface EffectEnvelopeLike {
  effectId?: string;
  provenance?: string;
  profile?: { name?: string } | null;
  patch?: { kind?: string | null; scope?: string | null; itemId?: string | null } | null;
  optimistic?: {
    kind?: string;
    detail?: string;
    confirmation?: unknown;
    rollback?: { kind?: string } | null;
  } | null;
}

export function summarizeEffectEnvelope(effect: unknown): Record<string, unknown> | null {
  if (!effect || typeof effect !== "object") return null;
  const envelope = effect as EffectEnvelopeLike;
  const confirmation = envelope.optimistic?.confirmation;
  return {
    effectId: envelope.effectId ?? null,
    provenance: envelope.provenance ?? null,
    profile: envelope.profile?.name ?? null,
    patch: envelope.patch
      ? { kind: envelope.patch.kind ?? null, scope: envelope.patch.scope ?? null, itemId: envelope.patch.itemId ?? null }
      : null,
    optimistic: envelope.optimistic
      ? {
          kind: envelope.optimistic.kind ?? null,
          rollback: envelope.optimistic.rollback?.kind ?? null,
          ...(confirmation && typeof confirmation === "object"
            ? { confirmation }
            : {}),
        }
      : null,
  };
}
