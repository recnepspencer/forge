function cloneParamValue(value) {
  if (typeof globalThis.structuredClone === "function") {
    try {
      return globalThis.structuredClone(value);
    } catch {
      // Fall through to structural cloning for plain JS values.
    }
  }
  if (Array.isArray(value)) {
    return value.map(cloneParamValue);
  }
  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value).map(([key, entryValue]) => [
        key,
        cloneParamValue(entryValue),
      ]),
    );
  }
  return value;
}

function deepFreezeParamValue(value) {
  if (!value || typeof value !== "object") {
    return value;
  }
  if (Array.isArray(value)) {
    for (const entry of value) {
      deepFreezeParamValue(entry);
    }
    return Object.freeze(value);
  }
  for (const entryValue of Object.values(value)) {
    deepFreezeParamValue(entryValue);
  }
  return Object.freeze(value);
}

function createCanonicalParamSnapshot(params) {
  return deepFreezeParamValue(cloneParamValue(params));
}

export { createCanonicalParamSnapshot };
