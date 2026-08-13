export function evaluateWorkerFirstDeclarativeSpec(rootSession, family, spec, operation) {
  if (!spec || typeof spec !== "object" || Array.isArray(spec)) {
    throw new TypeError(`${operation}(...) requires a declarative ${family} spec object`);
  }
  const dependencyIds = normalizeDeclarativeReadIds(spec.reads, operation);
  for (const id of dependencyIds) {
    if (!rootSession.hasKnownSignalId(id)) {
      throw new TypeError(
        `${operation}(...) can read only currently available worker-first signals; \`${id}\` is not currently available`,
      );
    }
  }
  if (spec.when !== undefined) {
    if (!spec.when || typeof spec.when !== "object" || Array.isArray(spec.when)) {
      throw new TypeError(`${operation}(...) requires spec.when as a condition object when provided`);
    }
    evaluateExpr(spec.when.expr, (id) => rootSession.readSignalValue(id), operation);
  }
  return Object.freeze({
    value: evaluateExpr(spec.expr, (id) => rootSession.readSignalValue(id), operation),
    dependencyIds,
  });
}

/**
 * Tip-local declarative evaluation over an explicit tip reader.
 * Used by host tip projection before worker catch-up.
 */
export function evaluateWorkerFirstDeclarativeTip(spec, readSignalValue, operation = "tip.project") {
  if (!spec || typeof spec !== "object" || Array.isArray(spec) || !spec.expr) {
    throw new TypeError(`${operation}(...) requires a declarative tip spec with expr`);
  }
  if (typeof readSignalValue !== "function") {
    throw new TypeError(`${operation}(...) requires a tip signal reader`);
  }
  if (spec.when != null) {
    if (typeof spec.when !== "object" || Array.isArray(spec.when)) {
      throw new TypeError(`${operation}(...) requires spec.when as a condition object when provided`);
    }
    evaluateExpr(spec.when.expr, readSignalValue, operation);
  }
  return evaluateExpr(spec.expr, readSignalValue, operation);
}

function normalizeDeclarativeReadIds(reads, operation) {
  if (reads === undefined) {
    return [];
  }
  if (!Array.isArray(reads)) {
    throw new TypeError(`${operation}(...) requires spec.reads as an array when provided`);
  }
  return reads.map((entry) => {
    if (typeof entry === "string" && entry.length > 0) {
      return entry;
    }
    if (entry && typeof entry === "object" && typeof entry.id === "string" && entry.id.length > 0) {
      return entry.id;
    }
    throw new TypeError(
      `${operation}(...) requires every spec.reads entry to be a non-empty signal id or read descriptor`,
    );
  });
}

function evaluateExpr(expr, readSignal, operation) {
  if (!expr || typeof expr !== "object" || Array.isArray(expr) || typeof expr.kind !== "string") {
    throw new TypeError(`${operation}(...) requires a supported declarative expr`);
  }
  switch (expr.kind) {
    case "value": return expr.value;
    case "read": return readSignal(expr.id);
    case "get": return readObjectField(evaluateExpr(expr.target, readSignal, operation), expr.field);
    case "at": return readArrayIndex(
      evaluateExpr(expr.target, readSignal, operation),
      evaluateExpr(expr.index, readSignal, operation),
    );
    case "first": return readFirst(evaluateExpr(expr.target, readSignal, operation));
    case "last": return readLast(evaluateExpr(expr.target, readSignal, operation));
    case "slice": return readSlice(
      evaluateExpr(expr.target, readSignal, operation),
      evaluateExpr(expr.start, readSignal, operation),
      expr.end === undefined ? undefined : evaluateExpr(expr.end, readSignal, operation),
    );
    case "join": return requireArray(evaluateExpr(expr.target, readSignal, operation)).join(
      String(evaluateExpr(expr.separator, readSignal, operation)),
    );
    case "flatten": return requireArray(evaluateExpr(expr.target, readSignal, operation)).flat();
    case "object": return Object.fromEntries(
      expr.fields.map(([key, value]) => [key, evaluateExpr(value, readSignal, operation)]),
    );
    case "array": return expr.items.map((item) => evaluateExpr(item, readSignal, operation));
    case "sum": return evaluateMany(expr.args, readSignal, operation).reduce((a, b) => Number(a) + Number(b), 0);
    case "multiply": return evaluateMany(expr.args, readSignal, operation).reduce((a, b) => Number(a) * Number(b), 1);
    case "concat": return evaluateMany(expr.args, readSignal, operation).map(String).join("");
    case "coalesce": return evaluateMany(expr.args, readSignal, operation).find((value) => value !== null && value !== undefined) ?? null;
    case "length": return readLength(evaluateExpr(expr.target, readSignal, operation));
    case "contains": return requireArray(evaluateExpr(expr.target, readSignal, operation))
      .includes(evaluateExpr(expr.value, readSignal, operation));
    case "mergeObjects": return Object.assign({}, ...evaluateMany(expr.args, readSignal, operation).map(requireObject));
    case "keys": return Object.keys(requireObject(evaluateExpr(expr.target, readSignal, operation)));
    case "values": return Object.values(requireObject(evaluateExpr(expr.target, readSignal, operation)));
    case "hasField": return Object.prototype.hasOwnProperty.call(
      requireObject(evaluateExpr(expr.target, readSignal, operation)),
      expr.field,
    );
    case "pick": return pickFields(requireObject(evaluateExpr(expr.target, readSignal, operation)), expr.fields);
    case "omit": return omitFields(requireObject(evaluateExpr(expr.target, readSignal, operation)), expr.fields);
    case "append": return [
      ...requireArray(evaluateExpr(expr.target, readSignal, operation)),
      evaluateExpr(expr.value, readSignal, operation),
    ];
    case "abs": return Math.abs(Number(evaluateExpr(expr.target, readSignal, operation)));
    case "min": return Math.min(...evaluateMany(expr.args, readSignal, operation).map(Number));
    case "max": return Math.max(...evaluateMany(expr.args, readSignal, operation).map(Number));
    case "sqrt": return Math.sqrt(Number(evaluateExpr(expr.target, readSignal, operation)));
    case "sin": return Math.sin(Number(evaluateExpr(expr.target, readSignal, operation)));
    case "cos": return Math.cos(Number(evaluateExpr(expr.target, readSignal, operation)));
    case "floor": return Math.floor(Number(evaluateExpr(expr.target, readSignal, operation)));
    case "mod": return Number(evaluateExpr(expr.left, readSignal, operation)) % Number(evaluateExpr(expr.right, readSignal, operation));
    case "clamp": return clampValue(
      Number(evaluateExpr(expr.value, readSignal, operation)),
      Number(evaluateExpr(expr.min, readSignal, operation)),
      Number(evaluateExpr(expr.max, readSignal, operation)),
    );
    case "atan2": return Math.atan2(
      Number(evaluateExpr(expr.y, readSignal, operation)),
      Number(evaluateExpr(expr.x, readSignal, operation)),
    );
    case "subtract": return Number(evaluateExpr(expr.left, readSignal, operation)) - Number(evaluateExpr(expr.right, readSignal, operation));
    case "divide": return Number(evaluateExpr(expr.left, readSignal, operation)) / Number(evaluateExpr(expr.right, readSignal, operation));
    case "eq": return evaluateExpr(expr.left, readSignal, operation) === evaluateExpr(expr.right, readSignal, operation);
    case "neq": return evaluateExpr(expr.left, readSignal, operation) !== evaluateExpr(expr.right, readSignal, operation);
    case "gt": return Number(evaluateExpr(expr.left, readSignal, operation)) > Number(evaluateExpr(expr.right, readSignal, operation));
    case "gte": return Number(evaluateExpr(expr.left, readSignal, operation)) >= Number(evaluateExpr(expr.right, readSignal, operation));
    case "lt": return Number(evaluateExpr(expr.left, readSignal, operation)) < Number(evaluateExpr(expr.right, readSignal, operation));
    case "lte": return Number(evaluateExpr(expr.left, readSignal, operation)) <= Number(evaluateExpr(expr.right, readSignal, operation));
    case "and": return evaluateMany(expr.args, readSignal, operation).every(Boolean);
    case "or": return evaluateMany(expr.args, readSignal, operation).some(Boolean);
    case "not": return !evaluateExpr(expr.arg, readSignal, operation);
    case "if": return evaluateExpr(expr.condition, readSignal, operation)
      ? evaluateExpr(expr.thenExpr, readSignal, operation)
      : evaluateExpr(expr.elseExpr, readSignal, operation);
    default:
      throw new TypeError(`${operation}(...) expr kind "${expr.kind}" is unsupported on the current worker-first sync surface`);
  }
}

function evaluateMany(args, readSignal, operation) {
  return Array.isArray(args)
    ? args.map((arg) => evaluateExpr(arg, readSignal, operation))
    : [];
}

function requireObject(value) {
  if (value && typeof value === "object" && !Array.isArray(value)) {
    return value;
  }
  throw new TypeError("worker-first sync declarative evaluation expected an object value");
}

function requireArray(value) {
  if (Array.isArray(value)) {
    return value;
  }
  throw new TypeError("worker-first sync declarative evaluation expected an array value");
}

function readObjectField(value, field) {
  const objectValue = requireObject(value);
  return objectValue[field] ?? null;
}

function readArrayIndex(value, index) {
  return requireArray(value)[Number(index)] ?? null;
}

function readFirst(value) {
  return requireArray(value)[0] ?? null;
}

function readLast(value) {
  const items = requireArray(value);
  return items.length === 0 ? null : items[items.length - 1];
}

function readSlice(value, start, end) {
  return requireArray(value).slice(Number(start), end === undefined ? undefined : Number(end));
}

function readLength(value) {
  if (Array.isArray(value) || typeof value === "string") {
    return value.length;
  }
  return Object.keys(requireObject(value)).length;
}

function pickFields(value, fields) {
  const source = requireObject(value);
  return Object.fromEntries(fields.map((field) => [field, source[field] ?? null]));
}

function omitFields(value, fields) {
  const source = { ...requireObject(value) };
  for (const field of fields) {
    delete source[field];
  }
  return source;
}

function clampValue(value, min, max) {
  return Math.min(Math.max(value, min), max);
}
