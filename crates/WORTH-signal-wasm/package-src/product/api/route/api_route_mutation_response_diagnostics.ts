const MUTATION_RESPONSE_DIAGNOSTIC_KINDS = Object.freeze([
  "validation",
  "warnings",
]);

function lowerMutationResponseDiagnostics(route, semanticFinalizer, response, diagnostics) {
  if (diagnostics === undefined) {
    return Object.freeze([]);
  }
  if (!Array.isArray(diagnostics)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) diagnostics must be an array of declared diagnostic mappings`,
    );
  }
  if (semanticFinalizer !== "update" && diagnostics.length > 0) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) diagnostics are currently admitted only for update/save responses`,
    );
  }
  return Object.freeze(
    diagnostics.map((diagnostic, index) =>
      lowerMutationResponseDiagnostic(route, response, diagnostic, index)),
  );
}

function lowerMutationResponseDiagnostic(route, response, diagnostic, index) {
  if (!diagnostic || typeof diagnostic !== "object" || Array.isArray(diagnostic)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) diagnostics[${index}] must be a diagnostic declaration object`,
    );
  }
  if (!MUTATION_RESPONSE_DIAGNOSTIC_KINDS.includes(diagnostic.kind)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) diagnostics[${index}] kind must be validation or warnings`,
    );
  }
  if (response.kind !== "detail") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) diagnostics[${index}] requires a detail response lens`,
    );
  }
  if (typeof diagnostic.field !== "string" || diagnostic.field.length === 0) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) diagnostics[${index}] requires a non-empty response field`,
    );
  }
  const responseDefinition = response.fields?.definitions?.[diagnostic.field];
  if (responseDefinition === undefined) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) diagnostics[${index}] field "${diagnostic.field}" is not declared on the mutation response lens`,
    );
  }
  return Object.freeze({
    diagnosticId: `mutationDiagnostic${index + 1}`,
    kind: diagnostic.kind,
    field: diagnostic.field,
    extract(responseValue) {
      return responseDefinition.read(responseValue);
    },
  });
}

export { lowerMutationResponseDiagnostics };
