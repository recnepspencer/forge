function createHashString(route, hashSchema, hash) {
  const normalized = normalizeHashInput(route, hashSchema, hash);
  return normalized === undefined ? "" : `#${encodeURIComponent(normalized)}`;
}

function normalizeHashInput(route, hashSchema, hash) {
  if (hashSchema === null) {
    if (hash === undefined) {
      return undefined;
    }
    throw new TypeError(`signals.router.route("${route}") does not declare hash state`);
  }
  if (hash === undefined) {
    return undefined;
  }
  if (typeof hash !== "string") {
    throw new TypeError(`signals.router.route("${route}") hash must be a string`);
  }
  return hash;
}

function parseHashState(hashSchema, rawHash) {
  if (hashSchema === null) {
    return rawHash.length === 0 ? undefined : null;
  }
  if (rawHash.length === 0) {
    return undefined;
  }
  try {
    return decodeURIComponent(rawHash.slice(1));
  } catch {
    return null;
  }
}

export {
  createHashString,
  normalizeHashInput,
  parseHashState,
};
