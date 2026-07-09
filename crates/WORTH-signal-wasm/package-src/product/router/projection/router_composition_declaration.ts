import { CONTROLLER_CONTRACT } from "../../symbols.js";

function normalizeRouteControllers(route, controllers) {
  return normalizeNamedCompositionRecord(
    route,
    "controllers",
    controllers,
    isControllerContract,
    `signals.router.route("${route}") controllers must be controller artifacts created by signals.controller(...) or scope.controller(...)`,
  );
}

function normalizeRouteGraphs(route, graphs) {
  return normalizeNamedCompositionRecord(
    route,
    "graphs",
    graphs,
    isPublishedSignalGraph,
    `signals.router.route("${route}") graphs must be published graph artifacts created by signals.graph(...)`,
  );
}

function normalizeNamedCompositionRecord(route, fieldName, value, predicate, errorMessage) {
  if (value === undefined) {
    return Object.freeze({});
  }
  if (!isPlainObject(value)) {
    throw new TypeError(
      `signals.router.route("${route}") ${fieldName} must be an object when provided`,
    );
  }
  const normalized = {};
  for (const [key, entry] of Object.entries(value)) {
    if (!predicate(entry)) {
      throw new TypeError(errorMessage);
    }
    normalized[key] = entry;
  }
  return Object.freeze(normalized);
}

function isControllerContract(value) {
  return Boolean(value && value[CONTROLLER_CONTRACT] === true);
}

function isPublishedSignalGraph(value) {
  return Boolean(
    value &&
    typeof value.id === "string" &&
    isPlainObject(value.inputs) &&
    isPlainObject(value.outputs) &&
    typeof value.contract === "function" &&
    typeof value.summary === "function" &&
    typeof value.exportDefinition === "function",
  );
}

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

export {
  normalizeRouteControllers,
  normalizeRouteGraphs,
};
