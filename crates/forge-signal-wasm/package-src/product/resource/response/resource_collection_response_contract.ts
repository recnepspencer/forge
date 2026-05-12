import {
  createCollectionResponse,
  requireResourceCollectionResponse,
} from "./resource_collection_response_factory.js";

function collection(options) {
  if (options === undefined) {
    return function defineCollectionResponse(collectionOptions) {
      return collection(collectionOptions);
    };
  }
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError("resource.response.collection(...) requires an options object");
  }
  return createCollectionResponse(
    "resource.response.collection(...)",
    options,
    { topology: "customCollection", itemField: null },
  );
}

function array(options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError("resource.response.array(...) requires an options object");
  }
  return createCollectionResponse(
    "resource.response.array(...)",
    {
      ...options,
      items(value) {
        if (!Array.isArray(value)) {
          throw new TypeError(
            "resource.response.array(...) requires list/paged values to stay direct arrays",
          );
        }
        return value;
      },
      replaceItems(_value, nextItems) {
        return [...nextItems];
      },
    },
    { topology: "directArray", itemField: null },
  );
}

function objectItems() {
  return function defineObjectItems(options) {
    if (!options || typeof options !== "object" || Array.isArray(options)) {
      throw new TypeError(
        "resource.response.objectItems<T>()(...) requires an options object",
      );
    }
    const field = requireObjectItemsField(options.field);
    return createCollectionResponse(
      "resource.response.objectItems<T>()(...)",
      {
        ...options,
        items(value) {
          return requireObjectItemsArray(value, field);
        },
        replaceItems(value, nextItems) {
          requireObjectItemsArray(value, field);
          return {
            ...value,
            [field]: [...nextItems],
          };
        },
      },
      { topology: "objectItems", itemField: field },
    );
  };
}

function entityStore() {
  return function defineEntityStore(options) {
    if (!options || typeof options !== "object" || Array.isArray(options)) {
      throw new TypeError(
        "resource.response.entityStore<T>()(...) requires an options object",
      );
    }
    if (typeof options.entities !== "function") {
      throw new TypeError(
        "resource.response.entityStore<T>()(...) requires entities(value)",
      );
    }
    if (typeof options.replaceEntities !== "function") {
      throw new TypeError(
        "resource.response.entityStore<T>()(...) requires replaceEntities(value, nextEntities)",
      );
    }
    if (typeof options.replaceEntity !== "function") {
      throw new TypeError(
        "resource.response.entityStore<T>()(...) requires replaceEntity(value, itemId, nextItem)",
      );
    }
    return createCollectionResponse(
      "resource.response.entityStore<T>()(...)",
      {
        ...options,
        items(value) {
          return Object.values(
            requireEntityStoreIdentityRecord(
              options.entities(value),
              options.itemId,
              "entities(value)",
            ),
          );
        },
        replaceItems(value, nextItems) {
          requireEntityStoreIdentityRecord(
            options.entities(value),
            options.itemId,
            "entities(value)",
          );
          return options.replaceEntities(
            value,
            createEntityStoreRecord(options.itemId, nextItems),
          );
        },
        readItem(value, itemIdValue) {
          return readEntityStoreItem(
            options.entities(value),
            options.itemId,
            itemIdValue,
            "entities(value)",
          );
        },
        replaceItem(value, itemIdValue, nextItem) {
          const currentEntity = readEntityStoreItem(
            options.entities(value),
            options.itemId,
            itemIdValue,
            "entities(value)",
          );
          if (!currentEntity.found) {
            throw new RangeError(
              `resource.response.entityStore<T>()(...) could not find entity id "${itemIdValue}"`,
            );
          }
          const nextValue = options.replaceEntity(value, itemIdValue, nextItem);
          const replacedEntity = readEntityStoreItem(
            options.entities(nextValue),
            options.itemId,
            itemIdValue,
            "replaceEntity(value, itemId, nextItem)",
          );
          if (!replacedEntity.found) {
            throw new TypeError(
              `resource.response.entityStore<T>()(...) requires replaceEntity(value, itemId, nextItem) to preserve entity id "${itemIdValue}"`,
            );
          }
          return nextValue;
        },
      },
      { topology: "entityStore", itemField: null },
    );
  };
}

function requireObjectItemsField(field) {
  if (typeof field !== "string" || field.length === 0) {
    throw new TypeError(
      "resource.response.objectItems<T>()(...) requires a non-empty field name",
    );
  }
  return field;
}

function requireObjectItemsArray(value, field) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(
      `resource.response.objectItems<T>() field "${field}" requires object response values`,
    );
  }
  const items = value[field];
  if (!Array.isArray(items)) {
    throw new TypeError(
      `resource.response.objectItems<T>() field "${field}" requires an array`,
    );
  }
  return items;
}

function requireEntityStoreRecord(entities) {
  if (!entities || typeof entities !== "object" || Array.isArray(entities)) {
    throw new TypeError(
      "resource.response.entityStore<T>()(...) requires entities(value) to return an object record",
    );
  }
  return entities;
}

function requireEntityStoreIdentityRecord(rawEntities, itemId, source) {
  const entities = requireEntityStoreRecord(rawEntities);
  for (const [entityKey, entity] of Object.entries(entities)) {
    requireEntityStoreItemIdentity(entityKey, entity, itemId, source);
  }
  return entities;
}

function readEntityStoreItem(rawEntities, itemId, itemIdValue, source) {
  const entities = requireEntityStoreRecord(rawEntities);
  if (!Object.prototype.hasOwnProperty.call(entities, itemIdValue)) {
    return Object.freeze({ found: false, item: null });
  }
  if (!Object.prototype.propertyIsEnumerable.call(entities, itemIdValue)) {
    throw new TypeError(
      `resource.response.entityStore<T>()(...) requires ${source} entity id "${itemIdValue}" to be enumerable`,
    );
  }
  const item = entities[itemIdValue];
  requireEntityStoreItemIdentity(itemIdValue, item, itemId, source);
  return Object.freeze({ found: true, item });
}

function requireEntityStoreItemIdentity(entityKey, entity, itemId, source) {
  const actualItemId = itemId(entity);
  if (actualItemId !== entityKey) {
    throw new TypeError(
      `resource.response.entityStore<T>()(...) requires ${source} entity key "${entityKey}" to match itemId(item) "${actualItemId}"`,
    );
  }
}

function createEntityStoreRecord(itemId, nextItems) {
  const entities = {};
  for (const item of nextItems) {
    const key = itemId(item);
    if (key in entities) {
      throw new TypeError(
        `resource.response.entityStore<T>()(...) cannot replace duplicated entity id "${key}"`,
      );
    }
    entities[key] = item;
  }
  return entities;
}

export {
  array,
  collection,
  entityStore,
  objectItems,
  requireResourceCollectionResponse,
};
