import type { ScenarioPhase } from "./resourcesSectionSupport";

function scenarioStatus(phase: ScenarioPhase): string {
  if (phase === "idle") return "Add three inventory requests optimistically.";
  if (phase === "optimistic") return "All three are pending. Approve the independent goggles request.";
  if (phase === "siblingConfirmed") return "The goggles are safe. Now return a failed solvent response.";
  return "The solvent and its dependent kit closed. The approved goggles stayed.";
}

export function ResourcesScenarioGuide({
  busy,
  phase,
  ready,
  onSubmit,
  onApprove,
  onReject,
  onReset,
}: {
  busy: boolean;
  phase: ScenarioPhase;
  ready: boolean;
  onSubmit: () => void;
  onApprove: () => void;
  onReject: () => void;
  onReset: () => void;
}): React.ReactElement {
  const action = phase === "idle"
    ? { action: onSubmit, label: "Add request lines", step: "Step 1 of 3", tone: "default" as const }
    : phase === "optimistic"
      ? { action: onApprove, label: "Approve goggles", step: "Step 2 of 3", tone: "default" as const }
      : phase === "siblingConfirmed"
        ? { action: onReject, label: "Reject solvent", step: "Step 3 of 3", tone: "danger" as const }
        : { action: onReset, label: "Reset demo", step: "Complete", tone: "default" as const };

  return (
    <section className={`po-story po-story-${phase}`} aria-label="Inventory approval demo controls">
      <div className="po-story-copy">
        <span className="po-story-kicker">Interactive approval sequence</span>
        <p aria-live="polite">{busy ? "Applying response..." : scenarioStatus(phase)}</p>
      </div>
      <div className="po-story-action">
        <span>{action.step}</span>
        <button
          className={action.tone === "danger" ? "is-danger" : undefined}
          disabled={!ready || busy}
          onClick={action.action}
          type="button"
        >
          {action.label}
        </button>
      </div>
      {phase !== "idle" && phase !== "settled" ? (
        <button className="po-control-reset" disabled={busy} onClick={onReset} type="button">Reset</button>
      ) : null}
    </section>
  );
}
