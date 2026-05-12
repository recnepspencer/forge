import { FormDeclarationError } from "../form_errors.js";
import { normalizeInputAdapter } from "../input_adapters.js";
import { fieldPathKey, parseFieldPath } from "../values/value_paths.js";

const FIELD_DECLARATION_BRAND = Symbol("forge.form.fieldDeclaration");

export function materializeFieldDeclarations(declaration) {
  const factory = createFieldDeclarationFactory();
  const declared =
    typeof declaration.fields === "function"
      ? declaration.fields({ field: factory.field })
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
        path: field.path,
        segments: Object.freeze(field.segments),
        inputAdapter: normalizeInputAdapter(field.options),
        parse: typeof field.options.parse === "function" ? field.options.parse : null,
      });
    }),
  );
}

function createFieldDeclarationFactory() {
  return {
    field(path, options = {}) {
      const segments = parseFieldPath(path);
      return Object.freeze({
        [FIELD_DECLARATION_BRAND]: true,
        path: fieldPathKey(segments),
        segments,
        options,
      });
    },
  };
}
