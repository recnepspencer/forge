import {
  ROUTE_ADMITTED_CAPABILITY,
} from "../../router_symbols.js";
import { createRouteFormsAuthorityArtifact } from "./router_forms_authority_artifact.js";
import {
  createRouteBreadcrumbEntry,
  createRouteBreadcrumbTrail,
} from "../breadcrumb/router_breadcrumb_artifact.js";
import { createAdmittedRouteResourceCapability } from "../router_resource_capability.js";

function createAdmittedRouteCapability(projectedRouteCapability, routeDeclaration) {
  const admittedRouteCapability = Object.freeze({
    [ROUTE_ADMITTED_CAPABILITY]: true,
    kind: "admittedRouteCapability",
    routeId: projectedRouteCapability.routeId,
    href: projectedRouteCapability.href,
    params: projectedRouteCapability.params,
    search: projectedRouteCapability.search,
    hash: projectedRouteCapability.hash,
    controllerNames() {
      return projectedRouteCapability.controllerNames();
    },
    controller(name) {
      return createAdmittedControllerCapability(projectedRouteCapability.controller(name));
    },
    graphNames() {
      return projectedRouteCapability.graphNames();
    },
    graph(name) {
      return createAdmittedGraphCapability(projectedRouteCapability.graph(name));
    },
    resourceNames() {
      return projectedRouteCapability.resourceNames();
    },
    resource(name) {
      projectedRouteCapability.resource(name);
      return createAdmittedRouteResourceCapability(
        projectedRouteCapability,
        name,
        routeDeclaration.resources[name],
      );
    },
    breadcrumb() {
      return createRouteBreadcrumbEntry(admittedRouteCapability, routeDeclaration.breadcrumb);
    },
    breadcrumbTrail(options) {
      return createRouteBreadcrumbTrail(
        admittedRouteCapability,
        routeDeclaration.breadcrumb,
        options,
      );
    },
    descriptor() {
      return projectedRouteCapability.descriptor();
    },
    canonical() {
      return projectedRouteCapability.canonical();
    },
    verification() {
      return projectedRouteCapability.verification();
    },
  });
  const formsAuthority = createRouteFormsAuthorityArtifact(routeDeclaration, admittedRouteCapability);
  return Object.freeze({
    ...admittedRouteCapability,
    formsAuthority() {
      return formsAuthority;
    },
  });
}

function createAdmittedControllerCapability(projectedControllerCapability) {
  return Object.freeze({
    kind: "admittedControllerCapability",
    routeId: projectedControllerCapability.routeId,
    name: projectedControllerCapability.name,
    inputNames() {
      return projectedControllerCapability.inputNames();
    },
    outputNames() {
      return projectedControllerCapability.outputNames();
    },
    internalNames() {
      return projectedControllerCapability.internalNames();
    },
  });
}

function createAdmittedGraphCapability(projectedGraphCapability) {
  return Object.freeze({
    kind: "admittedGraphCapability",
    routeId: projectedGraphCapability.routeId,
    name: projectedGraphCapability.name,
    graphId: projectedGraphCapability.graphId,
    summary() {
      return projectedGraphCapability.summary();
    },
    contract() {
      return projectedGraphCapability.contract();
    },
    inputNames() {
      return projectedGraphCapability.inputNames();
    },
    outputNames() {
      return projectedGraphCapability.outputNames();
    },
  });
}

export {
  createAdmittedRouteCapability,
};
