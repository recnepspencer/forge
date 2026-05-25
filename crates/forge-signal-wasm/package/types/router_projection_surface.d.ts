import type {
  RouteBreadcrumbEntry,
  RouteBreadcrumbTrail,
} from "./router_breadcrumb_surface.js";
import type {
  RoutePathParams,
} from "./router/route_types.js";
import type {
  RouteAdmissionFacts,
  RouteAdmissionPlan,
  CanonicalRouteArtifact,
  CanonicalUrlAuthority,
  RouterDescriptor,
  RouterHashField,
  RouterHashInput,
  RouterRouteDeclaration,
  RouterSearchMatch,
  RouterSearchSchema,
} from "./router_surface.js";
import type {
  RouteControllerMap,
  RouteGraphMap,
} from "./router_composition_surface.js";
import type {
  ControllerContract,
} from "./controller_surface.js";
import type {
  PublishedGraphContractSurface,
  PublishedGraphSummary,
  PublishedSignalGraph,
} from "./graph_surface.js";
import type {
  CanonicalRouteVerificationPackage,
} from "./router_verification_surface.js";
import type {
  ProjectedRouteResourceCapability,
} from "./router_resource_surface.js";
import type {
  SpeculativeRouteBranchOptions,
  SpeculativeRouteBranchPlan,
} from "./router_speculation_surface.js";
import type {
  ProjectedRoutePrefetchArtifact,
  RoutePrefetchTrigger,
} from "./router_transition_surface.js";

declare const forgeSignalRouteLayoutDeclarationBrand: unique symbol;
declare const forgeSignalProjectedRouteCapabilityBrand: unique symbol;
declare const forgeSignalProjectedLayoutPlacementBrand: unique symbol;
declare const forgeSignalProjectedRouteCandidateBrand: unique symbol;
declare const forgeSignalProjectedRouteCandidateVerificationPackageBrand: unique symbol;
declare const forgeSignalProjectedOutletContractBrand: unique symbol;
declare const forgeSignalProjectedOutletVerificationPackageBrand: unique symbol;

export interface RouterLayoutOptions {
  outlet?: string;
}

export interface RouterLayoutDeclaration<
  TRoute extends string = string,
  TSearch extends RouterSearchSchema = Record<string, never>,
  THash extends RouterHashField<unknown> | null = null,
  TControllers extends RouteControllerMap = Record<string, never>,
  TGraphs extends RouteGraphMap = Record<string, never>,
  TChildren extends Record<string, unknown> = Record<string, unknown>,
> {
  readonly [forgeSignalRouteLayoutDeclarationBrand]: "routeLayoutDeclaration";
  readonly route: RouterRouteDeclaration<TRoute, TSearch, THash, TControllers, TGraphs>;
  readonly outletId: string;
  readonly children: TChildren;
}

export interface ProjectedRouteCandidateVerificationPackage {
  readonly canonicalUrlDigest: string;
  readonly projectedRouteDigest: string;
  readonly routeCompositionDigest: string;
  readonly layoutStackDigest: string;
  readonly outletDigest: string;
  readonly outletStackDigest: string;
  readonly projectedCandidateDigest: string;
  readonly [forgeSignalProjectedRouteCandidateVerificationPackageBrand]: "projectedRouteCandidateVerificationPackage";
}

export interface ProjectedOutletContractVerificationPackage {
  readonly outletDigest: string;
  readonly occupantDigest: string;
  readonly outletContractDigest: string;
  readonly [forgeSignalProjectedOutletVerificationPackageBrand]: "projectedOutletVerificationPackage";
}

export interface ProjectedControllerCapability<
  TController extends ControllerContract = ControllerContract,
> {
  readonly kind: "projectedControllerCapability";
  readonly routeId: string;
  readonly name: string;
  inputNames(): ReadonlyArray<Extract<keyof TController["inputs"], string>>;
  outputNames(): ReadonlyArray<Extract<keyof TController["outputs"], string>>;
  internalNames(): ReadonlyArray<Extract<keyof TController["internal"], string>>;
}

export interface ProjectedGraphCapability<
  TGraph extends PublishedSignalGraph = PublishedSignalGraph,
> {
  readonly kind: "projectedGraphCapability";
  readonly routeId: string;
  readonly name: string;
  readonly graphId: string;
  summary(): PublishedGraphSummary;
  contract(): PublishedGraphContractSurface;
  inputNames(): ReadonlyArray<string>;
  outputNames(): ReadonlyArray<string>;
}

export interface ProjectedRouteCapability<
  TRoute extends string = string,
  TSearch extends RouterSearchSchema = Record<string, never>,
  THash extends RouterHashField<unknown> | null = null,
  TControllers extends RouteControllerMap = Record<string, never>,
  TGraphs extends RouteGraphMap = Record<string, never>,
> {
  readonly kind: "projectedRouteCapability";
  readonly routeId: string;
  readonly href: string;
  readonly params: RoutePathParams<TRoute>;
  readonly search: RouterSearchMatch<TSearch>;
  readonly hash: THash extends RouterHashField<unknown> ? RouterHashInput<THash> | undefined : undefined;
  controllerNames(): ReadonlyArray<Extract<keyof TControllers, string>>;
  controller<TName extends keyof TControllers>(
    name: TName,
  ): ProjectedControllerCapability<TControllers[TName]>;
  graphNames(): ReadonlyArray<Extract<keyof TGraphs, string>>;
  graph<TName extends keyof TGraphs>(
    name: TName,
  ): ProjectedGraphCapability<TGraphs[TName]>;
  resourceNames(): ReadonlyArray<string>;
  resource(name: string): ProjectedRouteResourceCapability;
  breadcrumb(): RouteBreadcrumbEntry | null;
  breadcrumbTrail(): RouteBreadcrumbTrail | null;
  descriptor(): RouterDescriptor<TRoute, THash>;
  canonical(): CanonicalRouteArtifact<TRoute, TSearch, THash>;
  verification(): CanonicalRouteVerificationPackage;
  readonly [forgeSignalProjectedRouteCapabilityBrand]: "projectedRouteCapability";
}

export interface ProjectedOutletDescriptor {
  readonly outletId: string | null;
  readonly parentLayoutRouteId: string | null;
  readonly occupantRouteId: string;
  readonly occupantKind: "projectedLayoutPlacement" | "projectedRouteCapability";
}

export type ProjectedOutletOccupant =
  | ProjectedLayoutPlacement
  | ProjectedRouteCapability;

export interface ProjectedOutletContract<
  TOccupant extends ProjectedOutletOccupant = ProjectedOutletOccupant,
> {
  readonly kind: "projectedOutletContract";
  readonly outletId: string | null;
  readonly parentLayoutRouteId: string | null;
  readonly occupantRouteId: string;
  occupant(): TOccupant;
  descriptor(): ProjectedOutletDescriptor;
  verification(): ProjectedOutletContractVerificationPackage;
  readonly [forgeSignalProjectedOutletContractBrand]: "projectedOutletContract";
}

export interface ProjectedLayoutPlacement<
  TRoute extends string = string,
  TSearch extends RouterSearchSchema = Record<string, never>,
  THash extends RouterHashField<unknown> | null = null,
  TControllers extends RouteControllerMap = Record<string, never>,
  TGraphs extends RouteGraphMap = Record<string, never>,
> {
  readonly kind: "projectedLayoutPlacement";
  readonly outletId: string;
  readonly routeId: string;
  capability(): ProjectedRouteCapability<TRoute, TSearch, THash, TControllers, TGraphs>;
  outlet(): ProjectedOutletContract;
  descriptor(): RouterDescriptor<TRoute, THash>;
  verification(): CanonicalRouteVerificationPackage;
  readonly [forgeSignalProjectedLayoutPlacementBrand]: "projectedLayoutPlacement";
}

export interface ProjectedRouteCandidate<
  TRouteCapability extends ProjectedRouteCapability = ProjectedRouteCapability,
  TLayoutPlacement extends ProjectedLayoutPlacement = ProjectedLayoutPlacement,
> {
  readonly kind: "projectedCandidate";
  readonly href: string;
  readonly routeId: string;
  canonicalUrl(): CanonicalUrlAuthority;
  route(): TRouteCapability;
  layouts(): ReadonlyArray<TLayoutPlacement>;
  outlet(): ProjectedOutletContract<TRouteCapability>;
  outlets(): ReadonlyArray<ProjectedOutletContract>;
  warmup(trigger?: RoutePrefetchTrigger): ProjectedRoutePrefetchArtifact;
  prefetch(trigger?: RoutePrefetchTrigger): ProjectedRoutePrefetchArtifact;
  speculate(
    options?: SpeculativeRouteBranchOptions,
  ): SpeculativeRouteBranchPlan<ProjectedRouteCandidate<TRouteCapability, TLayoutPlacement>>;
  admission(facts?: RouteAdmissionFacts): RouteAdmissionPlan<ProjectedRouteCandidate<TRouteCapability, TLayoutPlacement>>;
  verification(): ProjectedRouteCandidateVerificationPackage;
  readonly [forgeSignalProjectedRouteCandidateBrand]: "projectedRouteCandidate";
}
