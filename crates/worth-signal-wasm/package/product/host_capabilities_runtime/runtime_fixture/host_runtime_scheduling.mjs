export function flushMicrotasks() {
  return new Promise((resolve) => queueMicrotask(resolve));
}

export function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
