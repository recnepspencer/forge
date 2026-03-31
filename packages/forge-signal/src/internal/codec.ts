const EXPR_KINDS = new Set([
  "value",
  "read",
  "get",
  "at",
  "first",
  "last",
  "slice",
  "join",
  "flatten",
  "object",
  "array",
  "sum",
  "multiply",
  "concat",
  "coalesce",
  "length",
  "contains",
  "mergeObjects",
  "keys",
  "values",
  "hasField",
  "pick",
  "omit",
  "append",
  "abs",
  "min",
  "max",
  "sqrt",
  "sin",
  "cos",
  "floor",
  "mod",
  "clamp",
  "atan2",
  "subtract",
  "divide",
  "eq",
  "neq",
  "gt",
  "gte",
  "lt",
  "lte",
  "and",
  "or",
  "not",
  "if",
]);

type SignalScalar = null | boolean | number | string;
type SignalValue =
  | SignalScalar
  | SignalValue[]
  | { [key: string]: SignalValue };

type EncodedSignalValue =
  | "null"
  | { bool: boolean }
  | { number: number }
  | { string: string }
  | { array: EncodedSignalValue[] }
  | { object: Array<[string, EncodedSignalValue]> };

type BuiltSpec<T = unknown> = T & { build?: () => T };

export function materializeSpec<T>(spec: BuiltSpec<T> | T): T {
  return spec && typeof spec === "object" && "build" in spec && typeof spec.build === "function"
    ? spec.build()
    : (spec as T);
}

export function encodeSignalValue(value: SignalValue): EncodedSignalValue {
  if (value === null) return "null";
  if (typeof value === "boolean") return { bool: value };
  if (typeof value === "number") return { number: value };
  if (typeof value === "string") return { string: value };
  if (Array.isArray(value)) {
    return { array: value.map(encodeSignalValue) };
  }
  if (typeof value === "object") {
    return {
      object: Object.entries(value).map(([key, entry]) => [key, encodeSignalValue(entry as SignalValue)]),
    };
  }
  throw new Error(`Unsupported signal value: ${String(value)}`);
}

export function decodeSignalValue(value: unknown): SignalValue {
  if (value === "null") return null;
  if (value && typeof value === "object") {
    if ("bool" in value) return (value as { bool: boolean }).bool;
    if ("number" in value) return (value as { number: number }).number;
    if ("string" in value) return (value as { string: string }).string;
    if ("array" in value) return (value as { array: unknown[] }).array.map(decodeSignalValue);
    if ("object" in value) {
      return Object.fromEntries(
        (value as { object: Array<[string, unknown]> }).object.map(([key, entry]) => [key, decodeSignalValue(entry)]),
      );
    }
  }
  return value as SignalValue;
}

export function normalizeExpr(input: any): any {
  if (!input || typeof input !== "object" || !("kind" in input)) {
    return { kind: "value", value: encodeSignalValue(input as SignalValue) };
  }
  if (!EXPR_KINDS.has(input.kind)) {
    throw new Error(`Unsupported expression kind: ${input.kind}`);
  }
  switch (input.kind) {
    case "value":
      return { kind: "value", value: encodeSignalValue(input.value as SignalValue) };
    case "read":
      return { kind: "read", id: input.id };
    case "get":
      return { kind: "get", target: normalizeExpr(input.target), field: input.field };
    case "at":
      return {
        kind: "at",
        target: normalizeExpr(input.target),
        index: normalizeExpr(input.index),
      };
    case "first":
    case "last":
    case "flatten":
    case "length":
    case "keys":
    case "values":
    case "abs":
    case "sqrt":
    case "sin":
    case "cos":
    case "floor":
      return { kind: input.kind, target: normalizeExpr(input.target) };
    case "slice":
      return {
        kind: "slice",
        target: normalizeExpr(input.target),
        start: normalizeExpr(input.start),
        end: input.end === undefined ? undefined : normalizeExpr(input.end),
      };
    case "join":
      return {
        kind: "join",
        target: normalizeExpr(input.target),
        separator: normalizeExpr(input.separator),
      };
    case "object":
      return {
        kind: "object",
        fields: normalizeObjectFields(input.fields),
      };
    case "array":
      return { kind: "array", items: input.items.map(normalizeExpr) };
    case "sum":
    case "multiply":
    case "concat":
    case "coalesce":
    case "mergeObjects":
    case "and":
    case "or":
    case "min":
    case "max":
      return { kind: input.kind, args: input.args.map(normalizeExpr) };
    case "not":
      return { kind: "not", arg: normalizeExpr(input.arg) };
    case "contains":
      return {
        kind: "contains",
        target: normalizeExpr(input.target),
        value: normalizeExpr(input.value),
      };
    case "hasField":
      return {
        kind: "hasField",
        target: normalizeExpr(input.target),
        field: input.field,
      };
    case "pick":
    case "omit":
      return {
        kind: input.kind,
        target: normalizeExpr(input.target),
        fields: [...input.fields],
      };
    case "append":
      return {
        kind: "append",
        target: normalizeExpr(input.target),
        value: normalizeExpr(input.value),
      };
    case "subtract":
    case "divide":
    case "eq":
    case "neq":
    case "gt":
    case "gte":
    case "lt":
    case "lte":
    case "mod":
      return {
        kind: input.kind,
        left: normalizeExpr(input.left),
        right: normalizeExpr(input.right),
      };
    case "clamp":
      return {
        kind: "clamp",
        value: normalizeExpr(input.value),
        min: normalizeExpr(input.min),
        max: normalizeExpr(input.max),
      };
    case "atan2":
      return {
        kind: "atan2",
        y: normalizeExpr(input.y),
        x: normalizeExpr(input.x),
      };
    case "if":
      return {
        kind: "if",
        condition: normalizeExpr(input.condition),
        thenExpr: normalizeExpr(input.thenExpr),
        elseExpr: normalizeExpr(input.elseExpr),
      };
    default:
      return input;
  }
}

function normalizeObjectFields(fields: Array<[string, any]> | Record<string, any>) {
  if (Array.isArray(fields)) {
    return fields.map(([key, value]) => [key, normalizeExpr(value)]);
  }
  return Object.entries(fields).map(([key, value]) => [key, normalizeExpr(value)]);
}

function normalizeCondition(condition: any) {
  return condition ? { expr: normalizeExpr(condition.expr) } : undefined;
}

function normalizeIdentity(identity: any) {
  if (!identity) return undefined;
  if (identity.kind === "exact") return { kind: "exact" };
  return { kind: "expr", expr: normalizeExpr(identity.expr) };
}

export function normalizeSourceSpec(spec: any) {
  const normalized = materializeSpec(spec);
  return {
    id: normalized.id,
    initial: encodeSignalValue((normalized.initial ?? null) as SignalValue),
  };
}

export function normalizeRecipeSpec(spec: any) {
  const normalized = materializeSpec(spec);
  return {
    id: normalized.id,
    reads: normalized.reads ?? [],
    expr: normalizeExpr(normalized.expr),
    when: normalizeCondition(normalized.when),
    identity: normalizeIdentity(normalized.identity),
  };
}

export function normalizeSourceFamilySpec(spec: any) {
  const normalized = materializeSpec(spec);
  return {
    familyId: normalized.familyId,
    initial: encodeSignalValue((normalized.initial ?? null) as SignalValue),
  };
}

export function normalizeRecipeFamilySpec(spec: any) {
  const normalized = materializeSpec(spec);
  return {
    familyId: normalized.familyId,
    reads: (normalized.reads ?? []).map(normalizeRecipeFamilyRead),
    expr: normalizeExpr(normalized.expr),
    when: normalizeCondition(normalized.when),
    identity: normalizeIdentity(normalized.identity),
  };
}

function normalizeRecipeFamilyRead(read: any) {
  if (!read || typeof read !== "object") {
    throw new Error("Recipe family reads must be declared explicitly.");
  }
  if (read.kind === "signal" && typeof read.id === "string") {
    return { kind: "signal", id: read.id };
  }
  if (read.kind === "keyed" && typeof read.familyId === "string") {
    return { kind: "keyed", familyId: read.familyId };
  }
  throw new Error("Invalid recipe family read.");
}

export function normalizeTransactionOp(op: any) {
  if (op.kind === "set") {
    return { kind: "set", id: op.id, value: encodeSignalValue(op.value as SignalValue) };
  }
  if (op.kind === "setPackedGridRgba") {
    return {
      kind: "setPackedGridRgba",
      familyId: op.familyId,
      width: op.width,
      height: op.height,
      rgba: op.rgba,
    };
  }
  if (op.kind === "setManyKeyed") {
    return {
      kind: "setManyKeyed",
      familyId: op.familyId,
      values: op.values.map(({ key, value }: { key: string; value: SignalValue }) => ({
        key,
        value: encodeSignalValue(value),
      })),
    };
  }
  return {
    kind: "setMany",
    values: op.values.map(({ id, value }: { id: string; value: SignalValue }) => ({
      id,
      value: encodeSignalValue(value),
    })),
  };
}

function decodeStoredState(state: any) {
  return {
    sources: state.sources.map((source: any) => ({
      ...source,
      value: decodeSignalValue(source.value),
    })),
    recipes: state.recipes.map((recipe: any) => ({
      ...recipe,
      value: decodeSignalValue(recipe.value),
    })),
  };
}

export function decodeDefinitions(definitions: any) {
  return {
    ...definitions,
    sources: definitions.sources.map((source: any) => ({
      ...source,
      initial: decodeSignalValue(source.initial),
    })),
  };
}

export function decodeRuntimeEnvelope(envelope: any) {
  return {
    ...envelope,
    definitions: decodeDefinitions(envelope.definitions),
    snapshot: {
      ...envelope.snapshot,
      state: decodeStoredState(envelope.snapshot.state),
    },
  };
}

export function decodeSnapshotEnvelope(snapshot: any) {
  return {
    ...decodeBigInts(snapshot),
    state: decodeStoredState(snapshot.state),
  };
}

export function normalizeRuntimeEnvelope(envelope: any) {
  return {
    definitions: {
      ...envelope.definitions,
      sources: (envelope.definitions.sources ?? []).map(normalizeSourceSpec),
      recipes: (envelope.definitions.recipes ?? []).map(normalizeRecipeSpec),
      sourceFamilies: (envelope.definitions.sourceFamilies ?? []).map(normalizeSourceFamilySpec),
      recipeFamilies: (envelope.definitions.recipeFamilies ?? []).map(normalizeRecipeFamilySpec),
    },
    snapshot: {
      ...envelope.snapshot,
      state: {
        ...envelope.snapshot.state,
        sources: envelope.snapshot.state.sources.map((source: any) => ({
          ...source,
          value: encodeSignalValue(source.value as SignalValue),
        })),
        recipes: envelope.snapshot.state.recipes.map((recipe: any) => ({
          ...recipe,
          value: encodeSignalValue(recipe.value as SignalValue),
        })),
      },
    },
  };
}

function decodeBigInts(value: any): any {
  if (typeof value === "bigint") {
    const asNumber = Number(value);
    if (!Number.isSafeInteger(asNumber)) {
      throw new Error(`BigInt ${value.toString()} exceeds JS safe integer range`);
    }
    return asNumber;
  }
  if (Array.isArray(value)) {
    return value.map(decodeBigInts);
  }
  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value).map(([key, entry]) => [key, decodeBigInts(entry)]),
    );
  }
  return value;
}

function normalizeBigInts(value: any): any {
  if (typeof value === "bigint") {
    const asNumber = Number(value);
    if (!Number.isSafeInteger(asNumber)) {
      throw new Error(`BigInt ${value.toString()} exceeds JS safe integer range`);
    }
    return asNumber;
  }
  if (Array.isArray(value)) {
    return value.map(normalizeBigInts);
  }
  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value).map(([key, entry]) => [key, normalizeBigInts(entry)]),
    );
  }
  return value;
}

export function normalizeSnapshotEnvelope(snapshot: any) {
  return normalizeBigInts({
    ...snapshot,
    state: {
      ...snapshot.state,
      sources: snapshot.state.sources.map((source: any) => ({
        ...source,
        value: encodeSignalValue(source.value as SignalValue),
      })),
      recipes: snapshot.state.recipes.map((recipe: any) => ({
        ...recipe,
        value: encodeSignalValue(recipe.value as SignalValue),
      })),
    },
  });
}

export function compositeKeyedId(familyId: string, key: string) {
  return `${familyId}::${key}`;
}
