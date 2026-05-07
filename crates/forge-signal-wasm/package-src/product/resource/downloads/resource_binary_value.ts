import { requireBinaryDescriptors } from "./resource_binary_descriptor.js";

const RESOURCE_BINARY_VALUE_BRAND = Symbol("forgeSignal.resourceBinaryValue");

function resourceBinaryValue(options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError("resourceBinaryValue(...) requires an options object");
  }
  if (!("value" in options)) {
    throw new TypeError("resourceBinaryValue(...) requires value");
  }
  return Object.freeze({
    value: options.value,
    descriptors: requireBinaryDescriptors(options.descriptors),
    [RESOURCE_BINARY_VALUE_BRAND]: "resourceBinaryValue",
  });
}

function isResourceBinaryValue(value) {
  return !!value && value[RESOURCE_BINARY_VALUE_BRAND] === "resourceBinaryValue";
}

export { isResourceBinaryValue, resourceBinaryValue };
