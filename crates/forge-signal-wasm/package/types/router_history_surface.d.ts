import type {
  RawLocationAuthority,
} from "./router_authority_surface.js";
import type {
  RouteRestoredBreadcrumbs,
  RouteCarriedBreadcrumbs,
  RouteBreadcrumbEntry,
  RouteBreadcrumbProvenance,
} from "./router_breadcrumb_surface.js";
import type {
  RouteHistoryReplayResult,
  RouteHistoryRestoreResult,
  RouteReplayHistoryFacade,
  RouteRestoreBoundary,
  RouteRestoreHistoryFacade,
} from "./router_restore_surface.js";
import type {
  RouteLocation,
} from "./router_surface.js";
import type {
  RouteOutcome,
  RouteAdmissionFacts,
} from "./router_admission_surface.js";
import type {
  RouterBrowserHistoryInspection,
  RouterBrowserHistoryInspectionVerificationPackage,
  RouterBrowserHistoryInspectionSummary,
  RouterBrowserHistoryOutletComposition,
  RouterBrowserHistoryOutletCompositionVerificationPackage,
  RouterNavigationAuditability,
} from "./router_history_auditability_surface.js";
import type {
  RouterHydrationAdmissionReport,
} from "./router_hydration_surface.js";

declare const forgeSignalRouterBrowserHistoryIngressBrand: unique symbol;
declare const forgeSignalRouterBrowserHistoryIngressVerificationBrand: unique symbol;
declare const forgeSignalRouterBrowserHistoryAdmissionVerificationBrand: unique symbol;
declare const forgeSignalRouterBrowserAuthorityCoherenceBrand: unique symbol;
declare const forgeSignalRouterBrowserAuthorityCoherenceVerificationBrand: unique symbol;
declare const forgeSignalRouterBrowserHistoryWritebackBrand: unique symbol;
declare const forgeSignalRouterBrowserHistoryWritebackVerificationBrand: unique symbol;
declare const forgeSignalRouterBrowserHistoryWritebackReportVerificationBrand: unique symbol;
declare const forgeSignalRouterBrowserHistoryStoryEntryVerificationBrand: unique symbol;
declare const forgeSignalRouterBrowserHistoryBoundaryEventVerificationBrand: unique symbol;
declare const forgeSignalRouterBrowserHistoryStoryVerificationBrand: unique symbol;
declare const forgeSignalRouterBrowserHistoryBackProvenanceVerificationBrand: unique symbol;
declare const forgeSignalRouterBrowserHistoryBreadcrumbTrailVerificationBrand: unique symbol;

export type BrowserHistoryNavigationKind =
  | "load"
  | "pushstate"
  | "replacestate"
  | "popstate"
  | "manual"
  | "external";

export type BrowserAuthorityCoherenceKind =
  | "sameTab"
  | "crossTab"
  | "externalNavigation";

export interface RouterBrowserAuthorityCoherenceVerificationPackage {
  readonly browserAuthorityCoherenceDigest: string;
  readonly [forgeSignalRouterBrowserAuthorityCoherenceVerificationBrand]: "routerBrowserAuthorityCoherenceVerificationPackage";
}

export interface RouterBrowserAuthorityCoherence {
  readonly kind: "routerBrowserAuthorityCoherence";
  readonly coherenceKind: BrowserAuthorityCoherenceKind;
  readonly channelId: string | null;
  readonly sourceTabId: string | null;
  readonly expectedRouteId: string | null;
  verification(): RouterBrowserAuthorityCoherenceVerificationPackage;
  readonly [forgeSignalRouterBrowserAuthorityCoherenceBrand]: "routerBrowserAuthorityCoherence";
}

export interface RouterBrowserAuthorityCoherenceNamespace {
  sameTab(options?: { readonly sourceTabId?: string; readonly channelId?: string; readonly expectedRouteId?: string }): RouterBrowserAuthorityCoherence;
  crossTab(channelId: string, options: { readonly sourceTabId: string; readonly expectedRouteId?: string }): RouterBrowserAuthorityCoherence;
  externalNavigation(options?: { readonly sourceTabId?: string; readonly channelId?: string; readonly expectedRouteId?: string }): RouterBrowserAuthorityCoherence;
}

export interface RouterBrowserHistoryIngressVerificationPackage {
  readonly browserHistoryEnvelopeDigest: string;
  readonly [forgeSignalRouterBrowserHistoryIngressVerificationBrand]: "routerBrowserHistoryIngressVerificationPackage";
}

export interface RouterBrowserHistoryIngressOptions {
  readonly routeIdentity?: string;
  readonly runtimeRouteSourceId?: string;
  readonly routeValue?: unknown;
  readonly runtimeContinuitySourceId?: string;
  readonly continuityValue?: unknown;
  readonly coherence?: RouterBrowserAuthorityCoherence;
  readonly restoredBreadcrumbs?: RouteRestoredBreadcrumbs;
  readonly carriedBreadcrumbs?: RouteCarriedBreadcrumbs;
  readonly restoreBoundary?: RouteRestoreBoundary;
}

export interface RouterBrowserHistoryWritebackOptions {
  readonly routeIdentity?: string;
  readonly runtimeRouteSourceId?: string;
  readonly routeValue?: unknown;
  readonly runtimeContinuitySourceId?: string;
  readonly continuityValue?: unknown;
  readonly coherence?: RouterBrowserAuthorityCoherence;
  readonly restoredBreadcrumbs?: RouteRestoredBreadcrumbs;
  readonly carriedBreadcrumbs?: RouteCarriedBreadcrumbs;
  readonly restoreBoundary?: RouteRestoreBoundary;
}

export interface RouterBrowserHistoryLocalWritebackOptions
  extends RouterBrowserHistoryWritebackOptions {
  readonly routeIdentity: string;
}

export interface RouterBrowserHistoryIngress {
  readonly kind: "routerBrowserHistoryIngress";
  readonly navigationKind: BrowserHistoryNavigationKind;
  readonly rawLocation: RawLocationAuthority;
  readonly routeIdentity: string | null;
  readonly runtimeRouteSourceId: string | null;
  readonly routeValue: unknown;
  readonly runtimeContinuitySourceId: string | null;
  readonly continuityValue: unknown;
  readonly coherence: RouterBrowserAuthorityCoherence | null;
  readonly restoredBreadcrumbs: RouteRestoredBreadcrumbs | null;
  readonly carriedBreadcrumbs: RouteCarriedBreadcrumbs | null;
  readonly restoreBoundary: RouteRestoreBoundary | null;
  verification(): RouterBrowserHistoryIngressVerificationPackage;
  readonly [forgeSignalRouterBrowserHistoryIngressBrand]: "routerBrowserHistoryIngress";
}

export interface RouterBrowserHistoryNamespace {
  load(location: string | RawLocationAuthority, options?: RouterBrowserHistoryIngressOptions): RouterBrowserHistoryIngress;
  push(location: string | RawLocationAuthority, options?: RouterBrowserHistoryIngressOptions): RouterBrowserHistoryIngress;
  replace(location: string | RawLocationAuthority, options?: RouterBrowserHistoryIngressOptions): RouterBrowserHistoryIngress;
  pop(location: string | RawLocationAuthority, options?: RouterBrowserHistoryIngressOptions): RouterBrowserHistoryIngress;
  manual(location: string | RawLocationAuthority, options?: RouterBrowserHistoryIngressOptions): RouterBrowserHistoryIngress;
  external(location: string | RawLocationAuthority, options?: RouterBrowserHistoryIngressOptions): RouterBrowserHistoryIngress;
  story(initialReport?: RouterBrowserHistoryBoundaryReport): RouterBrowserHistoryStory;
  readonly coherence: RouterBrowserAuthorityCoherenceNamespace;
  readonly writeback: RouterBrowserHistoryWritebackNamespace;
}

export interface RouterBrowserHistoryAdmissionVerificationPackage {
  readonly browserHistoryEnvelopeDigest: string;
  readonly routeTruthDigest: string;
  readonly continuityDigest: string;
  readonly [forgeSignalRouterBrowserHistoryAdmissionVerificationBrand]: "routerBrowserHistoryAdmissionVerificationPackage";
}

export interface RouterBrowserHistoryAdmissionReport<
  TRouteOutcome extends RouteOutcome = RouteOutcome,
> {
  readonly envelopeFamily: "browserHistoryIngress";
  readonly navigationKind: BrowserHistoryNavigationKind;
  readonly rawLocationHref: string;
  readonly routeIdentity: string | null;
  readonly runtimeRouteSourceId?: string | null;
  readonly runtimeContinuitySourceId?: string | null;
  coherence(): RouterBrowserAuthorityCoherence | null;
  restoredBreadcrumbs(): RouteRestoredBreadcrumbs | null;
  carriedBreadcrumbs(): RouteCarriedBreadcrumbs | null;
  restoreBoundary(): RouteRestoreBoundary | null;
  outcome(): TRouteOutcome;
  diagnostics(): {
    readonly boundarySource: "browserHistoryIngress";
    readonly boundaryArtifact:
      | "routeTruthConverged"
      | "routeTruthDriftedFromAuthority"
      | "routeOutcomeNotAdmitted";
    readonly navigationKind: BrowserHistoryNavigationKind;
    readonly rawLocationHref: string;
    readonly routeIdentity: string | null;
    readonly coherenceKind: BrowserAuthorityCoherenceKind | null;
    readonly outcomeKind: TRouteOutcome["kind"];
    readonly routeId: string | null;
    readonly href: string | null;
  };
  verification(): RouterBrowserHistoryAdmissionVerificationPackage;
}

export type RouterBrowserHistoryWritebackTarget =
  | string
  | RawLocationAuthority
  | RouteLocation<any, any, any>;

export interface RouterBrowserHistoryWritebackVerificationPackage {
  readonly browserHistoryWritebackDigest: string;
  readonly [forgeSignalRouterBrowserHistoryWritebackVerificationBrand]: "routerBrowserHistoryWritebackVerificationPackage";
}

export interface RouterBrowserHistoryWriteback {
  readonly kind: "routerBrowserHistoryWriteback";
  readonly navigationKind: "pushstate" | "replacestate" | "external";
  readonly targetKind: "local" | "external";
  readonly targetHref: string;
  readonly rawLocation: RawLocationAuthority | null;
  readonly routeIdentity: string | null;
  readonly runtimeRouteSourceId: string | null;
  readonly routeValue: unknown;
  readonly runtimeContinuitySourceId: string | null;
  readonly continuityValue: unknown;
  readonly coherence: RouterBrowserAuthorityCoherence | null;
  readonly restoredBreadcrumbs: RouteRestoredBreadcrumbs | null;
  readonly carriedBreadcrumbs: RouteCarriedBreadcrumbs | null;
  readonly restoreBoundary: RouteRestoreBoundary | null;
  verification(): RouterBrowserHistoryWritebackVerificationPackage;
  readonly [forgeSignalRouterBrowserHistoryWritebackBrand]: "routerBrowserHistoryWriteback";
}

export interface RouterBrowserHistoryWritebackNamespace {
  push(target: RouterBrowserHistoryWritebackTarget, options: RouterBrowserHistoryLocalWritebackOptions): RouterBrowserHistoryWriteback;
  replace(target: RouterBrowserHistoryWritebackTarget, options: RouterBrowserHistoryLocalWritebackOptions): RouterBrowserHistoryWriteback;
  external(target: string, options?: RouterBrowserHistoryWritebackOptions): RouterBrowserHistoryWriteback;
}

export interface RouterBrowserHistoryWritebackReportVerificationPackage {
  readonly browserHistoryWritebackDigest: string;
  readonly routeTruthDigest: string;
  readonly boundaryStoryDigest: string;
  readonly [forgeSignalRouterBrowserHistoryWritebackReportVerificationBrand]: "routerBrowserHistoryWritebackReportVerificationPackage";
}

export interface RouterBrowserHistoryWritebackReport<
  TRouteOutcome extends RouteOutcome | null = RouteOutcome | null,
> {
  readonly envelopeFamily: "browserHistoryWriteback";
  readonly navigationKind: "pushstate" | "replacestate" | "external";
  readonly targetKind: "local" | "external";
  readonly targetHref: string;
  readonly routeIdentity: string | null;
  readonly runtimeRouteSourceId?: string | null;
  readonly runtimeContinuitySourceId?: string | null;
  coherence(): RouterBrowserAuthorityCoherence | null;
  restoredBreadcrumbs(): RouteRestoredBreadcrumbs | null;
  carriedBreadcrumbs(): RouteCarriedBreadcrumbs | null;
  restoreBoundary(): RouteRestoreBoundary | null;
  outcome(): TRouteOutcome;
  diagnostics(): {
    readonly boundarySource: "browserHistoryWriteback";
    readonly boundaryArtifact:
      | "routeTruthConverged"
      | "routeTruthDriftedFromAuthority"
      | "routeOutcomeNotAdmitted"
      | "externalNavigationEscaped";
    readonly coherenceKind: BrowserAuthorityCoherenceKind | null;
    readonly navigationKind: "pushstate" | "replacestate" | "external";
    readonly targetKind: "local" | "external";
    readonly targetHref: string;
    readonly routeIdentity: string | null;
    readonly outcomeKind: TRouteOutcome extends RouteOutcome ? TRouteOutcome["kind"] : TRouteOutcome extends null ? null : RouteOutcome["kind"] | null;
    readonly routeId: string | null;
    readonly href: string | null;
  };
  verification(): RouterBrowserHistoryWritebackReportVerificationPackage;
}

export type RouterBrowserHistoryBoundaryReport =
  | RouterBrowserHistoryAdmissionReport<any>
  | RouterBrowserHistoryWritebackReport<any>;

export interface RouterBrowserHistoryBoundaryEventVerificationPackage {
  readonly boundaryEventDigest: string;
  readonly [forgeSignalRouterBrowserHistoryBoundaryEventVerificationBrand]: "routerBrowserHistoryBoundaryEventVerificationPackage";
}

export interface RouterBrowserHistoryStoryEntryVerificationPackage {
  readonly routeHistoryEntryDigest: string;
  readonly [forgeSignalRouterBrowserHistoryStoryEntryVerificationBrand]: "routerBrowserHistoryStoryEntryVerificationPackage";
}

export interface RouterBrowserHistoryStoryEntry {
  readonly kind: "routeHistoryEntry";
  readonly eventIndex: number;
  readonly boundarySource: "browserHistoryIngress" | "browserHistoryWriteback";
  readonly boundaryArtifact: "routeTruthConverged" | "routeTruthDriftedFromAuthority";
  readonly navigationKind: BrowserHistoryNavigationKind | "pushstate" | "replacestate";
  readonly routeId: string;
  readonly href: string;
  readonly routeIdentity: string | null;
  readonly coherenceKind: BrowserAuthorityCoherenceKind | null;
  previous(): RouterBrowserHistoryStoryEntry | null;
  restoreBoundary(): RouteRestoreBoundary | null;
  restore(
    history: RouteRestoreHistoryFacade,
  ): RouteHistoryRestoreResult | Promise<RouteHistoryRestoreResult>;
  replay(
    history: RouteReplayHistoryFacade,
  ): RouteHistoryReplayResult | Promise<RouteHistoryReplayResult>;
  breadcrumbTrail(): RouterBrowserHistoryBreadcrumbTrail | null;
  outletComposition(): RouterBrowserHistoryOutletComposition | null;
  route(): unknown;
  verification(): RouterBrowserHistoryStoryEntryVerificationPackage;
}

export interface RouterBrowserHistoryBoundaryEvent {
  readonly kind: "browserHistoryBoundaryEvent";
  readonly eventIndex: number;
  readonly envelopeFamily: "browserHistoryIngress" | "browserHistoryWriteback";
  readonly boundarySource: "browserHistoryIngress" | "browserHistoryWriteback";
  readonly boundaryArtifact:
    | "routeTruthConverged"
    | "routeTruthDriftedFromAuthority"
    | "routeOutcomeNotAdmitted"
    | "externalNavigationEscaped";
  readonly navigationKind: BrowserHistoryNavigationKind | "pushstate" | "replacestate";
  readonly targetHref: string;
  readonly routeIdentity: string | null;
  readonly coherenceKind: BrowserAuthorityCoherenceKind | null;
  readonly advancedRouteTruth: boolean;
  readonly outcomeKind: RouteOutcome["kind"] | null;
  readonly routeId: string | null;
  readonly href: string | null;
  readonly routeTruthEntry: RouterBrowserHistoryStoryEntry | null;
  verification(): RouterBrowserHistoryBoundaryEventVerificationPackage;
}

export interface RouterBrowserHistoryStoryVerificationPackage {
  readonly historyStoryDigest: string;
  readonly latestBoundaryEventDigest: string | null;
  readonly currentRouteTruthEventDigest: string | null;
  readonly currentEntryDigest: string | null;
  readonly backEntryDigest: string | null;
  readonly [forgeSignalRouterBrowserHistoryStoryVerificationBrand]: "routerBrowserHistoryStoryVerificationPackage";
}

export interface RouterBrowserHistoryBackProvenanceVerificationPackage {
  readonly backProvenanceDigest: string;
  readonly [forgeSignalRouterBrowserHistoryBackProvenanceVerificationBrand]: "routerBrowserHistoryBackProvenanceVerificationPackage";
}

export interface RouterBrowserHistoryBreadcrumbTrailVerificationPackage {
  readonly breadcrumbTrailDigest: string;
  readonly [forgeSignalRouterBrowserHistoryBreadcrumbTrailVerificationBrand]: "routerBrowserHistoryBreadcrumbTrailVerificationPackage";
}

export interface RouterBrowserHistoryBackProvenance {
  readonly kind: "browserHistoryBackProvenance";
  readonly available: boolean;
  readonly current: RouterBrowserHistoryStoryEntry | null;
  readonly previous: RouterBrowserHistoryStoryEntry | null;
  restoreBoundary(): RouteRestoreBoundary | null;
  restore(
    history: RouteRestoreHistoryFacade,
  ): RouteHistoryRestoreResult | Promise<RouteHistoryRestoreResult>;
  replay(
    history: RouteReplayHistoryFacade,
  ): RouteHistoryReplayResult | Promise<RouteHistoryReplayResult>;
  verification(): RouterBrowserHistoryBackProvenanceVerificationPackage;
}

export interface RouterBrowserHistoryBreadcrumbEntry extends RouteBreadcrumbEntry {
  restoreBoundary(): RouteRestoreBoundary | null;
  restore(
    history: RouteRestoreHistoryFacade,
  ): RouteHistoryRestoreResult | Promise<RouteHistoryRestoreResult>;
  replay(
    history: RouteReplayHistoryFacade,
  ): RouteHistoryReplayResult | Promise<RouteHistoryReplayResult>;
}

export interface RouterBrowserHistoryBreadcrumbTrail {
  readonly kind: "browserHistoryBreadcrumbTrail";
  readonly entries: ReadonlyArray<RouterBrowserHistoryBreadcrumbEntry>;
  verification(): RouterBrowserHistoryBreadcrumbTrailVerificationPackage;
}

export interface RouterBrowserHistoryStory {
  record(report: RouterBrowserHistoryBoundaryReport): RouterBrowserHistoryBoundaryEvent;
  events(): ReadonlyArray<RouterBrowserHistoryBoundaryEvent>;
  admittedEntries(): ReadonlyArray<RouterBrowserHistoryStoryEntry>;
  current(): RouterBrowserHistoryStoryEntry | null;
  latestBoundaryEvent(): RouterBrowserHistoryBoundaryEvent | null;
  currentRouteTruthEvent(): RouterBrowserHistoryBoundaryEvent | null;
  back(): RouterBrowserHistoryStoryEntry | null;
  breadcrumbs(): ReadonlyArray<RouterBrowserHistoryStoryEntry>;
  backProvenance(): RouterBrowserHistoryBackProvenance;
  breadcrumbTrail(): RouterBrowserHistoryBreadcrumbTrail;
  inspection(): RouterBrowserHistoryInspection;
  auditability(
    hydrationReport?: RouterHydrationAdmissionReport | null,
  ): RouterNavigationAuditability;
  verification(): RouterBrowserHistoryStoryVerificationPackage;
}

export type {
  RouteAdmissionFacts,
};
