import { assertResponseLensAdmitsPatch } from "../../response/resource_response_lens_proof.js";

function assertLinePatchRecordAdmitsPatch(patchRecord, patch) {
  assertResponseLensAdmitsPatch(patchRecord.responseLensProof, patch);
  if (patch.kind === "summary") {
    assertLineSummaryPatchScopeAdmitted(patchRecord);
  }
}

function assertLineSummaryPatchScopeAdmitted(patchRecord) {
  const summaryPatchScope = patchRecord.reconcile?.summaries?.patchScope ?? null;
  if (patchRecord.familyKind === "paged" && summaryPatchScope !== "pageWindow") {
    throw new TypeError(
      'paged resource lines require resourceValueSummaries.pageWindow(...) for narrow summary patch(...) admission',
    );
  }
  if (patchRecord.familyKind !== "paged" && summaryPatchScope === "pageWindow") {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit resourceValueSummaries.pageWindow(...) summary patch(...)`,
    );
  }
}

export { assertLinePatchRecordAdmitsPatch };
