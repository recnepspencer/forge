import type {
  ResourceLine,
  ResourceLineFreshness,
  ResourceLineStatus,
} from "./resource/resource_lifecycle.js";
import type {
  CollectionResourceFamily,
  DetailResourceFamily,
  PagedResourceFamily,
} from "./resource/resource_family_surfaces.js";
import type {
  ResourceLineDiagnosticsSummary,
} from "./resource/resource_line_summary.js";
import type {
  ResourceLineDescriptor,
} from "./resource/resource_request_descriptor.js";

declare const forgeSignalRouteResourceDeclarationBrand: unique symbol;

export type RouteResourceLineFamily =
  | DetailResourceFamily<any, any, any>
  | CollectionResourceFamily<any, any, any, any>
  | PagedResourceFamily<any, any, any, any>;

export type RouteResourcePrefetchPosture =
  | "hover"
  | "focus"
  | "viewport"
  | "intent";

export interface RouteResourceResolveContext {
  readonly routeId: string;
  readonly href: string;
  readonly params: Readonly<Record<string, string>>;
  readonly search: Readonly<Record<string, unknown>>;
  readonly hash: unknown;
}

export interface RouteResourceDeclarationVerificationPackage {
  readonly routeResourceBindingDigest: string;
}

export interface RouteResourcePrefetchVerificationPackage {
  readonly routeResourcePrefetchDigest: string;
}

export interface RouteResourceCurrentState {
  readonly descriptor: ResourceLineDescriptor<unknown>;
  readonly status: ResourceLineStatus;
  readonly freshness: ResourceLineFreshness;
  readonly diagnosticsSummary: ResourceLineDiagnosticsSummary;
}

export interface RouteResourceDeclaration<
  TFamily extends RouteResourceLineFamily = RouteResourceLineFamily,
> {
  readonly family: TFamily;
  readonly resolveParams: (
    route: RouteResourceResolveContext,
  ) => Parameters<TFamily["line"]>[0];
  readonly prefetch: RouteResourcePrefetchPosture;
  readonly [forgeSignalRouteResourceDeclarationBrand]: "routeResourceDeclaration";
}

export type RouteResourceMap = Record<string, RouteResourceDeclaration>;

export interface ProjectedRouteResourceCapability {
  readonly kind: "projectedRouteResourceCapability";
  readonly routeId: string;
  readonly name: string;
  prefetchPosture(): RouteResourcePrefetchPosture;
  prefetch(trigger?: RouteResourcePrefetchPosture): RouteResourcePrefetchArtifact;
  warmup(trigger?: RouteResourcePrefetchPosture): RouteResourcePrefetchArtifact;
  verification(): RouteResourceDeclarationVerificationPackage;
}

export interface RouteResourcePrefetchArtifact extends RouteResourceCurrentState {
  readonly kind: "routeResourcePrefetch";
  readonly routeId: string;
  readonly href: string;
  readonly name: string;
  readonly prefetchPosture: RouteResourcePrefetchPosture;
  readonly trigger: RouteResourcePrefetchPosture;
  line(): ResourceLine;
  current(): RouteResourceCurrentState;
  free(): void;
  [Symbol.dispose](): void;
  verification(): RouteResourcePrefetchVerificationPackage;
}

export interface AdmittedRouteResourceCapability {
  readonly kind: "admittedRouteResourceCapability";
  readonly routeId: string;
  readonly name: string;
  prefetchPosture(): RouteResourcePrefetchPosture;
  line(): ResourceLine;
  current(): RouteResourceCurrentState;
  verification(): RouteResourceDeclarationVerificationPackage;
}
