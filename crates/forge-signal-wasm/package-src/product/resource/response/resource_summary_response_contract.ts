import { createResponseLensProof } from "./resource_response_lens_proof.js";

const RESOURCE_SUMMARY_RESPONSE = Symbol("forgeSignal.resourceSummaryResponse");

function summary() {
  return function defineSummaryResponse() {
    const source = "resource.response.summary<T>()";
    return Object.freeze({
      kind: "summary",
      source,
      lensProof: createResponseLensProof({
        source,
        topology: "summary",
        itemField: null,
        aspectNames: [],
        jsonAspectNames: [],
        summaryNames: [],
        summaryPatchScope: null,
      }),
      [RESOURCE_SUMMARY_RESPONSE]: "resourceSummaryResponse",
    });
  };
}

function requireResourceSummaryResponse(value, kind) {
  if (
    !value ||
    typeof value !== "object" ||
    value[RESOURCE_SUMMARY_RESPONSE] !== "resourceSummaryResponse"
  ) {
    throw new TypeError(`${kind} requires a resource.response summary contract`);
  }
  return value;
}

function isResourceSummaryResponse(value) {
  return Boolean(
    value &&
    typeof value === "object" &&
    value[RESOURCE_SUMMARY_RESPONSE] === "resourceSummaryResponse",
  );
}

export {
  isResourceSummaryResponse,
  requireResourceSummaryResponse,
  summary,
};
