import { FormsSectionCodeSample } from "./FormsSectionCodeSample";
import { DemoPreface } from "./DemoPreface";
import "./formsSection.css";
import { COUNTRY_OPTIONS, currency } from "./formsSectionSupport";
import { selectedRegionSummary, useFormsSectionState } from "./formsSectionHooks";

interface FormsSectionProps {
  onNavigate: (path: string) => void;
}

function fieldNote(message?: string) {
  return <small className={message ? "" : "forms-field-spacer"}>{message ?? "placeholder"}</small>;
}

export function FormsSection({ onNavigate }: FormsSectionProps) {
  const {
    draft,
    regionsOpen,
    regionsRef,
    priceError,
    shippingError,
    carrierErrors,
    visibleMessages,
    isDirty,
    diagnosticsOutput,
    patchDraft,
    resetDraft,
    setRegionsOpen,
    toggleRegion,
    updateCarrierEmail,
  } = useFormsSectionState();

  return (
    <div className="xai-section-band accent-forms">
      <div className="xai-section-heading">
        <span className="xai-section-eyebrow">03 / Forms</span>
        <h2>Edit a product form. Watch Forge track the state.</h2>
        <p>
          Change the price, margin, shipping regions, or reviewer email. Forge shows
          what changed, what is invalid, and whether the form can submit.
        </p>
      </div>

      <DemoPreface demoId={3} />

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
          <h3>Edit a product form.</h3>
          <p>Change fields and watch validation, dirty state, and submit readiness update.</p>

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
                onClick={resetDraft}
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
        <div className="signals-cta-copy">Change the form and inspect the exact state Forge uses to enable or block submit.</div>
        <div className="xai-section-actions">
          <button className="xai-button xai-button-primary" onClick={() => onNavigate("#/demos/3")} type="button">
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
