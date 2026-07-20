import React from "react";

export interface EffectOutcome {
  readonly effectId: string;
  readonly label: string;
  readonly lifecycle: string;
  readonly terminal: string | null;
}

interface OutcomePresentation {
  readonly label: string;
  readonly detail: string;
  readonly tone: "pending" | "accepted" | "rejected" | "cancelled";
}

function presentOutcome(outcome: EffectOutcome): OutcomePresentation {
  if (outcome.terminal === "merged") {
    return {
      label: "Accepted",
      detail: "Added to the purchase order and reconciled with the server.",
      tone: "accepted",
    };
  }
  if (outcome.terminal === "rejectedAndRetired") {
    return {
      label: "Rejected",
      detail: "Supplier approval failed for this controlled material.",
      tone: "rejected",
    };
  }
  if (outcome.terminal === "dependencyCancelled") {
    return {
      label: "Cancelled",
      detail: "Removed because its required controlled material was rejected.",
      tone: "cancelled",
    };
  }
  return {
    label: outcome.lifecycle === "ResponseRecorded" ? "Server responded" : "Checking",
    detail: "Visible optimistically while approval is still pending.",
    tone: "pending",
  };
}

export function ResourcesOutcomeActivity({
  outcomes,
  onSelect,
  selectedId,
}: {
  outcomes: readonly EffectOutcome[];
  onSelect: (effectId: string) => void;
  selectedId: string | null;
}): React.ReactElement {
  return (
    <section className="po-activity" aria-labelledby="po-activity-title">
      <header className="po-activity-head">
        <span>Approval and sync activity</span>
        <small id="po-activity-title">Live from PO-1142</small>
      </header>
      {outcomes.length === 0 ? (
        <div className="po-activity-empty">
          <strong>No pending changes</strong>
          <span>Run the scenario to submit three inventory lines.</span>
        </div>
      ) : (
        <ol className="po-activity-list" aria-live="polite">
          {outcomes.map((outcome) => {
            const presentation = presentOutcome(outcome);
            return (
              <li key={outcome.effectId}>
                <button
                  aria-pressed={selectedId === outcome.effectId}
                  className={`po-activity-row po-activity-${presentation.tone}`}
                  onClick={() => onSelect(outcome.effectId)}
                  type="button"
                >
                  <span className="po-activity-marker" aria-hidden="true" />
                  <span className="po-activity-copy">
                    <strong>{outcome.label}</strong>
                    <small>{presentation.detail}</small>
                  </span>
                  <span className="po-activity-status">{presentation.label}</span>
                </button>
              </li>
            );
          })}
        </ol>
      )}
      <p className="po-activity-footnote">Select an event to inspect its runtime audit receipt.</p>
    </section>
  );
}
