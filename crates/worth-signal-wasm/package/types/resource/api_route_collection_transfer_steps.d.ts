import type {
  DirectMultipartUploadTransportOptions,
  CallbackProcessingJobOptions,
  SignedUploadTransportOptions,
  WebhookProcessingJobOptions,
} from "./resource_postures.js";
import type { ApiRouteProcessingKind, ApiRouteUploadKind } from "./api_route_transfer_kinds.js";

export type ApiRouteCollectionTransferStep<
  TRoute extends string,
  TRequestParams,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TWithSigned,
  TWithMultipart,
  TWithPoll,
  TWithCallback,
  TWithWebhook,
> = (TUploadKind extends "none"
  ? {
      signedUpload(options?: SignedUploadTransportOptions): TWithSigned;
      multipartUpload(options?: DirectMultipartUploadTransportOptions): TWithMultipart;
    }
  : {}) &
  (TProcessingKind extends "none"
    ? {
        processing(kind: "poll"): TWithPoll;
        processing(kind: "callback", options: CallbackProcessingJobOptions): TWithCallback;
        processing(kind: "webhook", options: WebhookProcessingJobOptions): TWithWebhook;
      }
    : {});
