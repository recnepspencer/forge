function createRoutePrerequisiteEvaluationContext(projectedCandidate, normalizedFacts, consumedSourceValues) {
  const facts = consumedSourceValues.size === 0
    ? normalizedFacts
    : freezeConsumedSourceFacts(consumedSourceValues);
  return Object.freeze({
    routeId: projectedCandidate.routeId,
    href: projectedCandidate.href,
    params: projectedCandidate.route().params,
    search: projectedCandidate.route().search,
    hash: projectedCandidate.route().hash,
    facts,
    consume(source) {
      const consumedSourceValue = consumedSourceValues.get(source);
      if (consumedSourceValue === undefined) {
        throw new TypeError(
          `route prerequisite attempted to consume undeclared source "${source?.name ?? "unknown"}"`,
        );
      }
      return consumedSourceValue;
    },
    consumedSources() {
      return Object.freeze([...consumedSourceValues.keys()]);
    },
    allow(options = {}) {
      return {
        kind: "allow",
        reason: options.reason ?? "admitted",
        detail: options.detail ?? null,
      };
    },
    redirect(options) {
      return {
        kind: "redirect",
        href: requireArtifactHref("redirect", options?.href),
        reason: options.reason ?? "redirect",
        detail: options.detail ?? null,
      };
    },
    notFound(options = {}) {
      return {
        kind: "notFound",
        reason: options.reason ?? "notFound",
        detail: options.detail ?? null,
      };
    },
    forbidden(options = {}) {
      return {
        kind: "forbidden",
        reason: options.reason ?? "forbidden",
        detail: options.detail ?? null,
      };
    },
    unavailable(options = {}) {
      return {
        kind: "unavailable",
        reason: options.reason ?? "unavailable",
        detail: options.detail ?? null,
      };
    },
    denied(options = {}) {
      return {
        kind: "denied",
        reason: options.reason ?? "denied",
        detail: options.detail ?? null,
      };
    },
  });
}

function flattenRoutePrerequisiteDeclarations(matchedDeclarations) {
  return Object.freeze(matchedDeclarations.flatMap((declaration) => declaration.admission));
}

function normalizeRoutePrerequisiteSourceValues(prerequisiteDeclaration, normalizedFacts) {
  const consumedSourceValues = new Map();
  for (const source of prerequisiteDeclaration.consumes) {
    if (!(source.name in normalizedFacts)) {
      throw new TypeError(
        `route prerequisite "${prerequisiteDeclaration.name}" requires declared source "${source.name}" in route admission facts`,
      );
    }
    consumedSourceValues.set(source, normalizeRouteAdmissionSourceValue(prerequisiteDeclaration.name, source, normalizedFacts[source.name]));
  }
  return consumedSourceValues;
}

function summarizeConsumedSources(prerequisiteDeclaration) {
  return Object.freeze(prerequisiteDeclaration.consumes.map((source) => Object.freeze({
    name: source.name,
    family: source.family,
    valueKind: source.valueKind,
  })));
}

function normalizeRouteAdmissionFacts(facts) {
  if (facts === undefined) {
    return Object.freeze({});
  }
  if (facts === null || typeof facts !== "object" || Array.isArray(facts)) {
    throw new TypeError("route admission facts must be an object when provided");
  }
  return Object.freeze({ ...facts });
}

function normalizeRoutePrerequisiteArtifact(prerequisiteName, value) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(
      `route prerequisite "${prerequisiteName}" must return an admission artifact created from the evaluation context`,
    );
  }
  const kind = value.kind;
  if (!isRouteAdmissionArtifactKind(kind)) {
    throw new TypeError(
      `route prerequisite "${prerequisiteName}" returned unsupported admission artifact kind "${String(kind)}"`,
    );
  }
  return Object.freeze({
    kind,
    prerequisite: prerequisiteName,
    href: kind === "redirect" ? requireArtifactHref(prerequisiteName, value.href) : null,
    reason: normalizeArtifactText(prerequisiteName, "reason", value.reason),
    detail: value.detail === undefined || value.detail === null
      ? null
      : normalizeArtifactText(prerequisiteName, "detail", value.detail),
  });
}

function isRouteAdmissionArtifactKind(kind) {
  return [
    "allow",
    "redirect",
    "notFound",
    "forbidden",
    "unavailable",
    "denied",
  ].includes(kind);
}

function requireArtifactHref(sourceLabel, href) {
  if (typeof href !== "string" || href.length === 0 || !href.startsWith("/")) {
    throw new TypeError(
      `route admission ${sourceLabel} artifacts require a local href starting with /`,
    );
  }
  return href;
}

function normalizeArtifactText(prerequisiteName, field, value) {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new TypeError(
      `route prerequisite "${prerequisiteName}" ${field} must be a non-empty string`,
    );
  }
  return value;
}

function normalizeRouteAdmissionSourceValue(prerequisiteName, source, value) {
  if (source.valueKind === "string") {
    if (typeof value !== "string") {
      throw new TypeError(
        `route prerequisite "${prerequisiteName}" source "${source.name}" must be a string`,
      );
    }
    return value;
  }
  if (source.valueKind === "number") {
    if (typeof value !== "number" || !Number.isFinite(value)) {
      throw new TypeError(
        `route prerequisite "${prerequisiteName}" source "${source.name}" must be a finite number`,
      );
    }
    return value;
  }
  if (typeof value !== "boolean") {
    throw new TypeError(
      `route prerequisite "${prerequisiteName}" source "${source.name}" must be a boolean`,
    );
  }
  return value;
}

function freezeConsumedSourceFacts(consumedSourceValues) {
  const facts = {};
  for (const [source, value] of consumedSourceValues.entries()) {
    facts[source.name] = value;
  }
  return Object.freeze(facts);
}

export {
  createRoutePrerequisiteEvaluationContext,
  flattenRoutePrerequisiteDeclarations,
  normalizeRouteAdmissionFacts,
  normalizeRoutePrerequisiteArtifact,
  normalizeRoutePrerequisiteSourceValues,
  summarizeConsumedSources,
};
