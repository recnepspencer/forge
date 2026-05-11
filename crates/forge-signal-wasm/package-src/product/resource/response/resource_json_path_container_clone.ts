import {
  requireDenseJsonArray,
  requireJsonObjectPropertyName,
} from "./resource_json_path_value_compatibility.js";

function writeJsonContainerSegment(container, segment, nextSegmentValue, aspect) {
  return Array.isArray(container)
    ? cloneJsonArrayContainerWithSegment(container, segment, nextSegmentValue, aspect)
    : cloneJsonObjectContainerWithSegment(container, segment, nextSegmentValue, aspect);
}

function cloneJsonArrayContainerWithSegment(container, segment, nextSegmentValue, aspect) {
  requireDenseJsonArray(container, aspect);
  const nextArray = [];
  for (let index = 0; index < container.length; index += 1) {
    const descriptor = requireJsonContainerDataDescriptor(container, index, aspect);
    nextArray[index] = index === segment ? nextSegmentValue : descriptor.value;
  }
  return nextArray;
}

function cloneJsonObjectContainerWithSegment(container, segment, nextSegmentValue, aspect) {
  const nextObject = createJsonObjectContainerCopy(container);
  for (const [key, descriptor] of Object.entries(Object.getOwnPropertyDescriptors(container))) {
    requireJsonObjectPropertyName(aspect, key);
    if (!Object.prototype.hasOwnProperty.call(descriptor, "value")) {
      throw new TypeError(
        `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" rejects accessor JSON path segment "${key}"`,
      );
    }
    if (!descriptor.enumerable) {
      continue;
    }
    nextObject[key] = key === segment ? nextSegmentValue : descriptor.value;
  }
  if (!Object.prototype.hasOwnProperty.call(nextObject, segment)) {
    nextObject[segment] = nextSegmentValue;
  }
  return nextObject;
}

function createJsonObjectContainerCopy(container) {
  return Object.getPrototypeOf(container) === null
    ? Object.create(null)
    : {};
}

function requireJsonContainerDataDescriptor(container, segment, aspect) {
  const descriptor = Object.getOwnPropertyDescriptor(container, segment);
  if (descriptor === undefined) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" requires existing JSON path array index "${segment}"`,
    );
  }
  if (!Object.prototype.hasOwnProperty.call(descriptor, "value")) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" rejects accessor JSON path segment "${segment}"`,
    );
  }
  return descriptor;
}

export { writeJsonContainerSegment };
