import type { SignalValue } from "../model.js";
import type { ResourceNamespace } from "./resource_namespace.js";
import type {
  ResourceAuthPosture,
  ResourceContinuationPosture,
  ResourceProcessingJobPosture,
  ResourceRequestContext,
  ResourceUploadTransportPosture,
} from "./resource_postures.js";
import type { ResourceEffectProfile } from "./resource_effect_profiles.js";
import type { ApiRouteBuilder } from "./api_route_builder.js";
import type { ApiRouteConstraint } from "./api_route_types.js";

type ApiRouteHeaders<TParams> =
  | Record<string, string>
  | ((params: TParams) => Record<string, string>);

export interface ApiScopedDefaults<
  TParams extends object = Record<string, SignalValue>,
> {
  baseUrl?: string | ((params: TParams) => string);
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  headers?: ApiRouteHeaders<TParams>;
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  processingJob?:
    | ResourceProcessingJobPosture
    | ((params: TParams) => ResourceProcessingJobPosture);
  uploadTransport?:
    | ResourceUploadTransportPosture
    | ((params: TParams) => ResourceUploadTransportPosture);
  effects?:
    | ResourceEffectProfile
    | ((params: TParams) => ResourceEffectProfile);
}

export interface ApiStableScopedDefaults {
  baseUrl?: string;
  auth?: ResourceAuthPosture;
  headers?: Record<string, string>;
  requestContext?: ResourceRequestContext;
  continuation?: ResourceContinuationPosture;
  processingJob?: ResourceProcessingJobPosture;
  uploadTransport?: ResourceUploadTransportPosture;
  effects?: ResourceEffectProfile;
}

export interface ApiNamespace
  extends Pick<ResourceNamespace, "detail" | "collection" | "paged"> {
  scope<TParams extends object = Record<string, SignalValue>>(
    options?: ApiScopedDefaults<TParams>,
  ): ApiNamespace;
  scope(
    scopeId: string,
    options?: ApiStableScopedDefaults,
  ): ApiScopedNamespace;
  url<TRoute extends string>(
    route: TRoute & ApiRouteConstraint<TRoute>,
  ): ApiRouteBuilder<TRoute>;
}

export interface ApiScopedNamespace extends ApiNamespace {
  readonly scopeId: string;
}

export interface ApiFactory {
  <TParams extends object = Record<string, SignalValue>>(
    options?: ApiScopedDefaults<TParams>,
  ): ApiNamespace;
}

export interface ApiScopeFactory {
  (
    scopeId: string,
    options?: ApiStableScopedDefaults,
  ): ApiScopedNamespace;
}
