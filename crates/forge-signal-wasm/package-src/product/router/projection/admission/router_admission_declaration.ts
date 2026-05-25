import {
  ROUTE_PREREQUISITE_DECLARATION,
} from "../../router_symbols.js";
import {
  normalizeRouteAdmissionSourceDeclarations,
} from "./router_admission_source_declaration.js";

function createRoutePrerequisiteDeclaration(name, evaluateOrOptions) {
  if (typeof name !== "string" || name.trim().length === 0) {
    throw new TypeError("signals.router.prerequisite(...) requires a non-empty prerequisite name");
  }
  const options = normalizeRoutePrerequisiteOptions(name, evaluateOrOptions);
  if (typeof options.evaluate !== "function") {
    throw new TypeError(
      `signals.router.prerequisite("${name}") requires an evaluation function`,
    );
  }
  return Object.freeze({
    [ROUTE_PREREQUISITE_DECLARATION]: true,
    name,
    consumes: options.consumes,
    evaluate: options.evaluate,
  });
}

function isRoutePrerequisiteDeclaration(value) {
  return Boolean(value && value[ROUTE_PREREQUISITE_DECLARATION] === true);
}

function normalizeRouteAdmissionDeclarations(route, admission) {
  if (admission === undefined) {
    return Object.freeze([]);
  }
  if (!Array.isArray(admission)) {
    throw new TypeError(
      `signals.router.route("${route}") admission must be an array of signals.router.prerequisite(...) declarations`,
    );
  }
  for (const declaration of admission) {
    if (!isRoutePrerequisiteDeclaration(declaration)) {
      throw new TypeError(
        `signals.router.route("${route}") admission entries must be created with signals.router.prerequisite(...)`,
      );
    }
  }
  return Object.freeze(admission.slice());
}

function normalizeRoutePrerequisiteOptions(name, evaluateOrOptions) {
  if (typeof evaluateOrOptions === "function") {
    return Object.freeze({
      consumes: Object.freeze([]),
      evaluate: evaluateOrOptions,
    });
  }
  if (!evaluateOrOptions || typeof evaluateOrOptions !== "object" || Array.isArray(evaluateOrOptions)) {
    throw new TypeError(
      `signals.router.prerequisite("${name}") requires an evaluation function or { consumes, evaluate } options`,
    );
  }
  return Object.freeze({
    consumes: normalizeRouteAdmissionSourceDeclarations(name, evaluateOrOptions.consumes),
    evaluate: evaluateOrOptions.evaluate,
  });
}

export {
  createRoutePrerequisiteDeclaration,
  isRoutePrerequisiteDeclaration,
  normalizeRouteAdmissionDeclarations,
};
