import { createFormReadView } from "../read_views.js";
import { normalizeAvailabilityArtifact } from "./artifacts.js";

export function evaluateAvailability(availabilityDeclarations, form) {
  const readView = createFormReadView(form);
  const artifacts = availabilityDeclarations.map((declaration) =>
    normalizeAvailabilityArtifact(runAvailabilityResolver(declaration, readView), declaration),
  );
  return Object.freeze({
    artifacts: Object.freeze(artifacts),
    summary: summarizeAvailability(artifacts),
    counters: availabilityCounters(availabilityDeclarations, artifacts),
    dependencyBreadth: Object.freeze(
      availabilityDeclarations.map((declaration) => ({
        id: declaration.id,
        scope: declaration.scope,
        ownerId: declaration.ownerId,
        fields: declaration.fields ?? Object.freeze([]),
        dependencies: declaration.dependencies,
      })),
    ),
  });
}

function availabilityCounters(declarations, artifacts) {
  const scopeCounts = scopeSummary(artifacts);
  return Object.freeze({
    costBasis: "derivedFullReportScan",
    incrementalStatus: "notIncremental",
    declarations: declarations.length,
    dependencyReads: declarations.reduce((total, declaration) => (
      total + declaration.dependencies.length
    ), 0),
    fieldRegionMemberships: declarations.reduce((total, declaration) => (
      total + (declaration.fields?.length ?? 0)
    ), 0),
    blockingArtifacts: artifacts.filter((artifact) => (
      artifact.state === "blocked" || artifact.state === "unavailable"
    )).length,
    scopeCounts,
  });
}

function runAvailabilityResolver(declaration, readView) {
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
    dependencies: declaration.dependencies,
  });
}

function summarizeAvailability(artifacts) {
  const summary = {
    enabled: 0,
    disabled: 0,
    hidden: 0,
    readonly: 0,
    required: 0,
    omitted: 0,
    blocked: 0,
    unavailable: 0,
  };
  for (const artifact of artifacts) {
    summary[artifact.state] += 1;
  }
  return Object.freeze({
    ...summary,
    byScope: scopeSummary(artifacts),
  });
}

function scopeSummary(artifacts) {
  const summary = {
    field: 0,
    action: 0,
    control: 0,
    group: 0,
    section: 0,
  };
  for (const artifact of artifacts) {
    summary[artifact.scope] += 1;
  }
  return Object.freeze(summary);
}
