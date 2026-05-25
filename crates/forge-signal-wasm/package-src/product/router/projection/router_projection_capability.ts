import {
  ROUTE_LAYOUT_PLACEMENT,
  ROUTE_PROJECTED_CAPABILITY,
} from "../router_symbols.js";
import {
  createRouteBreadcrumbEntry,
  createRouteBreadcrumbTrail,
} from "./breadcrumb/router_breadcrumb_artifact.js";
import { createProjectedRouteResourceCapability } from "./router_resource_capability.js";

function createProjectedLayoutPlacement(layoutDeclaration, layoutLocation, outletId) {
  const projectedCapability = createProjectedRouteCapability(layoutDeclaration, layoutLocation);
  let projectedOutletContract = null;
  return {
    placement: Object.freeze({
      [ROUTE_LAYOUT_PLACEMENT]: true,
      kind: "projectedLayoutPlacement",
      outletId,
      routeId: projectedCapability.routeId,
      capability() {
        return projectedCapability;
      },
      outlet() {
        return projectedOutletContract;
      },
      descriptor() {
        return projectedCapability.descriptor();
      },
      verification() {
        return projectedCapability.verification();
      },
    }),
    bindOutlet(contract) {
      projectedOutletContract = contract;
    },
  };
}

function createProjectedRouteCapability(routeDeclaration, routeLocation) {
  const canonicalArtifact = routeLocation.canonical();
  const controllers = routeDeclaration.controllers;
  const graphs = routeDeclaration.graphs;
  const resources = routeDeclaration.resources;
  return Object.freeze({
    [ROUTE_PROJECTED_CAPABILITY]: true,
    kind: "projectedRouteCapability",
    routeId: routeLocation.routeId,
    href: routeLocation.href,
    params: routeLocation.params,
    search: routeLocation.search,
    hash: routeLocation.hash,
    controllerNames() {
      return Object.freeze(Object.keys(controllers));
    },
    controller(name) {
      return createProjectedControllerCapability(
        routeLocation.routeId,
        name,
        requireNamedProjectedCompositionEntry(routeLocation.routeId, "controller", controllers, name),
      );
    },
    graphNames() {
      return Object.freeze(Object.keys(graphs));
    },
    graph(name) {
      return createProjectedGraphCapability(
        routeLocation.routeId,
        name,
        requireNamedProjectedCompositionEntry(routeLocation.routeId, "graph", graphs, name),
      );
    },
    resourceNames() {
      return Object.freeze(Object.keys(resources));
    },
    resource(name) {
      return createProjectedRouteResourceCapability(
        routeLocation,
        name,
        requireNamedProjectedCompositionEntry(routeLocation.routeId, "resource", resources, name),
      );
    },
    breadcrumb() {
      return createRouteBreadcrumbEntry(this, routeDeclaration.breadcrumb);
    },
    breadcrumbTrail(options) {
      return createRouteBreadcrumbTrail(this, routeDeclaration.breadcrumb, options);
    },
    descriptor() {
      return routeLocation.descriptor();
    },
    canonical() {
      return canonicalArtifact;
    },
    verification() {
      return canonicalArtifact.verification();
    },
  });
}

function createProjectedControllerCapability(routeId, name, controllerContract) {
  return Object.freeze({
    kind: "projectedControllerCapability",
    routeId,
    name,
    inputNames() {
      return Object.freeze(Object.keys(controllerContract.inputs));
    },
    outputNames() {
      return Object.freeze(Object.keys(controllerContract.outputs));
    },
    internalNames() {
      return Object.freeze(Object.keys(controllerContract.internal));
    },
  });
}

function createProjectedGraphCapability(routeId, name, graph) {
  const summary = graph.summary();
  const contract = graph.contract();
  return Object.freeze({
    kind: "projectedGraphCapability",
    routeId,
    name,
    graphId: graph.id,
    summary() {
      return summary;
    },
    contract() {
      return contract;
    },
    inputNames() {
      return summary.inputNames;
    },
    outputNames() {
      return summary.outputNames;
    },
  });
}

function requireNamedProjectedCompositionEntry(routeId, family, entries, name) {
  if (!(name in entries)) {
    throw new TypeError(
      `projected route capability "${routeId}" does not expose ${family} "${String(name)}"`,
    );
  }
  return entries[name];
}

export {
  createProjectedLayoutPlacement,
  createProjectedRouteCapability,
};
