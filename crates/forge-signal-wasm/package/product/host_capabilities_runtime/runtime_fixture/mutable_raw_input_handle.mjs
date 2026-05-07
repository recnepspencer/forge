export function createMutableRawInputHandle(id, runtimeState) {
  return {
    id,
    get() {
      return runtimeState.values.get(id);
    },
    peek() {
      return runtimeState.values.get(id);
    },
    free() {},
  };
}
