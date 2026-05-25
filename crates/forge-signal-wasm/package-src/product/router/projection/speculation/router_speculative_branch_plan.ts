import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";
import { ROUTE_SPECULATIVE_BRANCH_PLAN } from "../../router_symbols.js";
import {
  createSpeculativePendingOutcome,
  createSpeculativeRejectedOutcome,
} from "./router_speculative_branch_outcome.js";
import { openSpeculativeRouteBranchSession } from "./router_speculative_branch_session.js";

const COMMIT_POSTURES = Object.freeze([
  "merge-preview-before-commit",
  "direct-merge-commit",
]);
const DISCARD_POSTURES = Object.freeze([
  "discard-speculative-branch",
  "keep-branch-pending",
]);
const VISIBLE_POSTURES = Object.freeze([
  "preserve-visible-until-commit",
  "allow-visible-flicker-before-commit",
]);

function createSpeculativeRouteBranchPlan(candidate, options = {}) {
  const normalizedOptions = normalizeSpeculativeBranchOptions(options);
  const branchName = normalizedOptions.branchName
    ?? createDefaultSpeculativeBranchName(candidate);
  const branching = Object.freeze({
    candidateTruth: "branch-native-candidate-route",
    branchLifecycle: "create-branch-before-commit",
    branchName,
    commitPosture: normalizedOptions.commitPosture,
    discardPosture: normalizedOptions.discardPosture,
    visiblePosture: normalizedOptions.visiblePosture,
    dirtyExit: "evaluate-dirty-before-commit",
  });
  const diagnostics = Object.freeze({
    flickerSuppression:
      normalizedOptions.visiblePosture === "preserve-visible-until-commit"
        ? "suppresses-visible-flicker-until-commit"
        : "allows-visible-flicker-before-commit",
    commitDisposition:
      normalizedOptions.commitPosture === "merge-preview-before-commit"
        ? "requires-merge-preview-before-commit"
        : "allows-direct-merge-commit",
    discardDisposition:
      normalizedOptions.discardPosture === "discard-speculative-branch"
        ? "discard-ends-speculation"
        : "discard-keeps-branch-pending",
    pendingDisposition: "candidate-route-remains-pending-until-commit",
    dirtyExitDisposition: "requires-dirty-evaluation-before-commit",
  });
  const verification = Object.freeze({
    projectedCandidateDigest: candidate.verification().projectedCandidateDigest,
    speculativeBranchDigest: createCanonicalDigest("speculative-branch-plan", {
      routeId: candidate.routeId,
      href: candidate.href,
      branchName,
      commitPosture: normalizedOptions.commitPosture,
      discardPosture: normalizedOptions.discardPosture,
      visiblePosture: normalizedOptions.visiblePosture,
    }),
    speculativeLifecycleDigest: createCanonicalDigest("speculative-branch-lifecycle", {
      candidateTruth: branching.candidateTruth,
      branchLifecycle: branching.branchLifecycle,
      dirtyExit: branching.dirtyExit,
      commitPosture: branching.commitPosture,
      discardPosture: branching.discardPosture,
      visiblePosture: branching.visiblePosture,
    }),
    speculativeDiagnosticsDigest: createCanonicalDigest("speculative-branch-diagnostics", diagnostics),
  });
  return Object.freeze({
    [ROUTE_SPECULATIVE_BRANCH_PLAN]: true,
    kind: "speculativeBranchPlan",
    href: candidate.href,
    routeId: candidate.routeId,
    candidate() {
      return candidate;
    },
    branching() {
      return branching;
    },
    diagnostics() {
      return diagnostics;
    },
    open(history) {
      return openSpeculativeRouteBranchSession(this, history);
    },
    async evaluate(facts = {}) {
      const routeOutcome = await candidate.admission(facts).resolve();
      return routeOutcome.kind === "admitted"
        ? createSpeculativePendingOutcome(this, routeOutcome)
        : createSpeculativeRejectedOutcome(this, routeOutcome);
    },
    verification() {
      return verification;
    },
  });
}

function normalizeSpeculativeBranchOptions(options) {
  if (!isPlainObject(options)) {
    throw new TypeError(
      "projectedCandidate.speculate(...) options must be an object when provided",
    );
  }
  const unknownKeys = Object.keys(options).filter(
    (key) => !["branchName", "commitPosture", "discardPosture", "visiblePosture"].includes(key),
  );
  if (unknownKeys.length > 0) {
    throw new TypeError(
      `projectedCandidate.speculate(...) does not support: ${unknownKeys.join(", ")}`,
    );
  }
  const branchName = normalizeOptionalBranchName(options.branchName);
  const commitPosture = normalizeSpeculativeOption(
    options.commitPosture,
    COMMIT_POSTURES,
    "projectedCandidate.speculate(...) commitPosture",
    "merge-preview-before-commit",
  );
  const discardPosture = normalizeSpeculativeOption(
    options.discardPosture,
    DISCARD_POSTURES,
    "projectedCandidate.speculate(...) discardPosture",
    "discard-speculative-branch",
  );
  const visiblePosture = normalizeSpeculativeOption(
    options.visiblePosture,
    VISIBLE_POSTURES,
    "projectedCandidate.speculate(...) visiblePosture",
    "preserve-visible-until-commit",
  );
  return Object.freeze({
    branchName,
    commitPosture,
    discardPosture,
    visiblePosture,
  });
}

function normalizeSpeculativeOption(value, allowedValues, label, fallback) {
  if (value === undefined) {
    return fallback;
  }
  if (!allowedValues.includes(value)) {
    throw new TypeError(`${label} must be one of ${allowedValues.join(", ")}`);
  }
  return value;
}

function normalizeOptionalBranchName(value) {
  if (value === undefined) {
    return undefined;
  }
  if (typeof value !== "string" || value.length === 0) {
    throw new TypeError(
      "projectedCandidate.speculate(...) branchName must be a non-empty string when provided",
    );
  }
  return value;
}

function createDefaultSpeculativeBranchName(candidate) {
  return `speculative:${candidate.routeId}:${encodeURIComponent(candidate.href)}`;
}

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

export {
  createSpeculativeRouteBranchPlan,
};
