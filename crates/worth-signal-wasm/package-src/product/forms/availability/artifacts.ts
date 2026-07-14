import { FormDeclarationError } from "../form_errors.js";

const AVAILABILITY_STATES = new Set([
  "enabled",
  "disabled",
  "hidden",
  "readonly",
  "required",
  "omitted",
  "blocked",
  "unavailable",
]);
const DRAFT_POLICIES = new Set(["preserve", "clear", "freeze", "omit"]);
const BLOCKING_STATES = new Set(["blocked", "unavailable"]);

export function normalizeAvailabilityArtifact(artifact, declaration) {
  if (artifact == null || artifact === true) {
    return availabilityArtifact(declaration, { state: "enabled" });
  }
  if (typeof artifact === "string") {
    return availabilityArtifact(declaration, { state: artifact });
  }
  if (!artifact || typeof artifact !== "object") {
    throw new FormDeclarationError("availability resolver returned an undeclared artifact shape", {
      artifact,
    });
  }
  return availabilityArtifact(declaration, artifact);
}

export function availabilityReadinessBlockers(availability) {
  return availability.artifacts
    .filter((artifact) => BLOCKING_STATES.has(artifact.state))
    .map((artifact) => Object.freeze({
      kind: `availability:${artifact.state}`,
      field: artifact.scope === "field" ? artifact.ownerId : undefined,
      action: artifact.scope === "action" ? artifact.ownerId : undefined,
      control: artifact.scope === "control" ? artifact.ownerId : undefined,
      group: artifact.scope === "group" ? artifact.ownerId : undefined,
      section: artifact.scope === "section" ? artifact.ownerId : undefined,
      fields: artifact.fields,
      reason: artifact.reason ?? `${artifact.ownerId} is ${artifact.state}`,
    }));
}

export function availabilityEditBlocker(availability, fieldId) {
  const artifact = fieldAvailabilityArtifact(availability, fieldId);
  if (!artifact) {
    return null;
  }
  if (
    artifact.state === "disabled" ||
    artifact.state === "hidden" ||
    artifact.state === "readonly" ||
    artifact.state === "omitted" ||
    artifact.state === "blocked" ||
    artifact.state === "unavailable" ||
    artifact.draftPolicy === "freeze" ||
    artifact.draftPolicy === "clear" ||
    artifact.draftPolicy === "omit"
  ) {
    return Object.freeze({
      kind: `availability:${artifact.state}`,
      field: fieldId,
      reason: artifact.reason ?? `${fieldId} is ${artifact.state}`,
      availability: artifact,
    });
  }
  return null;
}

export function fieldAvailabilityArtifact(availability, fieldId) {
  return availability.artifacts.find((artifact) => (
    artifact.scope === "field" && artifact.ownerId === fieldId
  )) ?? null;
}

export function omittedFieldIds(availability) {
  return new Set(
    availability.artifacts
      .filter((artifact) => (
        artifact.scope === "field" &&
        (artifact.state === "omitted" || artifact.draftPolicy === "omit")
      ))
      .map((artifact) => artifact.ownerId),
  );
}

export function clearedFieldIds(availability) {
  return new Set(
    availability.artifacts
      .filter((artifact) => (
        artifact.scope === "field" &&
        artifact.draftPolicy === "clear"
      ))
      .map((artifact) => artifact.ownerId),
  );
}

function availabilityArtifact(declaration, artifact) {
  const state = artifact.state ?? "enabled";
  if (!AVAILABILITY_STATES.has(state)) {
    throw new FormDeclarationError("availability artifact state is not supported", {
      state,
    });
  }
  const draftPolicy = artifact.draftPolicy ?? "preserve";
  if (!DRAFT_POLICIES.has(draftPolicy)) {
    throw new FormDeclarationError("availability draft policy is not supported", {
      draftPolicy,
    });
  }
  if (declaration.scope !== "field" && draftPolicy !== "preserve") {
    throw new FormDeclarationError("availability draft policy only applies to fields", {
      scope: declaration.scope,
      ownerId: declaration.ownerId,
      draftPolicy,
    });
  }
  return Object.freeze({
    kind: "availability",
    id: declaration.id,
    scope: declaration.scope,
    ownerId: declaration.ownerId,
    fields: declaration.fields ?? Object.freeze([]),
    state,
    draftPolicy,
    dependencies: declaration.dependencies,
    ...(artifact.reason === undefined ? {} : { reason: String(artifact.reason) }),
  });
}
