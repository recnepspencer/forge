function createLineRegistryEntry(materialization, handle) {
  return Object.freeze({
    materialization,
    handle,
  });
}

export { createLineRegistryEntry };
