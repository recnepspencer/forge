function createLineRegistryEntry(lineBacking, handle) {
  const entry = { handle };
  Object.defineProperty(entry, "materialization", {
    enumerable: true,
    configurable: false,
    get() {
      return lineBacking.current();
    },
  });
  return Object.freeze(entry);
}

export { createLineRegistryEntry };
