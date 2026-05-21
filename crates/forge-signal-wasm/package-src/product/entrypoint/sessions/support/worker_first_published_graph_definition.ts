import { materializeWorkerCachedValue } from "./worker_cached_value.js";

export function normalizePublishedGraphSessionOptions(options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError("createWorkerFirstPublishedGraphSession(...) expects an options object");
  }
  const { definition, workerUrl, ...unknownOptions } = options;
  const unknownKeys = Object.keys(unknownOptions);
  if (unknownKeys.length > 0) {
    throw new TypeError(
      `createWorkerFirstPublishedGraphSession(...) does not support: ${unknownKeys.join(", ")}`,
    );
  }
  const normalizedDefinition = requirePublishedGraphDefinition(definition);
  return Object.freeze({
    definition: normalizedDefinition,
    inputDescriptorsByName: indexBy(normalizedDefinition.inputDescriptors, "inputName"),
    inputAuthoritiesByName: normalizedDefinition.operationalContract.authorities ?? Object.freeze({}),
    initialValuesBySourceId: buildInitialValuesBySourceId(normalizedDefinition),
    outputDescriptorsByName: indexBy(normalizedDefinition.descriptors, "outputName"),
    trackedInputIds: Object.freeze(normalizedDefinition.inputDescriptors.map((entry) => entry.sourceId)),
    trackedOutputIds: Object.freeze(normalizedDefinition.descriptors.map((entry) => entry.publishedId)),
    workerUrl: workerUrl === undefined ? null : workerUrl,
  });
}

function requirePublishedGraphDefinition(definition) {
  if (!definition || typeof definition !== "object" || Array.isArray(definition)) {
    throw new TypeError("createWorkerFirstPublishedGraphSession(...) requires an exported graph definition");
  }
  if (!definition.compatibility?.definitions) {
    throw new TypeError("createWorkerFirstPublishedGraphSession(...) requires definition.compatibility.definitions");
  }
  if (!Array.isArray(definition.inputDescriptors) || !Array.isArray(definition.descriptors)) {
    throw new TypeError("createWorkerFirstPublishedGraphSession(...) requires exported graph descriptors");
  }
  return definition;
}

function buildInitialValuesBySourceId(definition) {
  const map = new Map();
  for (const source of definition.compatibility.definitions.sources ?? []) {
    map.set(source.id, materializeWorkerCachedValue(source.initial));
  }
  return map;
}

function indexBy(entries, field) {
  const record = Object.create(null);
  for (const entry of entries) {
    record[entry[field]] = entry;
  }
  return Object.freeze(record);
}
