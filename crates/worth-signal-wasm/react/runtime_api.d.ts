import type { JSX, ReactNode } from "react";

import type {
  BrowserHistoryStoryReactLike,
  BrowserHistoryStoryView,
  ReactSignalsStore,
  SignalHandleLike,
  SignalsDiagnosticsSnapshot,
  SignalsHistoryReactLike,
  SignalsHistoryView,
  SignalsLike,
  OptionalSignalValueResult,
} from "./model.js";
import type {
  RouterSessionOptions,
  RouterSessionRoutesReactLike,
  RouterSessionView,
  SignalsWithRouterLike,
} from "./router_model.js";

export declare function createReactSignalsStore<TSignals extends SignalsLike>(
  signals: TSignals,
): ReactSignalsStore<TSignals>;

export declare function ReactSignalsStoreProvider(props: {
  store: ReactSignalsStore;
  children?: ReactNode;
}): JSX.Element;

export declare function useReactSignalsStore(): ReactSignalsStore;

export declare function useSignalValue<T = unknown>(
  signal: SignalHandleLike,
  store: ReactSignalsStore,
): T;

export declare function useOutputValue<T = unknown>(
  output: SignalHandleLike,
  store: ReactSignalsStore,
): T;

export declare function useOptionalSignalValue<TValue = unknown, TInactive = undefined>(
  signal: SignalHandleLike | null | undefined,
  store: ReactSignalsStore,
  options?: {
    inactiveValue?: TInactive;
  },
): OptionalSignalValueResult<TValue, TInactive>;

export declare function useSignalsDiagnostics(
  store?: ReactSignalsStore,
): SignalsDiagnosticsSnapshot;

export declare function useSignalsDiagnosticsValue<TValue>(
  selector: (snapshot: SignalsDiagnosticsSnapshot) => TValue,
  store?: ReactSignalsStore,
): TValue;

export declare function useSignalsHistory<
  TBranch = unknown,
  THistory extends SignalsHistoryReactLike<TBranch> = SignalsHistoryReactLike<TBranch>,
>(
  history: THistory,
): SignalsHistoryView<TBranch>;

export declare function useBrowserHistoryStory<
  TEntry = unknown,
  TEvent = unknown,
  TBreadcrumbTrail = unknown,
  TBackProvenance = unknown,
  TStory extends BrowserHistoryStoryReactLike<
    TEntry,
    TEvent,
    TBreadcrumbTrail,
    TBackProvenance
  > = BrowserHistoryStoryReactLike<TEntry, TEvent, TBreadcrumbTrail, TBackProvenance>,
>(
  story: TStory,
): BrowserHistoryStoryView<TEntry, TEvent, TBreadcrumbTrail, TBackProvenance>;

export declare function useRouterSession<
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
>;
