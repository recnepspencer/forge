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
      return Object.freeze({
        name,
        id,
        family: field.family,
        path: field.path,
        segments: Object.freeze(field.segments),
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
