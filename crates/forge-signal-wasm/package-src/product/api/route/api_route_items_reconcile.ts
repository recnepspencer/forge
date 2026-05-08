import { resourceCollectionShape } from "../../resource/reconciliation/resource_collection_shape.js";
import { resourceItemAspects } from "../../resource/reconciliation/resource_item_aspects.js";
import { resourceValueSummaries } from "../../resource/reconciliation/resource_value_summaries.js";
import { requireResourceCollectionResponse } from "../../resource/response/resource_collection_response_contract.js";
import { createResourceCollectionResponseReconcile } from "../../resource/response/resource_collection_response_reconcile.js";

function createApiRouteItemsState() {
  return Object.freeze({
    declared: false,
    source: null,
    itemIdentity: null,
    response: null,
    reconcileMode: "directArray",
    items: null,
    replaceItems: null,
    aspects: Object.freeze({}),
    summaries: Object.freeze({}),
    summaryPatchScope: null,
  });
}

function requireApiRouteItemsState(itemIdentity, route) {
  if (typeof itemIdentity !== "function") {
    throw new TypeError(
      `api.url("${route}").items(...) requires itemIdentity(item)`,
    );
  }
  return Object.freeze({
    declared: true,
    source: "items",
    itemIdentity,
    response: null,
    reconcileMode: "directArray",
    items: null,
    replaceItems: null,
    aspects: Object.freeze({}),
    summaries: Object.freeze({}),
    summaryPatchScope: null,
  });
}

function requireApiRouteItemsReconcileState(
  state,
  route,
  items,
  replaceItems,
) {
  requireDeclaredApiRouteItemsState(state, route, "reconcile");
  if (Object.keys(state.summaries).length > 0) {
    throw new TypeError(
      `api.url("${route}").items(...).reconcile(...) must be declared before summary(...) or pageWindowSummary(...)`,
    );
  }
  if (typeof items !== "function") {
    throw new TypeError(
      `api.url("${route}").items(...).reconcile(...) requires items(value)`,
    );
  }
  if (typeof replaceItems !== "function") {
    throw new TypeError(
      `api.url("${route}").items(...).reconcile(...) requires replaceItems(value, nextItems)`,
    );
  }
  return Object.freeze({
    ...state,
    reconcileMode: "custom",
    items,
    replaceItems,
  });
}

function requireApiRouteResponseItemsState(response, route) {
  const collectionResponse = requireResourceCollectionResponse(
    response,
    `api.url("${route}").response(...)`,
  );
  return Object.freeze({
    declared: true,
    source: "response",
    itemIdentity: collectionResponse.itemIdentity,
    response: collectionResponse,
    reconcileMode: "responseCollection",
    items: null,
    replaceItems: null,
    aspects: Object.freeze({}),
    summaries: Object.freeze({}),
    summaryPatchScope: null,
  });
}

function extendApiRouteItemsAspect(state, route, name, read, write) {
  requireDeclaredApiRouteItemsState(state, route, "aspect");
  const aspect = requireApiRouteItemsName(route, "aspect", name);
  requireApiRouteItemsReadWrite(route, `aspect("${aspect}")`, read, write);
  if (aspect in state.aspects) {
    throw new TypeError(
      `api.url("${route}").items(...).aspect("${aspect}", ...) already exists in this route lane`,
    );
  }
  return Object.freeze({
    ...state,
    aspects: Object.freeze({
      ...state.aspects,
      [aspect]: Object.freeze({
        read,
        write,
      }),
    }),
  });
}

function extendApiRouteItemsSummary(
  state,
  route,
  name,
  read,
  write,
  patchScope,
) {
  requireDeclaredApiRouteItemsState(state, route, "summary");
  const summary = requireApiRouteItemsName(route, "summary", name);
  const methodName =
    patchScope === "pageWindow" ? "pageWindowSummary" : "summary";
  requireApiRouteItemsReadWrite(
    route,
    `${methodName}("${summary}")`,
    read,
    write,
  );
  if (summary in state.summaries) {
    throw new TypeError(
      `api.url("${route}").items(...).${methodName}("${summary}", ...) already exists in this route lane`,
    );
  }
  if (state.summaryPatchScope !== null && state.summaryPatchScope !== patchScope) {
    throw new TypeError(
      `api.url("${route}").items(...) cannot mix summary(...) with pageWindowSummary(...) in one route lane`,
    );
  }
  return Object.freeze({
    ...state,
    summaries: Object.freeze({
      ...state.summaries,
      [summary]: Object.freeze({
        read,
        write,
      }),
    }),
    summaryPatchScope: patchScope,
  });
}

function createApiRouteItemsReconcile(route, state) {
  if (state.reconcileMode === "responseCollection") {
    return createResourceCollectionResponseReconcile(state.response);
  }
  const aspects =
    Object.keys(state.aspects).length === 0
      ? undefined
      : resourceItemAspects(state.aspects);
  const summaries =
    Object.keys(state.summaries).length === 0
      ? undefined
      : state.summaryPatchScope === "pageWindow"
        ? resourceValueSummaries.pageWindow(state.summaries)
        : resourceValueSummaries(state.summaries);
  return resourceCollectionShape({
    items(value) {
      return state.reconcileMode === "custom"
        ? requireApiRouteItemsValueArray(
            route,
            state.items(value),
            "items(value)",
          )
        : requireApiDirectArrayValue(value, route);
    },
    replaceItems(value, nextItems) {
      if (state.reconcileMode === "custom") {
        return state.replaceItems(value, nextItems);
      }
      requireApiDirectArrayValue(value, route);
      return [...nextItems];
    },
    aspects,
    summaries,
  });
}

function requireDeclaredApiRouteItemsState(state, route, methodName) {
  if (!state.declared) {
    throw new TypeError(
      `api.url("${route}").${methodName}(...) requires items(itemIdentity) first`,
    );
  }
}

function requireApiRouteItemsName(route, kind, name) {
  if (typeof name !== "string" || name.length === 0) {
    throw new TypeError(
      `api.url("${route}").items(...) ${kind} names must be non-empty strings`,
    );
  }
  return name;
}

function requireApiRouteItemsReadWrite(route, label, read, write) {
  if (typeof read !== "function") {
    throw new TypeError(
      `api.url("${route}").items(...).${label} requires read(...)`,
    );
  }
  if (typeof write !== "function") {
    throw new TypeError(
      `api.url("${route}").items(...).${label} requires write(...)`,
    );
  }
}

function requireApiRouteItemsValueArray(route, value, label) {
  if (!Array.isArray(value)) {
    throw new TypeError(
      `api.url("${route}").items(...).reconcile(...) requires ${label} to return an array of items`,
    );
  }
  return value;
}

function requireApiDirectArrayValue(value, route) {
  if (!Array.isArray(value)) {
    throw new TypeError(
      `api.url("${route}").items(...) requires list/paged values to stay direct arrays`,
    );
  }
  return value;
}

export {
  createApiRouteItemsReconcile,
  createApiRouteItemsState,
  extendApiRouteItemsAspect,
  extendApiRouteItemsSummary,
  requireApiRouteResponseItemsState,
  requireApiRouteItemsReconcileState,
  requireApiRouteItemsState,
};
