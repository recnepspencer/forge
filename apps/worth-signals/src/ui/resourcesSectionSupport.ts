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
  | "arming"
  | "cliffhanger"
  | "diverged"
  | "healed"
  | "batchRunning"
  | "batchSettled";

export interface PanelEvent {
  id: string;
  tone: "info" | "success" | "error";
  title: string;
  detail: string;
}

interface RuntimeEffectReader {
  get(effectId: string): unknown;
  open(): readonly { readonly effectId: string }[];
  projection(): unknown;
  counters(): unknown;
}

export function createRuntimeReceipt(
  effects: RuntimeEffectReader,
  effectId: string,
  settlement: unknown = null,
): Readonly<Record<string, unknown>> {
  return Object.freeze({
    effect: effects.get(effectId),
    openEffectIds: Object.freeze(
      effects.open().map((entry) => entry.effectId),
    ),
    projection: effects.projection(),
    counters: effects.counters(),
    settlement,
  });
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

/** Depends on the risky request and is cancelled when that parent is rejected. */
export const LINE_DEPENDENT: PoLine = Object.freeze({
  id: "line-074",
  label: "Calibration certificate",
  qty: "3 records",
  sync: "syncing",
});

export const REJECT_MESSAGE = "Vendor is not qualified for calibrated equipment.";

export const FETCH_DELAY_MS = 1_200;
export const HEALING_REFETCH_DELAY_MS = 3_000;
export const BEAT_SIBLINGS_AT_MS = 1_000;
export const BEAT_CONFIRM_SAFE_AT_MS = 2_400;
export const BEAT_CLIFFHANGER_AT_MS = 3_400;

/** The referee: what the scripted server knows each record's status to be. */
export type ServerRecordStatus = "pending" | "confirmed" | "rejected" | "cancelled";

export interface ServerTruthRecord {
  readonly line: PoLine;
  readonly status: ServerRecordStatus;
  readonly atMs: number;
}

export type ServerTruth = readonly ServerTruthRecord[];

export function initialServerTruth(): ServerTruth {
  return [{ line: EXISTING_LINE, status: "confirmed", atMs: 0 }];
}

export function serverTruthAdmit(truth: ServerTruth, line: PoLine): ServerTruth {
  return [
    ...truth.filter((record) => record.line.id !== line.id),
    { line, status: "pending", atMs: performance.now() },
  ];
}

export function serverTruthSettle(truth: ServerTruth, lineId: string, accepted: boolean): ServerTruth {
  return truth.map((record) => (record.line.id === lineId
    ? { ...record, status: accepted ? "confirmed" as const : "rejected" as const, atMs: performance.now() }
    : record));
}

export function serverTruthCancel(truth: ServerTruth, lineId: string): ServerTruth {
  return truth.map((record) => (record.line.id === lineId
    ? { ...record, status: "cancelled" as const, atMs: performance.now() }
    : record));
}

export type AgreementKind = "matches" | "speculating" | "wrong";

export interface Agreement {
  readonly kind: AgreementKind;
  /** Server-confirmed records absent from the screen. */
  readonly missingLabels: readonly string[];
  /** Records on screen the server has rejected or cancelled. */
  readonly phantomLabels: readonly string[];
  readonly pendingCount: number;
}

export function computeAgreement(
  lines: readonly PoLine[] | null | undefined,
  truth: ServerTruth,
): Agreement | null {
  if (!lines) return null;
  const onScreen = new Set(lines.map((line) => line.id));
  const missingLabels = truth
    .filter((record) => record.status === "confirmed" && !onScreen.has(record.line.id))
    .map((record) => record.line.label);
  const statusById = new Map(truth.map((record) => [record.line.id, record.status]));
  const phantomLabels = lines
    .filter((line) => {
      const status = statusById.get(line.id);
      return status === "rejected" || status === "cancelled";
    })
    .map((line) => line.label);
  const pendingCount = lines.filter((line) => statusById.get(line.id) === "pending").length;
  if (missingLabels.length > 0 || phantomLabels.length > 0) {
    return { kind: "wrong", missingLabels, phantomLabels, pendingCount };
  }
  if (pendingCount > 0) {
    return { kind: "speculating", missingLabels, phantomLabels, pendingCount };
  }
  return { kind: "matches", missingLabels, phantomLabels, pendingCount };
}

export interface ConcurrentScenarioRequest {
  readonly line: PoLine;
  readonly accepted: boolean;
  readonly dependsOnLineId?: string;
}

export interface ConcurrentScenario {
  readonly seed: number;
  readonly requests: readonly ConcurrentScenarioRequest[];
  readonly settlementOrder: readonly string[];
}

export function buildTenRequestScenario(seed: number): ConcurrentScenario {
  const outcomes = [true, true, false, false, true, false, true, true, false, false];
  const dependencyIndexes = new Map([[1, 0], [3, 2], [7, 6], [9, 8]]);
  const requests = outcomes.map((accepted, index): ConcurrentScenarioRequest => {
    const line: PoLine = {
      id: `line-${80 + index}`,
      label: `Controlled material ${index + 1}`,
      qty: `${index + 1} lot${index === 0 ? "" : "s"}`,
      sync: "syncing",
    };
    const dependencyIndex = dependencyIndexes.get(index);
    return Object.freeze({
      line: Object.freeze(line),
      accepted,
      ...(dependencyIndex === undefined
        ? {}
        : { dependsOnLineId: `line-${80 + dependencyIndex}` }),
    });
  });
  const settlementOrder = seededShuffle(requests.map((request) => request.line.id), seed);
  return Object.freeze({
    seed,
    requests: Object.freeze(requests),
    settlementOrder: Object.freeze(settlementOrder),
  });
}

function seededShuffle<T>(values: readonly T[], seed: number): T[] {
  const shuffled = [...values];
  let state = seed >>> 0;
  for (let index = shuffled.length - 1; index > 0; index -= 1) {
    state = (Math.imul(state, 1_664_525) + 1_013_904_223) >>> 0;
    const target = state % (index + 1);
    [shuffled[index], shuffled[target]] = [shuffled[target], shuffled[index]];
  }
  return shuffled;
}

interface PendingSave {
  resolve: (line: PoLine) => void;
  reject: (reason: Error) => void;
  line: PoLine;
}

type PendingDecision = { kind: "settle"; accepted: boolean } | { kind: "cancel" };

export interface PoServer {
  fetchLines(delayMs?: number): Promise<PoLine[]>;
  save(line: PoLine): Promise<PoLine>;
  settle(lineId: string, accepted: boolean): void;
  cancel(lineId: string): void;
  reset(): void;
}

export function createPoServer(): PoServer {
  let lines: PoLine[] = [{ ...EXISTING_LINE }];
  const pending = new Map<string, PendingSave>();
  const decisions = new Map<string, PendingDecision>();

  const decide = (entry: PendingSave, decision: PendingDecision): void => {
    if (decision.kind === "cancel") {
      entry.reject(new Error("Request cancelled because its dependency was rejected."));
      return;
    }
    if (!decision.accepted) {
      entry.reject(new Error(REJECT_MESSAGE));
      return;
    }
    const saved: PoLine = {
      id: entry.line.id,
      label: entry.line.label,
      qty: entry.line.qty,
      sync: "synced",
    };
    lines = [...lines.filter((line) => line.id !== saved.id), saved];
    entry.resolve(saved);
  };

  return {
    fetchLines(delayMs = FETCH_DELAY_MS) {
      return new Promise((resolve) => {
        window.setTimeout(() => resolve(lines.map((line) => ({ ...line }))), delayMs);
      });
    },
    save(line) {
      return new Promise((resolve, reject) => {
        const entry = { resolve, reject, line };
        const decision = decisions.get(line.id);
        if (decision) {
          decisions.delete(line.id);
          decide(entry, decision);
        } else {
          pending.set(line.id, entry);
        }
      });
    },
    settle(lineId, accepted) {
      const entry = pending.get(lineId);
      pending.delete(lineId);
      if (entry) decide(entry, { kind: "settle", accepted });
      else decisions.set(lineId, { kind: "settle", accepted });
    },
    cancel(lineId) {
      const entry = pending.get(lineId);
      pending.delete(lineId);
      if (entry) decide(entry, { kind: "cancel" });
      else decisions.set(lineId, { kind: "cancel" });
    },
    reset() {
      lines = [{ ...EXISTING_LINE }];
      pending.clear();
      decisions.clear();
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

export function formatOffset(atMs: number, baseMs: number | null): string {
  if (baseMs === null) return "—";
  return `t+${((atMs - baseMs) / 1000).toFixed(1)}s`;
}
