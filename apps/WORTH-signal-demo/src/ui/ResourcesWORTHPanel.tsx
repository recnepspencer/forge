import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import {
  resourcePatch,
  type ResourceEffectSettlementResult,
  type ResourcePatchExecutionResult,
} from "worth-signals-wasm";

import { createDemoSignals } from "../platform/createDemoSignals";

import { useSignal } from "./Demos";
import { ResourcesOutcomeActivity, type EffectOutcome } from "./ResourcesOutcomeActivity";
import {
  ConvergenceReceipt,
  MedicalInventoryHeader,
  MedicalInventorySidebar,
  PoPanel,
  type ConvergenceFacts,
} from "./ResourcesSectionParts";
import { pushPanelEvent, type PanelController, type PanelProps } from "./resourcesSectionPanels";
import {
  PO_NUMBER,
  computeAgreement,
  createPanelEvent,
  createPoServer,
  createRuntimeReceipt,
  type PanelEvent,
  type PoLine,
  type PoWriteBody,
} from "./resourcesSectionSupport";

type SignalsRuntime = Awaited<ReturnType<typeof createDemoSignals>>;

interface EffectSummaryLike {
  readonly lifecycle: string;
  readonly terminal: { readonly kind: string } | null;
}

interface AdmittedEffect {
  readonly effectId: string;
  readonly lineId: string;
  readonly label: string;
}

function admittedEffectId(result: ResourcePatchExecutionResult): string {
  if ("effectId" in result && typeof result.effectId === "string") {
    return result.effectId;
  }
  throw new TypeError("effect admission did not return an effect identity");
}

function retiredEffectIds(result: ResourceEffectSettlementResult): readonly string[] {
  if (!("retired" in result) || !Array.isArray(result.retired)) return [];
  return result.retired.flatMap((entry) => {
    const effectId = "effectId" in entry ? entry.effectId : null;
    return typeof effectId === "string" ? [effectId] : [];
  });
}

export function ResourcesWORTHPanel({
  highlightId,
  onController,
  phase,
  serverTruth,
}: PanelProps) {
  const store = useMemo(() => createPoServer(), []);
  const [signals, setSignals] = useState<SignalsRuntime | null>(null);
  const [runtimeCycle, setRuntimeCycle] = useState(0);
  const [bootError, setBootError] = useState<string | null>(null);
  const [events, setEvents] = useState<PanelEvent[]>([]);
  const [outcomes, setOutcomes] = useState<readonly EffectOutcome[]>([]);
  const [settlementSnapshot, setSettlementSnapshot] = useState<ReadonlyMap<string, unknown>>(
    new Map(),
  );
  const [selectedEffectId, setSelectedEffectId] = useState<string | null>(null);
  const attemptRef = useRef(0);
  const admissionByLine = useRef(new Map<string, Promise<string>>());
  const persistenceByLine = useRef(new Map<string, Promise<void>>());
  const dependencyByLine = useRef(new Map<string, string>());
  const admittedEffects = useRef<AdmittedEffect[]>([]);
  const lastSettlementByEffect = useRef(new Map<string, unknown>());

  useEffect(() => {
    let active = true;
    let runtime: SignalsRuntime | null = null;
    void createDemoSignals().then((created) => {
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
  }, [runtimeCycle]);

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

  /** Snapshot runtime-issued effect truth for React rendering. */
  const refreshOutcomes = useCallback(() => {
    if (!WORTH) return;
    setOutcomes(admittedEffects.current.flatMap((entry) => {
      const summary = WORTH.line.effects().get(entry.effectId) as EffectSummaryLike | null;
      if (!summary) return [];
      return [{
        effectId: entry.effectId,
        label: entry.label,
        lifecycle: summary.lifecycle,
        terminal: summary.terminal?.kind ?? null,
      }];
    }));
    setSettlementSnapshot(new Map(lastSettlementByEffect.current));
  }, [WORTH]);

  useEffect(() => {
    if (!WORTH) return;
    void WORTH.line.awaitSettlement().then(() => refreshOutcomes());
  }, [WORTH, refreshOutcomes]);

  const lineSignal = useMemo(() => (WORTH ? WORTH.line.signal() : null), [WORTH]);
  const watchedValue = useSignal<readonly PoLine[] | null>(signals as unknown, lineSignal);
  const lines = WORTH ? ((WORTH.line.value() as readonly PoLine[] | null) ?? watchedValue ?? null) : null;
  const loading = !WORTH || (!lines && !bootError);

  const agreement = useMemo(() => computeAgreement(lines, serverTruth), [lines, serverTruth]);

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
      refreshOutcomes();
      const waiting = settlement.kind === "responseRecorded";
      pushPanelEvent(setEvents, createPanelEvent(
        "success",
        `${draft.label} ${waiting ? "accepted" : "confirmed"}`,
        waiting ? "response recorded; closeout waits for its required item" : "the accepted request reconciled and closed",
      ));
    } catch (error) {
      const terminal = WORTH.line.effects().get(effectId)?.terminal;
      if (terminal?.kind === "dependencyCancelled") {
        refreshOutcomes();
        pushPanelEvent(setEvents, createPanelEvent(
          "error",
          `${draft.label} cancelled`,
          "its required controlled material was rejected; the runtime cancelled this related request",
        ));
        return;
      }
      const settlement = await WORTH.line.effects().reject(effectId, {
        responseId: `demo:${draft.id}:rejected`,
      });
      lastSettlementByEffect.current.set(effectId, settlement);
      cancelRetiredDependents(settlement, effectId);
      refreshOutcomes();
      const message = error instanceof Error ? error.message : "server rejected the write";
      pushPanelEvent(setEvents, createPanelEvent("error", `${draft.label} failed`, message));
    }
  }, [cancelRetiredDependents, refreshOutcomes, WORTH, store]);

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
        });
        refreshOutcomes();
        pushPanelEvent(setEvents, createPanelEvent(
          "info",
          `Adding ${draft.label}...`,
          parentId ? "linked to its required inventory request" : "tracked as an independent inventory request",
        ));
        return effectId;
      })();
      admissionByLine.current.set(draft.id, admission);
      if (options.dependsOnLineId) {
        dependencyByLine.current.set(draft.id, options.dependsOnLineId);
      }
      const persistence = admission.then((effectId) => persistEffect(draft, effectId)).catch((error) => {
        pushPanelEvent(setEvents, createPanelEvent(
          "error",
          `${draft.label} was not admitted`,
          error instanceof Error ? error.message : "unknown admission failure",
        ));
      });
      persistenceByLine.current.set(draft.id, persistence);
      return admission.then(() => undefined);
    },
    settle: async (lineId, accepted) => {
      store.settle(lineId, accepted);
      await persistenceByLine.current.get(lineId);
      const dependentWrites = [...dependencyByLine.current.entries()]
        .filter(([, parentLineId]) => parentLineId === lineId)
        .map(([dependentLineId]) => persistenceByLine.current.get(dependentLineId));
      await Promise.all(dependentWrites);
    },
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
      persistenceByLine.current.clear();
      dependencyByLine.current.clear();
      admittedEffects.current = [];
      lastSettlementByEffect.current.clear();
      setOutcomes([]);
      setSettlementSnapshot(new Map());
      setEvents([]);
      setSelectedEffectId(null);
      setBootError(null);
      setSignals(null);
      setRuntimeCycle((current) => current + 1);
    },
  }), [refreshOutcomes, WORTH, persistEffect, store]);

  useEffect(() => {
    onController(WORTH ? controller : null);
    return () => onController(null);
  }, [controller, onController, WORTH]);

  const convergence = useMemo<ConvergenceFacts | null>(() => {
    if (phase !== "settled" || !WORTH) return null;
    const counters = WORTH.line.effects().counters() as { openEffectCount?: number };
    return {
      matchesServer: agreement?.kind === "matches",
      openEffectCount: counters.openEffectCount ?? 0,
      mergedCount: outcomes.filter((outcome) => outcome.terminal === "merged").length,
      rejectedCount: outcomes.filter((outcome) => outcome.terminal === "rejectedAndRetired").length,
      cancelledCount: outcomes.filter((outcome) => outcome.terminal === "dependencyCancelled").length,
    };
  }, [agreement?.kind, outcomes, WORTH, phase]);

  const exportReceipts = useCallback(() => {
    if (!WORTH) return;
    const artifact = {
      exportedAt: new Date().toISOString(),
      scenario: "medical-inventory-concurrent-approvals",
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

  const selectedLabel = outcomes.find((entry) => entry.effectId === selectedEffectId)?.label ?? null;

  return (
    <div className="po-column">
      <MedicalInventoryHeader />
      <MedicalInventorySidebar />
      <PoPanel
        agreement={agreement}
        error={bootError}
        events={events}
        highlightId={highlightId}
        lines={lines}
        loading={loading}
        serverTruth={serverTruth}
      />
      <ResourcesOutcomeActivity
        outcomes={outcomes}
        onSelect={(effectId) => setSelectedEffectId((current) => (current === effectId ? null : effectId))}
        selectedId={selectedEffectId}
      />
      {selectedReceipt ? (
        <details className="po-receipt" open>
          <summary>
            Audit receipt: {selectedLabel ?? selectedEffectId}
            <button className="po-receipt-export" onClick={exportReceipts} type="button">Export audit JSON</button>
          </summary>
          <pre>{JSON.stringify(selectedReceipt, null, 2)}</pre>
        </details>
      ) : null}
      {convergence ? <ConvergenceReceipt facts={convergence} /> : null}
    </div>
  );
}
