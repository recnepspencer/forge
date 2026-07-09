import { resourcePatch } from "../../resource/reconciliation/resource_patch.js";

const CREATE_INSERT_ADMITTED_COLLECTION_TOPOLOGIES = Object.freeze([
  null,
  "connection",
  "directArray",
  "discriminatedTuple",
  "groupedCollection",
  "objectItems",
  "customCollection",
  "entityStore",
  "mapCollection",
  "namedCollection",
  "recursiveTree",
  "sparsePage",
]);

const DELETE_ADMITTED_COLLECTION_TOPOLOGIES = Object.freeze([
  null,
  "connection",
  "directArray",
  "discriminatedTuple",
  "groupedCollection",
  "objectItems",
  "customCollection",
  "entityStore",
  "mapCollection",
  "namedCollection",
  "recursiveTree",
  "sparsePage",
]);

function lowerCollectionReconciliation(route, semanticFinalizer, response, familyMetadata, collection, index) {
  if (!collection || typeof collection !== "object" || Array.isArray(collection)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] collection must be a target declaration object`,
    );
  }
  if (familyMetadata.familyKind === "detail") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] collection item reconciliation requires a collection or paged read family`,
    );
  }
  const patchRecord = familyMetadata.patchRecord;
  if (!patchRecord.narrowItem || typeof patchRecord.itemIdentity !== "function") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] collection item reconciliation requires a target family with item reconciliation`,
    );
  }
  if (collection.kind === "item") {
    if (semanticFinalizer !== "update" && semanticFinalizer !== "remove") {
      throw new TypeError(
        `api.url("${route}").response(...).create/update/remove(...) collection item replacement is currently admitted only for update/save and remove/delete responses`,
      );
    }
    const executionKind =
      semanticFinalizer === "remove" ? "exactCollectionTombstone" : "exactCollectionItem";
    const targetDigest =
      semanticFinalizer === "remove" ? "collection:tombstone" : "collection:item";
    return Object.freeze({
      kind: "item",
      executionKind,
      targetDigest,
      readItemId(responseValue) {
        return patchRecord.itemIdentity(responseValue);
      },
      createPatch(responseValue) {
        const itemId = patchRecord.itemIdentity(responseValue);
        return resourcePatch.item({
          itemId,
          nextItem: responseValue,
        });
      },
    });
  }
  if (collection.kind === "insert") {
    if (semanticFinalizer !== "create") {
      throw new TypeError(
        `api.url("${route}").response(...).create/update/remove(...) collection insert reconciliation is currently admitted only for create responses`,
      );
    }
    assertCreatePlacementTopologyAdmitted(route, patchRecord, index);
    if (collection.placement !== "append" && collection.placement !== "prepend") {
      throw new TypeError(
        `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] collection.placement must be append or prepend`,
      );
    }
    return Object.freeze({
      kind: "insert",
      executionKind: "exactCollectionInsert",
      placement: collection.placement,
      targetDigest: `collection:insert:${collection.placement}`,
      readItemId(responseValue) {
        return patchRecord.itemIdentity(responseValue);
      },
      createPatch(responseValue) {
        const itemId = patchRecord.itemIdentity(responseValue);
        return resourcePatch.insert({
          itemId,
          placement: collection.placement,
          nextItem: responseValue,
        });
      },
    });
  }
  if (collection.kind === "delete") {
    if (semanticFinalizer !== "remove") {
      throw new TypeError(
        `api.url("${route}").response(...).create/update/remove(...) collection deletion is currently admitted only for remove/delete responses`,
      );
    }
    assertDeleteTopologyAdmitted(route, patchRecord, index);
    if (collection.itemId !== undefined && typeof collection.itemId !== "function") {
      throw new TypeError(
        `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] collection.delete.itemId must be a function when provided`,
      );
    }
    if (
      semanticFinalizer === "remove"
      && response.kind !== "detail"
      && typeof collection.itemId !== "function"
    ) {
      throw new TypeError(
        `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] collection delete reconciliation requires collection.itemId(...) when the mutation response lens does not carry canonical item identity`,
      );
    }
    return Object.freeze({
      kind: "delete",
      executionKind: "exactCollectionDelete",
      targetDigest: typeof collection.itemId === "function"
        ? "collection:delete:declaredItemId"
        : "collection:delete",
      readItemId(responseValue, mutationParams) {
        return typeof collection.itemId === "function"
          ? collection.itemId(mutationParams, responseValue)
          : patchRecord.itemIdentity(responseValue);
      },
      createPatch(responseValue, mutationParams) {
        return resourcePatch.delete({
          itemId:
            typeof collection.itemId === "function"
              ? collection.itemId(mutationParams, responseValue)
              : patchRecord.itemIdentity(responseValue),
        });
      },
    });
  }
  throw new TypeError(
    `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] collection kind must be item, insert, or delete`,
  );
}

function assertCreatePlacementTopologyAdmitted(route, patchRecord, index) {
  const topology = patchRecord.responseLensProof?.topology ?? null;
  if (CREATE_INSERT_ADMITTED_COLLECTION_TOPOLOGIES.includes(topology)) {
    return;
  }
  throw new TypeError(
    `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] collection insert reconciliation is not admitted for ${topology} targets before advanced placement topology support lands`,
  );
}

function assertDeleteTopologyAdmitted(route, patchRecord, index) {
  const topology = patchRecord.responseLensProof?.topology ?? null;
  if (DELETE_ADMITTED_COLLECTION_TOPOLOGIES.includes(topology)) {
    return;
  }
  throw new TypeError(
    `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] collection delete reconciliation is not admitted for ${topology} targets before exact deletion support lands`,
  );
}

export { lowerCollectionReconciliation };
