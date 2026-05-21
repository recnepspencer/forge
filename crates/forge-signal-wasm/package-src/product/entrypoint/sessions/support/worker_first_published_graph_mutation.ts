import {
  mergePatchValue,
} from "../../../graph_authoring_support.js";
import { cloneWorkerCachedValue } from "./worker_cached_value.js";

export function planWorkerFirstPublishedGraphMutation({
  definition,
  inputDescriptorsByName,
  inputAuthoritiesByName,
  initialValuesBySourceId,
  cachedInputs,
  mutation,
}) {
  if (!mutation || typeof mutation !== "object" || Array.isArray(mutation)) {
    throw new TypeError(
      `worker-first published graph ${definition.id} apply(...) expects a mutation object`,
    );
  }
  const { writes, patches, reset, commands, ...unknownMutationFields } = mutation;
  if (commands !== undefined && Object.keys(commands).length > 0) {
    throw new TypeError(
      `worker-first published graph ${definition.id} does not admit graph commands`,
    );
  }
  const unknownKeys = Object.keys(unknownMutationFields);
  if (unknownKeys.length > 0) {
    throw new TypeError(
      `worker-first published graph ${definition.id} apply(...) does not support: ${unknownKeys.join(", ")}`,
    );
  }
  const transactionOps = [];
  for (const [inputName, nextValue] of Object.entries(
    normalizeOperationRecord(writes, definition.id, "writes"),
  )) {
    const descriptor = requireKnownInputDescriptor(
      inputDescriptorsByName,
      definition.id,
      inputName,
    );
    requireOperationalAuthority(
      inputAuthoritiesByName,
      definition.id,
      inputName,
      "writeInputs(...)",
      "supportsWrite",
    );
    transactionOps.push({
      kind: "set",
      id: descriptor.sourceId,
      value: cloneWorkerCachedValue(nextValue),
    });
  }
  for (const [inputName, patchValue] of Object.entries(
    normalizeOperationRecord(patches, definition.id, "patches"),
  )) {
    const descriptor = requireKnownInputDescriptor(
      inputDescriptorsByName,
      definition.id,
      inputName,
    );
    requireOperationalAuthority(
      inputAuthoritiesByName,
      definition.id,
      inputName,
      "patchInputs(...)",
      "supportsPatch",
    );
    if (!cachedInputs.has(descriptor.sourceId)) {
      throw missingReadbackError(definition.id, "input", inputName);
    }
    transactionOps.push({
      kind: "set",
      id: descriptor.sourceId,
      value: mergePatchValue(
        definition.id,
        inputName,
        cachedInputs.get(descriptor.sourceId),
        patchValue,
      ),
    });
  }
  for (const inputName of normalizeResetList(reset, definition)) {
    const descriptor = requireKnownInputDescriptor(
      inputDescriptorsByName,
      definition.id,
      inputName,
    );
    requireOperationalAuthority(
      inputAuthoritiesByName,
      definition.id,
      inputName,
      "resetInputs(...)",
      "supportsReset",
    );
    if (!initialValuesBySourceId.has(descriptor.sourceId)) {
      throw new TypeError(
        `worker-first published graph ${definition.id} resetInputs(...) cannot recover initial value for public input \`${inputName}\``,
      );
    }
    transactionOps.push({
      kind: "set",
      id: descriptor.sourceId,
      value: cloneWorkerCachedValue(initialValuesBySourceId.get(descriptor.sourceId)),
    });
  }
  if (transactionOps.length === 0) {
    throw new TypeError(
      `worker-first published graph ${definition.id} apply(...) requires at least one write, patch, or reset`,
    );
  }
  return Object.freeze(transactionOps);
}

export function requireKnownInputDescriptor(inputDescriptorsByName, graphId, inputName) {
  const descriptor = inputDescriptorsByName[inputName];
  if (!descriptor) {
    throw new TypeError(
      `worker-first published graph ${graphId} does not expose public input \`${inputName}\``,
    );
  }
  return descriptor;
}

export function requireKnownOutputDescriptor(outputDescriptorsByName, graphId, outputName) {
  const descriptor = outputDescriptorsByName[outputName];
  if (!descriptor) {
    throw new TypeError(
      `worker-first published graph ${graphId} does not expose published output \`${outputName}\``,
    );
  }
  return descriptor;
}

export function missingReadbackError(graphId, family, name) {
  return new TypeError(
    `worker-first published graph ${graphId} has no cached ${family} readback for \`${name}\`; initialize the session or refresh worker truth first`,
  );
}

function normalizeOperationRecord(value, graphId, fieldName) {
  if (value === undefined) {
    return Object.create(null);
  }
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(
      `worker-first published graph ${graphId} ${fieldName} must be an object when provided`,
    );
  }
  return value;
}

function normalizeResetList(reset, definition) {
  if (reset === undefined) {
    return [];
  }
  if (!Array.isArray(reset) || !reset.every((entry) => typeof entry === "string" && entry.length > 0)) {
    throw new TypeError(
      `worker-first published graph ${definition.id} reset must be an array of public input names when provided`,
    );
  }
  return reset;
}

function requireOperationalAuthority(authorities, graphId, inputName, operation, capability) {
  const authority = authorities[inputName];
  if (!authority) {
    throw new TypeError(
      `worker-first published graph ${graphId} ${operation} cannot use unknown public input \`${inputName}\``,
    );
  }
  if (!authority[capability]) {
    throw new TypeError(
      `worker-first published graph ${graphId} ${operation} cannot mutate public input \`${inputName}\` because the operational contract denies ${capability}`,
    );
  }
  return authority;
}
