import type {
  RawLocationAuthority,
} from "./router_authority_surface.js";
import type {
  RouteOutcome,
} from "./router_admission_surface.js";

declare const WORTHSignalRouterHydrationHandoffBrand: unique symbol;
declare const WORTHSignalRouterHydrationHandoffVerificationBrand: unique symbol;
declare const WORTHSignalRouterHydrationAdmissionVerificationBrand: unique symbol;

export interface RouterHydrationHandoffVerificationPackage {
  readonly hydrationHandoffDigest: string;
  readonly [WORTHSignalRouterHydrationHandoffVerificationBrand]: "routerHydrationHandoffVerificationPackage";
}

export interface RouterHydrationServerOptions {
  readonly serverRouteIdentity: string;
  readonly serverHref?: string;
}

export interface RouterHydrationHandoff {
  readonly kind: "routerHydrationHandoff";
  readonly hydrationKind: "server";
  readonly rawLocation: RawLocationAuthority;
  readonly serverRouteIdentity: string;
  readonly serverHref: string | null;
  verification(): RouterHydrationHandoffVerificationPackage;
  readonly [WORTHSignalRouterHydrationHandoffBrand]: "routerHydrationHandoff";
}

export interface RouterHydrationNamespace {
  server(
    location: string | RawLocationAuthority,
    options: RouterHydrationServerOptions,
  ): RouterHydrationHandoff;
}

export interface RouterHydrationAdmissionVerificationPackage {
  readonly hydrationHandoffDigest: string;
  readonly routeTruthDigest: string;
  readonly hydrationBoundaryDigest: string;
  readonly [WORTHSignalRouterHydrationAdmissionVerificationBrand]: "routerHydrationAdmissionVerificationPackage";
}

export interface RouterHydrationAdmissionReport<
  TRouteOutcome extends RouteOutcome = RouteOutcome,
> {
  readonly envelopeFamily: "hydrationHandoff";
  readonly hydrationKind: "server";
  readonly rawLocationHref: string;
  readonly serverRouteIdentity: string;
  readonly serverHref: string | null;
  outcome(): TRouteOutcome;
  diagnostics(): {
    readonly boundarySource: "hydrationHandoff";
    readonly boundaryArtifact:
      | "routeTruthMatchedServer"
      | "routeTruthDriftedFromServer"
      | "routeOutcomeNotAdmitted";
    readonly hydrationKind: "server";
    readonly rawLocationHref: string;
    readonly serverRouteIdentity: string;
    readonly serverHref: string | null;
    readonly outcomeKind: TRouteOutcome["kind"];
    readonly routeId: string | null;
    readonly href: string | null;
  };
  verification(): RouterHydrationAdmissionVerificationPackage;
}
