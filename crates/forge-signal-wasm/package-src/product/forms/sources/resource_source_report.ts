import { readResourceLineHandle } from "./form_sources.js";
import { readResourceLineProof } from "./resource_line_proof.js";
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
  const proof = readResourceLineProof(line, request, summary, mutationResponse);
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
    effectProfile: proof.effectProfile,
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
      effectProfile: proof.effectProfile,
      rollback: proof.rollback,
      visibleSelection: proof.visibleSelection,
      history: proof.history,
      verification: proof.verification,
      mutationResponse: proof.mutationResponse,
      counters,
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
      reason: `resource-backed source is waiting on ${report.status.operation}`,
    }));
  }
  if (report.status.kind === "rejected") {
    blockers.push(Object.freeze({
      kind: "resource:rejected",
      ...shared,
      resourceOperation: report.status.operation,
      reason: `resource-backed source request was rejected during ${report.status.operation}: ${report.status.message}`,
    }));
  }
  if (report.status.kind === "timedOut") {
    blockers.push(Object.freeze({
      kind: "resource:timedOut",
      ...shared,
      resourceOperation: report.status.operation,
      reason: `resource-backed source timed out during ${report.status.operation}`,
    }));
  }
  if (report.freshness.kind === "stale") {
    blockers.push(Object.freeze({
      kind: "resource:stale",
      ...shared,
      resourceFreshnessReason: report.freshness.reason,
      reason: `resource-backed source is stale because ${report.freshness.reason}`,
    }));
  }
  return Object.freeze(blockers);
}
