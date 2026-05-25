import { createRoutePatternProjectionShape } from "../../route/route_pattern.js";
import { isRouteDeclaration } from "../router_declaration.js";
import { createRouteReference } from "../router_location.js";
import { isRouteLayoutDeclaration } from "./router_layout_declaration.js";
import {
  attachProjectionRoot,
  createRouteLayoutReference,
} from "./router_projection_candidate.js";

function defineRoutes(definitions, scopeId) {
  if (!isPlainObject(definitions)) {
    throw new TypeError("signals.router.define(...) requires an object of route declarations");
  }
  const { resolvedTree, childNodes, projectedLeaves } = resolveRouteTree(definitions, [], scopeId);
  assertNoAmbiguousProjectedRoutes(projectedLeaves);
  return attachProjectionRoot(resolvedTree, childNodes);
}

function resolveRouteTree(definitions, path, scopeId) {
  const resolvedTree = {};
  const childNodes = [];
  const projectedLeaves = [];
  for (const [key, value] of Object.entries(definitions)) {
    const nextPath = [...path, key];
    if (isRouteDeclaration(value)) {
      const routeReference = createRouteReference(value, nextPath, scopeId);
      resolvedTree[key] = routeReference;
      childNodes.push({ kind: "route", key, declaration: value, reference: routeReference });
      projectedLeaves.push(createProjectedLeafRecord(nextPath, scopeId, value));
      continue;
    }
    if (isRouteLayoutDeclaration(value)) {
      const layoutReference = createRouteReference(value.route, nextPath, scopeId);
      const resolvedChildren = resolveRouteTree(value.children, nextPath, scopeId);
      const projectedLayout = createRouteLayoutReference(
        layoutReference,
        value.outletId,
        resolvedChildren.resolvedTree,
      );
      resolvedTree[key] = projectedLayout;
      childNodes.push({
        kind: "layout",
        key,
        outletId: value.outletId,
        declaration: value.route,
        reference: projectedLayout,
        children: resolvedChildren.childNodes,
      });
      projectedLeaves.push(...resolvedChildren.projectedLeaves);
      continue;
    }
    if (isPlainObject(value)) {
      const resolvedChildren = resolveRouteTree(value, nextPath, scopeId);
      resolvedTree[key] = Object.freeze(resolvedChildren.resolvedTree);
      childNodes.push({
        kind: "namespace",
        key,
        children: resolvedChildren.childNodes,
      });
      projectedLeaves.push(...resolvedChildren.projectedLeaves);
      continue;
    }
    throw new TypeError(
      `signals.router.define(...) expected route(...), layout(...), or nested objects at "${nextPath.join(".")}"`,
    );
  }
  return {
    resolvedTree,
    childNodes: Object.freeze(childNodes),
    projectedLeaves: Object.freeze(projectedLeaves),
  };
}

function createProjectedLeafRecord(path, scopeId, declaration) {
  return Object.freeze({
    routeId: scopeId ? `${scopeId}:${path.join(".")}` : path.join("."),
    route: declaration.route,
    projectionShape: createRoutePatternProjectionShape(declaration.pattern),
    search: declaration.search,
  });
}

function assertNoAmbiguousProjectedRoutes(projectedLeaves) {
  for (let leftIndex = 0; leftIndex < projectedLeaves.length; leftIndex += 1) {
    for (let rightIndex = leftIndex + 1; rightIndex < projectedLeaves.length; rightIndex += 1) {
      const left = projectedLeaves[leftIndex];
      const right = projectedLeaves[rightIndex];
      if (left.projectionShape !== right.projectionShape) {
        continue;
      }
      if (!routeSearchSchemasOverlap(left.search, right.search)) {
        continue;
      }
      throw new TypeError(
        `signals.router.define(...) projects ambiguous route truth between "${left.routeId}" (${left.route}) and "${right.routeId}" (${right.route}); equivalent path shapes overlap and declared search/hash contracts do not disambiguate them`,
      );
    }
  }
}

function routeSearchSchemasOverlap(left, right) {
  const keys = new Set([...Object.keys(left), ...Object.keys(right)]);
  for (const key of keys) {
    const leftField = left[key];
    const rightField = right[key];
    if (leftField === undefined) {
      if (rightField.required) {
        return false;
      }
      continue;
    }
    if (rightField === undefined) {
      if (leftField.required) {
        return false;
      }
      continue;
    }
    if (!routeSearchFieldsOverlap(leftField, rightField)) {
      return false;
    }
  }
  return true;
}

function routeSearchFieldsOverlap(leftField, rightField) {
  if (leftField.valueKind === rightField.valueKind) {
    return true;
  }
  if (leftField.valueKind === "string" || rightField.valueKind === "string") {
    return true;
  }
  return leftField.valueKind === "boolean" && rightField.valueKind === "boolean";
}

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

export { defineRoutes };
