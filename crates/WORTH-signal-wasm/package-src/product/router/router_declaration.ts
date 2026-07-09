import { parseRoutePattern } from "../route/route_pattern.js";
import { normalizeRouteAdmissionDeclarations } from "./projection/admission/router_admission_declaration.js";
import { normalizeRouteFormsAuthorityDeclaration } from "./projection/admission/router_forms_authority_declaration.js";
import { normalizeRouteRecoveryDeclarations } from "./projection/recovery/router_recovery_declaration.js";
import {
  normalizeRouteControllers,
  normalizeRouteGraphs,
} from "./projection/router_composition_declaration.js";
import { normalizeRouteBreadcrumbDeclaration } from "./projection/breadcrumb/router_breadcrumb_declaration.js";
import { normalizeRouteResources } from "./projection/router_resource_declaration.js";
import { isHashField, isSearchField } from "./router_fields.js";
import { ROUTE_DECLARATION } from "./router_symbols.js";

function createRouteDeclaration(route, options) {
  const pattern = parseRoutePattern(route, "signals.router.route(...)");
  const search = normalizeSearchSchema(route, options.search);
  const hash = normalizeHashSchema(route, options.hash);
  const controllers = normalizeRouteControllers(route, options.controllers);
  const graphs = normalizeRouteGraphs(route, options.graphs);
  const resources = normalizeRouteResources(route, options.resources);
  const breadcrumb = normalizeRouteBreadcrumbDeclaration(route, options.breadcrumb);
  const admission = normalizeRouteAdmissionDeclarations(route, options.admission);
  const recovery = normalizeRouteRecoveryDeclarations(route, options.recovery);
  const forms = normalizeRouteFormsAuthorityDeclaration(route, options.forms);
  return Object.freeze({
    [ROUTE_DECLARATION]: true,
    route,
    pattern,
    search,
    hash,
    controllers,
    graphs,
    resources,
    breadcrumb,
    admission,
    recovery,
    forms,
  });
}

function isRouteDeclaration(value) {
  return Boolean(value && value[ROUTE_DECLARATION] === true);
}

function normalizeSearchSchema(route, search) {
  if (search === undefined) {
    return Object.freeze({});
  }
  if (!isPlainObject(search)) {
    throw new TypeError(
      `signals.router.route("${route}") search must be an object of declared search fields`,
    );
  }
  const normalized = {};
  for (const [key, value] of Object.entries(search)) {
    if (!isSearchField(value)) {
      throw new TypeError(
        `signals.router.route("${route}") search["${key}"] must be declared with signals.router.search`,
      );
    }
    normalized[key] = value;
  }
  return Object.freeze(normalized);
}

function normalizeHashSchema(route, hash) {
  if (hash === undefined) {
    return null;
  }
  if (!isHashField(hash)) {
    throw new TypeError(
      `signals.router.route("${route}") hash must be declared with signals.router.hash`,
    );
  }
  return hash;
}

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

export {
  createRouteDeclaration,
  isRouteDeclaration,
};
