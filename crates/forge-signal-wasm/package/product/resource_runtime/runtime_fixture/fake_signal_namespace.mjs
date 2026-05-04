function createFakeSignalNamespace(scopeId = "root", historyOverrides = {}) {
  let nextHandleId = 0;

  function createHandle(readCurrentValue) {
    let released = false;
    const handle = () => {
      if (released) {
        throw new Error("fake signal handle was used after free()");
      }
      return readCurrentValue();
    };
    nextHandleId += 1;
    handle.id = `${scopeId}.signal${nextHandleId}`;
    handle.get = handle;
    handle.free = () => {
      released = true;
    };
    return handle;
  }

  return {
    scope(childLocalScopeId) {
      return createFakeSignalNamespace(
        `${scopeId}.${childLocalScopeId}`,
        historyOverrides,
      );
    },
    input(initialValue) {
      let currentValue = initialValue;
      const handle = createHandle(() => currentValue);
      handle.set = (nextValue) => {
        handle();
        currentValue = nextValue;
      };
      return handle;
    },
    computed(project) {
      return createHandle(() => project());
    },
    history() {
      return {
        replay_for(id) {
          return { id, family: "replay" };
        },
        lineage_for(id) {
          return { id, family: "lineage" };
        },
        ...historyOverrides,
      };
    },
  };
}

export { createFakeSignalNamespace };
