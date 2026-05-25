import {
  ROUTE_ADMISSION_SOURCE_DECLARATION,
} from "../../router_symbols.js";

const ROUTE_ADMISSION_SOURCE_FAMILIES = Object.freeze([
  "hostCapability",
  "resourceTruth",
  "graphTruth",
]);

const ROUTE_ADMISSION_SOURCE_VALUE_KINDS = Object.freeze([
  "string",
  "number",
  "boolean",
]);

function createRouteAdmissionSourceNamespace(family) {
  validateRouteAdmissionSourceFamily(family);
  return Object.freeze({
    string(name) {
      return createRouteAdmissionSourceDeclaration(family, "string", name);
    },
    number(name) {
      return createRouteAdmissionSourceDeclaration(family, "number", name);
    },
    boolean(name) {
      return createRouteAdmissionSourceDeclaration(family, "boolean", name);
    },
  });
}

function createRouteAdmissionSourceDeclaration(family, valueKind, name) {
  validateRouteAdmissionSourceFamily(family);
  validateRouteAdmissionSourceValueKind(valueKind);
  if (typeof name !== "string" || name.trim().length === 0) {
    throw new TypeError("route admission sources require a non-empty source name");
  }
  return Object.freeze({
    [ROUTE_ADMISSION_SOURCE_DECLARATION]: true,
    family,
    valueKind,
    name,
  });
}

function isRouteAdmissionSourceDeclaration(value) {
  return Boolean(value && value[ROUTE_ADMISSION_SOURCE_DECLARATION] === true);
}

function normalizeRouteAdmissionSourceDeclarations(prerequisiteName, consumes) {
  if (consumes === undefined) {
    return Object.freeze([]);
  }
  if (!Array.isArray(consumes)) {
    throw new TypeError(
      `signals.router.prerequisite("${prerequisiteName}") consumes must be an array of declared admission sources`,
    );
  }
  const normalized = consumes.map((source) => {
    if (!isRouteAdmissionSourceDeclaration(source)) {
      throw new TypeError(
        `signals.router.prerequisite("${prerequisiteName}") consumes entries must be declared with signals.router.host/resource/graph`,
      );
    }
    return source;
  });
  return Object.freeze(normalized);
}

function validateRouteAdmissionSourceFamily(family) {
  if (!ROUTE_ADMISSION_SOURCE_FAMILIES.includes(family)) {
    throw new TypeError(`unsupported route admission source family "${String(family)}"`);
  }
}

function validateRouteAdmissionSourceValueKind(valueKind) {
  if (!ROUTE_ADMISSION_SOURCE_VALUE_KINDS.includes(valueKind)) {
    throw new TypeError(`unsupported route admission source value kind "${String(valueKind)}"`);
  }
}

export {
  createRouteAdmissionSourceNamespace,
  isRouteAdmissionSourceDeclaration,
  normalizeRouteAdmissionSourceDeclarations,
};
