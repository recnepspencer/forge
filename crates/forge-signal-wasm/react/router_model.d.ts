import type {
  BrowserHistoryStoryReactLike,
  BrowserHistoryStoryView,
  ReactSignalsStore,
} from "./model.js";

export interface RouterHistoryMethodMapReactLike<
  TLocation = unknown,
  TIngress = unknown,
  TIngressOptions extends object = Record<string, unknown>,
> {
  load(location: TLocation, options?: TIngressOptions): TIngress;
  push(location: TLocation, options?: TIngressOptions): TIngress;
  replace(location: TLocation, options?: TIngressOptions): TIngress;
  pop(location: TLocation, options?: TIngressOptions): TIngress;
  manual(location: TLocation, options?: TIngressOptions): TIngress;
  external(location: TLocation, options?: TIngressOptions): TIngress;
}

export interface RouterBrowserHistoryNamespaceReactLike<
  TLocation = unknown,
  TIngress = unknown,
  TReport = unknown,
  TStory extends BrowserHistoryStoryReactLike = BrowserHistoryStoryReactLike,
  TIngressOptions extends object = Record<string, unknown>,
> extends RouterHistoryMethodMapReactLike<TLocation, TIngress, TIngressOptions> {
  story(initialReport?: TReport): TStory;
}

export interface RouterNamespaceReactLike<
  TLocation = unknown,
  TIngress = unknown,
  TReport = unknown,
  TStory extends BrowserHistoryStoryReactLike = BrowserHistoryStoryReactLike,
  TIngressOptions extends object = Record<string, unknown>,
  TCarriedBreadcrumbs = unknown,
> {
  readonly browserHistory: RouterBrowserHistoryNamespaceReactLike<
    TLocation,
    TIngress,
    TReport,
    TStory,
    TIngressOptions
  >;
  carryBreadcrumbs?(trail: ReturnType<TStory["breadcrumbTrail"]>): TCarriedBreadcrumbs;
}

export interface SignalsWithRouterLike<
  TLocation = unknown,
  TIngress = unknown,
  TReport = unknown,
  TStory extends BrowserHistoryStoryReactLike = BrowserHistoryStoryReactLike,
  TIngressOptions extends object = Record<string, unknown>,
  TCarriedBreadcrumbs = unknown,
> {
  readonly router: RouterNamespaceReactLike<
    TLocation,
    TIngress,
    TReport,
    TStory,
    TIngressOptions,
    TCarriedBreadcrumbs
  >;
}

export interface RouterSessionRoutesReactLike<
  TIngress = unknown,
  TReport = unknown,
  TFacts = unknown,
> {
  admitBrowserHistoryIngress(ingress: TIngress, facts?: TFacts): Promise<TReport>;
}

export interface RouterSessionOptions<
  TSignals extends SignalsWithRouterLike = SignalsWithRouterLike,
  TLocation = unknown,
  TFacts = unknown,
> {
  readonly history: "browser";
  readonly store?: ReactSignalsStore<TSignals>;
  readonly initialLocation?: TLocation | null;
  readonly initialFacts?: TFacts;
}

export interface RouterSessionNavigateOptions<
  TIngressOptions extends object = Record<string, unknown>,
  TFacts = unknown,
> {
  readonly historyMethod?: "push" | "replace" | "load" | "pop" | "manual" | "external";
  readonly carryBreadcrumbs?: boolean;
  readonly ingress?: TIngressOptions;
  readonly facts?: TFacts;
}

export interface RouterSessionView<
  TCurrentRoute = unknown,
  TStoryView extends BrowserHistoryStoryView = BrowserHistoryStoryView,
  TBreadcrumbs = unknown,
  TLocation = unknown,
  TReport = unknown,
  TIngressOptions extends object = Record<string, unknown>,
  TFacts = unknown,
> {
  readonly currentRoute: TCurrentRoute | null;
  readonly story: TStoryView;
  readonly breadcrumbs: TBreadcrumbs;
  navigate(
    location: TLocation,
    options?: RouterSessionNavigateOptions<TIngressOptions, TFacts>,
  ): Promise<TReport>;
}
