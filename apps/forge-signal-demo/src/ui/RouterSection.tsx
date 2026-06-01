import React from "react";
import { createSignals } from "forge-signal-wasm";

import "./routerSection.css";
import { RouterSectionBrowserSurface } from "./RouterSectionBrowserSurface";
import { RouterSectionCodeSample } from "./RouterSectionCodeSample";
import {
  buildRouterSectionModel,
  formatBrowserResult,
  formatSequenceResult,
  roleLabels,
  type SessionRole,
} from "./routerSectionSupport";

interface RouterSectionProps {
  onNavigate: (path: string) => void;
}

export function RouterSection({ onNavigate }: RouterSectionProps): React.ReactElement {
  const [signals, setSignals] = React.useState<any>(null);
  const [model, setModel] = React.useState<any>(null);
  const [story, setStory] = React.useState<any>(null);
  const [role, setRole] = React.useState<SessionRole>("admin");
  const [activeTarget, setActiveTarget] = React.useState("/catalog");
  const [currentReport, setCurrentReport] = React.useState<any>(null);
  const [pageLine, setPageLine] = React.useState<any>(null);
  const [pageRevision, setPageRevision] = React.useState(0);
  const [storyRevision, setStoryRevision] = React.useState(0);
  const [replayRole, setReplayRole] = React.useState<SessionRole>("admin");
  const [replayResult, setReplayResult] = React.useState<any>(null);
  const [bootError, setBootError] = React.useState<string | null>(null);
  const [isNavigating, setIsNavigating] = React.useState(false);
  const navigationTokenRef = React.useRef(0);
  const storySubscriptionRef = React.useRef<(() => void) | null>(null);

  const installStory = React.useCallback((nextStory: any) => {
    storySubscriptionRef.current?.();
    storySubscriptionRef.current = nextStory.subscribe(() => {
      setStoryRevision((value) => value + 1);
    });
    setStory(nextStory);
    setStoryRevision((value) => value + 1);
  }, []);

  React.useEffect(() => {
    let cancelled = false;

    createSignals({ deployment: "mainThreadCompatibility" })
      .then((instance) => {
        if (cancelled) {
          return;
        }

        const nextModel = buildRouterSectionModel(instance);
        setSignals(instance);
        setModel(nextModel);
        installStory(instance.router.browserHistory.story());
        setActiveTarget(nextModel.initialTarget);
      })
      .catch((error) => {
        console.error("Failed to boot router section signals", error);
        if (!cancelled) {
          setBootError("Router runtime failed to initialize.");
        }
      });

    return () => {
      cancelled = true;
      storySubscriptionRef.current?.();
      storySubscriptionRef.current = null;
    };
  }, [installStory]);

  React.useEffect(() => {
    if (!signals || !model) {
      return;
    }

    installStory(signals.router.browserHistory.story());
    setCurrentReport(null);
    setPageLine(null);
  }, [signals, model, role, installStory]);

  React.useEffect(() => {
    if (!signals || !model || !story) {
      return;
    }

    const token = ++navigationTokenRef.current;
    const navigationKind = story.events().length === 0 ? "load" : "push";
    setIsNavigating(true);
    setPageLine(null);
    setPageRevision((value) => value + 1);

    (async () => {
      const ingress =
        navigationKind === "load"
          ? signals.router.browserHistory.load(activeTarget, {
              routeIdentity: "router-section",
            })
          : signals.router.browserHistory.push(activeTarget, {
              routeIdentity: "router-section",
            });
      const report = await model.routes.admitBrowserHistoryIngress(ingress, { role });

      if (token !== navigationTokenRef.current) {
        return;
      }

      story.record(report);
      setCurrentReport(report);

      const outcome = report.outcome();
      if (outcome.kind !== "admitted" || !outcome.route().resourceNames().includes("page")) {
        setPageLine(null);
        setPageRevision((value) => value + 1);
        setIsNavigating(false);
        return;
      }

      const nextLine = outcome.route().resource("page").line();
      setPageLine(nextLine);
      setPageRevision((value) => value + 1);
      nextLine.invalidate();
      nextLine.refresh();
      await nextLine.awaitSettlement();

      if (token === navigationTokenRef.current) {
        setPageRevision((value) => value + 1);
        setIsNavigating(false);
      }
    })().catch((error) => {
      console.error("Failed to admit router section route", error);
      if (token === navigationTokenRef.current) {
        setIsNavigating(false);
      }
    });
  }, [signals, model, story, role, activeTarget]);

  const replayTargets = React.useMemo(
    () => story?.events?.().map((event: any) => event.targetHref).filter(Boolean) ?? [],
    [story, storyRevision],
  );

  React.useEffect(() => {
    if (!model) {
      return;
    }

    if (replayTargets.length === 0) {
      setReplayResult(null);
      return;
    }

    let cancelled = false;

    (async () => {
      const scenario = model.routes.simulateSequence(replayTargets);
      const result = await scenario.run({ facts: { role: replayRole } });
      if (!cancelled) {
        setReplayResult(result);
      }
    })().catch((error) => {
      console.error("Failed to replay router section workflow", error);
    });

    return () => {
      cancelled = true;
    };
  }, [model, replayRole, replayTargets]);

  const currentOutcome = currentReport?.outcome?.() ?? null;
  const pageStatusKind = pageLine?.status?.().kind ?? null;
  const pageValue = pageLine && pageStatusKind === "fulfilled" ? pageLine.value() : null;
  const browserPath = React.useMemo(() => {
    if (!currentReport) {
      return activeTarget;
    }

    const outcome = currentReport.outcome();
    if (outcome.kind === "redirect") {
      return outcome.artifact().href ?? currentReport.rawLocationHref;
    }

    return story?.current?.()?.href ?? currentReport.rawLocationHref;
  }, [activeTarget, currentReport, storyRevision, story]);

  const browserResult = React.useMemo(
    () => formatBrowserResult(role, currentReport, story, pageLine),
    [role, currentReport, story, storyRevision, pageLine, pageStatusKind, pageRevision],
  );
  const replayOutput = React.useMemo(
    () => formatSequenceResult(replayRole, replayResult),
    [replayRole, replayResult],
  );

  if (bootError) {
    return <div className="xai-section-band accent-router">{bootError}</div>;
  }

  if (!signals || !model || !story) {
    return (
      <div className="xai-section-band accent-router">
        <div className="xai-section-heading">
          <span className="xai-section-eyebrow">03 / Router</span>
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
        <span className="xai-section-eyebrow">03 / Router</span>
        <h2>Routing is where guards, loading, and permissions meet.</h2>
        <p>
          Browser ingress, route admission, route-local resources, and replayable
          history all come from the same router surface.
        </p>
      </div>

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
            <div className="router-role-toggle">
              {(["loggedOut", "user", "admin"] as SessionRole[]).map((nextRole) => (
                <button
                  key={nextRole}
                  className={role === nextRole ? "is-active" : ""}
                  onClick={() => setRole(nextRole)}
                  type="button"
                >
                  {roleLabels[nextRole]}
                </button>
              ))}
            </div>
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
              {model.routeOptions.map((route: { path: string; label: string }) => (
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
          <div className="router-role-toggle">
            {(["loggedOut", "user", "admin"] as SessionRole[]).map((nextRole) => (
              <button
                key={nextRole}
                className={replayRole === nextRole ? "is-active" : ""}
                onClick={() => setReplayRole(nextRole)}
                type="button"
              >
                {roleLabels[nextRole]}
              </button>
            ))}
          </div>
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
          <button className="xai-button xai-button-primary" onClick={() => onNavigate("#/demos/3")} type="button">
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
