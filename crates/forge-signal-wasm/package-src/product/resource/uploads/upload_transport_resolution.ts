import { requireResourceUploadTransportPosture } from "./upload_transport_posture.js";
import { resourceUploadTransport } from "./resource_upload_transport.js";

function resolveResourceUploadTransportPosture(input, params, family) {
  if (input === undefined) {
    return resourceUploadTransport.none();
  }
  const value = typeof input === "function" ? input(params) : input;
  return requireResourceUploadTransportPosture(value, family);
}

export { resolveResourceUploadTransportPosture };
