import type {
  RawLocationAuthority,
} from "./router_authority_surface.js";
import type {
  RouteOutcome,
} from "./router_admission_surface.js";

declare const forgeSignalRouterHydrationHandoffBrand: unique symbol;
declare const forgeSignalRouterHydrationHandoffVerificationBrand: unique symbol;
declare const forgeSignalRouterHydrationAdmissionVerificationBrand: unique symbol;

export interface RouterHydrationHandoffVerificationPackage {
  readonly hydrationHandoffDigest: string;
  readonly [forgeSignalRouterHydrationHandoffVerificationBrand]: "routerHydrationHandoffVerificationPackage";
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
  readonly [forgeSignalRouterHydrationHandoffBrand]: "routerHydrationHandoff";
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
  readonly [forgeSignalRouterHydrationAdmissionVerificationBrand]: "routerHydrationAdmissionVerificationPackage";
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
