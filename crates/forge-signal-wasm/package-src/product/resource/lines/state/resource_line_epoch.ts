function createResourceLineEpoch() {
  let version = 0;
  const lineBackings = new Set();
  return Object.freeze({
    register(lineBacking) {
      lineBackings.add(lineBacking);
    },
    unregister(lineBacking) {
      lineBackings.delete(lineBacking);
    },
    captureAll() {
      for (const lineBacking of lineBackings) {
        lineBacking.captureCurrentSnapshot();
      }
    },
    version() {
      return version;
    },
    invalidateAll() {
      version += 1;
      return version;
    },
  });
}

export { createResourceLineEpoch };
