import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { DxCorner } from "./DxCorner";
import { ResourcesWORTHPanel } from "./ResourcesWORTHPanel";
import { ResourcesSectionCodeSample } from "./ResourcesSectionCodeSample";
import { CliffhangerCard, ServerTruthStrip, VerdictLine } from "./ResourcesSectionParts";
import { ResourcesTanStackPanel } from "./ResourcesTanStackPanel";
import {
  type AgreementEvidence,
  type PanelController,
} from "./resourcesSectionPanels";
import "./resourcesSection.css";
import "./resourcesPanel.css";
import "./resourcesModelStrips.css";
import {
  BEAT_CLIFFHANGER_AT_MS,
  BEAT_CONFIRM_SAFE_AT_MS,
  BEAT_SIBLINGS_AT_MS,
  LINE_DEPENDENT,
  LINE_RISKY,
  LINE_SAFE,
  buildTenRequestScenario,
  initialServerTruth,
  serverTruthAdmit,
  serverTruthCancel,
  serverTruthSettle,
  type Agreement,
  type PanelVariant,
  type ScenarioPhase,
  type ServerTruth,
} from "./resourcesSectionSupport";

const QueryProvider = QueryClientProvider as unknown as (props: {
  children?: React.ReactNode;
  client: QueryClient;
}) => React.ReactNode | Promise<React.ReactNode>;

const DX_SAMPLE = `const admission = await line.patch(
  resourcePatch.dependsOn(insertLine(draft), parentEffectIds),
);

try {
  const saved = await saveLine(draft);
  await line.effects().confirm(admission.effectId, {
    responseId: saved.requestId,
    serverPatch: insertLine(saved.line),
  });
} catch (response) {
  await line.effects().reject(admission.effectId, {
    responseId: response.requestId,
  });
}`;

interface ResourcesSectionProps {
  onNavigate: (path: string) => void;
}

export function ResourcesSection({ onNavigate }: ResourcesSectionProps) {
  const tanstackController = useRef<PanelController | null>(null);
  const WORTHController = useRef<PanelController | null>(null);
  const queryClient = useMemo(() => new QueryClient({
    defaultOptions: { queries: { refetchOnWindowFocus: false } },
  }), []);
  const timersRef = useRef<number[]>([]);
  const phaseRef = useRef<ScenarioPhase>("idle");

  const [phase, setPhaseState] = useState<ScenarioPhase>("idle");
  const [baseMs, setBaseMs] = useState<number | null>(null);
  const [serverTruth, setServerTruth] = useState<ServerTruth>(initialServerTruth());
  const [seed, setSeed] = useState(7_031);

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

  const handleAgreement = useCallback((
    variant: PanelVariant,
    agreement: Agreement | null,
    evidence: AgreementEvidence,
  ) => {
    if (variant !== "tanstack") return;
    if (
      phaseRef.current === "diverged"
      && evidence === "refetchCompleted"
      && agreement?.kind === "matches"
    ) {
      phaseRef.current = "healed";
      setPhaseState("healed");
    }
  }, []);

  const playCollision = useCallback(() => {
    clearSchedules();
    setBaseMs(performance.now());
    setServerTruth(serverTruthAdmit(initialServerTruth(), LINE_RISKY));
    void tanstackController.current?.addLine(LINE_RISKY);
    void WORTHController.current?.addLine(LINE_RISKY);
    setPhase("arming");
    schedule(() => {
      setServerTruth((truth) => serverTruthAdmit(serverTruthAdmit(truth, LINE_SAFE), LINE_DEPENDENT));
      void tanstackController.current?.addLine(LINE_SAFE);
      void WORTHController.current?.addLine(LINE_SAFE);
      const dependency = { dependsOnLineId: LINE_RISKY.id };
      void tanstackController.current?.addLine(LINE_DEPENDENT, dependency);
      void WORTHController.current?.addLine(LINE_DEPENDENT, dependency);
    }, BEAT_SIBLINGS_AT_MS);
    schedule(() => {
      setServerTruth((truth) => serverTruthSettle(truth, LINE_SAFE.id, true));
      settleBoth(LINE_SAFE.id, true);
    }, BEAT_CONFIRM_SAFE_AT_MS);
    schedule(() => {
      setPhase("cliffhanger");
    }, BEAT_CLIFFHANGER_AT_MS);
  }, [clearSchedules, schedule, setPhase, settleBoth]);

  const deliverRejection = useCallback(() => {
    setServerTruth((truth) => serverTruthCancel(
      serverTruthSettle(truth, LINE_RISKY.id, false),
      LINE_DEPENDENT.id,
    ));
    settleBoth(LINE_RISKY.id, false);
    setPhase("diverged");
  }, [setPhase, settleBoth]);

  const resetBoth = useCallback(async () => {
    clearSchedules();
    setBaseMs(null);
    setServerTruth(initialServerTruth());
    await Promise.all([
      tanstackController.current?.reset(),
      WORTHController.current?.reset(),
    ]);
    setPhase("idle");
  }, [clearSchedules, setPhase]);

  const runTenRequests = useCallback(async () => {
    clearSchedules();
    await Promise.all([
      tanstackController.current?.reset(),
      WORTHController.current?.reset(),
    ]);
    const scenario = buildTenRequestScenario(seed);
    setBaseMs(performance.now());
    setServerTruth(scenario.requests.reduce(
      (truth, request) => serverTruthAdmit(truth, request.line),
      initialServerTruth(),
    ));
    setPhase("batchRunning");
    const admissions: Array<void | Promise<void>> = [];
    for (const request of scenario.requests) {
      const options = request.dependsOnLineId
        ? { dependsOnLineId: request.dependsOnLineId }
        : undefined;
      admissions.push(
        tanstackController.current?.addLine(request.line, options),
        WORTHController.current?.addLine(request.line, options),
      );
    }
    await Promise.all(admissions);
    const outcomeByLine = new Map(scenario.requests.map((request) => [request.line.id, request.accepted]));
    const dependentsOf = (lineId: string) => scenario.requests
      .filter((request) => request.dependsOnLineId === lineId)
      .map((request) => request.line.id);
    scenario.settlementOrder.forEach((lineId, index) => {
      schedule(() => {
        const accepted = outcomeByLine.get(lineId) ?? false;
        setServerTruth((truth) => {
          const status = truth.find((record) => record.line.id === lineId)?.status;
          if (status !== "pending") return truth;
          let next = serverTruthSettle(truth, lineId, accepted);
          if (!accepted) {
            for (const dependentId of dependentsOf(lineId)) {
              if (next.find((record) => record.line.id === dependentId)?.status === "pending") {
                next = serverTruthCancel(next, dependentId);
              }
            }
          }
          return next;
        });
        settleBoth(lineId, accepted);
        if (index === scenario.settlementOrder.length - 1) {
          setPhase("batchSettled");
          setSeed((current) => current + 1);
        }
      }, 900 + index * 320);
    });
  }, [clearSchedules, schedule, seed, setPhase, settleBoth]);

  const highlightId = phase === "diverged" || phase === "healed" ? LINE_SAFE.id : null;

  return (
    <div className="accent-resources po-section">
      <p className="po-intro">
        Two screens receive the same clicks and the same server outcomes. The strip below is the
        referee: what the server actually knows. Each screen wears a live badge comparing what it
        shows against that truth — and under each screen sits its model: one shared cache value on
        the left, one branch per write on the right.
      </p>

      <ServerTruthStrip baseMs={baseMs} truth={serverTruth} />

      <div className="po-control-bar">
        <p className="po-scenario-explainer" id="po-batch-rules">
          The 10-request stress test runs concurrent optimistic updates: <strong>5 succeed, 4 reject, and
          1 is cancelled when its parent fails.</strong> Watch TanStack Query and Worth Signals
          diverge.
        </p>
        <div className="po-control-actions">
          <button
            className="po-control-button"
            disabled={phase !== "idle"}
            onClick={playCollision}
            type="button"
          >
            Play the collision
          </button>
          <button
            aria-describedby="po-batch-rules"
            className="po-control-button po-control-button-ghost"
            disabled={phase !== "idle" && phase !== "healed" && phase !== "batchSettled"}
            onClick={() => void runTenRequests()}
            type="button"
          >
            Run 10 mixed outcomes · seed {seed}
          </button>
          <button
            className="po-control-button po-control-button-ghost"
            disabled={phase === "idle"}
            onClick={() => void resetBoth()}
            type="button"
          >
            Reset both
          </button>
        </div>
      </div>

      {phase === "cliffhanger" ? <CliffhangerCard onDeliver={deliverRejection} /> : null}

      {phase === "diverged" || phase === "healed" ? <VerdictLine healed={phase === "healed"} /> : null}

      {phase === "batchRunning" || phase === "batchSettled" ? (
        <aside className="po-solo-note" role="status">
          {phase === "batchRunning"
            ? "Five successes, four rejections, and one dependency cancellation are settling in seeded random order — watch the left current value diverge while the right one stays honest."
            : "All ten server outcomes were delivered. The runtime receipt below turns green when closeout finishes — every number in it is runtime-issued."}
        </aside>
      ) : null}

      <div className="po-grid">
        <QueryProvider client={queryClient}>
          <ResourcesTanStackPanel
            baseMs={baseMs}
            highlightId={highlightId}
            onAgreement={handleAgreement}
            onController={(controller) => { tanstackController.current = controller; }}
            phase={phase}
            serverTruth={serverTruth}
          />
        </QueryProvider>
        <ResourcesWORTHPanel
          baseMs={baseMs}
          highlightId={highlightId}
          onAgreement={handleAgreement}
          onController={(controller) => { WORTHController.current = controller; }}
          phase={phase}
          serverTruth={serverTruth}
        />
      </div>

      <section className="signals-code-section" aria-labelledby="po-code-title">
        <h2 id="po-code-title">Each request owns a branch; dependencies are explicit</h2>
        <ResourcesSectionCodeSample
          liveLine={phase === "diverged" || phase === "healed"
            ? "// → rejected parent + dependent retired · confirmed sibling preserved"
            : null}
        />
      </section>

      <DxCorner
        code={DX_SAMPLE}
        filename="save-po-line.ts"
        receipts={[
          { claim: "Every optimistic request has an identity.", api: "admission.effectId · line.effects().get(effectId)" },
          { claim: "Dependencies are declared, not inferred.", api: "resourcePatch.dependsOn(patch, [parentEffectId])" },
          { claim: "Failure retires branches instead of patching backward.", api: "line.effects().reject(effectId)" },
        ]}
        subtitle="The application reports server outcomes. The runtime owns projection, dependency closeout, canonical reconciliation, and retirement."
      />

      <div className="signals-docs-row">
        <button onClick={() => onNavigate("#/docs/resources/effects/README")} type="button">
          Read optimistic updates <span aria-hidden="true">→</span>
        </button>
      </div>
    </div>
  );
}
