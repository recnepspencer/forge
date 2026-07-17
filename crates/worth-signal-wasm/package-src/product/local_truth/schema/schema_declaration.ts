import {
  canonicalDigest,
  deepFreeze,
  immutableClone,
  isPlainRecord,
} from "../support/canonical.js";

const DECLARED_SCHEMAS = new WeakSet();
const VALUE_TYPES = new Set(["any", "boolean", "number", "string"]);
const EQUIVALENCE_KINDS = new Set(["exact", "numberEpsilon"]);
const COST_CLASSES = new Set(["constant", "linearInValue"]);

export function declareLocalTruthSchema(declaration) {
  requirePlainRecord(declaration, "local truth schema declaration");
  requireIdentifier(declaration.id, "schema id");
  if (!Array.isArray(declaration.aspects) || declaration.aspects.length === 0) {
    throw new TypeError("local truth schema requires at least one declared aspect");
  }
  const aspects = declaration.aspects.map(normalizeAspect);
  aspects.sort((left, right) => left.id.localeCompare(right.id));
  rejectDuplicates(aspects.map((aspect) => aspect.id), "aspect id");
  rejectDuplicates(aspects.map((aspect) => aspect.field), "aspect field");
  const identityInput = {
    id: declaration.id,
    version: declaration.version ?? 1,
    aspects,
  };
  const schema = deepFreeze({
    artifactFamily: "DeclaredLocalTruthSchema",
    authorityKind: "typescriptInMemoryLocalTruth",
    id: declaration.id,
    version: declaration.version ?? 1,
    identity: `local-truth-schema:${canonicalDigest(identityInput)}`,
    aspects,
  });
  DECLARED_SCHEMAS.add(schema);
  return schema;
}

export function requireDeclaredSchema(value, operation) {
  if (!value || !DECLARED_SCHEMAS.has(value)) {
    throw new TypeError(`${operation} requires a schema created by declareLocalTruthSchema(...)`);
  }
  return value;
}

export function restoreDeclaredSchema(serialized) {
  if (!serialized || serialized.artifactFamily !== "DeclaredLocalTruthSchema") {
    throw new TypeError("serialized local truth schema is invalid");
  }
  return declareLocalTruthSchema(serialized);
}

export function validateSchemaValue(schema, value, operation = "local truth value") {
  if (!isPlainRecord(value)) {
    throw new TypeError(`${operation} must be a top-level plain object`);
  }
  for (const aspect of schema.aspects) {
    validateDeclaredAspectValue(aspect, value[aspect.field], operation);
  }
  return immutableClone(value);
}

export function materializeAspect(schema, currentValue, aspectId, nextAspectValue) {
  const aspect = requireAspect(schema, aspectId);
  validateDeclaredAspectValue(aspect, nextAspectValue, `aspect ${aspectId}`);
  const nextValue = immutableClone({
    ...currentValue,
    [aspect.field]: immutableClone(nextAspectValue),
  });
  for (const other of schema.aspects) {
    if (
      other.id !== aspectId
      && canonicalDigest(nextValue[other.field]) !== canonicalDigest(currentValue[other.field])
    ) {
      throw new Error(`materializer for ${aspectId} changed undeclared field ${other.field}`);
    }
  }
  return nextValue;
}

export function extractAspect(schema, value, aspectId) {
  return value[requireAspect(schema, aspectId).field];
}

export function validateAspectCandidate(schema, aspectId, value, operation) {
  const aspect = requireAspect(schema, aspectId);
  validateDeclaredAspectValue(aspect, value, operation);
  return immutableClone(value);
}

export function aspectsEquivalent(schema, aspectId, left, right) {
  const { equivalence } = requireAspect(schema, aspectId);
  if (equivalence.kind === "exact") {
    return canonicalDigest(left) === canonicalDigest(right);
  }
  return typeof left === "number"
    && typeof right === "number"
    && Math.abs(left - right) <= equivalence.epsilon;
}

export function requireAspect(schema, aspectId) {
  const aspect = schema.aspects.find((candidate) => candidate.id === aspectId);
  if (!aspect) {
    throw new TypeError(`schema ${schema.id} does not declare aspect ${String(aspectId)}`);
  }
  return aspect;
}

function normalizeAspect(rawAspect, index) {
  requirePlainRecord(rawAspect, `aspect declaration at index ${index}`);
  requireIdentifier(rawAspect.id, `aspect id at index ${index}`);
  requireIdentifier(rawAspect.field, `aspect field at index ${index}`);
  const valueType = rawAspect.valueType ?? "any";
  if (!VALUE_TYPES.has(valueType)) {
    throw new TypeError(`aspect ${rawAspect.id} has unsupported valueType ${String(valueType)}`);
  }
  const equivalence = normalizeEquivalence(rawAspect.equivalence, rawAspect.id);
  const costClass = rawAspect.costClass ?? "constant";
  if (!COST_CLASSES.has(costClass)) {
    throw new TypeError(`aspect ${rawAspect.id} has unsupported costClass ${String(costClass)}`);
  }
  return deepFreeze({ id: rawAspect.id, field: rawAspect.field, valueType, equivalence, costClass });
}

function normalizeEquivalence(rawEquivalence, aspectId) {
  if (!rawEquivalence || typeof rawEquivalence !== "object") {
    throw new TypeError(`aspect ${aspectId} requires an explicit equivalence posture`);
  }
  const kind = rawEquivalence.kind;
  if (!EQUIVALENCE_KINDS.has(kind)) {
    throw new TypeError(`aspect ${aspectId} has unsupported equivalence ${String(kind)}`);
  }
  if (kind === "numberEpsilon") {
    if (typeof rawEquivalence.epsilon !== "number" || rawEquivalence.epsilon < 0) {
      throw new TypeError(`aspect ${aspectId} numberEpsilon requires a non-negative epsilon`);
    }
    return deepFreeze({ kind, epsilon: rawEquivalence.epsilon });
  }
  return deepFreeze({ kind });
}

function validateDeclaredAspectValue(aspect, value, operation) {
  if (aspect.valueType !== "any" && typeof value !== aspect.valueType) {
    throw new TypeError(`${operation} requires ${aspect.valueType} field ${aspect.field}`);
  }
  if (aspect.valueType === "number" && !Number.isFinite(value)) {
    throw new TypeError(`${operation} requires finite number field ${aspect.field}`);
  }
}

function requireIdentifier(value, label) {
  if (typeof value !== "string" || value.trim() === "") {
    throw new TypeError(`${label} must be a non-empty string`);
  }
}

function requirePlainRecord(value, label) {
  if (!isPlainRecord(value)) {
    throw new TypeError(`${label} must be a plain object`);
  }
}

function rejectDuplicates(values, label) {
  if (new Set(values).size !== values.length) {
    throw new TypeError(`local truth schema contains duplicate ${label}`);
  }
}
