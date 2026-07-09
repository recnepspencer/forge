import "./routerSection.css";
import { DemoPreface } from "./DemoPreface";
import { RouterSectionBrowserSurface } from "./RouterSectionBrowserSurface";
import { RouterSectionCodeSample } from "./RouterSectionCodeSample";
import {
  roleLabels,
  useRouterSectionState,
  type SessionRole,
} from "./routerSectionHooks";

interface RouterSectionProps {
  onNavigate: (path: string) => void;
}

function RouterRoleToggle({
  activeRole,
  onSelect,
}: {
  activeRole: SessionRole;
  onSelect: (role: SessionRole) => void;
}) {
  return (
    <div className="router-role-toggle">
      {(["loggedOut", "user", "admin"] as SessionRole[]).map((nextRole) => (
        <button
          key={nextRole}
          className={activeRole === nextRole ? "is-active" : ""}
          onClick={() => onSelect(nextRole)}
          type="button"
        >
          {roleLabels[nextRole]}
        </button>
      ))}
    </div>
  );
}

export function RouterSection({ onNavigate }: RouterSectionProps) {
  const {
    activeTarget,
    bootError,
    browserPath,
    browserResult,
    currentOutcome,
    isNavigating,
    model,
    pageStatusKind,
    pageValue,
    replayOutput,
    replayResult,
    replayRole,
    role,
    routeOptions,
    setActiveTarget,
    setReplayRole,
    setRole,
    signalsReady,
  } = useRouterSectionState();

  if (bootError) {
    return <div className="xai-section-band accent-router">{bootError}</div>;
  }

  if (!signalsReady || !model) {
    return (
      <div className="xai-section-band accent-router">
        <div className="xai-section-heading">
          <span className="xai-section-eyebrow">04 / Router</span>
          <h2>Routing is where guards, loading, and permissions meet.</h2>
        </div>
        <article className="router-browser-card">
          <div className="router-browser-stage is-loading">
            <div className="router-spinner" aria-hidden="true" />
          </div>
        </article>
      </div>
    );
  }

  return (
    <div className="xai-section-band accent-router">
      <div className="xai-section-heading">
        <span className="xai-section-eyebrow">04 / Router</span>
        <h2>Routing is where guards, loading, and permissions meet.</h2>
        <p>
          Browser ingress, route admission, route-local resources, and replayable
          history all come from the same router surface.
        </p>
      </div>

      <DemoPreface demoId={4} />

      <article className="router-test-card">
        <div className="forms-card-topline">
          <span>Router authoring</span>
        </div>
        <h3>Typed routes, prerequisites, and route-local resources.</h3>
        <RouterSectionCodeSample />
      </article>

      <div className="router-live-layout">
        <div className="router-browser-column">
          <div className="router-role-bar router-role-bar-inline">
            <span>Session role</span>
            <RouterRoleToggle activeRole={role} onSelect={setRole} />
          </div>

          <article className="router-browser-card">
            <div className="router-browser-chrome">
              <div className="router-browser-dots" aria-hidden="true">
                <span />
                <span />
                <span />
              </div>
              <div className="router-browser-path">{browserPath}</div>
            </div>

            <div className="router-browser-nav">
              {routeOptions.map((route) => (
                <button
                  key={route.path}
                  className={activeTarget === route.path ? "is-active" : ""}
                  onClick={() => setActiveTarget(route.path)}
                  type="button"
                >
                  {route.label}
                </button>
              ))}
            </div>

            <RouterSectionBrowserSurface
              isNavigating={isNavigating}
              outcome={currentOutcome}
              pageData={pageValue}
              statusKind={pageStatusKind}
            />
          </article>
        </div>

        <article className="router-state-card">
          <div className="forms-card-topline">
            <span>Route history story</span>
          </div>
          <h3>One session grows until the role changes.</h3>
          <pre className="router-state-output">{browserResult}</pre>
        </article>
      </div>

      <article className="router-test-card">
        <div className="forms-card-topline">
          <span>Simplify testing</span>
        </div>
        <h3>Replay the session history you just created.</h3>
        <p className="router-replay-copy">
          Switch roles and rerun the same navigation history without rebuilding
          the workflow by hand.
        </p>

        <div className="router-replay-row">
          <span>Replay this session as</span>
          <RouterRoleToggle activeRole={replayRole} onSelect={setReplayRole} />
        </div>

        <pre className="router-code-output">
          {replayResult
            ? replayOutput
            : `{
  "role": "${replayRole}",
  "steps": [],
  "current": null,
  "history": []
}`}
        </pre>
      </article>

      <div className="signals-cta-row">
        <div className="signals-cta-copy">
          Switch roles to change admission outcomes, then replay the same session
          history in one click.
        </div>
        <div className="xai-section-actions">
          <button className="xai-button xai-button-primary" onClick={() => onNavigate("#/demos/4")} type="button">
            Open router demo
          </button>
          <button className="xai-button xai-button-secondary" onClick={() => onNavigate("#/docs/router/index")} type="button">
            Read router docs
          </button>
        </div>
      </div>
    </div>
  );
}
