import { createFormReadView } from "../read_views.js";
import { stableValueDigest } from "../values/value_paths.js";
import { normalizeAdmissionArtifact } from "./artifacts.js";

export function evaluateAdmission(admissionDeclarations, form, fieldDeclarations) {
  const readView = createFormReadView(form);
  const binding = currentAdmissionBinding(form, fieldDeclarations);
  const artifacts = admissionDeclarations.map((declaration) =>
    normalizeAdmissionArtifact(runAdmissionResolver(declaration, readView, binding), declaration, binding),
  );
  return Object.freeze({
    artifacts: Object.freeze(artifacts),
    summary: summarizeAdmission(artifacts),
    counters: admissionCounters(admissionDeclarations, artifacts),
    dependencyBreadth: Object.freeze(
      admissionDeclarations.map((declaration) => ({
        id: declaration.id,
        scope: declaration.scope,
        ownerId: declaration.ownerId,
        capability: declaration.capability,
        dependencies: declaration.dependencies,
      })),
    ),
  });
}

function admissionCounters(declarations, artifacts) {
  return Object.freeze({
    costBasis: "derivedFullReportScan",
    incrementalStatus: "notIncremental",
    declarations: declarations.length,
    dependencyReads: declarations.reduce((total, declaration) => (
      total + declaration.dependencies.length
    ), 0),
    fieldScopes: declarations.filter((declaration) => declaration.scope === "field").length,
    actionScopes: declarations.filter((declaration) => declaration.scope === "action").length,
    regulatedArtifacts: artifacts.filter((artifact) => artifact.binding !== undefined).length,
    staleRegulatedArtifacts: artifacts.filter((artifact) => artifact.stale?.isStale === true).length,
  });
}

function runAdmissionResolver(declaration, readView, binding) {
  const values = Object.fromEntries(
    declaration.dependencies.map((fieldId) => [
      fieldId,
      readView.field(fieldId).effectiveValue(),
    ]),
  );
  return declaration.resolver(values, {
    form: readView,
    scope: declaration.scope,
    ownerId: declaration.ownerId,
    capability: declaration.capability,
    dependencies: declaration.dependencies,
    binding,
  });
}

function currentAdmissionBinding(form, fieldDeclarations) {
  const sourceDigest = stableValueDigest(form.source());
  const patchDigest = form.patchPlan().equivalenceDigest;
  const schemaDigest = stableValueDigest(
    fieldDeclarations.map((field) => ({
      id: field.id,
      path: field.path,
      inputAdapterTier: field.inputAdapter.tier,
    })),
  );
  return Object.freeze({
    sourceDigest,
    patchDigest,
    schemaDigest,
    bindingDigest: stableValueDigest({
      sourceDigest,
      patchDigest,
      schemaDigest,
    }),
  });
}

function summarizeAdmission(artifacts) {
  const summary = {
    admitted: 0,
    denied: 0,
    blocked: 0,
    unavailable: 0,
    requiresApproval: 0,
    requiresSignature: 0,
    requiresReview: 0,
    requiresReason: 0,
  };
  for (const artifact of artifacts) {
    summary[artifact.posture] += 1;
  }
  return Object.freeze(summary);
}
