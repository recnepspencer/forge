import { readResourceLineHandle } from "./form_sources.js";
import { readFormResourceExternalCompatibilityReport } from "./resource_external_compatibility_report.js";
import { readFormResourceLifecycleReport } from "./resource_lifecycle_report.js";
import { readResourceLineProof } from "./resource_line_proof.js";
import { readResourceTransferReport } from "./resource_transfer_report.js";
import { stableValueDigest } from "../values/value_paths.js";

export function readResourceSourceReport(source) {
  const line = readResourceLineHandle(source);
  if (line === null) {
    return null;
  }
  const descriptor = line.descriptor();
  const request = line.request();
  const summary = line.summary();
  const status = line.status();
  const freshness = line.freshness();
  const mutationResponse = line.mutationResponse();
  const proof = readResourceLineProof(line, request, summary, status, freshness, mutationResponse);
  const shape = readResourceShapeReport(descriptor);
  const transfer = readResourceTransferReport(source);
  const lifecycle = readFormResourceLifecycleReport(summary, status, freshness);
  const externalCompatibility = readFormResourceExternalCompatibilityReport(
    proof.verification.externalCompatibility,
  );
  const counters = Object.freeze({
    costBasis: "resourceLineProofRead",
    incrementalStatus: "notIncremental",
    descriptorReads: 1,
    requestReads: 1,
    summaryReads: 1,
    statusReads: 1,
    freshnessReads: 1,
    mutationResponseReads: 1,
    historyReads: 1,
    verificationPackageReads: 1,
    effectCloseoutMatrixReads: proof.effectProfile.profile === null ? 0 : 1,
    mutationResponseCloseoutMatrixReads: mutationResponse === null ? 0 : 1,
  });
  return Object.freeze({
    sourceKind: "resourceLine",
    descriptor,
    request,
    summary,
    status,
    freshness,
    shape,
    externalCompatibility,
    transfer,
    lifecycle,
    settlement: proof.settlement,
    effectProfile: proof.effectProfile,
    effects: proof.effects,
    rollback: proof.rollback,
    visibleSelection: proof.visibleSelection,
    history: proof.history,
    verification: proof.verification,
    mutationResponse: proof.mutationResponse,
    counters,
    digest: stableValueDigest({
      descriptor,
      request,
      summary,
      status,
      freshness,
      shape,
      externalCompatibility,
      transfer,
      lifecycle,
      settlement: proof.settlement,
      effectProfile: proof.effectProfile,
      effects: proof.effects,
      rollback: proof.rollback,
      visibleSelection: proof.visibleSelection,
      history: proof.history,
      verification: proof.verification,
      mutationResponse: proof.mutationResponse,
      counters,
    }),
  });
}

function readResourceShapeReport(descriptor) {
  const familyKind = descriptor.family.kind;
  const patchLowering = familyKind === "detail"
    ? "detailFieldJsonPathRegion"
    : familyKind === "collection"
      ? "collectionMembershipItemFieldJsonPathRegion"
      : "pagedWindowMembershipItemFieldJsonPathRegion";
  return Object.freeze({
    familyKind,
    familyId: descriptor.family.familyId,
    runtimeLineId: descriptor.runtimeLineId,
    scopeId: descriptor.scopeId,
    canonicalKey: descriptor.canonicalParams.canonicalKey,
    patchLowering,
    digest: stableValueDigest({
      familyKind,
      familyId: descriptor.family.familyId,
      runtimeLineId: descriptor.runtimeLineId,
      scopeId: descriptor.scopeId,
      canonicalKey: descriptor.canonicalParams.canonicalKey,
      patchLowering,
    }),
  });
}

export function resourceSourceReadinessBlockers(report, actionId = undefined) {
  if (report === null) {
    return Object.freeze([]);
  }
  const shared = actionId === undefined ? {} : { action: actionId };
  const blockers = [];
  if (report.status.kind === "pending") {
    blockers.push(Object.freeze({
      kind: "resource:pending",
      ...shared,
      resourceOperation: report.status.operation,
      reason: `resource line source is waiting on ${report.status.operation}`,
    }));
  }
  if (report.status.kind === "rejected") {
    blockers.push(Object.freeze({
      kind: "resource:rejected",
      ...shared,
      resourceOperation: report.status.operation,
      reason: `resource line source request was rejected during ${report.status.operation}: ${report.status.message}`,
    }));
  }
  if (report.status.kind === "timedOut") {
    blockers.push(Object.freeze({
      kind: "resource:timedOut",
      ...shared,
      resourceOperation: report.status.operation,
      reason: `resource line source timed out during ${report.status.operation}`,
    }));
  }
  if (report.freshness.kind === "stale" && report.freshness.reason === "deliveryInvalidate") {
    blockers.push(Object.freeze({
      kind: "resource:deliveryBasisDrift",
      ...shared,
      reason: "resource line source delivery basis drifted after invalidate delivery",
    }));
  } else if (report.freshness.kind === "stale") {
    blockers.push(Object.freeze({
      kind: "resource:stale",
      ...shared,
      resourceFreshnessReason: report.freshness.reason,
      reason: `resource line source is stale because ${report.freshness.reason}`,
    }));
  }
  return Object.freeze(blockers);
}
