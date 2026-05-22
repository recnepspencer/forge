export function canonicalDiagnosticJson(value) {
  return JSON.stringify(stableDiagnosticClone(value));
}

export function digestCanonicalDiagnosticValue(value) {
  return digestDiagnosticString(canonicalDiagnosticJson(value));
}

export function digestDiagnosticString(value) {
  let hash = 2166136261;
  for (let index = 0; index < value.length; index += 1) {
    hash ^= value.charCodeAt(index);
    hash = Math.imul(hash, 16777619);
  }
  return `f1a-${(hash >>> 0).toString(16).padStart(8, "0")}`;
}

function stableDiagnosticClone(value) {
  if (Array.isArray(value)) {
    return value.map(stableDiagnosticClone);
  }
  if (value && typeof value === "object") {
    return Object.keys(value)
      .sort()
      .reduce((acc, key) => {
        acc[key] = stableDiagnosticClone(value[key]);
        return acc;
      }, {});
  }
  return value;
}
