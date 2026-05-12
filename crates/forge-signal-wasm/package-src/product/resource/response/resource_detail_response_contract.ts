import { createResponseLensProof } from "./resource_response_lens_proof.js";

const RESOURCE_DETAIL_RESPONSE = Symbol("forgeSignal.resourceDetailResponse");

function detail() {
  return function defineDetailResponse() {
    const source = "resource.response.detail<T>()";
    return Object.freeze({
      kind: "detail",
      source,
      lensProof: createResponseLensProof({
        source,
        topology: "detail",
        itemField: null,
        aspectNames: [],
        jsonAspectNames: [],
        summaryNames: [],
        summaryPatchScope: null,
      }),
      [RESOURCE_DETAIL_RESPONSE]: "resourceDetailResponse",
    });
  };
}

function requireResourceDetailResponse(value, kind) {
  if (
    !value ||
    typeof value !== "object" ||
    value[RESOURCE_DETAIL_RESPONSE] !== "resourceDetailResponse"
  ) {
    throw new TypeError(`${kind} requires a resource.response detail contract`);
  }
  return value;
}

function isResourceDetailResponse(value) {
  return Boolean(
    value &&
    typeof value === "object" &&
    value[RESOURCE_DETAIL_RESPONSE] === "resourceDetailResponse",
  );
}

export {
  detail,
  isResourceDetailResponse,
  requireResourceDetailResponse,
};
