import { createMutationResponsePayloadDigest } from "./resource_mutation_response_payload_digest.js";

function createMutationResponseDiagnosticFacts(declarations, responseValue) {
  const entries = Object.freeze(
    declarations.map((declaration) =>
      createMutationResponseDiagnosticFact(declaration, responseValue)),
  );
  return Object.freeze({
    entries,
    count: entries.length,
    digest: createMutationResponseDiagnosticsDigest(entries),
  });
}

function createMutationResponseDiagnosticFact(declaration, responseValue) {
  const value = declaration.extract(responseValue);
  return Object.freeze({
    diagnosticId: declaration.diagnosticId,
    kind: declaration.kind,
    field: declaration.field,
    value,
    valueDigest: createMutationResponsePayloadDigest(value),
  });
}

function createMutationResponseDiagnosticsDigest(entries) {
  if (entries.length === 0) {
    return "mutation-response-diagnostics|none";
  }
  return `mutation-response-diagnostics|${entries.map((entry) =>
    `${entry.diagnosticId}:${entry.kind}:${entry.field}:${entry.valueDigest}`).join(",")}`;
}

export { createMutationResponseDiagnosticFacts };
