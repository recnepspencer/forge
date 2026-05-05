import { requireResourceUploadTransportPosture } from "./upload_transport_posture.js";
import { resourceUploadTransport } from "./resource_upload_transport.js";
import {
  readTaggedRequestSourceResolution,
} from "../requests/request_source_metadata.js";

function resolveResourceUploadTransportPosture(input, params, family) {
  if (input === undefined) {
    return Object.freeze({
      value: resourceUploadTransport.none(),
      source: Object.freeze({ source: "default.uploadTransport" }),
    });
  }
  const tagged = readTaggedRequestSourceResolution(input, params);
  if (tagged !== null) {
    return Object.freeze({
      value: requireResourceUploadTransportPosture(tagged.value, family),
      source: tagged.source,
    });
  }
  const value = typeof input === "function" ? input(params) : input;
  return Object.freeze({
    value: requireResourceUploadTransportPosture(value, family),
    source: Object.freeze({ source: "endpoint.uploadTransport" }),
  });
}

export { resolveResourceUploadTransportPosture };
