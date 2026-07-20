import { useCallback, useRef, useState } from "react";

import { ResourcesScenarioGuide } from "./ResourcesScenarioGuide";
import { ResourcesSectionCodeSample } from "./ResourcesSectionCodeSample";
import { ResourcesWORTHPanel } from "./ResourcesWORTHPanel";
import type { PanelController } from "./resourcesSectionPanels";
import {
  LINE_DEPENDENT,
  LINE_RISKY,
  LINE_SAFE,
  initialServerTruth,
  serverTruthAdmit,
  serverTruthCancel,
  serverTruthSettle,
  type ScenarioPhase,
  type ServerTruth,
} from "./resourcesSectionSupport";
import "./resourcesSection.css";
import "./resourcesPanel.css";

interface ResourcesSectionProps {
  onNavigate: (path: string) => void;
}

function admittedScenarioTruth(): ServerTruth {
  return [LINE_RISKY, LINE_SAFE, LINE_DEPENDENT].reduce(
    (truth, line) => serverTruthAdmit(truth, line),
    initialServerTruth(),
  );
}

export function ResourcesSection({ onNavigate }: ResourcesSectionProps) {
  const controllerRef = useRef<PanelController | null>(null);
  const [ready, setReady] = useState(false);
  const [busy, setBusy] = useState(false);
  const [phase, setPhase] = useState<ScenarioPhase>("idle");
  const [serverTruth, setServerTruth] = useState<ServerTruth>(initialServerTruth());

  const resetScenario = useCallback(async () => {
    if (busy) return;
    setBusy(true);
    try {
      await controllerRef.current?.reset();
      setServerTruth(initialServerTruth());
      setPhase("idle");
    } finally {
      setBusy(false);
    }
  }, [busy]);

  const handleSubmitLines = useCallback(async () => {
    const controller = controllerRef.current;
    if (!controller || busy || phase !== "idle") return;
    setBusy(true);
    setServerTruth(admittedScenarioTruth());
    setPhase("optimistic");
    try {
      await controller.addLine(LINE_RISKY);
      await Promise.all([
        controller.addLine(LINE_SAFE),
        controller.addLine(LINE_DEPENDENT, { dependsOnLineId: LINE_RISKY.id }),
      ]);
    } finally {
      setBusy(false);
    }
  }, [busy, phase]);

  const handleApproveGoggles = useCallback(async () => {
    const controller = controllerRef.current;
    if (!controller || busy || phase !== "optimistic") return;
    setBusy(true);
    setServerTruth((truth) => serverTruthSettle(truth, LINE_SAFE.id, true));
    try {
      await controller.settle(LINE_SAFE.id, true);
      setPhase("siblingConfirmed");
    } finally {
      setBusy(false);
    }
  }, [busy, phase]);

  const handleRejectSolvent = useCallback(async () => {
    const controller = controllerRef.current;
    if (!controller || busy || phase !== "siblingConfirmed") return;
    setBusy(true);
    setServerTruth((truth) => serverTruthCancel(
      serverTruthSettle(truth, LINE_RISKY.id, false),
      LINE_DEPENDENT.id,
    ));
    try {
      await controller.settle(LINE_RISKY.id, false);
      setPhase("settled");
    } finally {
      setBusy(false);
    }
  }, [busy, phase]);

  const handleController = useCallback((controller: PanelController | null) => {
    controllerRef.current = controller;
    setReady(controller !== null);
  }, []);

  return (
    <div className="accent-resources po-section">
      <p className="po-intro">
        A real purchasing screen cannot freeze whenever a supplier check runs. It also cannot lose
        a valid line item because a different request failed. This order shows both requirements
        working together in the inventory staff actually use.
      </p>

      <div className="po-runtime-stage">
        <ResourcesScenarioGuide
          busy={busy}
          onApprove={() => void handleApproveGoggles()}
          onReject={() => void handleRejectSolvent()}
          onReset={() => void resetScenario()}
          onSubmit={() => void handleSubmitLines()}
          phase={phase}
          ready={ready}
        />
        <ResourcesWORTHPanel
          highlightId={phase === "settled" ? LINE_SAFE.id : null}
          onController={handleController}
          phase={phase}
          serverTruth={serverTruth}
        />
      </div>

      <section className="signals-code-section" aria-labelledby="po-code-title">
        <span className="po-code-kicker">Under the inventory screen</span>
        <h2 id="po-code-title">The application reports outcomes. Worth keeps every request separate.</h2>
        <p className="po-code-intro">
          No inverse patch. No shared snapshot. No broad refetch to make the screen look correct again.
        </p>
        <ResourcesSectionCodeSample
          liveLine={phase === "settled"
            ? "// 1 merged / 1 rejected / 1 dependency-cancelled / 0 open"
            : null}
        />
      </section>

      <div className="signals-docs-row">
        <button onClick={() => onNavigate("#/docs/resources/effects/README")} type="button">
          Learn optimistic effects <span aria-hidden="true">-&gt;</span>
        </button>
      </div>
    </div>
  );
}
