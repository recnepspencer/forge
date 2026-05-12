import {
  assertResponseLensAdmitsPatch,
  createResponseLensDenialError,
} from "../../response/resource_response_effect_locus_lowering.js";

function assertLinePatchRecordAdmitsPatch(patchRecord, patch) {
  assertResponseLensAdmitsPatch(patchRecord.responseLensProof, patch);
  if (patch.kind === "summary") {
    assertLineSummaryPatchScopeAdmitted(patchRecord, patch);
  }
}

function assertLineSummaryPatchScopeAdmitted(patchRecord, patch) {
  const summaryPatchScope = patchRecord.reconcile?.summaries?.patchScope ?? null;
  if (patchRecord.familyKind === "paged" && summaryPatchScope !== "pageWindow") {
    throw createLineSummaryScopeDenial(
      patchRecord,
      patch,
      "pagedSummaryScopeMismatch",
      'paged resource lines require resourceValueSummaries.pageWindow(...) for narrow summary patch(...) admission',
    );
  }
  if (patchRecord.familyKind !== "paged" && summaryPatchScope === "pageWindow") {
    throw createLineSummaryScopeDenial(
      patchRecord,
      patch,
      "listSummaryScopeMismatch",
      `${patchRecord.familyKind} resource lines do not admit resourceValueSummaries.pageWindow(...) summary patch(...)`,
    );
  }
}

function createLineSummaryScopeDenial(patchRecord, patch, reason, message) {
  if (patchRecord.responseLensProof === null) {
    return new TypeError(message);
  }
  return createResponseLensDenialError(
    patchRecord.responseLensProof,
    Object.freeze({ kind: "summary", summary: patch.summary }),
    reason,
    message,
  );
}

export { assertLinePatchRecordAdmitsPatch };
