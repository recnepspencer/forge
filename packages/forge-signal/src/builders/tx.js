export const tx = {
  set(id, value) {
    return { kind: "set", id, value };
  },
  setMany(values) {
    return { kind: "setMany", values };
  }
};
