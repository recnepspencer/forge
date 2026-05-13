import {
  isResourceBinaryValue,
  resourceBinaryValue,
} from "../../resource/downloads/resource_binary_value.js";
import { isProcessingResult } from "../../resource/processing/processing_result.js";
import { isUploadResult } from "../../resource/uploads/upload_result.js";
import { apiRouteDownloadsBuilder } from "./api_route_download_builder.js";

function applyOwnedBinaryDownloads(route, declaration, downloadsState) {
  const builderDownloads = downloadsState.declaration;
  if (builderDownloads !== undefined) {
    if ("downloads" in declaration && declaration.downloads !== undefined) {
      throw new TypeError(
        `api.url("${route}").downloads(...) owns downloads(...) in the pleasant lane`,
      );
    }
    return lowerOwnedBinaryDownloads(route, declaration, builderDownloads);
  }
  if (!("downloads" in declaration) || declaration.downloads === undefined) {
    return { ...declaration };
  }
  return lowerOwnedBinaryDownloads(route, declaration, declaration.downloads);
}

function lowerOwnedBinaryDownloads(route, declaration, downloads) {
  if (typeof declaration.downloads !== "function" && typeof downloads !== "function") {
    throw new TypeError(
      `api.url("${route}") downloads(...) must be declared as a function`,
    );
  }
  const { load, ...rest } = declaration;
  delete rest.downloads;
  return {
    ...rest,
    load(params, request) {
      const loaded = load(params, request);
      if (
        loaded
        && typeof loaded === "object"
        && typeof loaded.then === "function"
      ) {
        return loaded.then((value) =>
          lowerOwnedBinaryDownloadValue(route, params, value, downloads));
      }
      return lowerOwnedBinaryDownloadValue(route, params, loaded, downloads);
    },
  };
}

function lowerOwnedBinaryDownloadValue(route, params, value, downloads) {
  if (isProcessingResult(value) || isUploadResult(value)) {
    return value;
  }
  if (isResourceBinaryValue(value)) {
    throw new TypeError(
      `api.url("${route}") downloads(...) owns resourceBinaryValue(...) in the pleasant lane`,
    );
  }
  return resourceBinaryValue({
    value,
    descriptors: downloads(params, value, apiRouteDownloadsBuilder),
  });
}

export { applyOwnedBinaryDownloads };
