import React from "react";
import { createSignals } from "forge-signal-wasm";

import {
  buildRouterSectionModel,
  formatBrowserResult,
  formatSequenceResult,
  roleLabels,
  type SessionRole,
} from "./routerSectionSupport";

interface RouterSectionState {
  bootError: string | null;
  browserPath: string;
  browserResult: string;
  currentOutcome: any;
  isNavigating: boolean;
  model: any;
  pageStatusKind: string | null;
  pageValue: any;
  replayOutput: string;
  replayResult: any;
  replayRole: SessionRole;
  role: SessionRole;
  routeOptions: ReadonlyArray<{ path: string; label: string }>;
  setActiveTarget: React.Dispatch<React.SetStateAction<string>>;
  setReplayRole: React.Dispatch<React.SetStateAction<SessionRole>>;
  setRole: React.Dispatch<React.SetStateAction<SessionRole>>;
  signalsReady: boolean;
  activeTarget: string;
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
  const [role, setRole] = React.useState<SessionRole>("admin");
  const [activeTarget, setActiveTarget] = React.useState("/catalog");
  const [currentReport, setCurrentReport] = React.useState<any>(null);
  const [pageLine, setPageLine] = React.useState<any>(null);
  const [pageRevision, setPageRevision] = React.useState(0);
  const [replayRole, setReplayRole] = React.useState<SessionRole>("admin");
  const [replayResult, setReplayResult] = React.useState<any>(null);
  const [bootError, setBootError] = React.useState<string | null>(null);
  const [isNavigating, setIsNavigating] = React.useState(false);
  const navigationTokenRef = React.useRef(0);
  const storyRevision = useStorySubscription(story);

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

  React.useEffect(() => {
    if (!signals || !model) {
      return;
    }

    setStory(signals.router.browserHistory.story());
    setCurrentReport(null);
    setPageLine(null);
  }, [signals, model, role]);

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
          ? signals.router.browserHistory.load(activeTarget, { routeIdentity: "router-section" })
          : signals.router.browserHistory.push(activeTarget, { routeIdentity: "router-section" });
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
  }, [activeTarget, currentReport, story, storyRevision]);

  const browserResult = React.useMemo(
    () => formatBrowserResult(role, currentReport, story, pageLine),
    [role, currentReport, story, storyRevision, pageLine, pageStatusKind, pageRevision],
  );

  const replayOutput = React.useMemo(
    () => formatSequenceResult(replayRole, replayResult),
    [replayRole, replayResult],
  );

  return {
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
    routeOptions: model?.routeOptions ?? [],
    setActiveTarget,
    setReplayRole,
    setRole,
    signalsReady: Boolean(signals && model && story),
    activeTarget,
  };
}

export { roleLabels, type SessionRole };
