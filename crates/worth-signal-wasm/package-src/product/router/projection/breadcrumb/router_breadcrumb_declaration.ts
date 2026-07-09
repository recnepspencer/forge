import {
  ROUTE_BREADCRUMB_DECLARATION,
  ROUTE_BREADCRUMB_ENTRY_DECLARATION,
  ROUTE_BREADCRUMB_PARENT_DECLARATION,
  ROUTE_BREADCRUMB_TRAIL_DECLARATION,
} from "../../router_symbols.js";

function createRouteBreadcrumbDeclaration(options) {
  return Object.freeze({
    [ROUTE_BREADCRUMB_DECLARATION]: true,
    ...normalizeRouteBreadcrumbBase(
      options,
      "signals.router.breadcrumb(...)",
      true,
    ),
    parent: normalizeRouteBreadcrumbParentDeclaration(
      options?.parent,
      "signals.router.breadcrumb(...).parent",
    ),
  });
}

function createRouteBreadcrumbParentDeclaration(options) {
  return normalizeRouteBreadcrumbParentDeclaration(
    options,
    "signals.router.breadcrumbParent(...)",
  );
}

function createRouteBreadcrumbEntryDeclaration(options) {
  return Object.freeze({
    [ROUTE_BREADCRUMB_ENTRY_DECLARATION]: true,
    ...normalizeRouteBreadcrumbBase(
      options,
      "signals.router.breadcrumbEntry(...)",
      false,
    ),
  });
}

function createRouteBreadcrumbTrailDeclaration(entries) {
  if (!Array.isArray(entries) || entries.length === 0) {
    throw new TypeError(
      "signals.router.breadcrumbTrail(...) requires a non-empty array of breadcrumbEntry(...) declarations",
    );
  }
  return Object.freeze({
    [ROUTE_BREADCRUMB_TRAIL_DECLARATION]: true,
    entries: Object.freeze(
      entries.map((entry, index) => normalizeBreadcrumbParentSeed(
        entry,
        `signals.router.breadcrumbTrail(...)[${index}]`,
      )),
    ),
  });
}

function normalizeRouteBreadcrumbDeclaration(route, breadcrumb) {
  if (breadcrumb === undefined) {
    return null;
  }
  if (isRouteBreadcrumbDeclaration(breadcrumb)) {
    return breadcrumb;
  }
  throw new TypeError(
    `signals.router.route("${route}") breadcrumb must be declared with signals.router.breadcrumb(...)`,
  );
}

function isRouteBreadcrumbDeclaration(value) {
  return Boolean(value && value[ROUTE_BREADCRUMB_DECLARATION] === true);
}

function isRouteBreadcrumbEntryDeclaration(value) {
  return Boolean(value && value[ROUTE_BREADCRUMB_ENTRY_DECLARATION] === true);
}

function isRouteBreadcrumbParentDeclaration(value) {
  return Boolean(value && value[ROUTE_BREADCRUMB_PARENT_DECLARATION] === true);
}

function isRouteBreadcrumbTrailDeclaration(value) {
  return Boolean(value && value[ROUTE_BREADCRUMB_TRAIL_DECLARATION] === true);
}

function normalizeRouteBreadcrumbParentDeclaration(options, label) {
  if (options === undefined) {
    return null;
  }
  if (isRouteBreadcrumbParentDeclaration(options)) {
    return options;
  }
  if (!isPlainObject(options)) {
    throw new TypeError(`${label} must be an object when provided`);
  }
  const recompute = options.recompute;
  if (recompute !== undefined && typeof recompute !== "function") {
    throw new TypeError(`${label}.recompute must be a function when provided`);
  }
  const fallback = options.fallback === undefined
    ? null
    : normalizeBreadcrumbParentSeed(options.fallback, `${label}.fallback`);
  const carry = normalizeCarryOption(options.carry, `${label}.carry`);
  if (recompute === undefined && fallback === null && carry === false) {
    throw new TypeError(
      `${label} requires recompute(...), carry: true, or fallback when provided`,
    );
  }
  return Object.freeze({
    [ROUTE_BREADCRUMB_PARENT_DECLARATION]: true,
    recompute: recompute ?? null,
    carry,
    fallback,
  });
}

function normalizeBreadcrumbParentSeed(value, label) {
  if (isRouteBreadcrumbEntryDeclaration(value) || isRouteBreadcrumbTrailDeclaration(value)) {
    return value;
  }
  throw new TypeError(
    `${label} must be declared with signals.router.breadcrumbEntry(...) or signals.router.breadcrumbTrail(...)`,
  );
}

function normalizeRouteBreadcrumbBase(options, label, allowParent) {
  if (!isPlainObject(options)) {
    throw new TypeError(`${label} requires an options object`);
  }
  const id = normalizeNonEmptyString(options.id, `${label}.id`);
  const labelSource = normalizeLabelSource(options.label, `${label}.label`);
  const target = normalizeTargetSource(options.target, `${label}.target`);
  if (!allowParent && "parent" in options) {
    throw new TypeError(`${label} does not admit parent ancestry declarations`);
  }
  return {
    id,
    label: labelSource,
    target,
  };
}

function normalizeLabelSource(value, label) {
  if (typeof value === "string" || typeof value === "function") {
    return value;
  }
  throw new TypeError(`${label} must be a string or function`);
}

function normalizeTargetSource(value, label) {
  if (
    value === undefined
    || value === null
    || typeof value === "string"
    || typeof value === "function"
    || isObject(value)
  ) {
    return value ?? null;
  }
  throw new TypeError(`${label} must be a string, route artifact, function, or omitted`);
}

function normalizeNonEmptyString(value, label) {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new TypeError(`${label} must be a non-empty string`);
  }
  return value;
}

function normalizeCarryOption(value, label) {
  if (value === undefined) {
    return false;
  }
  if (value === true || value === false) {
    return value;
  }
  throw new TypeError(`${label} must be true or false when provided`);
}

function isObject(value) {
  return value !== null && typeof value === "object";
}

function isPlainObject(value) {
  return isObject(value) && !Array.isArray(value);
}

export {
  createRouteBreadcrumbDeclaration,
  createRouteBreadcrumbEntryDeclaration,
  createRouteBreadcrumbParentDeclaration,
  createRouteBreadcrumbTrailDeclaration,
  isRouteBreadcrumbDeclaration,
  isRouteBreadcrumbEntryDeclaration,
  isRouteBreadcrumbTrailDeclaration,
  normalizeRouteBreadcrumbDeclaration,
};
