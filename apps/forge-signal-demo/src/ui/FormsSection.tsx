import React from "react";
import { FormsSectionCodeSample } from "./FormsSectionCodeSample";
import "./formsSection.css";
import {
  COUNTRY_OPTIONS,
  SOURCE_DRAFT,
  currency,
  selectedRegionSummary,
  validateCarrierEmail,
  validatePrice,
  validateShippingSelection,
} from "./formsSectionSupport";
import type { RolloutDraft } from "./formsSectionSupport";

interface FormsSectionProps {
  onNavigate: (path: string) => void;
}

type CarrierErrorMap = Partial<Record<string, string>>;

function fieldNote(message?: string): React.ReactElement {
  return <small className={message ? "" : "forms-field-spacer"}>{message ?? "placeholder"}</small>;
}

function formatArray(values: string[]): string {
  return values.length > 0 ? `[${values.map((value) => `"${value}"`).join(", ")}]` : "[]";
}

export function FormsSection({ onNavigate }: FormsSectionProps): React.ReactElement {
  const [draft, setDraft] = React.useState<RolloutDraft>(SOURCE_DRAFT);
  const [regionsOpen, setRegionsOpen] = React.useState(false);
  const regionsRef = React.useRef<HTMLDivElement>(null);

  const priceError = validatePrice(draft.price, draft.baseCost, draft.targetMargin) ?? undefined;
  const shippingError = validateShippingSelection(draft.shippingRegions) ?? undefined;
  const carrierErrors = React.useMemo(() => {
    const next: CarrierErrorMap = {};
    for (const option of COUNTRY_OPTIONS) {
      const error = validateCarrierEmail(option.code, draft.shippingRegions, draft.carrierEmails);
      if (error) {
        next[option.code] = error;
      }
    }
    return next;
  }, [draft.carrierEmails, draft.shippingRegions]);

  const visibleMessages = [
    priceError,
    shippingError,
    ...Object.values(carrierErrors),
  ].filter(Boolean) as string[];
  const dirtyFields = React.useMemo(() => {
    return (Object.keys(SOURCE_DRAFT) as Array<keyof RolloutDraft>).filter(
      (key) => JSON.stringify(SOURCE_DRAFT[key]) !== JSON.stringify(draft[key]),
    );
  }, [draft]);
  const isDirty = dirtyFields.length > 0;

  const patchPlanSummary = React.useMemo(() => {
    const ops: string[] = [];
    if (draft.price !== SOURCE_DRAFT.price) {
      ops.push(`replace price -> ${currency.format(draft.price)}`);
    }
    if (JSON.stringify(draft.shippingRegions) !== JSON.stringify(SOURCE_DRAFT.shippingRegions)) {
      ops.push(`replace shippingRegions -> ${formatArray(draft.shippingRegions)}`);
    }
    for (const code of draft.shippingRegions) {
      const current = draft.carrierEmails[code] ?? "";
      const source = SOURCE_DRAFT.carrierEmails[code] ?? "";
      if (current !== source) {
        ops.push(`set carrierEmails.${code} -> "${current}"`);
      }
    }
    return ops;
  }, [draft]);

  const diagnosticsOutput = React.useMemo(() => {
    const inspectorLines = [
      "const source = form.source();",
      `> ${JSON.stringify(
        {
          price: SOURCE_DRAFT.price,
          shippingRegions: SOURCE_DRAFT.shippingRegions,
          carrierEmails: Object.fromEntries(
            Object.entries(SOURCE_DRAFT.carrierEmails).filter(([, value]) => value.length > 0),
          ),
        },
        null,
        2,
      )}`,
      "",
      "const dirty = form.dirty();",
      `> ${JSON.stringify({ isDirty: dirtyFields.length > 0, fields: dirtyFields }, null, 2)}`,
      "",
      "const patchPlan = form.patchPlan();",
      `> ${JSON.stringify({ empty: patchPlanSummary.length === 0, operations: patchPlanSummary }, null, 2)}`,
      "",
      'const readiness = form.readiness();',
      `> ${JSON.stringify({ canSubmit: visibleMessages.length === 0, blockers: visibleMessages }, null, 2)}`,
      "",
      'const savePlan = form.actionPlan("save");',
      `> ${JSON.stringify({ disabled: visibleMessages.length > 0, reason: visibleMessages[0] ?? null }, null, 2)}`,
      "",
      "const draft = form.draft;",
      `> ${JSON.stringify(
        {
          price: draft.price,
          shippingRegions: draft.shippingRegions,
          carrierEmails: Object.fromEntries(
            Object.entries(draft.carrierEmails).filter(([, value]) => value.length > 0),
          ),
        },
        null,
        2,
      )}`,
    ];
    return inspectorLines.join("\n");
  }, [dirtyFields, draft, patchPlanSummary, visibleMessages]);

  React.useEffect(() => {
    function handlePointerDown(event: PointerEvent): void {
      if (!regionsRef.current?.contains(event.target as Node)) {
        setRegionsOpen(false);
      }
    }

    if (!regionsOpen) {
      return;
    }

    window.addEventListener("pointerdown", handlePointerDown);
    return () => window.removeEventListener("pointerdown", handlePointerDown);
  }, [regionsOpen]);

  function patchDraft(next: Partial<RolloutDraft>): void {
    setDraft((current) => ({ ...current, ...next }));
  }

  function toggleRegion(code: string): void {
    setDraft((current) => {
      const selected = current.shippingRegions.includes(code)
        ? current.shippingRegions.filter((value) => value !== code)
        : [...current.shippingRegions, code];
      return { ...current, shippingRegions: selected };
    });
  }

  function updateCarrierEmail(code: string, value: string): void {
    setDraft((current) => ({
      ...current,
      carrierEmails: { ...current.carrierEmails, [code]: value },
    }));
  }

  return (
    <div className="xai-section-band accent-forms">
      <div className="xai-section-heading">
        <span className="xai-section-eyebrow">02 / Forms</span>
        <h2>Edit rollout settings and watch the controller keep score.</h2>
        <p>
          Pricing policy already moved to a 25% margin. This edit needs a higher
          retail price, country approval truth from the backend, and carrier emails
          for every selected shipping region.
        </p>
      </div>

      <article className="forms-code-card">
        <div className="forms-card-topline">
          <span>React authoring</span>
        </div>
        <h3>Source, approval reads, and submit all show up in one form.</h3>
        <FormsSectionCodeSample />
      </article>

      <div className="forms-live-stack">
        <article className="forms-live-card">
          <div className="forms-card-topline">
            <span>Live edit form</span>
          </div>
          <h3>Edit rollout settings.</h3>
          <p>Raise the price, pick shipping regions, and watch approval and carrier-email rules gate save automatically.</p>

          <form className="forms-edit-form" onSubmit={(event) => event.preventDefault()}>
            <div className="forms-meta-row">
              <div className="forms-meta-item">
                <span>Product</span>
                <strong>{draft.productName}</strong>
              </div>
              <div className="forms-meta-item">
                <span>Base cost</span>
                <strong>{currency.format(draft.baseCost)}</strong>
              </div>
            </div>

            <div className="forms-field-grid">
              <label className="forms-field">
                <span>Retail price</span>
                <input
                  className="forms-input"
                  onChange={(event) => patchDraft({ price: Number(event.target.value) })}
                  type="number"
                  value={draft.price}
                />
                {fieldNote(priceError)}
              </label>

              <label className="forms-field forms-field-readonly">
                <span>Target margin</span>
                <input className="forms-input" readOnly type="number" value={draft.targetMargin} />
                {fieldNote()}
              </label>
            </div>

            <div ref={regionsRef} className="forms-field forms-multiselect-field">
              <span>Shipping regions</span>
              <button
                className="forms-multiselect-trigger"
                onClick={() => setRegionsOpen((current) => !current)}
                type="button"
              >
                <span className="forms-multiselect-value">{selectedRegionSummary(draft.shippingRegions)}</span>
                <strong className="forms-multiselect-arrow" aria-hidden="true">
                  {regionsOpen ? "▲" : "▼"}
                </strong>
              </button>
              {fieldNote(shippingError)}
              {regionsOpen && (
                <div className="forms-multiselect-panel">
                  {COUNTRY_OPTIONS.map((option) => (
                    <label
                      key={option.code}
                      className={`forms-multiselect-option ${
                        draft.shippingRegions.includes(option.code) ? "is-selected" : ""
                      }`}
                    >
                      <input
                        checked={draft.shippingRegions.includes(option.code)}
                        onChange={() => toggleRegion(option.code)}
                        type="checkbox"
                      />
                      <div>
                        <strong>{option.label}</strong>
                        <span>{option.approved ? "Approved" : "Awaiting regulatory approval"}</span>
                      </div>
                      <span className="forms-multiselect-check" aria-hidden="true">
                        {draft.shippingRegions.includes(option.code) ? "✓" : ""}
                      </span>
                    </label>
                  ))}
                </div>
              )}
            </div>

            {draft.shippingRegions.length > 0 && (
              <div className="forms-field-grid">
                {draft.shippingRegions.map((code) => (
                  <label key={code} className="forms-field">
                    <span>{`${code} carrier email`}</span>
                    <input
                      className="forms-input"
                      onChange={(event) => updateCarrierEmail(code, event.target.value)}
                      placeholder={`${code.toLowerCase()}-ops@carrier.test`}
                      type="email"
                      value={draft.carrierEmails[code] ?? ""}
                    />
                    {fieldNote(carrierErrors[code])}
                  </label>
                ))}
              </div>
            )}

            <div className="forms-submit-row">
              <button
                className="forms-secondary-button"
                disabled={!isDirty}
                onClick={() => setDraft(SOURCE_DRAFT)}
                type="button"
              >
                Reset
              </button>
              <button className="forms-primary-button" disabled={visibleMessages.length > 0} type="submit">
                Save changes
              </button>
            </div>
          </form>
        </article>

        <article className="forms-diagnostics-card">
          <div className="forms-card-topline">
            <span>Form diagnostics</span>
          </div>
          <h3>Inspect the form like real runtime output.</h3>
          <div className="forms-code-output">
            <pre>{diagnosticsOutput}</pre>
          </div>
        </article>
      </div>

      <div className="signals-compare-strip">
        <article className="xai-compare-card xai-compare-card-typical">
          <span>Without Forge</span>
          <h4>Local form state plus parallel fetch glue</h4>
          <ul>
            <li>Load source separately from approval data</li>
            <li>Hand-roll multiselect validation against backend truth</li>
            <li>Track carrier emails for every selected region manually</li>
            <li>Keep submit disabled rules in sync by hand</li>
          </ul>
        </article>

        <article className="xai-compare-card xai-compare-card-forge">
          <span>With Forge</span>
          <h4>Read resources and form state share one surface</h4>
          <ul>
            <li>Source and approval reads both feed the controller</li>
            <li>Shipping validation can depend on fetched backend truth</li>
            <li>Every selected region gets a required carrier email</li>
            <li>Save posture comes from the same readiness model</li>
          </ul>
        </article>
      </div>

      <div className="signals-cta-row">
        <div className="signals-cta-copy">Change the rollout, select blocked countries, and watch the form shut down save until the backend rules are satisfied.</div>
        <div className="xai-section-actions">
          <button className="xai-button xai-button-primary" onClick={() => onNavigate("#/demos/2")} type="button">
            Open forms demo
          </button>
          <button className="xai-button xai-button-secondary" onClick={() => onNavigate("#/docs/forms/index")} type="button">
            Read forms docs
          </button>
        </div>
      </div>
    </div>
  );
}
