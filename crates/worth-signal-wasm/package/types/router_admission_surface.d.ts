import type {
  RouteBreadcrumbEntry,
  RouteBreadcrumbTrail,
} from "./router_breadcrumb_surface.js";
import type {
  ProjectedLayoutPlacement,
  ProjectedOutletContract,
  ProjectedRouteCandidate,
} from "./router_projection_surface.js";
import type {
  RouteControllerMap,
  RouteGraphMap,
} from "./router_composition_surface.js";
import type {
  CanonicalRouteArtifact,
  RouterDescriptor,
  RouterHashField,
  RouterHashInput,
  RouterSearchMatch,
  RouterSearchSchema,
} from "./router_surface.js";
import type {
  ControllerContract,
} from "./controller_surface.js";
import type {
  PublishedGraphContractSurface,
  PublishedGraphSummary,
  PublishedSignalGraph,
} from "./graph_surface.js";
import type {
  AdmittedRouteResourceCapability,
} from "./router_resource_surface.js";

declare const WorthSignalRoutePrerequisiteDeclarationBrand: unique symbol;
declare const WorthSignalRouteAdmissionSourceBrand: unique symbol;
declare const WorthSignalRouteRecoveryDeclarationBrand: unique symbol;
declare const WorthSignalRouteFormsAuthorityDeclarationBrand: unique symbol;
declare const WorthSignalRouteFormsAuthorityBrand: unique symbol;
declare const WorthSignalRouteAdmissionPlanBrand: unique symbol;
declare const WorthSignalAdmittedRouteCapabilityBrand: unique symbol;
declare const WorthSignalAdmittedControllerCapabilityBrand: unique symbol;
declare const WorthSignalAdmittedGraphCapabilityBrand: unique symbol;
declare const WorthSignalRouteOutcomeBrand: unique symbol;

export type RouteAdmissionFacts = Readonly<Record<string, unknown>>;
export type RouteAdmissionArtifactKind =
  | "allow"
  | "redirect"
  | "notFound"
  | "forbidden"
  | "unavailable"
  | "denied";

export interface RouteAdmissionArtifact {
  readonly kind: Exclude<RouteAdmissionArtifactKind, "allow">;
  readonly prerequisite: string | null;
  readonly href: string | null;
  readonly reason: string;
  readonly detail: string | null;
}

export interface RoutePrerequisiteArtifactBuilder {
  allow(options?: { reason?: string; detail?: string }): {
    readonly kind: "allow";
    readonly reason: string;
    readonly detail: string | null;
  };
  redirect(options: { href: string; reason?: string; detail?: string }): {
    readonly kind: "redirect";
    readonly href: string;
    readonly reason: string;
    readonly detail: string | null;
  };
  notFound(options?: { reason?: string; detail?: string }): {
    readonly kind: "notFound";
    readonly reason: string;
    readonly detail: string | null;
  };
  forbidden(options?: { reason?: string; detail?: string }): {
    readonly kind: "forbidden";
    readonly reason: string;
    readonly detail: string | null;
  };
  unavailable(options?: { reason?: string; detail?: string }): {
    readonly kind: "unavailable";
    readonly reason: string;
    readonly detail: string | null;
  };
  denied(options?: { reason?: string; detail?: string }): {
    readonly kind: "denied";
    readonly reason: string;
    readonly detail: string | null;
  };
}

export type RouteAdmissionSourceFamily =
  | "hostCapability"
  | "resourceTruth"
  | "graphTruth";

export type RouteAdmissionSourceValueKind =
  | "string"
  | "number"
  | "boolean";

export interface RouteAdmissionSource<
  TValue = unknown,
  TName extends string = string,
  TFamily extends RouteAdmissionSourceFamily = RouteAdmissionSourceFamily,
> {
  readonly name: TName;
  readonly family: TFamily;
  readonly valueKind: RouteAdmissionSourceValueKind;
  readonly [WorthSignalRouteAdmissionSourceBrand]: "routeAdmissionSource";
}

export interface RouteAdmissionSourceNamespace<TFamily extends RouteAdmissionSourceFamily> {
  string<const TName extends string>(name: TName): RouteAdmissionSource<string, TName, TFamily>;
  number<const TName extends string>(name: TName): RouteAdmissionSource<number, TName, TFamily>;
  boolean<const TName extends string>(name: TName): RouteAdmissionSource<boolean, TName, TFamily>;
}

export interface RoutePrerequisiteEvaluationContext<
  TRoute extends string = string,
  TSearch extends RouterSearchSchema = Record<string, never>,
  THash extends RouterHashField<unknown> | null = null,
  TConsumedSources extends ReadonlyArray<RouteAdmissionSource> = ReadonlyArray<RouteAdmissionSource>,
> extends RoutePrerequisiteArtifactBuilder {
  readonly routeId: string;
  readonly href: string;
  readonly params: import("./router/route_types.js").RoutePathParams<TRoute>;
  readonly search: RouterSearchMatch<TSearch>;
  readonly hash: THash extends RouterHashField<unknown> ? RouterHashInput<THash> | undefined : undefined;
  readonly facts: RouteAdmissionFacts;
  consume<TSource extends TConsumedSources[number]>(source: TSource): TSource extends RouteAdmissionSource<infer TValue, string, RouteAdmissionSourceFamily> ? TValue : never;
  consumedSources(): TConsumedSources;
}

export interface RoutePrerequisiteDeclaration<
  TConsumedSources extends ReadonlyArray<RouteAdmissionSource> = ReadonlyArray<RouteAdmissionSource>,
> {
  readonly name: string;
  readonly consumes: TConsumedSources;
  readonly [WorthSignalRoutePrerequisiteDeclarationBrand]: "routePrerequisiteDeclaration";
}

export interface RouteRecoveryArtifact {
  readonly kind: "fallback";
  readonly recovery: string;
  readonly href: string;
  readonly reason: string;
  readonly detail: string | null;
}

export interface RouteRecoveryEvaluationContext<
  TRoute extends string = string,
  TSearch extends RouterSearchSchema = Record<string, never>,
  THash extends RouterHashField<unknown> | null = null,
> {
  readonly routeId: string;
  readonly href: string;
  readonly params: import("./router/route_types.js").RoutePathParams<TRoute>;
  readonly search: RouterSearchMatch<TSearch>;
  readonly hash: THash extends RouterHashField<unknown> ? RouterHashInput<THash> | undefined : undefined;
  readonly facts: RouteAdmissionFacts;
  readonly terminalArtifact: RouteAdmissionArtifact;
  fallback(options: { href: string; reason: string; detail?: string | null }): {
    readonly kind: "fallback";
    readonly href: string;
    readonly reason: string;
    readonly detail: string | null;
  };
}

export interface RouteRecoveryDeclaration {
  readonly name: string;
  readonly [WorthSignalRouteRecoveryDeclarationBrand]: "routeRecoveryDeclaration";
}

export type RouteFormsAuthorityContinuity =
  | "preserve"
  | "freeze"
  | "discard"
  | "defer";

export interface RouteFormsAuthorityDeclaration {
  readonly surfaceId: string;
  readonly continuity: RouteFormsAuthorityContinuity;
  readonly reason: string | null;
  readonly [WorthSignalRouteFormsAuthorityDeclarationBrand]: "routeFormsAuthorityDeclaration";
}

export interface RouteFormsAuthorityVerificationPackage {
  readonly formsAuthorityDigest: string;
}

export interface RouteFormsAuthorityArtifact {
  readonly kind: "routeFormsAuthority";
  readonly routeId: string;
  readonly href: string;
  readonly scopeKind: "route";
  readonly surfaceId: string;
  readonly continuity: RouteFormsAuthorityContinuity;
  readonly reason: string | null;
  verification(): RouteFormsAuthorityVerificationPackage;
  readonly [WorthSignalRouteFormsAuthorityBrand]: "routeFormsAuthority";
}

export interface RouteAdmissionPlanVerificationPackage {
  readonly routeId: string | null;
  readonly admissionPlanDigest: string;
}

export interface RouteOutcomeVerificationPackage extends RouteAdmissionPlanVerificationPackage {
  readonly routeOutcomeDigest: string;
  readonly formsAuthorityDigest: string | null;
}

export interface RouteAdmissionDecisionProvenance {
  readonly routeId: string;
  readonly href: string;
  readonly kind: RouteAdmissionArtifactKind;
  readonly prerequisite: string | null;
  readonly artifactHref: string | null;
  readonly reason: string;
  readonly detail: string | null;
  readonly consumedSources: ReadonlyArray<{
    readonly name: string;
    readonly family: RouteAdmissionSourceFamily;
    readonly valueKind: RouteAdmissionSourceValueKind;
  }>;
}

export interface RouteRecoveryProvenance {
  readonly recovery: string;
  readonly href: string;
  readonly reason: string;
  readonly detail: string | null;
  readonly fromArtifactKind: Exclude<RouteAdmissionArtifactKind, "allow">;
  readonly fromRouteId: string;
  readonly fromHref: string;
  readonly toRouteId: string | null;
  readonly toHref: string | null;
}

export interface RouteAdmissionPlanProvenance {
  readonly attemptedRouteId: string;
  readonly attemptedHref: string;
  readonly prerequisiteNames: ReadonlyArray<string>;
  readonly recoveryNames: ReadonlyArray<string>;
  readonly consumedSources: ReadonlyArray<{
    readonly name: string;
    readonly family: RouteAdmissionSourceFamily;
    readonly valueKind: RouteAdmissionSourceValueKind;
  }>;
  readonly factsKeys: ReadonlyArray<string>;
}

export interface RouteOutcomeProvenance {
  readonly attemptedRouteId: string | null;
  readonly attemptedHref: string | null;
  readonly resolvedRouteId: string | null;
  readonly resolvedHref: string | null;
  readonly terminalSource:
    | "noProjectedCandidate"
    | "prerequisiteArtifact"
    | "admittedWithoutRecovery"
    | "recoveredOutcome";
  readonly terminalArtifact: RouteAdmissionArtifact | null;
  readonly prerequisiteDecisions: ReadonlyArray<RouteAdmissionDecisionProvenance>;
  readonly recoveryTrail: ReadonlyArray<RouteRecoveryProvenance>;
}

export interface AdmittedControllerCapability<
  TController extends ControllerContract = ControllerContract,
> {
  readonly kind: "admittedControllerCapability";
  readonly routeId: string;
  readonly name: string;
  inputNames(): ReadonlyArray<Extract<keyof TController["inputs"], string>>;
  outputNames(): ReadonlyArray<Extract<keyof TController["outputs"], string>>;
  internalNames(): ReadonlyArray<Extract<keyof TController["internal"], string>>;
  readonly [WorthSignalAdmittedControllerCapabilityBrand]: "admittedControllerCapability";
}

export interface AdmittedGraphCapability<
  TGraph extends PublishedSignalGraph = PublishedSignalGraph,
> {
  readonly kind: "admittedGraphCapability";
  readonly routeId: string;
  readonly name: string;
  readonly graphId: string;
  summary(): PublishedGraphSummary;
  contract(): PublishedGraphContractSurface;
  inputNames(): ReadonlyArray<string>;
  outputNames(): ReadonlyArray<string>;
  readonly [WorthSignalAdmittedGraphCapabilityBrand]: "admittedGraphCapability";
}

export interface AdmittedRouteCapability<
  TRoute extends string = string,
  TSearch extends RouterSearchSchema = RouterSearchSchema,
  THash extends RouterHashField<unknown> | null = RouterHashField<unknown> | null,
  TControllers extends RouteControllerMap = RouteControllerMap,
  TGraphs extends RouteGraphMap = RouteGraphMap,
> {
  readonly kind: "admittedRouteCapability";
  readonly routeId: string;
  readonly href: string;
  readonly params: import("./router/route_types.js").RoutePathParams<TRoute>;
  readonly search: RouterSearchMatch<TSearch>;
  readonly hash: THash extends RouterHashField<unknown> ? RouterHashInput<THash> | undefined : undefined;
  controllerNames(): ReadonlyArray<Extract<keyof TControllers, string>>;
  controller<TName extends keyof TControllers>(name: TName): AdmittedControllerCapability<TControllers[TName]>;
  graphNames(): ReadonlyArray<Extract<keyof TGraphs, string>>;
  graph<TName extends keyof TGraphs>(name: TName): AdmittedGraphCapability<TGraphs[TName]>;
  resourceNames(): ReadonlyArray<string>;
  resource(name: string): AdmittedRouteResourceCapability;
  breadcrumb(): RouteBreadcrumbEntry | null;
  breadcrumbTrail(): RouteBreadcrumbTrail | null;
  formsAuthority(): RouteFormsAuthorityArtifact | null;
  descriptor(): RouterDescriptor<TRoute, THash>;
  canonical(): CanonicalRouteArtifact<TRoute, TSearch, THash>;
  verification(): import("./router_verification_surface.js").CanonicalRouteVerificationPackage;
  readonly [WorthSignalAdmittedRouteCapabilityBrand]: "admittedRouteCapability";
}

export interface RouteAdmissionDiagnostics {
  readonly routeId: string | null;
  readonly outcomeKind: Exclude<RouteAdmissionArtifactKind, "allow"> | "admitted";
  readonly formsAuthority: RouteFormsAuthorityArtifact | null;
  readonly prerequisiteDecisions: ReadonlyArray<{
    readonly kind: RouteAdmissionArtifactKind;
    readonly prerequisite: string;
    readonly href: string | null;
    readonly reason: string;
    readonly detail: string | null;
    readonly consumedSources: ReadonlyArray<{
      readonly name: string;
      readonly family: RouteAdmissionSourceFamily;
      readonly valueKind: RouteAdmissionSourceValueKind;
    }>;
  }>;
  readonly recovery: {
    readonly recovery: string;
    readonly href: string;
    readonly reason: string;
    readonly detail: string | null;
    readonly fromArtifactKind: Exclude<RouteAdmissionArtifactKind, "allow">;
    readonly fromRouteId: string;
    readonly fromHref: string;
  } | null;
}

export interface AdmittedRouteOutcome<
  TRouteCapability extends AdmittedRouteCapability = AdmittedRouteCapability,
  TLayoutPlacement extends ProjectedLayoutPlacement = ProjectedLayoutPlacement,
> {
  readonly kind: "admitted";
  readonly routeId: string;
  readonly href: string;
  route(): TRouteCapability;
  layouts(): ReadonlyArray<TLayoutPlacement>;
  outlet(): ProjectedOutletContract;
  outlets(): ReadonlyArray<ProjectedOutletContract>;
  diagnostics(): RouteAdmissionDiagnostics;
  recovery(): RouteRecoveryArtifact | null;
  provenance(): RouteOutcomeProvenance;
  verification(): RouteOutcomeVerificationPackage;
  readonly [WorthSignalRouteOutcomeBrand]: "routeOutcome";
}

export interface NonAdmittedRouteOutcome {
  readonly kind: "redirect" | "notFound" | "forbidden" | "unavailable" | "denied";
  readonly routeId: string | null;
  readonly href: string | null;
  artifact(): RouteAdmissionArtifact;
  diagnostics(): RouteAdmissionDiagnostics;
  recovery(): RouteRecoveryArtifact | null;
  provenance(): RouteOutcomeProvenance;
  verification(): RouteOutcomeVerificationPackage;
  readonly [WorthSignalRouteOutcomeBrand]: "routeOutcome";
}

export type RouteOutcome<
  TRouteCapability extends AdmittedRouteCapability = AdmittedRouteCapability,
  TLayoutPlacement extends ProjectedLayoutPlacement = ProjectedLayoutPlacement,
> = AdmittedRouteOutcome<TRouteCapability, TLayoutPlacement> | NonAdmittedRouteOutcome;

export interface RouteAdmissionPlan<
  TProjectedCandidate extends ProjectedRouteCandidate = ProjectedRouteCandidate,
  TRouteOutcome extends RouteOutcome = RouteOutcome,
> {
  readonly kind: "routeAdmissionPlan";
  readonly routeId: string;
  readonly href: string;
  candidate(): TProjectedCandidate;
  prerequisiteNames(): ReadonlyArray<string>;
  recoveryNames(): ReadonlyArray<string>;
  provenance(): RouteAdmissionPlanProvenance;
  verification(): RouteAdmissionPlanVerificationPackage;
  resolve(): Promise<TRouteOutcome>;
  readonly [WorthSignalRouteAdmissionPlanBrand]: "routeAdmissionPlan";
}
