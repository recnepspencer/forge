import type {
  RawLocationAuthority,
} from "./router_authority_surface.js";
import type {
  ProjectedRoutePrefetchArtifact,
  RoutePrefetchTrigger,
} from "./router_transition_surface.js";

declare const WorthSignalRouterWarmupIngressBrand: unique symbol;
declare const WorthSignalRouterWarmupIngressVerificationBrand: unique symbol;
declare const WorthSignalRouterWarmupReportVerificationBrand: unique symbol;

export interface RouterWarmupIngressOptions {
  readonly sourceId?: string;
  readonly sourceValue?: unknown;
  readonly routeIdentity?: string;
}

export interface RouterWarmupIngressVerificationPackage {
  readonly routeWarmupIngressDigest: string;
  readonly [WorthSignalRouterWarmupIngressVerificationBrand]: "routerWarmupIngressVerificationPackage";
}

export interface RouterWarmupIngress {
  readonly kind: "routerWarmupIngress";
  readonly trigger: RoutePrefetchTrigger;
  readonly rawLocation: RawLocationAuthority;
  readonly sourceId: string | null;
  readonly sourceValue: unknown;
  readonly routeIdentity: string | null;
  verification(): RouterWarmupIngressVerificationPackage;
  readonly [WorthSignalRouterWarmupIngressBrand]: "routerWarmupIngress";
}

export interface RouterWarmupNamespace {
  hover(location: string | RawLocationAuthority, options?: RouterWarmupIngressOptions): RouterWarmupIngress;
  focus(location: string | RawLocationAuthority, options?: RouterWarmupIngressOptions): RouterWarmupIngress;
  viewport(location: string | RawLocationAuthority, options?: RouterWarmupIngressOptions): RouterWarmupIngress;
  intent(location: string | RawLocationAuthority, options?: RouterWarmupIngressOptions): RouterWarmupIngress;
}

export interface RouterWarmupReportVerificationPackage {
  readonly routeWarmupIngressDigest: string;
  readonly routeWarmupReportDigest: string;
  readonly [WorthSignalRouterWarmupReportVerificationBrand]: "routerWarmupReportVerificationPackage";
}

export interface RouterWarmupReport {
  readonly envelopeFamily: "routeWarmupIngress";
  readonly trigger: RoutePrefetchTrigger;
  readonly rawLocationHref: string;
  readonly routeIdentity: string | null;
  artifact(): ProjectedRoutePrefetchArtifact | null;
  diagnostics(): {
    readonly boundarySource: "routeWarmupIngress";
    readonly boundaryArtifact: "routeWarmupStarted" | "noMatchingWarmupResources" | "noProjectedCandidate";
    readonly trigger: RoutePrefetchTrigger;
    readonly rawLocationHref: string;
    readonly routeIdentity: string | null;
    readonly warmedResourceNames: ReadonlyArray<string>;
    readonly skippedResourceNames: ReadonlyArray<string>;
  };
  verification(): RouterWarmupReportVerificationPackage;
}
