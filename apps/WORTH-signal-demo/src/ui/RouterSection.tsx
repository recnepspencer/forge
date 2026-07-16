import { DxCorner } from "./DxCorner";
import { RouterSectionBrowserSurface } from "./RouterSectionBrowserSurface";
import { RouterSectionCodeSample } from "./RouterSectionCodeSample";
import { useRouterSectionState, type AccessLogEntry, type ReplayRow } from "./routerSectionHooks";
import { REPLAY_PERSONAS, roleLabels, type PlantRole } from "./routerSectionSupport";
import "./routerSection.css";

interface RouterSectionProps {
  onNavigate: (path: string) => void;
}

const ROUTER_DX_SAMPLE = `export function StepLink({ batchId, stepId }: StepLinkProps) {
  const ref = routes.stepExecute.to({ params: { batchId, stepId } });

  return (
    <a href={ref.href} onClick={(event) => go(event, ref)}>
      Execute step {stepId}
    </a>
  );
}`;

function OutcomeChip({ label, tone }: { label: string; tone: string }) {
  return <span className={`mfg-chip mfg-chip-${tone}`}>{label}</span>;
}

function AccessLogRow({ entry }: { entry: AccessLogEntry }) {
  return (
    <li className={`mfg-log-row is-${entry.outcome.tone}`}>
      <span className="mfg-log-time">{entry.at}</span>
      <div className="mfg-log-body">
        <p className="mfg-log-main">
          <strong>{roleLabels[entry.role]}</strong> → <code>{entry.target}</code>
        </p>
        <p className="mfg-log-detail">
          {entry.outcome.reason ? <code>{entry.outcome.reason}</code> : null}
          {entry.outcome.detail ? <span> {entry.outcome.detail}</span> : null}
          {!entry.outcome.reason && !entry.outcome.detail ? <span>admitted without conditions</span> : null}
        </p>
        <details className="signals-audit-payload">
          <summary>raw admission record</summary>
          <pre>{JSON.stringify(entry.raw, null, 2)}</pre>
        </details>
      </div>
      <OutcomeChip label={entry.outcome.label} tone={entry.outcome.tone} />
    </li>
  );
}

function ReplayTable({ rows }: { rows: ReplayRow[] }) {
  return (
    <div className="mfg-replay-scroll">
      <table className="mfg-replay-table">
        <thead>
          <tr>
            <th scope="col">recorded intent</th>
            {REPLAY_PERSONAS.map((persona) => (
              <th key={persona.id} scope="col">{persona.label}</th>
            ))}
          </tr>
        </thead>
        <tbody>
          {rows.map((row) => (
            <tr key={row.target}>
              <td><code>{row.target}</code></td>
              {REPLAY_PERSONAS.map((persona) => {
                const outcome = row.outcomes[persona.id];
                return (
                  <td key={persona.id}>
                    {outcome ? <OutcomeChip label={outcome.label} tone={outcome.tone} /> : <span className="mfg-chip-pending">…</span>}
                  </td>
                );
              })}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function exportAuditTrail(accessLog: AccessLogEntry[], replayRows: ReplayRow[], story: any): void {
  let storyArtifacts: unknown = null;
  try {
    storyArtifacts = {
      admittedEntries: story?.admittedEntries?.() ?? null,
      auditabilitySummary: story?.auditability?.()?.summary?.() ?? null,
      events: story?.events?.() ?? null,
    };
  } catch {
    storyArtifacts = "story artifacts unavailable";
  }
  const artifact = {
    exportedAt: new Date().toISOString(),
    scenario: "mes-step-admission",
    source: "routes.admitBrowserHistoryIngress(...) reports + story.auditability(), read from the Worth runtime",
    accessLog,
    replay: replayRows,
    story: storyArtifacts,
  };
  const blob = new Blob([JSON.stringify(artifact, null, 2)], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = "worth-audit-trail.json";
  anchor.click();
  URL.revokeObjectURL(url);
}

export function RouterSection({ onNavigate }: RouterSectionProps) {
  const {
    accessLog,
    activeTarget,
    bootError,
    currentOutcome,
    deviationGranted,
    effectiveRev,
    grantDeviation,
    isNavigating,
    navigate,
    pageValue,
    replayRows,
    revBumped,
    role,
    routeOptions,
    setRole,
    signalsReady,
    story,
    trainedRev,
  } = useRouterSectionState();

  if (bootError) {
    return <div className="mfg-section accent-router"><div className="signals-runtime-message">{bootError}</div></div>;
  }

  const latest = accessLog[0] ?? null;
  const liveCodeLine = latest
    ? `// → outcome: "${latest.outcome.label}"${latest.outcome.reason ? ` · reason: "${latest.outcome.reason}"` : ""}`
    : null;

  return (
    <div className="accent-router mfg-section">
      {!signalsReady ? <div className="signals-runtime-message">Connecting to the Worth runtime…</div> : null}

      {revBumped ? (
        <aside className="mfg-doc-banner" role="status">
          <strong>Document control</strong>
          <span>
            SOP-042 rev C is now effective. Your training record: rev {trainedRev}
            {trainedRev !== effectiveRev ? " — execution admission will change." : "."}
          </span>
        </aside>
      ) : null}

      {signalsReady ? (
        <>
          <div className="mfg-role-row" role="group" aria-label="Session role">
            <span>Signed in as</span>
            {(Object.keys(roleLabels) as PlantRole[]).map((option) => (
              <button
                className={option === role ? "is-active" : ""}
                key={option}
                onClick={() => setRole(option)}
                type="button"
              >
                {roleLabels[option]}
              </button>
            ))}
            <em>switching roles starts a fresh session — the audit trail keeps everything</em>
          </div>

          <section className="mfg-stage" aria-label="Manufacturing portal">
            <RouterSectionBrowserSurface
              activeTarget={activeTarget}
              deviationGranted={deviationGranted}
              effectiveRev={effectiveRev}
              isNavigating={isNavigating}
              onGrantDeviation={grantDeviation}
              onNavigate={navigate}
              outcome={currentOutcome}
              pageValue={pageValue}
              role={role}
              routeOptions={routeOptions}
              trainedRev={trainedRev}
            />

            <section className="mfg-log-panel" aria-label="Audit trail">
              <header className="signals-panel-head">
                <h3>Audit trail</h3>
                <code>story.auditability()</code>
                <button
                  className="signals-export-button"
                  onClick={() => exportAuditTrail(accessLog, replayRows, story)}
                  type="button"
                >
                  Export audit trail (JSON)
                </button>
              </header>
              <ul className="mfg-log">
                {accessLog.slice(0, 6).map((entry) => (
                  <AccessLogRow entry={entry} key={entry.id} />
                ))}
                {accessLog.length === 0 ? (
                  <li className="mfg-log-row"><span className="mfg-log-detail">first admission in flight…</span></li>
                ) : null}
              </ul>
              <p className="mfg-log-footnote">
                Every attempt is a recorded admission decision — including the denials. Nothing here is kept by the UI.
              </p>
            </section>
          </section>

          {replayRows.length > 0 ? (
            <section className="mfg-replay-panel" aria-label="Session replay">
              <header className="signals-panel-head">
                <h3>The inspector's question</h3>
                <code>routes.simulateSequence(recordedIntents).run({"{ facts }"})</code>
              </header>
              <p className="mfg-replay-sub">
                “Demonstrate that an operator trained on rev B could not have executed step 4.” The recorded
                session, re-asked under different facts — answered by the runtime, not log archaeology.
              </p>
              <ReplayTable rows={replayRows} />
            </section>
          ) : null}

          <section className="signals-code-section" aria-labelledby="mfg-code-title">
            <h2 id="mfg-code-title">Admission is a declaration, the audit trail is a by-product</h2>
            <RouterSectionCodeSample liveLine={liveCodeLine} />
          </section>

          <DxCorner
            code={ROUTER_DX_SAMPLE}
            filename="step-link.tsx"
            receipts={[
              {
                claim: "Guards return decisions, not booleans.",
                api: "forbidden({ reason, detail }) · allow({ reason })",
              },
              {
                claim: "Admission reads live facts — role, training, effective revision.",
                api: "admitBrowserHistoryIngress(ingress, session.facts)",
              },
              {
                claim: "Typed refs; the route warms its own data.",
                api: 'routes.stepExecute.to({ params }) · prefetch: "intent"',
              },
            ]}
            subtitle="Part 11-grade access control sounds like an MES project. Here it is a route declaration — and the audit trail is a by-product, not a pipeline."
          />
        </>
      ) : null}

      <div className="signals-docs-row">
        <button onClick={() => onNavigate("#/docs/router/index")} type="button">
          Explore routing in the documentation <span aria-hidden="true">→</span>
        </button>
      </div>
    </div>
  );
}
