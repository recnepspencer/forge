const PLAIN_OBJECT = Object.getPrototypeOf({});

export function isPlainRecord(value) {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    return false;
  }
  const prototype = Object.getPrototypeOf(value);
  return prototype === PLAIN_OBJECT || prototype === null;
}

export function deepClone(value) {
  if (Array.isArray(value)) {
    return value.map(deepClone);
  }
  if (isPlainRecord(value)) {
    return Object.fromEntries(
      Object.entries(value).map(([key, entry]) => [key, deepClone(entry)]),
    );
  }
  return value;
}

export function deepFreeze(value) {
  if (Array.isArray(value)) {
    value.forEach(deepFreeze);
    return Object.freeze(value);
  }
  if (isPlainRecord(value)) {
    Object.values(value).forEach(deepFreeze);
    return Object.freeze(value);
  }
  return value;
}

export function immutableClone(value) {
  return deepFreeze(deepClone(value));
}

export function canonicalStringify(value) {
  return JSON.stringify(canonicalize(value));
}

export function canonicalDigest(value) {
  const text = canonicalStringify(value);
  let hash = 0xcbf29ce484222325n;
  for (let index = 0; index < text.length; index += 1) {
    hash ^= BigInt(text.charCodeAt(index));
    hash = BigInt.asUintN(64, hash * 0x100000001b3n);
  }
  return hash.toString(16).padStart(16, "0");
}

export function canonicalId(family, value) {
  return `${family}:${canonicalDigest(value)}`;
}

function canonicalize(value) {
  if (Array.isArray(value)) {
    return value.map(canonicalize);
  }
  if (isPlainRecord(value)) {
    return Object.fromEntries(
      Object.keys(value)
        .sort()
        .map((key) => [key, canonicalize(value[key])]),
    );
  }
  if (typeof value === "bigint") {
    return { $bigint: value.toString() };
  }
  if (value === undefined) {
    return { $undefined: true };
  }
  if (typeof value === "number" && !Number.isFinite(value)) {
    return { $number: String(value) };
  }
  if (typeof value === "number" && Object.is(value, -0)) {
    return { $number: "-0" };
  }
  if (typeof value === "function" || typeof value === "symbol") {
    throw new TypeError(`unsupported canonical value type ${typeof value}`);
  }
  if (value !== null && typeof value === "object") {
    throw new TypeError(`unsupported canonical object ${value.constructor?.name ?? "unknown"}`);
  }
  return value;
}
