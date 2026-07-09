function createSearchString(route, schema, search) {
  const entries = [];
  for (const [key, field] of Object.entries(schema)) {
    const value = search[key];
    if (value === undefined) {
      if (field.required) {
        throw new TypeError(`signals.router.route("${route}") search requires "${key}"`);
      }
      continue;
    }
    entries.push(
      `${encodeURIComponent(key)}=${encodeURIComponent(encodeSearchValue(route, key, field, value))}`,
    );
  }
  return entries.length === 0 ? "" : `?${entries.join("&")}`;
}

function normalizeSearchInput(route, schema, search) {
  if (!isPlainObject(search)) {
    throw new TypeError(`signals.router.route("${route}") search input must be an object`);
  }
  const normalized = {};
  for (const key of Object.keys(search)) {
    if (!(key in schema)) {
      throw new TypeError(
        `signals.router.route("${route}") does not admit undeclared search param "${key}"`,
      );
    }
  }
  for (const [key, field] of Object.entries(schema)) {
    const value = search[key];
    if (value === undefined) {
      if (field.required) {
        throw new TypeError(`signals.router.route("${route}") search requires "${key}"`);
      }
      continue;
    }
    normalized[key] = coerceSearchInputValue(route, key, field, value);
  }
  return normalized;
}

function parseSearchState(schema, route, searchParams) {
  for (const key of searchParams.keys()) {
    if (!(key in schema)) {
      return null;
    }
  }
  const normalized = {};
  for (const [key, field] of Object.entries(schema)) {
    const values = searchParams.getAll(key);
    if (values.length === 0) {
      if (field.required) {
        return null;
      }
      continue;
    }
    if (values.length > 1) {
      return null;
    }
    const parsed = parseSearchValue(field, values[0]);
    if (parsed === null) {
      return null;
    }
    normalized[key] = parsed;
  }
  return Object.freeze(normalized);
}

function encodeSearchValue(route, key, field, value) {
  const normalized = coerceSearchInputValue(route, key, field, value);
  return field.valueKind === "boolean" ? (normalized ? "true" : "false") : String(normalized);
}

function coerceSearchInputValue(route, key, field, value) {
  if (field.valueKind === "string") {
    if (typeof value !== "string") {
      throw new TypeError(`signals.router.route("${route}") search["${key}"] must be a string`);
    }
    return value;
  }
  if (field.valueKind === "number") {
    if (typeof value !== "number" || !Number.isFinite(value)) {
      throw new TypeError(
        `signals.router.route("${route}") search["${key}"] must be a finite number`,
      );
    }
    return value;
  }
  if (typeof value !== "boolean") {
    throw new TypeError(`signals.router.route("${route}") search["${key}"] must be a boolean`);
  }
  return value;
}

function parseSearchValue(field, value) {
  if (field.valueKind === "string") {
    return value;
  }
  if (field.valueKind === "number") {
    if (value.trim().length === 0) {
      return null;
    }
    const parsed = Number(value);
    return Number.isFinite(parsed) ? parsed : null;
  }
  if (value === "true") {
    return true;
  }
  if (value === "false") {
    return false;
  }
  return null;
}

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

export {
  createSearchString,
  normalizeSearchInput,
  parseSearchState,
};
