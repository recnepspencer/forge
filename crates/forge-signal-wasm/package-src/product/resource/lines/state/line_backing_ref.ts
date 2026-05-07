function createLineBackingRef(
  resourceLineEpoch,
  materialize,
  snapshotCurrent,
  restoreSnapshot,
) {
  let currentMaterialization = null;
  let currentEpoch = -1;
  const retiredMaterializations = [];
  let continuitySnapshot = null;

  function retireCurrent() {
    if (currentMaterialization === null) {
      return;
    }
    continuitySnapshot = snapshotCurrent(currentMaterialization);
    retiredMaterializations.push(currentMaterialization);
  }

  function ensureCurrent() {
    const epochVersion = resourceLineEpoch.version();
    if (currentMaterialization !== null && currentEpoch === epochVersion) {
      return currentMaterialization;
    }
    retireCurrent();
    currentMaterialization = materialize();
    currentEpoch = epochVersion;
    if (continuitySnapshot !== null) {
      restoreSnapshot(currentMaterialization, continuitySnapshot);
    }
    return currentMaterialization;
  }

  return Object.freeze({
    current() {
      return ensureCurrent();
    },
    replace(nextMaterialization) {
      ensureCurrent();
      retireCurrent();
      currentMaterialization = nextMaterialization;
      currentEpoch = resourceLineEpoch.version();
      continuitySnapshot = null;
      return currentMaterialization;
    },
    forceRematerialize(materializeOverride = materialize) {
      retireCurrent();
      currentMaterialization = materializeOverride();
      currentEpoch = resourceLineEpoch.version();
      continuitySnapshot = null;
      return currentMaterialization;
    },
    captureCurrentSnapshot() {
      if (currentMaterialization === null) {
        return;
      }
      continuitySnapshot = snapshotCurrent(currentMaterialization);
    },
    retired() {
      return Object.freeze([...retiredMaterializations]);
    },
  });
}

export { createLineBackingRef };
