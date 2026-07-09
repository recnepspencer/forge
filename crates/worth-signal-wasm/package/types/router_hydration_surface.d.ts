import type {
  RawLocationAuthority,
} from "./router_authority_surface.js";
import type {
  RouteOutcome,
} from "./router_admission_surface.js";

declare const WorthSignalRouterHydrationHandoffBrand: unique symbol;
declare const WorthSignalRouterHydrationHandoffVerificationBrand: unique symbol;
declare const WorthSignalRouterHydrationAdmissionVerificationBrand: unique symbol;

export interface RouterHydrationHandoffVerificationPackage {
  readonly hydrationHandoffDigest: string;
  readonly [WorthSignalRouterHydrationHandoffVerificationBrand]: "routerHydrationHandoffVerificationPackage";
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
  readonly [WorthSignalRouterHydrationHandoffBrand]: "routerHydrationHandoff";
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
  readonly [WorthSignalRouterHydrationAdmissionVerificationBrand]: "routerHydrationAdmissionVerificationPackage";
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
