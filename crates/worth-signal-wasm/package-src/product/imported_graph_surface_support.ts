import { mergePatchValue, nullPrototypeRecord } from "./graph_authoring_support.js";
import { freezeObject } from "./graph_support.js";

export function buildImportedGraphOperationalContract(definition) {
  if (definition?.operationalContract) {
    return freezeObject(definition.operationalContract);
  }
  const emptyRecord = freezeObject(nullPrototypeRecord());
  return freezeObject({
    graph: freezeObject(definition.summary),
    writes: emptyRecord,
    patches: emptyRecord,
    commands: emptyRecord,
    authorities: emptyRecord,
    resettableInputNames: freezeObject([]),
  });
}

export function buildImportedGraphMutationContext(definition, snapshot) {
  const operationalContract = buildImportedGraphOperationalContract(definition);
  const inputDescriptorsByName = nullPrototypeRecord();
  for (const descriptor of definition.inputDescriptors) {
    inputDescriptorsByName[descriptor.inputName] = descriptor;
  }
  const initialValuesBySourceId = new Map(
    (snapshot?.snapshotEnvelope?.state?.sources ?? []).map((source) => [source?.id, source?.value]),
  );
  return freezeObject({
    operationalContract,
    inputDescriptorsByName: freezeObject(inputDescriptorsByName),
    inputAuthoritiesByName: freezeObject(
      operationalContract.authorities ?? nullPrototypeRecord(),
    ),
    initialValuesBySourceId,
    defaultResetInputNames: freezeObject(
      definition.inputDescriptors.map((descriptor) => descriptor.inputName),
    ),
  });
}

export function planImportedGraphMutation({
  label,
  graphId,
  inputDescriptorsByName,
  inputAuthoritiesByName,
  initialValuesBySourceId,
  readCurrentValue,
  mutation,
}) {
  if (!mutation || typeof mutation !== "object" || Array.isArray(mutation)) {
    throw new TypeError(`${label} apply(...) expects a mutation object`);
  }
  const { writes, patches, reset, commands, ...unknownFields } = mutation;
  if (commands !== undefined && Object.keys(commands).length > 0) {
    throw new TypeError(`${label} does not admit graph commands`);
  }
  const unknownKeys = Object.keys(unknownFields);
  if (unknownKeys.length > 0) {
    throw new TypeError(`${label} apply(...) does not support: ${unknownKeys.join(", ")}`);
  }
  const transactionOps = [];
  for (const [inputName, nextValue] of Object.entries(
    normalizeOperationRecord(writes, label, "writes"),
  )) {
    const descriptor = requireKnownInputDescriptor(inputDescriptorsByName, label, inputName);
    requireOperationalAuthority(
      inputAuthoritiesByName,
      label,
      inputName,
      "writeInputs(...)",
      "supportsWrite",
    );
    transactionOps.push({
      kind: "set",
      id: descriptor.sourceId,
      value: cloneImportedGraphValue(nextValue),
    });
  }
  for (const [inputName, patchValue] of Object.entries(
    normalizeOperationRecord(patches, label, "patches"),
  )) {
    const descriptor = requireKnownInputDescriptor(inputDescriptorsByName, label, inputName);
    requireOperationalAuthority(
      inputAuthoritiesByName,
      label,
      inputName,
      "patchInputs(...)",
      "supportsPatch",
    );
    transactionOps.push({
      kind: "set",
      id: descriptor.sourceId,
      value: mergePatchValue(
        graphId,
        inputName,
        cloneImportedGraphValue(readCurrentValue(descriptor.sourceId, inputName)),
        patchValue,
      ),
    });
  }
  for (const inputName of normalizeResetList(reset, label)) {
    const descriptor = requireKnownInputDescriptor(inputDescriptorsByName, label, inputName);
    requireOperationalAuthority(
      inputAuthoritiesByName,
      label,
      inputName,
      "resetInputs(...)",
      "supportsReset",
    );
    if (!initialValuesBySourceId.has(descriptor.sourceId)) {
      throw new TypeError(
        `${label} resetInputs(...) cannot recover initial value for public input \`${inputName}\``,
      );
    }
    transactionOps.push({
      kind: "set",
      id: descriptor.sourceId,
      value: cloneImportedGraphValue(initialValuesBySourceId.get(descriptor.sourceId)),
    });
  }
  if (transactionOps.length === 0) {
    throw new TypeError(
      `${label} apply(...) requires at least one write, patch, or reset`,
    );
  }
  return freezeObject(transactionOps);
}

export function buildImportedGraphSnapshotArtifact({
  definition,
  runtimeEnvelope,
  snapshotEnvelope,
  restoreMode = "SameRuntimeExact",
  contractHistory,
  importPosture,
}) {
  return freezeObject({
    id: definition.id,
    definition,
    runtimeEnvelope,
    snapshotEnvelope,
    restoreMode,
    contractHistory,
    importPosture,
  });
}

function normalizeOperationRecord(value, label, fieldName) {
  if (value === undefined) {
    return nullPrototypeRecord();
  }
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(`${label} ${fieldName} must be an object when provided`);
  }
  return value;
}

function normalizeResetList(reset, label) {
  if (reset === undefined) {
    return [];
  }
  if (!Array.isArray(reset) || !reset.every((name) => typeof name === "string" && name.length > 0)) {
    throw new TypeError(`${label} reset must be an array of public input names when provided`);
  }
  return reset;
}

function requireKnownInputDescriptor(inputDescriptorsByName, label, inputName) {
  const descriptor = inputDescriptorsByName[inputName];
  if (!descriptor) {
    throw new TypeError(`${label} does not expose public input \`${inputName}\``);
  }
  return descriptor;
}

function requireOperationalAuthority(authorities, label, inputName, operation, capability) {
  const authority = authorities[inputName];
  if (!authority) {
    throw new TypeError(`${label} ${operation} cannot use unknown public input \`${inputName}\``);
  }
  if (!authority[capability]) {
    throw new TypeError(
      `${label} ${operation} cannot mutate public input \`${inputName}\` because the operational contract denies ${capability}`,
    );
  }
  return authority;
}

function cloneImportedGraphValue(value) {
  if (typeof globalThis.structuredClone === "function") {
    try {
      return globalThis.structuredClone(value);
    } catch {
      // Fall through to shallow structural clone.
    }
  }
  if (Array.isArray(value)) {
    return value.map((entry) => cloneImportedGraphValue(entry));
  }
  if (value && typeof value === "object") {
    const clone = Object.create(Object.getPrototypeOf(value) ?? Object.prototype);
    for (const [key, entry] of Object.entries(value)) {
      clone[key] = cloneImportedGraphValue(entry);
    }
    return clone;
  }
  return value;
}
