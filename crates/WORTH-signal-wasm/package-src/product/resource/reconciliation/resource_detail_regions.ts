import {
  createResourceDetailRegionProof,
  requireResourceDetailRegionProof,
} from "../response/detail_region_proof.js";

const RESOURCE_DETAIL_REGIONS = Symbol("WORTHSignal.resourceDetailRegions");

function resourceDetailRegions(definitions) {
  if (!definitions || typeof definitions !== "object" || Array.isArray(definitions)) {
    throw new TypeError("resourceDetailRegions(...) requires a definition object");
  }
  const normalized = {};
  for (const [region, definition] of readRegionDeclarations(
    definitions,
    "resourceDetailRegions(...)",
  )) {
    normalized[region] = normalizeDetailRegionDefinition(
      region,
      definition,
      "resourceDetailRegions(...)",
    );
  }
  return Object.freeze({
    definitions: Object.freeze(normalized),
    [RESOURCE_DETAIL_REGIONS]: "resourceDetailRegions",
  });
}

function requireResourceDetailRegions(value, kind) {
  if (
    !value ||
    typeof value !== "object" ||
    value[RESOURCE_DETAIL_REGIONS] !== "resourceDetailRegions"
  ) {
    const label =
      kind === undefined
        ? "resourceDetailRegions(...)"
        : `${kind} requires detail regions created with resourceDetailRegions(...)`;
    throw new TypeError(label);
  }
  const normalized = {};
  for (const [region, definition] of readRegionDeclarations(
    value.definitions ?? {},
    kind ?? "resourceDetailRegions(...)",
  )) {
    if (!definition || typeof definition.read !== "function" || typeof definition.write !== "function") {
      throw new TypeError(
        `${kind ?? "resourceDetailRegions(...)"} requires valid detail region definitions`,
      );
    }
    normalized[region] = Object.freeze({
      read: definition.read,
      write: definition.write,
      identityBoundary: definition.identityBoundary,
      mergeGranularity: definition.mergeGranularity,
      regionProof: requireResourceDetailRegionProof(definition.regionProof, region),
    });
  }
  return Object.freeze({
    definitions: Object.freeze(normalized),
    [RESOURCE_DETAIL_REGIONS]: "resourceDetailRegions",
  });
}

function readRegionDeclarations(definitions, source) {
  const declarations = [];
  for (const key of Reflect.ownKeys(definitions)) {
    if (typeof key !== "string") {
      continue;
    }
    const descriptor = Object.getOwnPropertyDescriptor(definitions, key);
    if (descriptor === undefined || !descriptor.enumerable) {
      continue;
    }
    if (!Object.prototype.hasOwnProperty.call(descriptor, "value")) {
      throw new TypeError(
        `${source} rejects accessor detail region declaration "${key}"`,
      );
    }
    declarations.push(Object.freeze([key, descriptor.value]));
  }
  return declarations;
}

function normalizeDetailRegionDefinition(region, definition, source) {
  if (typeof region !== "string" || region.length === 0) {
    throw new TypeError(`${source} region names must be non-empty strings`);
  }
  if (!definition || typeof definition !== "object" || Array.isArray(definition)) {
    throw new TypeError(`${source} region "${region}" must be an object`);
  }
  if (typeof definition.read !== "function") {
    throw new TypeError(`${source} region "${region}" requires read(...)`);
  }
  if (typeof definition.write !== "function") {
    throw new TypeError(`${source} region "${region}" requires write(...)`);
  }
  const identityBoundary = requireRegionIdentityBoundary(
    definition.identityBoundary,
    region,
    source,
  );
  const mergeGranularity = requireRegionMergeGranularity(
    definition.mergeGranularity,
    region,
    source,
  );
  const cost = requireRegionCost(definition.cost, region, source);
  return Object.freeze({
    read: definition.read,
    write: definition.write,
    identityBoundary,
    mergeGranularity,
    regionProof: createResourceDetailRegionProof(region, {
      identityBoundary,
      mergeGranularity,
      cost,
    }),
  });
}

function requireRegionIdentityBoundary(identityBoundary, region, source) {
  if (identityBoundary === "inside" || identityBoundary === "outside") {
    return identityBoundary;
  }
  throw new TypeError(
    `${source} region "${region}" requires identityBoundary "inside" or "outside"`,
  );
}

function requireRegionMergeGranularity(mergeGranularity, region, source) {
  if (typeof mergeGranularity !== "string" || mergeGranularity.length === 0) {
    throw new TypeError(
      `${source} region "${region}" requires non-empty mergeGranularity`,
    );
  }
  return mergeGranularity;
}

function requireRegionCost(cost, region, source) {
  if (!cost || typeof cost !== "object" || Array.isArray(cost)) {
    throw new TypeError(`${source} region "${region}" requires cost metadata`);
  }
  return Object.freeze({
    traversalBreadth: requirePositiveBreadth(
      cost.traversalBreadth,
      region,
      "traversalBreadth",
      source,
    ),
    reconstructionBreadth: requirePositiveBreadth(
      cost.reconstructionBreadth,
      region,
      "reconstructionBreadth",
      source,
    ),
  });
}

function requirePositiveBreadth(value, region, label, source) {
  if (!Number.isSafeInteger(value) || value < 1) {
    throw new TypeError(
      `${source} region "${region}" requires positive safe integer ${label}`,
    );
  }
  return value;
}

export { requireResourceDetailRegions, resourceDetailRegions };
