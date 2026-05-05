import { normalizeForProof } from "./resource_verification_package_helpers.mjs";

function projectBehavioralConvergenceDigest(appPackage) {
  return {
    detail: projectBehavioralLineDigest(appPackage.detail),
    nativeCollection: projectBehavioralLineDigest(appPackage.nativeCollection),
    externalCollection: projectBehavioralLineDigest(appPackage.externalCollection),
    paged: projectBehavioralLineDigest(appPackage.paged),
    retryDetail: projectBehavioralLineDigest(appPackage.retryDetail),
    transferDetail: projectBehavioralLineDigest(appPackage.transferDetail),
  };
}

function projectBehavioralLineDigest(pkg) {
  return {
    committedValue: normalizeForProof(pkg.committedValue),
    requestPosture: normalizeForProof(pkg.requestPosture),
    processing: normalizeForProof(pkg.processing),
    upload: normalizeForProof(pkg.upload),
    lifecycle: normalizeForProof(pkg.lifecycle),
    continuity: normalizeForProof(pkg.continuity),
    reconciliation: normalizeForProof(pkg.reconciliation),
    diagnostics: normalizeForProof(pkg.diagnostics),
    binaryDownload: normalizeForProof(pkg.binaryDownload),
    deliveryProvenance: normalizeForProof(pkg.deliveryProvenance),
    externalCompatibility: normalizeForProof(pkg.externalCompatibility),
  };
}

export { projectBehavioralConvergenceDigest };
