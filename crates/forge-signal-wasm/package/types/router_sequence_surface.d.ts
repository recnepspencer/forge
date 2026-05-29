import type {
  RouteAdmissionFacts,
  RouteOutcome,
} from "./router_admission_surface.js";
import type {
  RawLocationAuthority,
} from "./router_authority_surface.js";
import type {
  BrowserHistoryNavigationKind,
  RouterBrowserHistoryAdmissionReport,
  RouterBrowserHistoryBackProvenance,
  RouterBrowserHistoryBoundaryEvent,
  RouterBrowserHistoryBreadcrumbTrail,
  RouterBrowserHistoryIngressOptions,
  RouterBrowserHistoryStory,
  RouterBrowserHistoryStoryEntry,
} from "./router_history_surface.js";
import type {
  RouteLocation,
} from "./router_surface.js";

export type RouterSequenceTarget =
  | string
  | RawLocationAuthority
  | RouteLocation<any, any, any>;

export interface RouterSequenceNavigationStep<
  TFacts = RouteAdmissionFacts,
> {
  readonly target: RouterSequenceTarget;
  readonly historyMethod?: "load" | "push" | "replace" | "pop" | "manual" | "external";
  readonly carryBreadcrumbs?: boolean;
  readonly ingress?: RouterBrowserHistoryIngressOptions;
  readonly facts?: TFacts;
}

export interface RouterSequenceStepResult<
  TRouteOutcome extends RouteOutcome = RouteOutcome,
  TFacts = RouteAdmissionFacts,
> {
  readonly index: number;
  readonly targetHref: string;
  readonly navigationKind: BrowserHistoryNavigationKind;
  readonly facts: TFacts;
  readonly report: RouterBrowserHistoryAdmissionReport<TRouteOutcome>;
  readonly event: RouterBrowserHistoryBoundaryEvent;
  readonly current: RouterBrowserHistoryStoryEntry | null;
  readonly breadcrumbTrail: RouterBrowserHistoryBreadcrumbTrail;
  readonly backProvenance: RouterBrowserHistoryBackProvenance;
}

export interface RouterSequenceReplay<
  TRouteOutcome extends RouteOutcome = RouteOutcome,
> {
  outcomes(): ReadonlyArray<TRouteOutcome>;
  breadcrumbTrail(): ReadonlyArray<RouterBrowserHistoryBreadcrumbTrail>;
  backProvenance(): ReadonlyArray<RouterBrowserHistoryBackProvenance>;
  currentEntries(): ReadonlyArray<RouterBrowserHistoryStoryEntry | null>;
}

export interface RouterSequenceDiagnosticsEntry {
  readonly index: number;
  readonly targetHref: string;
  readonly outcomeKind: string;
  readonly eventBoundaryArtifact: RouterBrowserHistoryBoundaryEvent["boundaryArtifact"];
}

export interface RouterSequenceDiagnostics {
  readonly hasFailures: boolean;
  readonly denied: ReadonlyArray<RouterSequenceDiagnosticsEntry>;
  readonly notAdmitted: ReadonlyArray<RouterSequenceDiagnosticsEntry>;
}

export interface RouterSequenceRunResult<
  TRouteOutcome extends RouteOutcome = RouteOutcome,
  TFacts = RouteAdmissionFacts,
> {
  readonly story: RouterBrowserHistoryStory;
  readonly steps: ReadonlyArray<RouterSequenceStepResult<TRouteOutcome, TFacts>>;
  readonly replay: RouterSequenceReplay<TRouteOutcome>;
  diagnostics(): RouterSequenceDiagnostics;
}

export interface RouterSequenceScenario<
  TRouteOutcome extends RouteOutcome = RouteOutcome,
  TFacts = RouteAdmissionFacts,
> {
  readonly steps: ReadonlyArray<RouterSequenceNavigationStep<TFacts>>;
  run(options?: { readonly facts?: TFacts }): Promise<RouterSequenceRunResult<TRouteOutcome, TFacts>>;
}
