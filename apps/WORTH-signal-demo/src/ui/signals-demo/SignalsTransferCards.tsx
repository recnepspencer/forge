import React from "react";

import {
  AMOUNT_MAX,
  type AuditEntry,
  currency,
  PRESET_SCENARIOS,
  REVIEW_THRESHOLD,
  wholeCurrency,
} from "./signalsTransferRuntime";

export function AmountCard({
  committedAmount,
  onCommit,
  onStage,
  selected,
  onSelect,
  stagedAmount,
}: {
  committedAmount: number;
  onCommit: (value: number) => void;
  onStage: (value: number) => void;
  selected: boolean;
  onSelect: () => void;
  stagedAmount: number;
}): React.ReactElement {
  const hasDraft = stagedAmount !== committedAmount;
  const fillPercent = Math.max(0, Math.min(100, (stagedAmount / AMOUNT_MAX) * 100));
  const thresholdPercent = (REVIEW_THRESHOLD / AMOUNT_MAX) * 100;

  return (
    <article
      className={`signals-card signals-amount-card${selected ? " is-selected" : ""}`}
      onClick={onSelect}
    >
      <header className="signals-card-head">
        <span>Requested amount</span>
        <code>input</code>
      </header>
      <div className="signals-currency-input">
        <b aria-hidden="true">$</b>
        <input
          aria-describedby="transfer-policy-threshold"
          aria-label="Requested amount in dollars"
          max={AMOUNT_MAX}
          min="0"
          onBlur={(event) => onCommit(event.currentTarget.valueAsNumber)}
          onChange={(event) => onStage(event.currentTarget.valueAsNumber)}
          onClick={(event) => event.stopPropagation()}
          onKeyDown={(event) => {
            if (event.key === "Enter") onCommit(event.currentTarget.valueAsNumber);
          }}
          step="100"
          type="number"
          value={Number.isFinite(stagedAmount) ? stagedAmount : ""}
        />
      </div>
      <div className="signals-slider-zone">
        <div className="signals-slider-track-wrap">
          <input
            aria-label="Requested amount slider"
            className="signals-slider"
            max={AMOUNT_MAX}
            min="0"
            onChange={(event) => onStage(event.currentTarget.valueAsNumber)}
            onClick={(event) => event.stopPropagation()}
            onKeyUp={(event) => onCommit(event.currentTarget.valueAsNumber)}
            onPointerUp={(event) => onCommit(event.currentTarget.valueAsNumber)}
            step="100"
            style={{ "--fill": `${fillPercent}%` } as React.CSSProperties}
            type="range"
            value={stagedAmount}
          />
          <span
            aria-hidden="true"
            className="signals-threshold-tick"
            style={{ left: `${thresholdPercent}%` }}
          >
            <i />
            <em>{wholeCurrency.format(REVIEW_THRESHOLD)}</em>
          </span>
        </div>
        <small id="transfer-policy-threshold">
          {hasDraft
            ? `Let go to commit ${wholeCurrency.format(stagedAmount)}. We do not record every nervous mouse twitch.`
            : `${wholeCurrency.format(REVIEW_THRESHOLD)} is the line: below it stays automatic; at or above it needs a human.`}
        </small>
      </div>
      <div className="signals-preset-row" role="group" aria-label="Preset transfers">
        {PRESET_SCENARIOS.map((preset) => (
          <button
            key={preset.label}
            onClick={(event) => {
              event.stopPropagation();
              onCommit(preset.amount);
            }}
            type="button"
          >
            {preset.label}
            <b>{wholeCurrency.format(preset.amount)}</b>
          </button>
        ))}
      </div>
    </article>
  );
}

export function DecisionCard({
  caption,
  className,
  label,
  onSelect,
  selected,
  value,
}: {
  caption: string;
  className?: string;
  label: string;
  onSelect: () => void;
  selected: boolean;
  value: string;
}): React.ReactElement {
  return (
    <article
      className={`signals-card signals-decision-card${selected ? " is-selected" : ""}${className ? ` ${className}` : ""}`}
    >
      <header className="signals-card-head">
        <span>{label}</span>
        <code>computed</code>
      </header>
      <strong aria-live="polite">{value}</strong>
      <small>{caption}</small>
      <button
        aria-controls="signals-why-panel"
        aria-label={`Ask Worth why ${label}`}
        className="signals-card-why-hint"
        onClick={onSelect}
        type="button"
      >
        Ask Worth why →
      </button>
    </article>
  );
}

export function AuditRow({ entry }: { entry: AuditEntry }): React.ReactElement {
  const laneChanged = entry.laneOutcome === "Refreshed";
  return (
    <li className="signals-audit-row">
      <span className="signals-audit-tx">
        {entry.kind === "created" ? "init" : `tx ${String(entry.revision).padStart(2, "0")}`}
      </span>
      <div className="signals-audit-body">
        {entry.kind === "created" ? (
          <p className="signals-audit-main">
            Ready to experiment — amount {wholeCurrency.format(entry.amountTo)} · fee {currency.format(entry.feeTo)} · review lane {entry.laneTo}
          </p>
        ) : (
          <>
            <p className="signals-audit-main">
              amount {wholeCurrency.format(entry.amountFrom)} → <strong>{wholeCurrency.format(entry.amountTo)}</strong>
            </p>
            <p className="signals-audit-chips">
              <span className="signals-chip signals-chip-fee">
                Fee ran: {currency.format(entry.feeFrom)} → {currency.format(entry.feeTo)}
              </span>
              {laneChanged ? (
                <span className="signals-chip signals-chip-flipped">
                  Review rule changed its answer: {entry.laneFrom} → {entry.laneTo}
                </span>
              ) : (
                <span className="signals-chip signals-chip-unchanged">
                  Review rule checked again and kept the same answer ({entry.laneTo})
                </span>
              )}
            </p>
          </>
        )}
        <details className="signals-audit-payload">
          <summary>runtime payload</summary>
          <pre>{JSON.stringify(entry.payload, null, 2)}</pre>
        </details>
      </div>
      <span className="signals-audit-meta">
        {entry.recomputedCount !== null
          ? `${entry.recomputedCount} ran${entry.stageCount !== null ? ` · ${entry.stageCount} stages` : ""}`
          : "ready"}
      </span>
    </li>
  );
}
