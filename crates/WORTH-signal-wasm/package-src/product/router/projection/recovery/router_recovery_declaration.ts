import {
  ROUTE_RECOVERY_DECLARATION,
} from "../../router_symbols.js";

function createRouteRecoveryDeclaration(name, evaluate) {
  if (typeof name !== "string" || name.trim().length === 0) {
    throw new TypeError("signals.router.recovery(...) requires a non-empty recovery name");
  }
  if (typeof evaluate !== "function") {
    throw new TypeError(
      `signals.router.recovery("${name}") requires an evaluation function`,
    );
  }
  return Object.freeze({
    [ROUTE_RECOVERY_DECLARATION]: true,
    name,
    evaluate,
  });
}

function isRouteRecoveryDeclaration(value) {
  return Boolean(value && value[ROUTE_RECOVERY_DECLARATION] === true);
}

function normalizeRouteRecoveryDeclarations(route, recovery) {
  if (recovery === undefined) {
    return Object.freeze([]);
  }
  if (!Array.isArray(recovery)) {
    throw new TypeError(
      `signals.router.route("${route}") recovery must be an array of signals.router.recovery(...) declarations`,
    );
  }
  for (const declaration of recovery) {
    if (!isRouteRecoveryDeclaration(declaration)) {
      throw new TypeError(
        `signals.router.route("${route}") recovery entries must be created with signals.router.recovery(...)`,
      );
    }
  }
  return Object.freeze(recovery.slice());
}

export {
  createRouteRecoveryDeclaration,
  normalizeRouteRecoveryDeclarations,
};
