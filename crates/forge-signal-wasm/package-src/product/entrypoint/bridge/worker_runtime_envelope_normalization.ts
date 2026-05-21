export function normalizeWorkerRuntimeDefinitionEnvelope(definitions) {
  return Object.freeze({
    ...definitions,
    recipes: Object.freeze(
      (definitions?.recipes ?? []).map((recipe) => Object.freeze({
        ...recipe,
        when: recipe?.when ?? null,
        producesAspects: recipe?.producesAspects ?? null,
      })),
    ),
  });
}

export function normalizeWorkerRuntimeEnvelope(envelope) {
  return Object.freeze({
    ...envelope,
    definitions: normalizeWorkerRuntimeDefinitionEnvelope(envelope?.definitions ?? {}),
  });
}
