export function cloneWorkerCachedValue(value) {
  if (typeof globalThis.structuredClone === "function") {
    try {
      return globalThis.structuredClone(value);
    } catch {}
  }
  return cloneWorkerCachedValueFallback(value);
}

export function materializeWorkerCachedValue(value) {
  if (typeof globalThis.structuredClone === "function") {
    try {
      return deepFreezeWorkerCachedValue(globalThis.structuredClone(value));
    } catch {}
  }
  return materializeWorkerCachedValueFallback(value);
}

function deepFreezeWorkerCachedValue(value) {
  if (!value || typeof value !== "object") {
    return value;
  }
  if (Array.isArray(value)) {
    for (const entry of value) {
      deepFreezeWorkerCachedValue(entry);
    }
    return Object.freeze(value);
  }
  for (const entry of Object.values(value)) {
    deepFreezeWorkerCachedValue(entry);
  }
  return Object.freeze(value);
}

function cloneWorkerCachedValueFallback(value) {
  if (Array.isArray(value)) {
    return value.map((entry) => cloneWorkerCachedValueFallback(entry));
  }
  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value).map(([key, entry]) => [key, cloneWorkerCachedValueFallback(entry)]),
    );
  }
  return value;
}

function materializeWorkerCachedValueFallback(value) {
  if (Array.isArray(value)) {
    return Object.freeze(
      value.map((entry) => materializeWorkerCachedValueFallback(entry)),
    );
  }
  if (value && typeof value === "object") {
    return Object.freeze(
      Object.fromEntries(
        Object.entries(value).map(([key, entry]) => [
          key,
          materializeWorkerCachedValueFallback(entry),
        ]),
      ),
    );
  }
  return value;
}
