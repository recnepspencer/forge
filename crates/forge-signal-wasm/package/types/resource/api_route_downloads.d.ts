import type {
  ResourceBinaryDescriptor,
  ResourceDownloadDescriptorState,
  ResourceDownloadIncompatible,
  ResourceDownloadReady,
  ResourceDownloadUnavailable,
} from "./resource_postures.js";

type ApiRouteBinaryDescriptorOptions = {
  label?: string | null;
  fileName?: string | null;
  mediaType?: string | null;
  byteLength?: number | null;
  download: ResourceDownloadDescriptorState;
};

export interface ApiRouteDownloadsBuilder {
  ready(options: {
    url: string;
    method?: "GET" | "POST";
    headers?: Record<string, string>;
    expiresAt?: string | null;
  }): ResourceDownloadReady;
  multipart(options: {
    url: string;
    headers?: Record<string, string>;
    fields?: Record<string, string>;
    objectKey?: string | null;
    expiresAt?: string | null;
  }): ResourceDownloadReady;
  unavailable(options: {
    reason: "notReady" | "unavailable";
    detail: string;
  }): ResourceDownloadUnavailable;
  incompatible(options: {
    reason: "staleDescriptor" | "transportBoundary";
    detail: string;
  }): ResourceDownloadIncompatible;
  file(id: string, options: ApiRouteBinaryDescriptorOptions): ResourceBinaryDescriptor;
  media(id: string, options: ApiRouteBinaryDescriptorOptions): ResourceBinaryDescriptor;
  export(id: string, options: ApiRouteBinaryDescriptorOptions): ResourceBinaryDescriptor;
}

export type ApiRouteDownloadsDeclaration<TParams, TValue> = (
  params: TParams,
  value: TValue,
  download: ApiRouteDownloadsBuilder,
) => readonly ResourceBinaryDescriptor[];
