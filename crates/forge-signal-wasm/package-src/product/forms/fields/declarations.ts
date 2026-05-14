import { FormDeclarationError } from "../form_errors.js";
import { normalizeInputAdapter } from "../input_adapters.js";
import { fieldPathKey, parseFieldPath } from "../values/value_paths.js";

const FIELD_DECLARATION_BRAND = Symbol("forge.form.fieldDeclaration");

export function materializeFieldDeclarations(declaration) {
  const factory = createFieldDeclarationFactory();
  const declared =
    typeof declaration.fields === "function"
      ? declaration.fields({
          field: factory.field,
          repeated: factory.repeated,
          attachment: factory.attachment,
        })
      : declaration.fields;
  if (!declared || typeof declared !== "object" || Array.isArray(declared)) {
    throw new FormDeclarationError("signals.form(...) requires a fields object");
  }
  const seenIds = new Set();
  return Object.freeze(
    Object.entries(declared).map(([name, field]) => {
      if (!field || field[FIELD_DECLARATION_BRAND] !== true) {
        throw new FormDeclarationError("form fields must be declared with field(path)", {
          name,
        });
      }
      const id = field.options.id ?? name;
      if (seenIds.has(id)) {
        throw new FormDeclarationError("form field ids must be unique", { id });
      }
      seenIds.add(id);
      const accessibility = normalizeFieldAccessibility(field.options, name, id, field.path);
      return Object.freeze({
        name,
        id,
        family: field.family,
        path: field.path,
        segments: Object.freeze(field.segments),
        accessibility,
        layout: normalizeFieldLayout(field.options, id),
        collectionIdentity: field.collectionIdentity,
        attachment: field.attachment,
        inputAdapter: normalizeInputAdapter(field.options),
        parse: typeof field.options.parse === "function" ? field.options.parse : null,
      });
    }),
  );
}

function createFieldDeclarationFactory() {
  return {
    field(path, options = {}) {
      return fieldDeclaration("scalar", path, options);
    },
    repeated(path, options = {}) {
      return fieldDeclaration("repeated", path, options, {
        collectionIdentity: normalizeCollectionIdentity(options),
      });
    },
    attachment(path, options = {}) {
      return fieldDeclaration("attachment", path, options, {
        attachment: normalizeAttachmentDeclaration(options),
      });
    },
  };
}

function normalizeFieldAccessibility(options, fieldName, fieldId, fieldPath) {
  const declared = options.accessibility ?? {};
  if (declared == null || typeof declared !== "object" || Array.isArray(declared)) {
    throw new FormDeclarationError("form field accessibility metadata must be an object", {
      field: fieldId,
      accessibility: declared,
    });
  }
  const describedBy = options.describedBy ?? declared.describedBy ?? [];
  if (!Array.isArray(describedBy) || describedBy.some((entry) => typeof entry !== "string" || entry.length === 0)) {
    throw new FormDeclarationError("form field accessibility describedBy entries must be non-empty strings", {
      field: fieldId,
      describedBy,
    });
  }
  return Object.freeze({
    label: stringOrFallback(options.label ?? declared.label, humanizeFieldLabel(fieldName, fieldPath)),
    description: stringOrNull(options.description ?? declared.description),
    summaryLabel: stringOrFallback(options.summaryLabel ?? declared.summaryLabel, humanizeFieldLabel(fieldName, fieldPath)),
    describedBy: Object.freeze([...describedBy]),
    readingOrder: finiteOrder(options.readingOrder ?? declared.readingOrder, "readingOrder"),
    focusOrder: finiteOrder(options.focusOrder ?? declared.focusOrder, "focusOrder"),
    summaryOrder: finiteOrder(options.summaryOrder ?? declared.summaryOrder, "summaryOrder"),
  });
}

function humanizeFieldLabel(fieldName, fieldPath) {
  const source = typeof fieldName === "string" && fieldName.length > 0
    ? fieldName
    : String(fieldPath).split(".").at(-1) ?? String(fieldPath);
  return source
    .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
    .replace(/[_-]+/g, " ")
    .replace(/\s+/g, " ")
    .trim()
    .replace(/^./, (value) => value.toUpperCase());
}

function stringOrFallback(value, fallback) {
  if (value === undefined || value === null || value === "") {
    return fallback;
  }
  return String(value);
}

function stringOrNull(value) {
  if (value === undefined || value === null || value === "") {
    return null;
  }
  return String(value);
}

function finiteOrder(value, label) {
  if (value === undefined || value === null) {
    return null;
  }
  if (typeof value !== "number" || !Number.isFinite(value)) {
    throw new FormDeclarationError(`form field accessibility ${label} must be a finite number`, {
      value,
    });
  }
  return value;
}

function normalizeFieldLayout(options, fieldId) {
  const declared = options.layout ?? {};
  if (declared == null || typeof declared !== "object" || Array.isArray(declared)) {
    throw new FormDeclarationError("form field layout metadata must be an object", {
      field: fieldId,
      layout: declared,
    });
  }
  const responsive = options.responsive ?? declared.responsive ?? [];
  if (!Array.isArray(responsive) || responsive.some((entry) => typeof entry !== "string" || entry.length === 0)) {
    throw new FormDeclarationError("form field layout responsive entries must be non-empty strings", {
      field: fieldId,
      responsive,
    });
  }
  return Object.freeze({
    row: stringOrFallback(options.row ?? declared.row, fieldId),
    column: stringOrFallback(options.column ?? declared.column, fieldId),
    density: enumValue(options.density ?? declared.density, ["compact", "comfortable", "spacious"], "comfortable", "density"),
    alignment: enumValue(options.alignment ?? declared.alignment, ["start", "center", "stretch"], "stretch", "alignment"),
    minHeight: nonNegativeNumber(options.minHeight ?? declared.minHeight, "minHeight"),
    grow: booleanValue(options.grow ?? declared.grow, false),
    wrap: booleanValue(options.wrap ?? declared.wrap, false),
    responsive: Object.freeze([...responsive]),
  });
}

function enumValue(value, allowed, fallback, label) {
  if (value === undefined || value === null) {
    return fallback;
  }
  if (!allowed.includes(value)) {
    throw new FormDeclarationError(`form field layout ${label} is not supported`, {
      value,
    });
  }
  return value;
}

function nonNegativeNumber(value, label) {
  if (value === undefined || value === null) {
    return null;
  }
  if (typeof value !== "number" || !Number.isFinite(value) || value < 0) {
    throw new FormDeclarationError(`form field layout ${label} must be a non-negative finite number`, {
      value,
    });
  }
  return value;
}

function booleanValue(value, fallback) {
  if (value === undefined || value === null) {
    return fallback;
  }
  if (typeof value !== "boolean") {
    throw new FormDeclarationError("form field layout boolean posture must be a boolean", {
      value,
    });
  }
  return value;
}

function fieldDeclaration(family, path, options = {}, extras = {}) {
  requireOptionsObject(options);
  const segments = parseFieldPath(path);
  return Object.freeze({
    [FIELD_DECLARATION_BRAND]: true,
    family,
    path: fieldPathKey(segments),
    segments,
    options,
    collectionIdentity: extras.collectionIdentity ?? null,
    attachment: extras.attachment ?? null,
  });
}

function normalizeCollectionIdentity(options) {
  const identity = options.itemIdentity ?? options.key;
  if (identity === undefined) {
    throw new FormDeclarationError(
      "repeated form fields require an explicit itemIdentity or key",
    );
  }
  if (typeof identity !== "string" && typeof identity !== "function") {
    throw new FormDeclarationError(
      "repeated form fields require itemIdentity to be a string or function",
    );
  }
  return Object.freeze({
    kind: typeof identity === "function" ? "resolver" : "field",
    field: typeof identity === "string" ? identity : null,
    resolver: typeof identity === "function" ? identity : null,
    posture: "stableItemIdentityRequired",
  });
}

function normalizeAttachmentDeclaration(options) {
  const identity = options.attachmentIdentity ?? options.digest;
  if (identity === undefined) {
    throw new FormDeclarationError(
      "attachment form fields require an explicit attachmentIdentity or digest",
    );
  }
  if (typeof identity !== "string" && typeof identity !== "function") {
    throw new FormDeclarationError(
      "attachment form fields require attachmentIdentity to be a string or function",
    );
  }
  return Object.freeze({
    identityKind: typeof identity === "function" ? "resolver" : "field",
    identityField: typeof identity === "string" ? identity : null,
    identityResolver: typeof identity === "function" ? identity : null,
    metadata: Object.freeze({ ...(options.metadata ?? {}) }),
    posture: "fileBlobIdentityAndMetadataDeclared",
  });
}

function requireOptionsObject(options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new FormDeclarationError("form field options must be an object");
  }
}
