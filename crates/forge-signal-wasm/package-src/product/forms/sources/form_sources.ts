export function readSource(source) {
  if (typeof source === "function") {
    return source();
  }
  if (source && typeof source.get === "function") {
    return source.get();
  }
  return source;
}
