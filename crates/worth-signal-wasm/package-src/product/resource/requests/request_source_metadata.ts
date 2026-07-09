const RESOURCE_REQUEST_SOURCE_RESOLVER = Symbol(
  "WorthSignal.resourceRequestSourceResolver",
);

function createTaggedRequestSourceInput(resolve) {
  const tagged = (params) => resolve(params).value;
  Object.defineProperty(tagged, RESOURCE_REQUEST_SOURCE_RESOLVER, {
    value: resolve,
    enumerable: false,
    configurable: false,
    writable: false,
  });
  return tagged;
}

function readTaggedRequestSourceResolution(input, params) {
  if (typeof input !== "function") {
    return null;
  }
  const resolver = input[RESOURCE_REQUEST_SOURCE_RESOLVER];
  if (typeof resolver !== "function") {
    return null;
  }
  return resolver(params);
}

export {
  createTaggedRequestSourceInput,
  readTaggedRequestSourceResolution,
};
