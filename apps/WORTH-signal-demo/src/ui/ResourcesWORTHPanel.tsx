import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import {
  createSignals,
  resourcePatch,
  type ResourceEffectSettlementResult,
  type ResourcePatchExecutionResult,
} from "worth-signal-wasm";

import { useSignal } from "./Demos";
import { BranchDagStrip, type DagLaneDatum } from "./ResourcesModelStrips";
import {
  ConvergenceReceipt,
  PlatformOwner,
  PoPanel,
  type ConvergenceFacts,
} from "./ResourcesSectionParts";
import { pushPanelEvent, type PanelController, type PanelProps } from "./resourcesSectionPanels";
import {
  EXISTING_LINE,
  PO_NUMBER,
  computeAgreement,
  createPanelEvent,
  createPoServer,
  createRuntimeReceipt,
  type PanelEvent,
  type PoLine,
  type PoWriteBody,
} from "./resourcesSectionSupport";

type SignalsRuntime = Awaited<ReturnType<typeof createSignals>>;

interface EffectSummaryLike {
  readonly lifecycle: string;
  readonly branchId: number;
  readonly dependencyEffectIds: readonly string[];
  readonly terminal: { readonly kind: string } | null;
}

interface AdmittedEffect {
  readonly effectId: string;
  readonly lineId: string;
  readonly label: string;
  readonly admittedAtMs: number;
}

function admittedEffectId(result: ResourcePatchExecutionResult): string {
  if ("effectId" in result && typeof result.effectId === "string") {
    return result.effectId;
  }
  throw new TypeError("branch-native patch admission did not return an effect identity");
}

function retiredEffectIds(result: ResourceEffectSettlementResult): readonly string[] {
  if (!("retired" in result) || !Array.isArray(result.retired)) return [];
  return result.retired.flatMap((entry) => {
    const effectId = "effectId" in entry ? entry.effectId : null;
    return typeof effectId === "string" ? [effectId] : [];
  });
}

export function ResourcesWORTHPanel({
  baseMs,
  highlightId,
  onAgreement,
  onController,
  phase,
  serverTruth,
}: PanelProps) {
  const store = useMemo(() => createPoServer(), []);
  const [signals, setSignals] = useState<SignalsRuntime | null>(null);
  const [bootError, setBootError] = useState<string | null>(null);
  const [events, setEvents] = useState<PanelEvent[]>([]);
  const [dagLanes, setDagLanes] = useState<readonly DagLaneDatum[]>([]);
  const [settlementSnapshot, setSettlementSnapshot] = useState<ReadonlyMap<string, unknown>>(
    new Map(),
  );
  const [selectedEffectId, setSelectedEffectId] = useState<string | null>(null);
  const attemptRef = useRef(0);
  const admissionByLine = useRef(new Map<string, Promise<string>>());
  const admittedEffects = useRef<AdmittedEffect[]>([]);
  const settledAtByEffect = useRef(new Map<string, number>());
  const lastSettlementByEffect = useRef(new Map<string, unknown>());

  useEffect(() => {
    let active = true;
    let runtime: SignalsRuntime | null = null;
    void createSignals().then((created) => {
      runtime = created;
      if (active) setSignals(created);
      else void created.terminate();
    }).catch((error) => {
      if (active) setBootError(error instanceof Error ? error.message : "Could not boot the Worth runtime.");
    });
    return () => {
      active = false;
      if (runtime) void runtime.terminate();
    };
  }, []);

  const WORTH = useMemo(() => {
    if (!signals) return null;
    const api = signals.api({
      baseUrl: "/api/procurement",
      effects: signals.resource.effects.branchNative(),
    });
    const linesFamily = api
      .url("/orders/:orderId/lines")
      .response(signals.resource.response.array({ itemId: (line: PoLine) => line.id }))
      .list({ load: () => store.fetchLines() });
    return { linesFamily, line: linesFamily.line({ orderId: PO_NUMBER }) };
  }, [signals, store]);

  /** Snapshot runtime-issued branch truth for React rendering. */
  const bumpDag = useCallback(() => {
    if (!WORTH) return;
    for (const entry of admittedEffects.current) {
      if (settledAtByEffect.current.has(entry.effectId)) continue;
      const summary = WORTH.line.effects().get(entry.effectId) as EffectSummaryLike | null;
      if (summary?.terminal) settledAtByEffect.current.set(entry.effectId, performance.now());
    }
    setDagLanes(admittedEffects.current.flatMap((entry) => {
      const summary = WORTH.line.effects().get(entry.effectId) as EffectSummaryLike | null;
      if (!summary) return [];
      return [{
        effectId: entry.effectId,
        label: entry.label,
        admittedAtMs: entry.admittedAtMs,
        settledAtMs: settledAtByEffect.current.get(entry.effectId) ?? null,
        parentEffectId: summary.dependencyEffectIds[0] ?? null,
        lifecycle: summary.lifecycle,
        terminal: summary.terminal?.kind ?? null,
        branchId: summary.branchId,
      }];
    }));
    setSettlementSnapshot(new Map(lastSettlementByEffect.current));
  }, [WORTH]);

  useEffect(() => {
    if (!WORTH) return;
    void WORTH.line.awaitSettlement().then(() => bumpDag());
  }, [WORTH, bumpDag]);

  const lineSignal = useMemo(() => (WORTH ? WORTH.line.signal() : null), [WORTH]);
  const watchedValue = useSignal<readonly PoLine[] | null>(signals as unknown, lineSignal);
  const lines = WORTH ? ((WORTH.line.value() as readonly PoLine[] | null) ?? watchedValue ?? null) : null;
  const loading = !WORTH || (!lines && !bootError);

  const agreement = useMemo(() => computeAgreement(lines, serverTruth), [lines, serverTruth]);

  useEffect(() => {
    onAgreement("worth", agreement, "live");
  }, [agreement, onAgreement]);

  const selectedReceipt = useMemo(() => {
    if (!WORTH || !selectedEffectId) return null;
    return createRuntimeReceipt(
      WORTH.line.effects(),
      selectedEffectId,
      settlementSnapshot.get(selectedEffectId) ?? null,
    );
  }, [WORTH, selectedEffectId, settlementSnapshot]);

  const cancelRetiredDependents = useCallback((
    result: ResourceEffectSettlementResult,
    settlingEffectId: string,
  ) => {
    for (const effectId of retiredEffectIds(result)) {
      if (effectId === settlingEffectId) continue;
      lastSettlementByEffect.current.set(effectId, result);
      const lineId = admittedEffects.current.find((entry) => entry.effectId === effectId)?.lineId;
      if (lineId) store.cancel(lineId);
    }
  }, [store]);

  const persistEffect = useCallback(async (draft: PoLine, effectId: string) => {
    if (!WORTH) return;
    try {
      const saved = await store.save({ ...draft, attempt: ++attemptRef.current } as PoWriteBody);
      const settlement = await WORTH.line.effects().confirm(effectId, {
        responseId: `demo:${draft.id}:confirmed`,
        serverPatch: WORTH.linesFamily.patch.insert({
          itemId: saved.id,
          placement: "append",
          nextItem: saved,
        }),
      });
      lastSettlementByEffect.current.set(effectId, settlement);
      cancelRetiredDependents(settlement, effectId);
      bumpDag();
      const waiting = settlement.kind === "responseRecorded";
      pushPanelEvent(setEvents, createPanelEvent(
        "success",
        `${draft.label} ${waiting ? "accepted" : "confirmed"}`,
        waiting ? "response recorded; canonical closeout waits for its dependencies" : "the effect branch merged and retired",
      ));
    } catch (error) {
      const terminal = WORTH.line.effects().get(effectId)?.terminal;
      if (terminal?.kind === "dependencyCancelled") {
        bumpDag();
        pushPanelEvent(setEvents, createPanelEvent(
          "error",
          `${draft.label} cancelled`,
          "its dependency was rejected; the runtime retired the dependent branch",
        ));
        return;
      }
      const settlement = await WORTH.line.effects().reject(effectId, {
        responseId: `demo:${draft.id}:rejected`,
      });
      lastSettlementByEffect.current.set(effectId, settlement);
      cancelRetiredDependents(settlement, effectId);
      bumpDag();
      const message = error instanceof Error ? error.message : "server rejected the write";
      pushPanelEvent(setEvents, createPanelEvent("error", `${draft.label} failed`, message));
    }
  }, [bumpDag, cancelRetiredDependents, WORTH, store]);

  const controller = useMemo<PanelController>(() => ({
    addLine: (poLine, options = {}) => {
      if (!WORTH) return;
      const draft: PoLine = { ...poLine, sync: "syncing" };
      const admission = (async () => {
        const parentId = options.dependsOnLineId
          ? await admissionByLine.current.get(options.dependsOnLineId)
          : null;
        if (options.dependsOnLineId && !parentId) {
          throw new TypeError(`missing dependency admission for ${options.dependsOnLineId}`);
        }
        const insert = WORTH.linesFamily.patch.insert({
          itemId: draft.id,
          placement: "append",
          nextItem: draft,
        });
        const patch = parentId
          ? resourcePatch.dependsOn(insert, [parentId])
          : insert;
        const result = await WORTH.line.patch(patch, {
          idempotencyKey: `demo:${draft.id}:${++attemptRef.current}`,
        });
        const effectId = admittedEffectId(result);
        admittedEffects.current.push({
          effectId,
          lineId: draft.id,
          label: draft.label,
          admittedAtMs: performance.now(),
        });
        bumpDag();
        pushPanelEvent(setEvents, createPanelEvent(
          "info",
          `Adding ${draft.label}â€¦`,
          parentId ? "admitted on a derived dependency basis" : "admitted on its own effect branch",
        ));
        return effectId;
      })();
      admissionByLine.current.set(draft.id, admission);
      void admission.then((effectId) => persistEffect(draft, effectId)).catch((error) => {
        pushPanelEvent(setEvents, createPanelEvent(
          "error",
          `${draft.label} was not admitted`,
          error instanceof Error ? error.message : "unknown admission failure",
        ));
      });
      return admission.then(() => undefined);
    },
    settle: (lineId, accepted) => store.settle(lineId, accepted),
    reset: async () => {
      if (WORTH) {
        const openEffects = [...WORTH.line.effects().open()].reverse();
        for (const effect of openEffects) {
          if (WORTH.line.effects().get(effect.effectId)?.lifecycle === "Retired") continue;
          await WORTH.line.effects().reject(effect.effectId, {
            responseId: `demo:reset:${effect.effectId}`,
          });
        }
      }
      store.reset();
      admissionByLine.current.clear();
      admittedEffects.current = [];
      settledAtByEffect.current.clear();
      lastSettlementByEffect.current.clear();
      setDagLanes([]);
      setSettlementSnapshot(new Map());
      setEvents([]);
      setSelectedEffectId(null);
      if (!WORTH) return;
      WORTH.line.invalidate();
      WORTH.line.refresh();
      const settlement = await WORTH.line.awaitSettlement({ timeoutMs: 5_000 });
      if (settlement.resultKind !== "fulfilled") {
        throw new TypeError(`Worth reset reload ended as ${settlement.resultKind}`);
      }
      const value = WORTH.line.value() as readonly PoLine[] | null;
      if (value?.length !== 1 || value[0]?.id !== EXISTING_LINE.id) {
        throw new TypeError("Worth reset reload did not restore the scripted server baseline");
      }
    },
  }), [bumpDag, WORTH, persistEffect, store]);

  useEffect(() => {
    onController(controller);
    return () => onController(null);
  }, [controller, onController]);

  const convergence = useMemo<ConvergenceFacts | null>(() => {
    if (phase !== "batchSettled" || !WORTH) return null;
    const counters = WORTH.line.effects().counters() as { openEffectCount?: number };
    return {
      matchesServer: agreement?.kind === "matches",
      openEffectCount: counters.openEffectCount ?? 0,
      mergedCount: dagLanes.filter((lane) => lane.terminal === "merged").length,
      rejectedCount: dagLanes.filter((lane) => lane.terminal === "rejectedAndRetired").length,
      cancelledCount: dagLanes.filter((lane) => lane.terminal === "dependencyCancelled").length,
    };
  }, [agreement?.kind, dagLanes, WORTH, phase]);

  const exportReceipts = useCallback(() => {
    if (!WORTH) return;
    const artifact = {
      exportedAt: new Date().toISOString(),
      scenario: "po-concurrent-effect-branches",
      receipts: admittedEffects.current.map((entry) => ({
        effectId: entry.effectId,
        lineId: entry.lineId,
        receipt: createRuntimeReceipt(
          WORTH.line.effects(),
          entry.effectId,
          lastSettlementByEffect.current.get(entry.effectId) ?? null,
        ),
      })),
      projection: WORTH.line.effects().projection(),
      counters: WORTH.line.effects().counters(),
      visibleValue: WORTH.line.value(),
    };
    const url = URL.createObjectURL(new Blob([JSON.stringify(artifact, null, 2)], { type: "application/json" }));
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.download = "worth-effect-receipts.json";
    anchor.click();
    URL.revokeObjectURL(url);
  }, [WORTH]);

  const live = phase === "arming" || phase === "diverged" || phase === "batchRunning";
  const selectedLabel = dagLanes.find((entry) => entry.effectId === selectedEffectId)?.label ?? null;

  return (
    <div className="po-column">
      <PlatformOwner
        description="One effect branch per write Â· isolated closeout Â· runtime receipts"
        title="Worth Signals"
        variant="worth"
      />
      <PoPanel
        agreement={agreement}
        caption="worker-first Â· branchNative() Â· line.effects().confirm / reject"
        error={bootError}
        events={events}
        highlightId={highlightId}
        lines={lines}
        loading={loading}
        serverTruth={serverTruth}
        title="Worth Signals"
        variant="worth"
      />
      <BranchDagStrip
        baseMs={baseMs}
        lanes={dagLanes}
        live={live}
        onSelect={(effectId) => setSelectedEffectId((current) => (current === effectId ? null : effectId))}
        selectedId={selectedEffectId}
      />
      {selectedReceipt ? (
        <details className="po-receipt" open>
          <summary>
            runtime receipt â€” {selectedLabel ?? selectedEffectId}
            <button className="po-receipt-export" onClick={exportReceipts} type="button">Export all (JSON)</button>
          </summary>
          <pre>{JSON.stringify(selectedReceipt, null, 2)}</pre>
        </details>
      ) : null}
      {convergence ? <ConvergenceReceipt facts={convergence} /> : null}
    </div>
  );
}
