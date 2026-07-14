import { useCallback, useEffect, useMemo, useRef } from "react";

import { useMaybeReactSignalsStore } from "./context.js";
import { useBrowserHistoryStory } from "./history.js";

import type {
  BrowserHistoryStoryReactLike,
  BrowserHistoryStoryView,
  ReactSignalsStore,
} from "./model.js";
import type {
  RouterSessionOptions,
  RouterSessionRoutesReactLike,
  RouterSessionView,
  RouterSessionNavigateOptions,
  SignalsWithRouterLike,
} from "./router_model.js";

type RouterSessionCacheEntry<TStory extends BrowserHistoryStoryReactLike> = {
  story: TStory;
  initialized: boolean;
};

const routerSessionCache = new WeakMap<object, WeakMap<object, RouterSessionCacheEntry<BrowserHistoryStoryReactLike>>>();

function requireRouterSessionStore<TSignals extends SignalsWithRouterLike>(
  explicitStore: ReactSignalsStore<TSignals> | undefined,
  providerStore: ReactSignalsStore | null,
): ReactSignalsStore<TSignals> {
  if (explicitStore) {
    return explicitStore;
  }
  if (providerStore) {
    return providerStore as ReactSignalsStore<TSignals>;
  }
  throw new TypeError(
    "React signals store was not provided. Wrap the tree with <ReactSignalsStoreProvider store={...}> or pass store explicitly.",
  );
}

function readDefaultBrowserLocation(): string | null {
  if (typeof window === "undefined" || !window.location) {
    return null;
  }
  return `${window.location.pathname}${window.location.search}${window.location.hash}`;
}

function createIngressOptions(
  signals: SignalsWithRouterLike,
  story: BrowserHistoryStoryReactLike,
  options: RouterSessionNavigateOptions<object, unknown> | undefined,
) {
  const baseOptions = options?.ingress ? { ...options.ingress } : {};
  if (
    options?.carryBreadcrumbs === false
    || typeof signals.router.carryBreadcrumbs !== "function"
  ) {
    return baseOptions;
  }
  return Object.freeze({
    ...baseOptions,
    carriedBreadcrumbs: signals.router.carryBreadcrumbs(story.breadcrumbTrail()),
  });
}

function selectHistoryMethod(
  historyMethod: RouterSessionNavigateOptions["historyMethod"],
) {
  return historyMethod ?? "push";
}

function getRetainedRouterSessionEntry<
  TStory extends BrowserHistoryStoryReactLike,
  TSignals extends SignalsWithRouterLike<any, any, any, TStory>,
  TRoutes extends RouterSessionRoutesReactLike,
>(
  signals: TSignals,
  routes: TRoutes,
): RouterSessionCacheEntry<TStory> {
  let routeCache = routerSessionCache.get(signals as object);
  if (!routeCache) {
    routeCache = new WeakMap();
    routerSessionCache.set(signals as object, routeCache);
  }
  const cached = routeCache.get(routes as object) as RouterSessionCacheEntry<TStory> | undefined;
  if (cached) {
    return cached;
  }
  const created = {
    story: signals.router.browserHistory.story(),
    initialized: false,
  } satisfies RouterSessionCacheEntry<TStory>;
  routeCache.set(routes as object, created);
  return created;
}

export function useRouterSession<
  TLocation = string,
  TIngress = unknown,
  TReport = unknown,
  TFacts = unknown,
  TStoryEntry = unknown,
  TStoryEvent = unknown,
  TBreadcrumbTrail = unknown,
  TBackProvenance = unknown,
  TStory extends BrowserHistoryStoryReactLike<
    TStoryEntry,
    TStoryEvent,
    TBreadcrumbTrail,
    TBackProvenance
  > = BrowserHistoryStoryReactLike<TStoryEntry, TStoryEvent, TBreadcrumbTrail, TBackProvenance>,
  TSignals extends SignalsWithRouterLike<
    TLocation,
    TIngress,
    TReport,
    TStory
  > = SignalsWithRouterLike<TLocation, TIngress, TReport, TStory>,
  TRoutes extends RouterSessionRoutesReactLike<TIngress, TReport, TFacts> = RouterSessionRoutesReactLike<TIngress, TReport, TFacts>,
>(
  routes: TRoutes,
  options: RouterSessionOptions<TSignals, TLocation, TFacts>,
): RouterSessionView<
  TStoryEntry,
  BrowserHistoryStoryView<TStoryEntry, TStoryEvent, TBreadcrumbTrail, TBackProvenance>,
  TBreadcrumbTrail,
  TLocation,
  TReport,
  Record<string, unknown>,
  TFacts
> {
  const providerStore = useMaybeReactSignalsStore();
  const resolvedStore = requireRouterSessionStore(options.store, providerStore);
  const sessionRef = useRef<RouterSessionCacheEntry<TStory> | null>(null);
  const retainedEntry = getRetainedRouterSessionEntry(
    resolvedStore.signals as TSignals,
    routes,
  ) as RouterSessionCacheEntry<TStory>;
  if (sessionRef.current === null || sessionRef.current.story !== retainedEntry.story) {
    sessionRef.current = retainedEntry;
  }

  const storyHandle = sessionRef.current!.story;
  const story = useBrowserHistoryStory(storyHandle);

  const navigate = useCallback(async (
    location: TLocation,
    navigateOptions?: RouterSessionNavigateOptions<Record<string, unknown>, TFacts>,
  ) => {
    const signals = resolvedStore.signals;
    const historyMethod = selectHistoryMethod(navigateOptions?.historyMethod);
    const ingress = signals.router.browserHistory[historyMethod](
      location,
      createIngressOptions(signals, storyHandle, navigateOptions),
    ) as TIngress;
    const report = await routes.admitBrowserHistoryIngress(
      ingress,
      navigateOptions?.facts,
    );
    storyHandle.record(report as TReport);
    return report;
  }, [resolvedStore, routes, storyHandle]);

  useEffect(() => {
    if (sessionRef.current?.initialized) {
      return;
    }
    if (sessionRef.current === null) {
      return;
    }
    sessionRef.current.initialized = true;
    const initialLocation = options.initialLocation ?? (readDefaultBrowserLocation() as TLocation | null);
    if (initialLocation === null) {
      return;
    }
    void navigate(initialLocation, {
      historyMethod: "load",
      carryBreadcrumbs: false,
      facts: options.initialFacts,
    });
  }, [navigate, options.initialFacts, options.initialLocation]);

  return useMemo(() => Object.freeze({
    currentRoute: story.current as TStoryEntry | null,
    story,
    breadcrumbs: story.breadcrumbTrail,
    navigate,
  }) as RouterSessionView<
    TStoryEntry,
    BrowserHistoryStoryView<TStoryEntry, TStoryEvent, TBreadcrumbTrail, TBackProvenance>,
    TBreadcrumbTrail,
    TLocation,
    TReport,
    Record<string, unknown>,
    TFacts
  >, [navigate, story]);
}
