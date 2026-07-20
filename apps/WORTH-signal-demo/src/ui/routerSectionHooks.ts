import React from "react";
import { createSignals } from "worth-signals-wasm";

import {
  currentRoutePresentation,
  routeRequestKey,
  type RoutePresentation,
} from "./routerSectionPresentation";
import {
  ROLE_TRAINED_REV,
  REPLAY_PERSONAS,
  buildRouterSectionModel,
  describeOutcome,
  roleLabels,
  type OutcomeView,
  type PlantRole,
  type SopRevision,
} from "./routerSectionSupport";

const REV_BUMP_DELAY_MS = 14_000;
const REPLAY_WINDOW = 6;

export interface AccessLogEntry {
  id: number;
  at: string;
  role: PlantRole;
  trainedRev: SopRevision;
  effectiveRev: SopRevision;
  target: string;
  outcome: OutcomeView;
  raw: unknown;
}

export interface ReplayRow {
  target: string;
  outcomes: Record<string, OutcomeView>;
}

interface RouterSectionState {
  accessLog: AccessLogEntry[];
  activeTarget: string;
  bootError: string | null;
  currentOutcome: OutcomeView | null;
  deviationGranted: boolean;
  effectiveRev: SopRevision;
  grantDeviation: () => void;
  isNavigating: boolean;
  model: any;
  navigate: (target: string) => void;
  pageValue: any;
  replayRows: ReplayRow[];
  revBumped: boolean;
  role: PlantRole;
  routeOptions: ReadonlyArray<{ path: string; label: string }>;
  setRole: (role: PlantRole) => void;
  signalsReady: boolean;
  story: any;
  storyRevision: number;
  trainedRev: SopRevision;
}

interface RouterPageLine {
  awaitSettlement: () => Promise<unknown>;
  invalidate: () => void;
  refresh: () => void;
  status: () => { kind: string };
  value: () => unknown;
}

function useStorySubscription(story: any): number {
  const [revision, setRevision] = React.useState(0);

  React.useEffect(() => {
    if (!story?.subscribe) {
      return;
    }
    const dispose = story.subscribe(() => {
      setRevision((value) => value + 1);
    });
    setRevision((value) => value + 1);
    return () => {
      dispose?.();
    };
  }, [story]);

  return revision;
}

export function useRouterSectionState(): RouterSectionState {
  const [signals, setSignals] = React.useState<any>(null);
  const [model, setModel] = React.useState<any>(null);
  const [story, setStory] = React.useState<any>(null);
  const [bootError, setBootError] = React.useState<string | null>(null);

  const [role, setRoleState] = React.useState<PlantRole>("operator");
  const [effectiveRev, setEffectiveRev] = React.useState<SopRevision>("B");
  const [deviationGranted, setDeviationGranted] = React.useState(false);
  const [activeTarget, setActiveTarget] = React.useState<string>("");
  const [navNonce, setNavNonce] = React.useState(0);

  const [presentation, setPresentation] = React.useState<
    RoutePresentation<unknown, RouterPageLine> | null
  >(null);
  const [, setPageRevision] = React.useState(0);
  const [isNavigating, setIsNavigating] = React.useState(false);

  const [accessLog, setAccessLog] = React.useState<AccessLogEntry[]>([]);
  const [replayRows, setReplayRows] = React.useState<ReplayRow[]>([]);
  const logIdRef = React.useRef(0);
  const navigationTokenRef = React.useRef(0);
  const storyRevision = useStorySubscription(story);

  const trainedRev = ROLE_TRAINED_REV[role];
  const revBumped = effectiveRev !== "B";

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
        setStory(instance.router.browserHistory.story());
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
    };
  }, []);

  // Document control bumps SOP-042 to rev C partway through the session.
  React.useEffect(() => {
    if (!model || revBumped) {
      return;
    }
    const handle = window.setTimeout(() => setEffectiveRev("C"), REV_BUMP_DELAY_MS);
    return () => window.clearTimeout(handle);
  }, [model, revBumped]);

  // A role switch is a fresh actor session: new story, same persistent access log.
  const setRole = React.useCallback((nextRole: PlantRole) => {
    setRoleState(nextRole);
    setDeviationGranted(false);
  }, []);

  React.useEffect(() => {
    if (!signals || !model) {
      return;
    }
    setStory(signals.router.browserHistory.story());
    setPresentation(null);
  }, [signals, model, role]);

  const navigate = React.useCallback((target: string) => {
    setActiveTarget(target);
    setNavNonce((value) => value + 1);
  }, []);

  const grantDeviation = React.useCallback(() => {
    setDeviationGranted(true);
    setNavNonce((value) => value + 1);
  }, []);

  React.useEffect(() => {
    if (!signals || !model || !story || !activeTarget) {
      return;
    }

    const token = ++navigationTokenRef.current;
    const requestKey = routeRequestKey({
      activeTarget,
      deviationGranted,
      effectiveRevision: effectiveRev,
      navigationNonce: navNonce,
      role,
    });
    const navigationKind = story.events().length === 0 ? "load" : "push";
    setIsNavigating(true);
    setPageRevision((value) => value + 1);

    const facts = {
      role,
      trainedRev: ROLE_TRAINED_REV[role],
      effectiveRev,
      underDeviation: deviationGranted,
    };

    (async () => {
      const ingress =
        navigationKind === "load"
          ? signals.router.browserHistory.load(activeTarget, { routeIdentity: "router-section" })
          : signals.router.browserHistory.push(activeTarget, { routeIdentity: "router-section" });
      const report = await model.routes.admitBrowserHistoryIngress(ingress, facts);

      if (token !== navigationTokenRef.current) {
        return;
      }

      story.record(report);
      const settledPresentation = { pageLine: null, report, requestKey };
      setPresentation(settledPresentation);

      const outcome = describeOutcome(report, deviationGranted);
      logIdRef.current += 1;
      setAccessLog((current) => [
        {
          id: logIdRef.current,
          at: new Date().toLocaleTimeString("en-US", { hour12: false }),
          role: facts.role,
          trainedRev: facts.trainedRev,
          effectiveRev: facts.effectiveRev,
          target: activeTarget,
          outcome,
          raw: {
            facts,
            outcomeKind: outcome.kind,
            reason: outcome.reason,
            detail: outcome.detail,
            rawLocationHref: report.rawLocationHref ?? null,
          },
        },
        ...current,
      ]);

      const rawOutcome = report.outcome();
      if (rawOutcome.kind !== "admitted" || !rawOutcome.route().resourceNames().includes("page")) {
        setPageRevision((value) => value + 1);
        setIsNavigating(false);
        return;
      }

      const nextLine = rawOutcome.route().resource("page").line();
      setPresentation({ ...settledPresentation, pageLine: nextLine });
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
    // effectiveRev is a deliberate dependency: a revision bump re-admits the
    // page you are standing on, and both outcomes land in the access log
  }, [signals, model, story, role, activeTarget, navNonce, effectiveRev, deviationGranted]);

  const replayTargets = React.useMemo(() => {
    const chronological = [...accessLog].reverse().map((entry) => entry.target);
    const window = chronological.slice(-REPLAY_WINDOW);
    return window.filter((target, index) => window.indexOf(target) === index);
  }, [accessLog]);

  React.useEffect(() => {
    if (!model || replayTargets.length === 0) {
      setReplayRows([]);
      return;
    }

    let cancelled = false;

    (async () => {
      const rows: ReplayRow[] = replayTargets.map((target) => ({ target, outcomes: {} }));
      for (const persona of REPLAY_PERSONAS) {
        const scenario = model.routes.simulateSequence(replayTargets);
        const result = await scenario.run({
          facts: { ...persona.facts, underDeviation: false },
        });
        if (cancelled) {
          return;
        }
        result?.steps?.forEach((step: any, index: number) => {
          if (rows[index]) {
            rows[index].outcomes[persona.id] = describeOutcome(step.report, false);
          }
        });
      }
      if (!cancelled) {
        setReplayRows(rows);
      }
    })().catch((error) => {
      console.error("Failed to replay router section session", error);
    });

    return () => {
      cancelled = true;
    };
  }, [model, replayTargets, effectiveRev]);

  const currentRequestKey = routeRequestKey({
    activeTarget,
    deviationGranted,
    effectiveRevision: effectiveRev,
    navigationNonce: navNonce,
    role,
  });
  const currentPresentation = currentRoutePresentation(presentation, currentRequestKey);
  const currentOutcome = React.useMemo(
    () =>
      currentPresentation
        ? describeOutcome(currentPresentation.report, deviationGranted)
        : null,
    [currentPresentation, deviationGranted],
  );

  const pageStatusKind = currentPresentation?.pageLine?.status?.().kind ?? null;
  const pageValue =
    currentPresentation?.pageLine && pageStatusKind === "fulfilled"
      ? currentPresentation.pageLine.value()
      : null;

  return {
    accessLog,
    activeTarget,
    bootError,
    currentOutcome,
    deviationGranted,
    effectiveRev,
    grantDeviation,
    isNavigating: isNavigating || currentPresentation === null,
    model,
    navigate,
    pageValue,
    replayRows,
    revBumped,
    role,
    routeOptions: model?.routeOptions ?? [],
    setRole,
    signalsReady: Boolean(signals && model && story),
    story,
    storyRevision,
    trainedRev,
  };
}

export { roleLabels, type PlantRole };
