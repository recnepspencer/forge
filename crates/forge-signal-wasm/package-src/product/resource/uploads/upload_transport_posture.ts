const RESOURCE_UPLOAD_TRANSPORT_POSTURE_BRAND = Symbol(
  "forgeSignal.resourceUploadTransportPosture",
);

function createResourceUploadTransportPosture(kind, fields) {
  return Object.freeze({
    kind,
    ...fields,
    [RESOURCE_UPLOAD_TRANSPORT_POSTURE_BRAND]: "resourceUploadTransportPosture",
  });
}

function requireResourceUploadTransportPosture(value, family) {
  if (
    !value ||
    value[RESOURCE_UPLOAD_TRANSPORT_POSTURE_BRAND] !==
      "resourceUploadTransportPosture"
  ) {
    throw new TypeError(
      `${family} resources require uploadTransport created with resourceUploadTransport.*()`,
    );
  }
  return value;
}

export {
  createResourceUploadTransportPosture,
  requireResourceUploadTransportPosture,
};
