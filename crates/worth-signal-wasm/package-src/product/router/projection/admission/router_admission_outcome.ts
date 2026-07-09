import {
  ROUTE_OUTCOME,
} from "../../router_symbols.js";
import {
  createCanonicalDigest,
} from "../../url_authority/router_verification_packages.js";
import {
  createAdmissionDecisionProvenance,
  createRecoveryTrailEntry,
  createRouteOutcomeProvenance,
} from "./router_admission_provenance.js";
import { createAdmittedRouteCapability } from "./router_admitted_capability.js";

function createNotFoundRouteOutcome(rawAuthority, normalizedFacts = {}) {
  const terminalArtifact = Object.freeze({
    kind: "notFound",
    prerequisite: null,
    reason: "noProjectedCandidate",
    detail: "No declared route projected a candidate from the provided route authority.",
    href: null,
  });
  const provenance = createRouteOutcomeProvenance({
    attemptedRouteId: null,
    attemptedHref: rawAuthority?.href ?? null,
    resolvedRouteId: null,
    resolvedHref: rawAuthority?.href ?? null,
    terminalSource: "noProjectedCandidate",
    terminalArtifact,
    prerequisiteDecisions: [],
    recoveryTrail: [],
  });
  const verification = Object.freeze({
    routeId: null,
    admissionPlanDigest: createCanonicalDigest("route-admission-plan", {
      routeId: null,
      canonicalUrlDigest: rawAuthority?.canonicalUrlDigest ?? null,
      prerequisiteNames: [],
      recoveryNames: [],
      factsKeys: Object.keys(normalizedFacts).sort(),
    }),
    formsAuthorityDigest: null,
    routeOutcomeDigest: createCanonicalDigest("route-outcome", {
      kind: "notFound",
      href: rawAuthority?.href ?? null,
      reason: "noProjectedCandidate",
      provenance,
    }),
  });
  return Object.freeze({
    [ROUTE_OUTCOME]: true,
    kind: "notFound",
    routeId: null,
    href: rawAuthority?.href ?? null,
    artifact() {
      return terminalArtifact;
    },
    diagnostics() {
      return Object.freeze({
        routeId: null,
        outcomeKind: "notFound",
        formsAuthority: null,
        prerequisiteDecisions: Object.freeze([]),
        recovery: null,
      });
    },
    recovery() {
      return null;
    },
    provenance() {
      return provenance;
    },
    verification() {
      return verification;
    },
  });
}

function createAdmittedRouteOutcome(
  projectedCandidate,
  routeDeclaration,
  prerequisiteDecisions,
  planVerification,
) {
  const admittedRouteCapability = createAdmittedRouteCapability(projectedCandidate.route(), routeDeclaration);
  const formsAuthority = admittedRouteCapability.formsAuthority();
  const provenance = createRouteOutcomeProvenance({
    attemptedRouteId: projectedCandidate.routeId,
    attemptedHref: projectedCandidate.href,
    resolvedRouteId: projectedCandidate.routeId,
    resolvedHref: projectedCandidate.href,
    terminalSource: "admittedWithoutRecovery",
    terminalArtifact: null,
    prerequisiteDecisions: prerequisiteDecisions.map((decision) => (
      createAdmissionDecisionProvenance(projectedCandidate, decision, decision.consumedSources)
    )),
    recoveryTrail: [],
  });
  const verification = Object.freeze({
    routeId: projectedCandidate.routeId,
    admissionPlanDigest: planVerification.admissionPlanDigest,
    formsAuthorityDigest: formsAuthority?.verification().formsAuthorityDigest ?? null,
    routeOutcomeDigest: createCanonicalDigest("route-outcome", {
      kind: "admitted",
      routeId: projectedCandidate.routeId,
      href: projectedCandidate.href,
      provenance,
    }),
  });
  return Object.freeze({
    [ROUTE_OUTCOME]: true,
    kind: "admitted",
    routeId: projectedCandidate.routeId,
    href: projectedCandidate.href,
    route() {
      return admittedRouteCapability;
    },
    layouts() {
      return projectedCandidate.layouts();
    },
    outlet() {
      return projectedCandidate.outlet();
    },
    outlets() {
      return projectedCandidate.outlets();
    },
    diagnostics() {
      return Object.freeze({
        routeId: projectedCandidate.routeId,
        outcomeKind: "admitted",
        formsAuthority,
        prerequisiteDecisions: Object.freeze(prerequisiteDecisions.slice()),
        recovery: null,
      });
    },
    recovery() {
      return null;
    },
    provenance() {
      return provenance;
    },
    verification() {
      return verification;
    },
  });
}

function createRejectedRouteOutcome(
  projectedCandidate,
  terminalArtifact,
  prerequisiteDecisions,
  planVerification,
) {
  const provenance = createRouteOutcomeProvenance({
    attemptedRouteId: projectedCandidate.routeId,
    attemptedHref: projectedCandidate.href,
    resolvedRouteId: projectedCandidate.routeId,
    resolvedHref: projectedCandidate.href,
    terminalSource: "prerequisiteArtifact",
    terminalArtifact,
    prerequisiteDecisions: prerequisiteDecisions.map((decision) => (
      createAdmissionDecisionProvenance(projectedCandidate, decision, decision.consumedSources)
    )),
    recoveryTrail: [],
  });
  const verification = Object.freeze({
    routeId: projectedCandidate.routeId,
    admissionPlanDigest: planVerification.admissionPlanDigest,
    formsAuthorityDigest: null,
    routeOutcomeDigest: createCanonicalDigest("route-outcome", {
      kind: terminalArtifact.kind,
      routeId: projectedCandidate.routeId,
      href: projectedCandidate.href,
      provenance,
    }),
  });
  return Object.freeze({
    [ROUTE_OUTCOME]: true,
    kind: terminalArtifact.kind,
    routeId: projectedCandidate.routeId,
    href: projectedCandidate.href,
    artifact() {
      return terminalArtifact;
    },
    diagnostics() {
      return Object.freeze({
        routeId: projectedCandidate.routeId,
        outcomeKind: terminalArtifact.kind,
        formsAuthority: null,
        prerequisiteDecisions: Object.freeze(prerequisiteDecisions.slice()),
        recovery: null,
      });
    },
    recovery() {
      return null;
    },
    provenance() {
      return provenance;
    },
    verification() {
      return verification;
    },
  });
}

function attachRouteRecoveryArtifact(
  routeOutcome,
  recoveryArtifact,
  originalCandidate,
  terminalArtifact,
  originalPrerequisiteDecisions,
) {
  const routeOutcomeVerification = routeOutcome.verification();
  const routeOutcomeDiagnostics = routeOutcome.diagnostics();
  const routeOutcomeProvenance = routeOutcome.provenance();
  const prerequisiteDecisions = Object.freeze([
    ...originalPrerequisiteDecisions,
    ...routeOutcomeDiagnostics.prerequisiteDecisions,
  ]);
  const recoverySummary = Object.freeze({
    recovery: recoveryArtifact.recovery,
    href: recoveryArtifact.href,
    reason: recoveryArtifact.reason,
    detail: recoveryArtifact.detail,
    fromArtifactKind: terminalArtifact.kind,
    fromRouteId: originalCandidate.routeId,
    fromHref: originalCandidate.href,
  });
  const recoveryTrailEntry = createRecoveryTrailEntry(
    recoveryArtifact,
    terminalArtifact,
    originalCandidate,
    routeOutcome,
  );
  const provenance = createRouteOutcomeProvenance({
    attemptedRouteId: originalCandidate.routeId,
    attemptedHref: originalCandidate.href,
    resolvedRouteId: routeOutcome.routeId,
    resolvedHref: routeOutcome.href,
    terminalSource: "recoveredOutcome",
    terminalArtifact: routeOutcome.kind === "admitted"
      ? null
      : routeOutcome.artifact(),
    prerequisiteDecisions: [
      ...originalPrerequisiteDecisions.map((decision) => (
        createAdmissionDecisionProvenance(originalCandidate, decision, decision.consumedSources)
      )),
      ...routeOutcomeProvenance.prerequisiteDecisions,
    ],
    recoveryTrail: [
      recoveryTrailEntry,
      ...routeOutcomeProvenance.recoveryTrail,
    ],
  });
  const verification = Object.freeze({
    routeId: routeOutcome.routeId,
    admissionPlanDigest: createCanonicalDigest("route-admission-plan", {
      attemptedRouteId: originalCandidate.routeId,
      attemptedHref: originalCandidate.href,
      recoveredRouteId: routeOutcome.routeId,
      recoveredHref: routeOutcome.href,
      recoveredAdmissionPlanDigest: routeOutcomeVerification.admissionPlanDigest,
      provenance,
    }),
    formsAuthorityDigest: routeOutcomeVerification.formsAuthorityDigest,
    routeOutcomeDigest: createCanonicalDigest("route-outcome", {
      kind: routeOutcome.kind,
      routeId: routeOutcome.routeId,
      href: routeOutcome.href,
      provenance,
    }),
  });
  return Object.freeze({
    ...routeOutcome,
    diagnostics() {
      return Object.freeze({
        ...routeOutcomeDiagnostics,
        prerequisiteDecisions,
        recovery: recoverySummary,
      });
    },
    recovery() {
      return recoveryArtifact;
    },
    provenance() {
      return provenance;
    },
    verification() {
      return verification;
    },
  });
}

export {
  attachRouteRecoveryArtifact,
  createAdmittedRouteOutcome,
  createNotFoundRouteOutcome,
  createRejectedRouteOutcome,
};
