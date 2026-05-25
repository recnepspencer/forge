import {
  ROUTE_ADMISSION_PLAN,
} from "../../router_symbols.js";
import {
  createCanonicalDigest,
} from "../../url_authority/router_verification_packages.js";
import {
  createNotFoundRouteOutcome,
  createAdmittedRouteOutcome,
  createRejectedRouteOutcome,
  attachRouteRecoveryArtifact,
} from "./router_admission_outcome.js";
import {
  createRouteAdmissionPlanProvenance,
} from "./router_admission_provenance.js";
import {
  createRoutePrerequisiteEvaluationContext,
  flattenRoutePrerequisiteDeclarations,
  normalizeRouteAdmissionFacts,
  normalizeRoutePrerequisiteArtifact,
  normalizeRoutePrerequisiteSourceValues,
  summarizeConsumedSources,
} from "./router_admission_prerequisite.js";
import {
  flattenRouteRecoveryDeclarations,
  resolveNearestValidRouteRecovery,
} from "../recovery/router_recovery_resolution.js";

function createProjectedRouteAdmissionPlan(
  projectedCandidate,
  matchedDeclarations,
  projectRouteCandidate,
  facts = {},
) {
  const normalizedFacts = normalizeRouteAdmissionFacts(facts);
  const routeDeclaration = matchedDeclarations.at(-1);
  if (!routeDeclaration) {
    throw new TypeError("route admission plan requires a matched leaf route declaration");
  }
  const prerequisiteDeclarations = flattenRoutePrerequisiteDeclarations(matchedDeclarations);
  const recoveryDeclarations = flattenRouteRecoveryDeclarations(matchedDeclarations);
  const prerequisiteNames = Object.freeze(prerequisiteDeclarations.map((entry) => entry.name));
  const recoveryNames = Object.freeze(recoveryDeclarations.map((entry) => entry.name));
  const consumedSources = summarizeConsumedSourcesForPlan(prerequisiteDeclarations);
  const provenance = createRouteAdmissionPlanProvenance(
    projectedCandidate,
    prerequisiteNames,
    recoveryNames,
    consumedSources,
    normalizedFacts,
  );
  const verification = Object.freeze({
    routeId: projectedCandidate.routeId,
    admissionPlanDigest: createCanonicalDigest("route-admission-plan", {
      routeId: projectedCandidate.routeId,
      canonicalUrlDigest: projectedCandidate.canonicalUrl().canonicalUrlDigest,
      provenance,
    }),
  });
  return Object.freeze({
    [ROUTE_ADMISSION_PLAN]: true,
    kind: "routeAdmissionPlan",
    routeId: projectedCandidate.routeId,
    href: projectedCandidate.href,
    candidate() {
      return projectedCandidate;
    },
    prerequisiteNames() {
      return prerequisiteNames;
    },
    recoveryNames() {
      return recoveryNames;
    },
    provenance() {
      return provenance;
    },
    verification() {
      return verification;
    },
    async resolve(recoveryTrace = Object.freeze([projectedCandidate.canonicalUrl().canonicalUrlDigest])) {
      return resolveRouteAdmissionOutcome(
        projectedCandidate,
        routeDeclaration,
        prerequisiteDeclarations,
        recoveryDeclarations,
        normalizedFacts,
        verification,
        projectRouteCandidate,
        recoveryTrace,
      );
    },
  });
}

async function resolveRouteAdmissionOutcome(
  projectedCandidate,
  routeDeclaration,
  prerequisiteDeclarations,
  recoveryDeclarations,
  normalizedFacts,
  planVerification,
  projectRouteCandidate,
  recoveryTrace,
) {
  const prerequisiteDecisions = [];
  for (const prerequisiteDeclaration of prerequisiteDeclarations) {
    const consumedSourceValues = normalizeRoutePrerequisiteSourceValues(
      prerequisiteDeclaration,
      normalizedFacts,
    );
    const prerequisiteArtifact = Object.freeze({
      ...normalizeRoutePrerequisiteArtifact(
        prerequisiteDeclaration.name,
        await prerequisiteDeclaration.evaluate(
          createRoutePrerequisiteEvaluationContext(
            projectedCandidate,
            normalizedFacts,
            consumedSourceValues,
          ),
        ),
      ),
      consumedSources: summarizeConsumedSources(prerequisiteDeclaration),
    });
    prerequisiteDecisions.push(prerequisiteArtifact);
    if (prerequisiteArtifact.kind === "allow") {
      continue;
    }
    const recoveredCandidateDecision = await resolveNearestValidRouteRecovery(
      projectedCandidate,
      prerequisiteArtifact,
      recoveryDeclarations,
      normalizedFacts,
      projectRouteCandidate,
    );
    if (recoveredCandidateDecision === null) {
      return createRejectedRouteOutcome(
        projectedCandidate,
        prerequisiteArtifact,
        prerequisiteDecisions,
        planVerification,
      );
    }
    const recoveredCanonicalDigest =
      recoveredCandidateDecision.recoveredCandidate.canonicalUrl().canonicalUrlDigest;
    if (recoveryTrace.includes(recoveredCanonicalDigest)) {
      throw new TypeError(
        `route recovery "${recoveredCandidateDecision.recoveryArtifact.recovery}" returned fallback href "${recoveredCandidateDecision.recoveryArtifact.href}" that repeats a prior recovery target`,
      );
    }
    const recoveredOutcome = await recoveredCandidateDecision.recoveredCandidate.admission(
      normalizedFacts,
    ).resolve(Object.freeze([...recoveryTrace, recoveredCanonicalDigest]));
    return attachRouteRecoveryArtifact(
      recoveredOutcome,
      recoveredCandidateDecision.recoveryArtifact,
      projectedCandidate,
      prerequisiteArtifact,
      prerequisiteDecisions,
    );
  }
  return createAdmittedRouteOutcome(
    projectedCandidate,
    routeDeclaration,
    prerequisiteDecisions,
    planVerification,
  );
}

function summarizeConsumedSourcesForPlan(prerequisiteDeclarations) {
  const seen = new Map();
  for (const declaration of prerequisiteDeclarations) {
    for (const source of declaration.consumes) {
      const current = seen.get(source.name);
      const next = Object.freeze({
        name: source.name,
        family: source.family,
        valueKind: source.valueKind,
      });
      if (!current) {
        seen.set(source.name, next);
        continue;
      }
      if (current.family !== next.family || current.valueKind !== next.valueKind) {
        throw new TypeError(
          `route admission plan declared conflicting source contracts for "${source.name}"`,
        );
      }
    }
  }
  return Object.freeze([...seen.values()]);
}

export {
  createNotFoundRouteOutcome,
  createProjectedRouteAdmissionPlan,
};
