export function createRawReadableHandle(id, value) {
  return {
    id,
    get() {
      return value;
    },
    peek() {
      return value;
    },
    free() {},
  };
}
