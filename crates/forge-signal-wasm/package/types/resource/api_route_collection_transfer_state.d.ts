import type {
  ApiRouteProcessingKind,
  ApiRouteUploadKind,
} from "./api_route_transfer_kinds.js";

export type ApiRouteTransferValue<
  TValue,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
> = {
  none: {
    none: TValue;
    signed: TValue | null;
    multipart: TValue | null;
  };
  poll: {
    none: TValue | null;
    signed: TValue | null;
    multipart: TValue | null;
  };
  callback: {
    none: TValue | null;
    signed: TValue | null;
    multipart: TValue | null;
  };
  webhook: {
    none: TValue | null;
    signed: TValue | null;
    multipart: TValue | null;
  };
}[TProcessingKind][TUploadKind];

export type ApiRouteDeclarationForTransferState<
  TStandard,
  TUpload,
  TProcessing,
  TProcessingUpload,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
> = {
  none: {
    none: TStandard;
    signed: TUpload;
    multipart: TUpload;
  };
  poll: {
    none: TProcessing;
    signed: TProcessingUpload;
    multipart: TProcessingUpload;
  };
  callback: {
    none: TProcessing;
    signed: TProcessingUpload;
    multipart: TProcessingUpload;
  };
  webhook: {
    none: TProcessing;
    signed: TProcessingUpload;
    multipart: TProcessingUpload;
  };
}[TProcessingKind][TUploadKind];
