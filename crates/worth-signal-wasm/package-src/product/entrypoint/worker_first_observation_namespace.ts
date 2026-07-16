function createWorkerFirstObservationNamespace(rootSession) {
  return Object.freeze({
    watch(target, callback) {
      return rootSession.watch(target, callback);
    },
    effect(target, callback) {
      return rootSession.effect(target, callback);
    },
    nuke(handle) {
      return rootSession.nuke(handle);
    },
  });
}

export { createWorkerFirstObservationNamespace };
