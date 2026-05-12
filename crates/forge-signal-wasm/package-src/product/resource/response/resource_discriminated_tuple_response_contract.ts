import {
  createCollectionResponse,
} from "./resource_collection_response_factory.js";

function discriminated() {
  return function defineDiscriminatedTupleResponse(options) {
    requireDiscriminatedTupleOptions(options);
    return createCollectionResponse(
      "resource.response.discriminated<T>()(...)",
      createDiscriminatedTupleAdapter(options),
      { topology: "discriminatedTuple", itemField: null },
    );
  };
}

function requireDiscriminatedTupleOptions(options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      "resource.response.discriminated<T>()(...) requires an options object",
    );
  }
  if (typeof options.discriminator !== "function") {
    throw new TypeError(
      "resource.response.discriminated<T>()(...) requires discriminator(value)",
    );
  }
  if (!options.variants || typeof options.variants !== "object") {
    throw new TypeError(
      "resource.response.discriminated<T>()(...) requires variants",
    );
  }
}

function createDiscriminatedTupleAdapter(options) {
  return {
    ...options,
    items(value) {
      return readActiveVariantItems(value, options, "items(value)");
    },
    replaceItems(value, nextItems) {
      const variant = readActiveVariant(value, options, "replaceItems(value)");
      const nextValue = variant.replaceItems(value, nextItems);
      const nextVariantKey = requireDiscriminatedTupleVariantKey(nextValue, options);
      if (nextVariantKey !== variant.key) {
        throw new TypeError(
          `resource.response.discriminated<T>()(...) requires replaceItems(value, nextItems) to preserve discriminator "${variant.key}"`,
        );
      }
      readActiveVariantItems(nextValue, options, "replaceItems(value, nextItems)");
      return nextValue;
    },
  };
}

function readActiveVariantItems(value, options, source) {
  const variant = readActiveVariant(value, options, source);
  const items = variant.items(value);
  if (!Array.isArray(items)) {
    throw new TypeError(
      `resource.response.discriminated<T>()(...) requires ${source} variant "${variant.key}" items(value) to return an array`,
    );
  }
  return items;
}

function readActiveVariant(value, options, source) {
  const key = requireDiscriminatedTupleVariantKey(value, options);
  const variant = options.variants[key];
  if (!variant || typeof variant !== "object") {
    throw new TypeError(
      `resource.response.discriminated<T>()(...) requires ${source} discriminator "${key}" to name a declared variant`,
    );
  }
  if (typeof variant.items !== "function") {
    throw new TypeError(
      `resource.response.discriminated<T>()(...) variant "${key}" requires items(value)`,
    );
  }
  if (typeof variant.replaceItems !== "function") {
    throw new TypeError(
      `resource.response.discriminated<T>()(...) variant "${key}" requires replaceItems(value, nextItems)`,
    );
  }
  return Object.freeze({ ...variant, key });
}

function requireDiscriminatedTupleVariantKey(value, options) {
  const key = options.discriminator(value);
  if (typeof key !== "string" || key.length === 0) {
    throw new TypeError(
      "resource.response.discriminated<T>()(...) requires discriminator(value) to return a non-empty string",
    );
  }
  return key;
}

export { discriminated };
