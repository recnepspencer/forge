import {
  roleLabels,
  type OutcomeView,
  type PlantRole,
  type SopRevision,
} from "./routerSectionSupport";
import type {
  BatchRecordPage,
  OverviewPage,
  ReleasePage,
  StepFourPage,
} from "./routerSectionSupport";

interface BrowserSurfaceProps {
  activeTarget: string;
  deviationGranted: boolean;
  effectiveRev: SopRevision;
  isNavigating: boolean;
  onGrantDeviation: () => void;
  onNavigate: (target: string) => void;
  outcome: OutcomeView | null;
  pageValue: unknown;
  role: PlantRole;
  routeOptions: ReadonlyArray<{ path: string; label: string }>;
  trainedRev: SopRevision;
}

function OverviewView({ page }: { page: OverviewPage }) {
  return (
    <div className="mfg-page">
      <h4>{page.line}</h4>
      <p className="mfg-page-sub">{page.shift}</p>
      <ul className="mfg-page-rows">
        {page.wip.map((entry) => (
          <li key={entry.batch}>
            <strong>{entry.batch}</strong>
            <span>{entry.status}</span>
          </li>
        ))}
      </ul>
    </div>
  );
}

function BatchRecordView({ page }: { page: BatchRecordPage }) {
  return (
    <div className="mfg-page">
      <h4>{page.batch} · {page.product}</h4>
      <p className="mfg-page-sub">{page.status}</p>
      <ul className="mfg-page-rows">
        {page.steps.map((entry) => (
          <li className={entry.status === "pending" ? "is-pending" : ""} key={entry.step}>
            <strong>{entry.step}</strong>
            <span>{entry.status}</span>
          </li>
        ))}
      </ul>
    </div>
  );
}

function StepView({ deviationGranted, page }: { deviationGranted: boolean; page: StepFourPage }) {
  return (
    <div className="mfg-page">
      {deviationGranted ? (
        <p className="mfg-deviation-ribbon">Executing under deviation DEV-0113 — recorded in the audit trail.</p>
      ) : null}
      <h4>{page.step}</h4>
      <p className="mfg-page-sub">{page.sop}</p>
      <ul className="mfg-page-rows">
        <li><strong>Spec</strong><span>{page.spec}</span></li>
        <li><strong>Instrument</strong><span>{page.instrument}</span></li>
      </ul>
      <button className="mfg-execute-button" type="button">Record torque readings</button>
    </div>
  );
}

function ReleaseView({ page }: { page: ReleasePage }) {
  return (
    <div className="mfg-page">
      <h4>Quality release · {page.batch}</h4>
      <p className="mfg-page-sub">Device history record checklist</p>
      <ul className="mfg-page-rows">
        {page.checklist.map((entry) => (
          <li key={entry.item}>
            <strong>{entry.item}</strong>
            <span>{entry.state}</span>
          </li>
        ))}
      </ul>
    </div>
  );
}

function DeniedView({
  canDeviate,
  onGrantDeviation,
  outcome,
}: {
  canDeviate: boolean;
  onGrantDeviation: () => void;
  outcome: OutcomeView;
}) {
  return (
    <div className="mfg-denied">
      <p className="mfg-denied-title">
        Access denied · <code>{outcome.reason ?? outcome.kind}</code>
      </p>
      {outcome.detail ? <p className="mfg-denied-detail">{outcome.detail}</p> : null}
      <p className="mfg-denied-note">This attempt is already in the audit trail — denials are records too.</p>
      {canDeviate ? (
        <button className="mfg-deviation-button" onClick={onGrantDeviation} type="button">
          Proceed under deviation DEV-0113
          <span>emergency execution · justification recorded</span>
        </button>
      ) : null}
    </div>
  );
}

export function RouterSectionBrowserSurface({
  activeTarget,
  deviationGranted,
  effectiveRev,
  isNavigating,
  onGrantDeviation,
  onNavigate,
  outcome,
  pageValue,
  role,
  routeOptions,
  trainedRev,
}: BrowserSurfaceProps) {
  const denied = outcome?.tone === "denied";
  const canDeviate =
    denied && outcome?.reason === "trainingSupersededByRevision" && !deviationGranted;

  return (
    <article className="mfg-browser-card">
      <header className="mfg-browser-chrome">
        <span className="signals-code-dots" aria-hidden="true"><i /><i /><i /></span>
        <code className="mfg-browser-url">mes.worth.example{activeTarget}</code>
        <span className="mfg-role-badge">
          {roleLabels[role]} · SOP-042 rev {trainedRev}
        </span>
      </header>

      <nav aria-label="Portal navigation" className="mfg-browser-nav">
        {routeOptions.map((option) => (
          <button
            className={option.path === activeTarget ? "is-active" : ""}
            key={option.path}
            onClick={() => onNavigate(option.path)}
            type="button"
          >
            {option.label}
            {option.path.includes("/steps/") && effectiveRev !== trainedRev ? (
              <i aria-hidden="true" className="mfg-nav-alert" />
            ) : null}
          </button>
        ))}
      </nav>

      <div className="mfg-browser-body">
        {isNavigating ? (
          <div className="mfg-browser-loading" role="status">
            <span className="mfg-spinner" aria-hidden="true" />
            <span>admitting…</span>
          </div>
        ) : denied && outcome ? (
          <DeniedView canDeviate={canDeviate} onGrantDeviation={onGrantDeviation} outcome={outcome} />
        ) : pageValue ? (
          activeTarget.includes("/steps/") ? (
            <StepView deviationGranted={deviationGranted} page={pageValue as StepFourPage} />
          ) : activeTarget.includes("/record") ? (
            <BatchRecordView page={pageValue as BatchRecordPage} />
          ) : activeTarget.includes("/release") ? (
            <ReleaseView page={pageValue as ReleasePage} />
          ) : (
            <OverviewView page={pageValue as OverviewPage} />
          )
        ) : (
          <div className="mfg-browser-loading">
            <span>waiting for the route</span>
          </div>
        )}
      </div>
    </article>
  );
}
