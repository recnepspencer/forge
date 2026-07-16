import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { QueryClient, QueryClientProvider, useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { createSignals } from "worth-signal-wasm";

import { useSignal } from "./Demos";
import { DxCorner } from "./DxCorner";
import { ResourcesSectionCodeSample } from "./ResourcesSectionCodeSample";
import {
  CallbackAftermath,
  ClaimTimeline,
  EffectLedger,
  PoPanel,
  RevealBanner,
} from "./ResourcesSectionParts";
import "./resourcesSection.css";
import {
  CONFIRM_SAFE_AFTER_MS,
  LINE_RISKY,
  LINE_SAFE,
  PO_NUMBER,
  REJECT_RISKY_AFTER_MS,
  SOLO_REJECT_AFTER_MS,
  claimSyncOf,
  createPanelEvent,
  createPoServer,
  summarizeEffectEnvelope,
  type ClaimEntry,
  type ClaimSync,
  type LedgerRow,
  type PanelEvent,
  type PanelVariant,
  type PoLine,
  type PoWriteBody,
  type ScenarioPhase,
} from "./resourcesSectionSupport";

const QueryProvider = QueryClientProvider as unknown as (props: {
  children?: React.ReactNode;
  client: QueryClient;
}) => React.ReactNode | Promise<React.ReactNode>;

type SignalsRuntime = Awaited<ReturnType<typeof createSignals>>;

interface PanelController {
  addLine(line: PoLine): void;
  settle(lineId: string, accepted: boolean): void;
  reset(): void | Promise<void>;
}

interface PanelProps {
  baseMs: number | null;
  highlightId: string | null;
  onClaim: (variant: PanelVariant, sync: ClaimSync) => void;
  onController: (controller: PanelController | null) => void;
}

function pushEvent(
  setter: (value: PanelEvent[] | ((current: PanelEvent[]) => PanelEvent[])) => void,
  event: PanelEvent,
): void {
  setter((current) => [event, ...current].slice(0, 4));
}

const DX_SAMPLE = `export function AddPoLine({ orderId, draft }: AddPoLineProps) {
  const write = useManagedResourceWrite({
    line: (body: PoLineDraft) =>
      saveLine.line({ orderId, lineId: body.id, body }),
    feedback: { success: "Line added", error: "The vendor check failed" },
    onFeedback: (feedback) => toast(feedback.title, feedback.description),
  });

  return (
    <button disabled={write.pending} onClick={() => void write.execute(draft)}>
      Add line
    </button>
  );
}`;

function TanStackPanel({ baseMs, highlightId, onClaim, onController }: PanelProps) {
  void baseMs;
  const store = useMemo(() => createPoServer(), []);
  const queryClient = useQueryClient();
  const [events, setEvents] = useState<PanelEvent[]>([]);

  const query = useQuery({
    queryKey: ["po", "lines"],
    queryFn: () => store.fetchLines(),
  });

  const mutation = useMutation({
    mutationFn: (line: PoLine) => store.save(line),
    onMutate: async (line) => {
      await queryClient.cancelQueries({ queryKey: ["po", "lines"] });
      const previous = queryClient.getQueryData<readonly PoLine[]>(["po", "lines"]) ?? [];
      queryClient.setQueryData(["po", "lines"], (current: readonly PoLine[] = []) => [
        ...current,
        { ...line, sync: "syncing" as const },
      ]);
      pushEvent(setEvents, createPanelEvent("info", `Adding ${line.label}…`, "onMutate snapshotted the cache and inserted the row."));
      return { previous };
    },
    onSuccess: (saved) => {
      queryClient.setQueryData(["po", "lines"], (current: readonly PoLine[] = []) =>
        current.map((line) => (line.id === saved.id ? saved : line)),
      );
      pushEvent(setEvents, createPanelEvent("success", `${saved.label} confirmed`, "onSuccess replaced the optimistic row."));
    },
    onError: (_error, line, context) => {
      queryClient.setQueryData(["po", "lines"], context?.previous ?? []);
      pushEvent(setEvents, createPanelEvent("error", `${line.label} failed`, "onError restored the snapshot taken in this mutation's onMutate."));
    },
    onSettled: () => {
      if (queryClient.isMutating() === 1) {
        void queryClient.invalidateQueries({ queryKey: ["po", "lines"] });
      }
    },
  });

  const lines = (query.data ?? null) as readonly PoLine[] | null;

  useEffect(() => {
    onClaim("tanstack", claimSyncOf(lines, LINE_SAFE.id));
  }, [lines, onClaim]);

  const controller = useMemo<PanelController>(() => ({
    addLine: (line) => {
      mutation.mutate({ ...line });
    },
    settle: (lineId, accepted) => {
      store.settle(lineId, accepted);
    },
    reset: async () => {
      store.reset();
      setEvents([]);
      await queryClient.resetQueries({ queryKey: ["po", "lines"] });
    },
  }), [mutation, queryClient, store]);

  useEffect(() => {
    onController(controller);
    return () => onController(null);
  }, [controller, onController]);

  return (
    <div className="po-column">
      <PoPanel
        caption='useQuery({ queryKey: ["po", "lines"] }) · onMutate / onError / onSettled'
        error={query.error instanceof Error ? query.error.message : null}
        events={events}
        highlightId={highlightId}
        lines={lines}
        loading={query.isLoading}
        refetching={query.isFetching && !query.isLoading}
        title="TanStack Query"
        variant="tanstack"
      />
      <CallbackAftermath cacheLines={lines} mutationStatus={mutation.status} />
    </div>
  );
}

function WORTHPanel({ baseMs, highlightId, onClaim, onController }: PanelProps) {
  const store = useMemo(() => createPoServer(), []);
  const [signals, setSignals] = useState<SignalsRuntime | null>(null);
  const [bootError, setBootError] = useState<string | null>(null);
  const [events, setEvents] = useState<PanelEvent[]>([]);
  const [ledger, setLedger] = useState<LedgerRow[]>([]);
  const [tick, setTick] = useState(0);
  const bootRef = useRef(false);
  const initRef = useRef(false);
  const attemptRef = useRef(0);
  const timersRef = useRef<number[]>([]);

  useEffect(() => {
    if (bootRef.current) return;
    bootRef.current = true;
    createSignals({ deployment: "mainThreadCompatibility" })
      .then(setSignals)
      .catch((error) => setBootError(error instanceof Error ? error.message : "Could not boot the Worth runtime."));
  }, []);

  useEffect(() => () => {
    timersRef.current.forEach((id) => window.clearTimeout(id));
  }, []);

  const pulse = useCallback(() => {
    [80, 320, 720, 1500].forEach((delay) => {
      timersRef.current.push(window.setTimeout(() => setTick((value) => value + 1), delay));
    });
  }, []);

  const pushLedger = useCallback((title: string, detail: string, payload: unknown) => {
    setLedger((current) => [
      ...current,
      {
        id: `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
        atMs: performance.now(),
        title,
        detail,
        payload,
      },
    ]);
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
      .list({
        load: () => store.fetchLines(),
      });
    const saveLine = api
      .url("/orders/:orderId/lines/:lineId")
      .response(signals.resource.response.detail<PoLine>()({ label: "label", qty: "qty", sync: "sync" }))
      .update({
        reconciles: [
          {
            family: linesFamily,
            params: () => ({ orderId: PO_NUMBER }),
            collection: { kind: "item" },
            fallback: "partialReconciliation",
          },
        ],
        load: ({ body }: { body: PoWriteBody }) => store.save(body),
      });
    return { linesFamily, line: linesFamily.line({ orderId: PO_NUMBER }), saveLine };
  }, [signals, store]);

  useEffect(() => {
    if (!WORTH || initRef.current) return;
    initRef.current = true;
    WORTH.line.refresh();
    void WORTH.line.awaitSettlement().then(() => {
      const value = WORTH.line.value() as readonly PoLine[] | null;
      pushLedger(
        "resident truth loaded",
        `initial load fulfilled — ${value?.length ?? 0} line(s) on ${PO_NUMBER}`,
        null,
      );
      pulse();
    });
  }, [WORTH, pulse, pushLedger]);

  const lineSignal = useMemo(() => (WORTH ? WORTH.line.signal() : null), [WORTH]);
  const watchedValue = useSignal<readonly PoLine[] | null>(signals as unknown, lineSignal);
  void tick;
  void watchedValue;
  const lines = WORTH ? ((WORTH.line.value() as readonly PoLine[] | null) ?? watchedValue ?? null) : null;
  const loading = !WORTH || (!lines && !bootError);

  useEffect(() => {
    onClaim("worth", claimSyncOf(lines, LINE_SAFE.id));
  }, [lines, onClaim]);

  const controller = useMemo<PanelController>(() => ({
    addLine: (poLine) => {
      if (!WORTH) return;
      const draft: PoLine = { ...poLine, sync: "syncing" };
      WORTH.line.patch(WORTH.linesFamily.patch.insert({
        itemId: draft.id,
        placement: "append",
        nextItem: draft,
      }));
      pushLedger(
        `optimistic insert admitted — ${draft.label}`,
        "localPatch on the current branch; the envelope records its own rollback posture",
        summarizeEffectEnvelope(WORTH.line.diagnostics().lastEffect),
      );
      pushEvent(setEvents, createPanelEvent("info", `Adding ${draft.label}…`, "the insert is an admitted effect, not a cache overwrite"));

      const execution = WORTH.saveLine.execute(
        { orderId: PO_NUMBER, lineId: draft.id, body: { ...draft, attempt: ++attemptRef.current } },
        { freeOnSettle: true },
      );
      void execution.settled().then((result: any) => {
        if (result?.resultKind === "rejected") {
          const message: string = result?.status?.message ?? "server rejected the write";
          pushEvent(setEvents, createPanelEvent("error", `${draft.label} failed`, message));
          pushLedger(
            `server rejected ${draft.label}`,
            `${message} — write freshness: ${result?.freshness?.reason ?? "n/a"}`,
            {
              resultKind: result?.resultKind ?? null,
              message,
              freshness: result?.freshness ?? null,
            },
          );
          WORTH.line.patch(WORTH.linesFamily.patch.delete({ itemId: draft.id }));
          pushLedger(
            `compensating patch removed ${draft.label}`,
            "item-scoped delete — no snapshot restore, nothing else on screen was touched",
            summarizeEffectEnvelope(WORTH.line.diagnostics().lastEffect),
          );
        } else {
          pushEvent(setEvents, createPanelEvent("success", `${draft.label} confirmed`, "the delivered patch replaced exactly one row"));
          pushLedger(
            `server confirmed ${draft.label}`,
            "delivered item patch consumed canonical server truth — other pending rows preserved",
            {
              mutationConfirmation: result?.mutationResponse?.confirmation?.kind ?? null,
              lastEffect: summarizeEffectEnvelope(WORTH.line.diagnostics().lastEffect),
            },
          );
        }
        pulse();
      }).catch(() => pulse());
      pulse();
    },
    settle: (lineId, accepted) => {
      store.settle(lineId, accepted);
    },
    reset: async () => {
      store.reset();
      setEvents([]);
      setLedger([]);
      if (!WORTH) return;
      WORTH.line.invalidate();
      WORTH.line.refresh();
      await WORTH.line.awaitSettlement().catch(() => null);
      const value = WORTH.line.value() as readonly PoLine[] | null;
      pushLedger("resident truth loaded", `reset — ${value?.length ?? 0} line(s) on ${PO_NUMBER}`, null);
      pulse();
    },
  }), [WORTH, pulse, pushLedger, store]);

  useEffect(() => {
    onController(controller);
    return () => onController(null);
  }, [controller, onController]);

  const exportLedger = useCallback(() => {
    if (!WORTH) return;
    let lifecycle: unknown = null;
    try {
      lifecycle = WORTH.line.history().lifecycle.map((entry: any) => ({
        event: entry.event,
        lastPatchKind: entry.lastPatchKind ?? null,
        lastPatchedItemId: entry.lastPatchedItemId ?? null,
      }));
    } catch {
      lifecycle = "lifecycle unavailable";
    }
    const artifact = {
      exportedAt: new Date().toISOString(),
      scenario: "po-overlapping-optimistic-writes",
      source: "line.diagnostics().lastEffect envelopes + line.history().lifecycle, read from the Worth runtime",
      ledger,
      lifecycle,
      visibleValue: WORTH.line.value(),
    };
    const blob = new Blob([JSON.stringify(artifact, null, 2)], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.download = "worth-effect-ledger.json";
    anchor.click();
    URL.revokeObjectURL(url);
  }, [WORTH, ledger]);

  return (
    <div className="po-column">
      <PoPanel
        caption="useResourceLine(poLines, { orderId }) · effects: branchNative()"
        error={bootError}
        events={events}
        highlightId={highlightId}
        lines={lines}
        loading={loading}
        title="Worth runtime"
        variant="worth"
      />
      <EffectLedger baseMs={baseMs} onExport={exportLedger} rows={ledger} />
    </div>
  );
}

interface ResourcesSectionProps {
  onNavigate: (path: string) => void;
}

export function ResourcesSection({ onNavigate }: ResourcesSectionProps) {
  const tanstackController = useRef<PanelController | null>(null);
  const WORTHController = useRef<PanelController | null>(null);
  const queryClient = useMemo(() => new QueryClient(), []);
  const timersRef = useRef<number[]>([]);
  const baseMsRef = useRef<number | null>(null);
  const phaseRef = useRef<ScenarioPhase>("idle");

  const [phase, setPhaseState] = useState<ScenarioPhase>("idle");
  const [baseMs, setBaseMs] = useState<number | null>(null);
  const [confirmedAtMs, setConfirmedAtMs] = useState<number | null>(null);
  const [healedAtMs, setHealedAtMs] = useState<number | null>(null);
  const [claims, setClaims] = useState<Record<PanelVariant, ClaimEntry[]>>({ tanstack: [], worth: [] });

  const setPhase = useCallback((next: ScenarioPhase) => {
    phaseRef.current = next;
    setPhaseState(next);
  }, []);

  useEffect(() => () => {
    timersRef.current.forEach((id) => window.clearTimeout(id));
  }, []);

  const schedule = useCallback((callback: () => void, delay: number) => {
    timersRef.current.push(window.setTimeout(callback, delay));
  }, []);

  const clearSchedules = useCallback(() => {
    timersRef.current.forEach((id) => window.clearTimeout(id));
    timersRef.current = [];
  }, []);

  const settleBoth = useCallback((lineId: string, accepted: boolean) => {
    tanstackController.current?.settle(lineId, accepted);
    WORTHController.current?.settle(lineId, accepted);
  }, []);

  const handleClaim = useCallback((variant: PanelVariant, sync: ClaimSync) => {
    if (baseMsRef.current === null) return;
    setClaims((current) => {
      const list = current[variant];
      if (list[list.length - 1]?.sync === sync) return current;
      return { ...current, [variant]: [...list, { atMs: performance.now(), sync }] };
    });
    if (variant === "tanstack" && sync === "synced" && phaseRef.current === "diverged") {
      setHealedAtMs(performance.now());
      phaseRef.current = "healed";
      setPhaseState("healed");
    }
  }, []);

  const startFirstLine = useCallback(() => {
    const now = performance.now();
    baseMsRef.current = now;
    setBaseMs(now);
    setClaims({ tanstack: [{ atMs: now, sync: "absent" }], worth: [{ atMs: now, sync: "absent" }] });
    tanstackController.current?.addLine(LINE_RISKY);
    WORTHController.current?.addLine(LINE_RISKY);
    setPhase("firstInFlight");
    schedule(() => {
      if (phaseRef.current !== "firstInFlight") return;
      settleBoth(LINE_RISKY.id, false);
      setPhase("settledSolo");
    }, SOLO_REJECT_AFTER_MS);
  }, [schedule, setPhase, settleBoth]);

  const startSecondLine = useCallback(() => {
    clearSchedules();
    tanstackController.current?.addLine(LINE_SAFE);
    WORTHController.current?.addLine(LINE_SAFE);
    setPhase("overlap");
    schedule(() => {
      settleBoth(LINE_SAFE.id, true);
      setConfirmedAtMs(performance.now());
      setPhase("confirmed");
    }, CONFIRM_SAFE_AFTER_MS);
    schedule(() => {
      settleBoth(LINE_RISKY.id, false);
      setPhase("diverged");
    }, REJECT_RISKY_AFTER_MS);
  }, [clearSchedules, schedule, setPhase, settleBoth]);

  const resetBoth = useCallback(() => {
    clearSchedules();
    baseMsRef.current = null;
    setBaseMs(null);
    setConfirmedAtMs(null);
    setHealedAtMs(null);
    setClaims({ tanstack: [], worth: [] });
    setPhase("idle");
    void tanstackController.current?.reset();
    void WORTHController.current?.reset();
  }, [clearSchedules, setPhase]);

  const highlightId = phase === "diverged" || phase === "healed" ? LINE_SAFE.id : null;

  return (
    <div className="accent-resources po-section">
      <p className="po-intro">
        Two people add lines to the same purchase order. Both writes are optimistic. The first one
        is going to fail — <em>after</em> the second one is confirmed. The left window is the
        callback model, written the way TanStack Query&apos;s own documentation recommends. The
        right window is the Worth runtime. Both get identical clicks and an identical server.
      </p>

      <div className="po-control-bar">
        <span>One scripted server. Both windows move together.</span>
        <div className="po-control-actions">
          <button
            className="po-control-button"
            disabled={phase !== "idle"}
            onClick={startFirstLine}
            type="button"
          >
            Add “Calibration kit”
          </button>
          <button
            className="po-control-button"
            disabled={phase !== "firstInFlight"}
            onClick={startSecondLine}
            type="button"
          >
            Add “Sterile tubing” — while the kit is still saving
          </button>
          <button
            className="po-control-button po-control-button-ghost"
            disabled={phase === "idle"}
            onClick={resetBoth}
            type="button"
          >
            Reset both
          </button>
        </div>
      </div>

      {phase === "settledSolo" ? (
        <aside className="po-solo-note" role="status">
          The vendor check failed, and one failed write at a time is the easy case — both sides
          rolled it back cleanly. Reset and add the tubing <strong>while the kit is still
          saving</strong>. That&apos;s where they stop agreeing.
        </aside>
      ) : null}

      <RevealBanner baseMs={baseMs} confirmedAtMs={confirmedAtMs} healedAtMs={healedAtMs} phase={phase} />

      <div className="po-grid">
        <QueryProvider client={queryClient}>
          <TanStackPanel
            baseMs={baseMs}
            highlightId={highlightId}
            onClaim={handleClaim}
            onController={(controller) => { tanstackController.current = controller; }}
          />
        </QueryProvider>
        <WORTHPanel
          baseMs={baseMs}
          highlightId={highlightId}
          onClaim={handleClaim}
          onController={(controller) => { WORTHController.current = controller; }}
        />
      </div>

      {baseMs !== null && phase !== "firstInFlight" && phase !== "settledSolo" ? (
        <ClaimTimeline baseMs={baseMs} claims={claims} confirmedAtMs={confirmedAtMs} />
      ) : null}

      <section className="signals-code-section" aria-labelledby="po-code-title">
        <h2 id="po-code-title">The write is a declaration — the receipts are a by-product</h2>
        <ResourcesSectionCodeSample
          liveLine={phase === "diverged" || phase === "healed"
            ? '// → optimistic.confirmation: "consumedCanonicalServerTruth" · one row replaced'
            : null}
        />
      </section>

      <DxCorner
        code={DX_SAMPLE}
        filename="add-po-line.tsx"
        receipts={[
          {
            claim: "Speculative vs confirmed is provenance, not vibes.",
            api: 'diagnostics().lastEffect.optimistic · "consumedCanonicalServerTruth"',
          },
          {
            claim: "Confirmations reconcile one item — they can't clobber the screen.",
            api: 'reconciles: [{ collection: { kind: "item" } }]',
          },
          {
            claim: "One hook owns the whole write lifecycle.",
            api: "useManagedResourceWrite({ line, feedback })",
          },
        ]}
        subtitle="This is the whole optimistic write — apply, confirm, reject, feedback. The snapshot bookkeeping you just watched go wrong on the left is not your job here."
      />

      <div className="signals-docs-row">
        <button onClick={() => onNavigate("#/docs/resources/index")} type="button">
          Explore resources in the documentation <span aria-hidden="true">→</span>
        </button>
      </div>
    </div>
  );
}
