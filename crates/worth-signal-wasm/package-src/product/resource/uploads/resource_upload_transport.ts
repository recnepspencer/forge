import { createResourceUploadTransportPosture } from "./upload_transport_posture.js";

const resourceUploadTransport = Object.freeze({
  none() {
    return createResourceUploadTransportPosture("none", {});
  },
  directMultipart(options = {}) {
    return createResourceUploadTransportPosture("directMultipart", {
      finalizeRequired: normalizeOptionalBoolean(
        options.finalizeRequired,
        false,
      ),
    });
  },
  signed(options = {}) {
    return createResourceUploadTransportPosture("signed", {
      method: normalizeSignedMethod(options.method),
      finalizeRequired: normalizeOptionalBoolean(
        options.finalizeRequired,
        true,
      ),
    });
  },
});

function normalizeSignedMethod(value) {
  if (value === undefined) {
    return "PUT";
  }
  if (value !== "PUT" && value !== "POST") {
    throw new TypeError(
      'resourceUploadTransport signed method must be "PUT" or "POST"',
    );
  }
  return value;
}

function normalizeOptionalBoolean(value, defaultValue) {
  if (value === undefined) {
    return defaultValue;
  }
  if (typeof value !== "boolean") {
    throw new TypeError(
      "resourceUploadTransport finalizeRequired must be a boolean",
    );
  }
  return value;
}

export { resourceUploadTransport };
