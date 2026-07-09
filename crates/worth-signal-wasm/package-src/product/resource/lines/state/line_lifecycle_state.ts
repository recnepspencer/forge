function createLineLifecycleState() {
  let released = false;
  let pendingReloadToken = 0;
  let pendingReloadOperation = null;
  let nextPendingReloadToken = 0;
  const ownedViews = new Set();
  return Object.freeze({
    addOwnedView(viewHandle) {
      ownedViews.add(viewHandle);
    },
    beginPendingReload(operation) {
      nextPendingReloadToken += 1;
      pendingReloadToken = nextPendingReloadToken;
      pendingReloadOperation = operation;
      return pendingReloadToken;
    },
    completePendingReload(token) {
      if (released || pendingReloadToken !== token) {
        return false;
      }
      pendingReloadToken = 0;
      pendingReloadOperation = null;
      return true;
    },
    isReleased() {
      return released;
    },
    markReleased() {
      released = true;
      pendingReloadToken = 0;
      pendingReloadOperation = null;
    },
    supersedePendingReload() {
      if (pendingReloadOperation === null) {
        return null;
      }
      const operation = pendingReloadOperation;
      pendingReloadToken = 0;
      pendingReloadOperation = null;
      return operation;
    },
    releaseOwnedViews() {
      for (const viewHandle of ownedViews) {
        viewHandle.free();
      }
      ownedViews.clear();
    },
  });
}

export { createLineLifecycleState };
