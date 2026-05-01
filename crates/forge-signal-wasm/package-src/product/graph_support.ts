import { wrapReadableSignal } from "./handles.js";
import { INPUT_BASELINE_VALUE } from "./symbols.js";

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function extractReadTargets(reads) {
  if (!Array.isArray(reads)) {
    return {
      signalIds: [],
      keyedFamilyIds: [],
    };
  }
  const signalIds = [];
  const keyedFamilyIds = [];
  for (const read of reads) {
    if (typeof read === "string" && read.length > 0) {
      signalIds.push(read);
      continue;
    }
    if (!read || typeof read !== "object") {
      continue;
    }
    if (read.kind === "keyed" && typeof read.familyId === "string" && read.familyId.length > 0) {
      keyedFamilyIds.push(read.familyId);
    }
    if (typeof read.id === "string" && read.id.length > 0) {
      signalIds.push(read.id);
    }
  }
  return {
    signalIds,
    keyedFamilyIds,
  };
}

export function freezeObject(object) {
  return Object.freeze(object);
}

function nullPrototypeRecord() {
  return Object.create(null);
}

function lastPathSegment(id) {
  if (typeof id !== "string" || id.length === 0) {
    return null;
  }
  const lastSeparator = id.lastIndexOf(".");
  return lastSeparator >= 0 ? id.slice(lastSeparator + 1) : id;
}

function buildUniqueAliasMap(entries) {
  const aliases = new Map();
  const ambiguous = new Set();
  for (const [alias, value] of entries) {
    if (typeof alias !== "string" || alias.length === 0) {
      continue;
    }
    if (ambiguous.has(alias)) {
      continue;
    }
    if (aliases.has(alias) && aliases.get(alias) !== value) {
      aliases.delete(alias);
      ambiguous.add(alias);
      continue;
    }
    aliases.set(alias, value);
  }
  return aliases;
}

function isPatchableSignalValue(value) {
  return isPlainObject(value);
}

export function buildCompatibilityDefinition(
  graphId,
  graphSummary,
  inputDescriptors,
  outputDescriptors,
  definitionEnvelope,
) {
  const sourceIds = new Set();
  const recipeIds = new Set();
  const sourceFamilyIds = new Set();
  const recipeFamilyIds = new Set();
  const unavailableCallbackIds = new Set();
  const sourceById = new Map(
    (definitionEnvelope?.sources ?? []).map((source) => [source.id, source]),
  );
  const recipeById = new Map(
    (definitionEnvelope?.recipes ?? []).map((recipe) => [recipe.id, recipe]),
  );
  const sourceFamilyById = new Map(
    (definitionEnvelope?.sourceFamilies ?? []).map((family) => [family.familyId, family]),
  );
  const recipeFamilyById = new Map(
    (definitionEnvelope?.recipeFamilies ?? []).map((family) => [family.familyId, family]),
  );
  const unavailableById = new Map(
    (definitionEnvelope?.unavailableCallbacks ?? []).map((artifact) => [artifact.id, artifact]),
  );
  const pendingSignalIds = [...graphSummary.publishedOutputIds, ...graphSummary.inputSourceIds];
  const pendingFamilyIds = [];
  const visitedSignalIds = new Set();
  const visitedFamilyIds = new Set();

  while (pendingSignalIds.length > 0 || pendingFamilyIds.length > 0) {
    const nextSignalId = pendingSignalIds.pop();
    if (typeof nextSignalId === "string" && nextSignalId.length > 0 && !visitedSignalIds.has(nextSignalId)) {
      visitedSignalIds.add(nextSignalId);

      const source = sourceById.get(nextSignalId);
      if (source) {
        sourceIds.add(nextSignalId);
      }

      const recipe = recipeById.get(nextSignalId);
      if (recipe) {
        recipeIds.add(nextSignalId);
        const targets = extractReadTargets(recipe.reads);
        pendingSignalIds.push(...targets.signalIds);
        pendingFamilyIds.push(...targets.keyedFamilyIds);
      }

      const unavailableCallback = unavailableById.get(nextSignalId);
      if (unavailableCallback) {
        unavailableCallbackIds.add(nextSignalId);
        pendingSignalIds.push(...(unavailableCallback.currentReads ?? []));
      }
    }

    const nextFamilyId = pendingFamilyIds.pop();
    if (typeof nextFamilyId !== "string" || nextFamilyId.length === 0 || visitedFamilyIds.has(nextFamilyId)) {
      continue;
    }
    visitedFamilyIds.add(nextFamilyId);

    const sourceFamily = sourceFamilyById.get(nextFamilyId);
    if (sourceFamily) {
      sourceFamilyIds.add(nextFamilyId);
    }

    const recipeFamily = recipeFamilyById.get(nextFamilyId);
    if (recipeFamily) {
      recipeFamilyIds.add(nextFamilyId);
      const targets = extractReadTargets(recipeFamily.reads);
      pendingSignalIds.push(...targets.signalIds);
      pendingFamilyIds.push(...targets.keyedFamilyIds);
    }
  }

  const publishedInputs = Object.create(null);
  const publishedOutputs = Object.create(null);
  for (const descriptor of inputDescriptors) {
    publishedInputs[descriptor.inputName] = descriptor.sourceId;
  }
  for (const descriptor of outputDescriptors) {
    publishedOutputs[descriptor.outputName] = descriptor.publishedId;
  }

  const filteredDefinitions = freezeObject({
    policy: definitionEnvelope?.policy ?? null,
    sources: freezeObject(
      (definitionEnvelope?.sources ?? []).filter((source) => sourceIds.has(source.id)),
    ),
    recipes: freezeObject(
      (definitionEnvelope?.recipes ?? []).filter((recipe) => recipeIds.has(recipe.id)),
    ),
    sourceFamilies: freezeObject(
      (definitionEnvelope?.sourceFamilies ?? []).filter((family) => sourceFamilyIds.has(family.familyId)),
    ),
    recipeFamilies: freezeObject(
      (definitionEnvelope?.recipeFamilies ?? []).filter((family) => recipeFamilyIds.has(family.familyId)),
    ),
    unavailableCallbacks: freezeObject(
      (definitionEnvelope?.unavailableCallbacks ?? []).filter((artifact) => unavailableCallbackIds.has(artifact.id)),
    ),
  });
  const contract = buildGraphContractSurface(graphSummary, inputDescriptors, outputDescriptors);

  return freezeObject({
    id: graphId,
    contract,
    inputs: freezeObject(publishedInputs),
    outputs: freezeObject(publishedOutputs),
    inputSourceIds: graphSummary.inputSourceIds,
    publishedOutputIds: graphSummary.publishedOutputIds,
    sourceIds: graphSummary.sourceIds,
    inputDescriptors,
    descriptors: outputDescriptors,
    definitions: filteredDefinitions,
  });
}

function buildOutputVersionMap(outputDescriptors, versions) {
  const versionById = new Map((versions ?? []).map((entry) => [entry.id, entry]));
  const record = nullPrototypeRecord();
  for (const outputName of outputDescriptors) {
    const version = versionById.get(outputName.publishedId);
    if (version) {
      record[outputName.outputName] = version;
    }
  }
  return freezeObject(record);
}

function buildInputVersionMap(inputDescriptors, versions) {
  const versionById = new Map((versions ?? []).map((entry) => [entry.id, entry]));
  const record = nullPrototypeRecord();
  for (const inputName of inputDescriptors) {
    const version = versionById.get(inputName.sourceId);
    if (version) {
      record[inputName.inputName] = version;
    }
  }
  return freezeObject(record);
}

export function buildGraphContractSurface(graphSummary, inputDescriptors, outputDescriptors) {
  const publishedInputs = nullPrototypeRecord();
  const publishedOutputs = nullPrototypeRecord();
  for (const descriptor of inputDescriptors) {
    publishedInputs[descriptor.inputName] = descriptor.sourceId;
  }
  for (const descriptor of outputDescriptors) {
    publishedOutputs[descriptor.outputName] = descriptor.publishedId;
  }
  return freezeObject({
    graph: graphSummary,
    inputs: freezeObject(publishedInputs),
    outputs: freezeObject(publishedOutputs),
    inputDescriptors,
    descriptors: outputDescriptors,
  });
}

export function buildGraphOperationalContractSurface(graphSummary, inputDescriptors, inputs) {
  const writes = nullPrototypeRecord();
  const patches = nullPrototypeRecord();
  const authorities = nullPrototypeRecord();
  const resettableInputNames = [];

  for (const descriptor of inputDescriptors) {
    const input = inputs[descriptor.inputName];
    const baselineValue = input[INPUT_BASELINE_VALUE];
    const authority = descriptor.authority;
    const writable = authority === "writable";
    const patchable = writable && isPatchableSignalValue(baselineValue);
    if (writable) {
      writes[descriptor.inputName] = descriptor.sourceId;
    }
    if (patchable) {
      patches[descriptor.inputName] = descriptor.sourceId;
    }
    if (writable) {
      resettableInputNames.push(descriptor.inputName);
    }
    authorities[descriptor.inputName] = freezeObject({
      inputName: descriptor.inputName,
      sourceId: descriptor.sourceId,
      authority,
      supportsWrite: writable,
      supportsPatch: patchable,
      supportsReset: writable,
    });
  }

  return freezeObject({
    graph: graphSummary,
    writes: freezeObject(writes),
    patches: freezeObject(patches),
    commands: freezeObject(nullPrototypeRecord()),
    authorities: freezeObject(authorities),
    resettableInputNames: freezeObject(resettableInputNames),
  });
}

export function requireKnownAuthority(authorities, graphId, inputName, operation, capability) {
  const authority = authorities[inputName];
  if (!authority) {
    throw new TypeError(
      `signals.graph \`${graphId}\` ${operation} cannot use unknown public input \`${inputName}\``,
    );
  }
  if (!authority[capability]) {
    throw new TypeError(
      `signals.graph \`${graphId}\` ${operation} cannot ${capability === "supportsWrite" ? "write" : capability === "supportsPatch" ? "patch" : "reset"} public input \`${inputName}\` because its authority is \`${authority.authority}\``,
    );
  }
  return authority;
}

export function buildGraphDependencyExplanationRecord(graphSummary, inputDescriptors, outputDescriptors, definitionEnvelope) {
  const inputNameBySourceId = new Map(inputDescriptors.map((descriptor) => [descriptor.sourceId, descriptor.inputName]));
  const inputSourceIdByAlias = buildUniqueAliasMap(
    inputDescriptors.flatMap((descriptor) => {
      const alias = lastPathSegment(descriptor.sourceId);
      return alias ? [[alias, descriptor.sourceId]] : [];
    }),
  );
  const recipeById = new Map(
    (definitionEnvelope?.recipes ?? []).map((recipe) => [recipe.id, recipe]),
  );
  const unavailableById = new Map(
    (definitionEnvelope?.unavailableCallbacks ?? []).map((artifact) => [artifact.id, artifact]),
  );
  const recipeFamilyById = new Map(
    (definitionEnvelope?.recipeFamilies ?? []).map((family) => [family.familyId, family]),
  );
  const recipeIdByAlias = buildUniqueAliasMap(
    [...recipeById.keys()].flatMap((id) => {
      const alias = lastPathSegment(id);
      return alias ? [[alias, id]] : [];
    }),
  );
  const unavailableIdByAlias = buildUniqueAliasMap(
    [...unavailableById.keys()].flatMap((id) => {
      const alias = lastPathSegment(id);
      return alias ? [[alias, id]] : [];
    }),
  );

  function resolvePublicInput(signalId) {
    if (inputNameBySourceId.has(signalId)) {
      return {
        inputName: inputNameBySourceId.get(signalId),
        sourceId: signalId,
      };
    }
    const canonicalSourceId = inputSourceIdByAlias.get(signalId);
    if (!canonicalSourceId) {
      return null;
    }
    return {
      inputName: inputNameBySourceId.get(canonicalSourceId),
      sourceId: canonicalSourceId,
    };
  }

  function resolveRecipe(signalId) {
    return recipeById.get(signalId) ?? recipeById.get(recipeIdByAlias.get(signalId));
  }

  function resolveUnavailable(signalId) {
    return unavailableById.get(signalId) ?? unavailableById.get(unavailableIdByAlias.get(signalId));
  }

  const dependencies = nullPrototypeRecord();
  for (const descriptor of outputDescriptors) {
    const publicInputSourceIdSet = new Set();
    const transitiveSignalIds = [];
    const pendingSignalIds = [descriptor.publishedId];
    const pendingFamilyIds = [];
    const seenSignalIds = new Set();
    const seenFamilyIds = new Set();
    let signalCursor = 0;
    let familyCursor = 0;

    while (signalCursor < pendingSignalIds.length || familyCursor < pendingFamilyIds.length) {
      const nextSignalId = signalCursor < pendingSignalIds.length
        ? pendingSignalIds[signalCursor++]
        : undefined;
      if (typeof nextSignalId === "string" && nextSignalId.length > 0 && !seenSignalIds.has(nextSignalId)) {
        seenSignalIds.add(nextSignalId);
        transitiveSignalIds.push(nextSignalId);

        const publicInput = resolvePublicInput(nextSignalId);
        if (publicInput) {
          publicInputSourceIdSet.add(publicInput.sourceId);
        }

        const recipe = resolveRecipe(nextSignalId);
        if (recipe) {
          const targets = extractReadTargets(recipe.reads);
          pendingSignalIds.push(...targets.signalIds);
          pendingFamilyIds.push(...targets.keyedFamilyIds);
        }

        const unavailableCallback = resolveUnavailable(nextSignalId);
        if (unavailableCallback) {
          pendingSignalIds.push(...(unavailableCallback.currentReads ?? []));
        }
      }

      const nextFamilyId = familyCursor < pendingFamilyIds.length
        ? pendingFamilyIds[familyCursor++]
        : undefined;
      if (typeof nextFamilyId !== "string" || nextFamilyId.length === 0 || seenFamilyIds.has(nextFamilyId)) {
        continue;
      }
      seenFamilyIds.add(nextFamilyId);
      const recipeFamily = recipeFamilyById.get(nextFamilyId);
      if (recipeFamily) {
        const targets = extractReadTargets(recipeFamily.reads);
        pendingSignalIds.push(...targets.signalIds);
        pendingFamilyIds.push(...targets.keyedFamilyIds);
      }
    }

    const orderedInputDescriptors = inputDescriptors.filter((inputDescriptor) => (
      publicInputSourceIdSet.has(inputDescriptor.sourceId)
    ));
    const publicInputNames = freezeObject(
      orderedInputDescriptors.map((inputDescriptor) => inputDescriptor.inputName),
    );
    const publicInputSourceIds = freezeObject(
      orderedInputDescriptors.map((inputDescriptor) => inputDescriptor.sourceId),
    );

    dependencies[descriptor.outputName] = freezeObject({
      graphId: graphSummary.id,
      outputName: descriptor.outputName,
      publishedId: descriptor.publishedId,
      sourceId: descriptor.sourceId,
      publicInputNames,
      publicInputSourceIds,
      transitiveSignalIds: freezeObject(transitiveSignalIds),
    });
  }
  return freezeObject(dependencies);
}

export function buildGraphContractSummary(graphSummary, contract, dependencyRecord) {
  return freezeObject({
    graph: graphSummary,
    contract,
    inputCount: graphSummary.inputCount,
    outputCount: graphSummary.outputCount,
    inputNames: graphSummary.inputNames,
    outputNames: graphSummary.outputNames,
    dependencies: dependencyRecord,
  });
}

function requireContractSurface(previousContract, graphId) {
  if (!isPlainObject(previousContract)) {
    throw new TypeError(
      `signals.graph \`${graphId}\` contractDelta(...) requires a previously captured graph contract surface`,
    );
  }
  if (!isPlainObject(previousContract.inputs) || !isPlainObject(previousContract.outputs)) {
    throw new TypeError(
      `signals.graph \`${graphId}\` contractDelta(...) requires a contract surface with inputs and outputs records`,
    );
  }
  return previousContract;
}

function compareContractEntries(previousEntries, currentEntries) {
  const added = [];
  const removed = [];
  const remapped = [];

  for (const [name, currentId] of Object.entries(currentEntries)) {
    if (!(name in previousEntries)) {
      added.push(name);
      continue;
    }
    if (previousEntries[name] !== currentId) {
      remapped.push(freezeObject({
        name,
        previousId: previousEntries[name],
        currentId,
      }));
    }
  }

  for (const name of Object.keys(previousEntries)) {
    if (!(name in currentEntries)) {
      removed.push(name);
    }
  }

  return freezeObject({
    added: freezeObject(added),
    removed: freezeObject(removed),
    remapped: freezeObject(remapped),
  });
}

function descriptorRecordByName(descriptors, nameField) {
  const record = new Map();
  for (const descriptor of descriptors ?? []) {
    const name = descriptor?.[nameField];
    if (typeof name === "string" && name.length > 0) {
      record.set(name, descriptor);
    }
  }
  return record;
}

function compareInputDescriptors(previousDescriptors, currentDescriptors) {
  const previousByName = descriptorRecordByName(previousDescriptors, "inputName");
  const changes = [];
  for (const descriptor of currentDescriptors ?? []) {
    const previous = previousByName.get(descriptor.inputName);
    if (!previous) {
      continue;
    }
    if (
      previous.sourceId !== descriptor.sourceId
      || previous.authority !== descriptor.authority
    ) {
      changes.push(freezeObject({
        inputName: descriptor.inputName,
        previousSourceId: previous.sourceId,
        currentSourceId: descriptor.sourceId,
        previousAuthority: previous.authority,
        currentAuthority: descriptor.authority,
      }));
    }
  }
  return freezeObject(changes);
}

function compareOutputDescriptors(previousDescriptors, currentDescriptors) {
  const previousByName = descriptorRecordByName(previousDescriptors, "outputName");
  const changes = [];
  for (const descriptor of currentDescriptors ?? []) {
    const previous = previousByName.get(descriptor.outputName);
    if (!previous) {
      continue;
    }
    if (
      previous.sourceId !== descriptor.sourceId
      || previous.publishedId !== descriptor.publishedId
      || previous.sourceKind !== descriptor.sourceKind
      || previous.publicationKind !== descriptor.publicationKind
    ) {
      changes.push(freezeObject({
        outputName: descriptor.outputName,
        previousSourceId: previous.sourceId,
        currentSourceId: descriptor.sourceId,
        previousPublishedId: previous.publishedId,
        currentPublishedId: descriptor.publishedId,
        previousSourceKind: previous.sourceKind,
        currentSourceKind: descriptor.sourceKind,
        previousPublicationKind: previous.publicationKind,
        currentPublicationKind: descriptor.publicationKind,
      }));
    }
  }
  return freezeObject(changes);
}

export function buildGraphContractDelta(graphContract, previousContract) {
  const validatedPreviousContract = requireContractSurface(previousContract, graphContract.graph.id);
  const inputs = compareContractEntries(validatedPreviousContract.inputs, graphContract.inputs);
  const outputs = compareContractEntries(validatedPreviousContract.outputs, graphContract.outputs);
  const inputDescriptorsChanged = compareInputDescriptors(
    validatedPreviousContract.inputDescriptors,
    graphContract.inputDescriptors,
  );
  const outputDescriptorsChanged = compareOutputDescriptors(
    validatedPreviousContract.descriptors,
    graphContract.descriptors,
  );
  const previousGraphId = typeof validatedPreviousContract.graph?.id === "string"
    ? validatedPreviousContract.graph.id
    : null;

  return freezeObject({
    graphId: graphContract.graph.id,
    previousGraphId,
    changed: (
      inputs.added.length > 0
      || inputs.removed.length > 0
      || inputs.remapped.length > 0
      || outputs.added.length > 0
      || outputs.removed.length > 0
      || outputs.remapped.length > 0
      || inputDescriptorsChanged.length > 0
      || outputDescriptorsChanged.length > 0
    ),
    inputs,
    outputs,
    inputDescriptorsChanged,
    outputDescriptorsChanged,
  });
}

export function buildGraphContractHistory(graphContract, baselineContract = null, restoreMode = "LiveRuntime") {
  const normalizedBaselineContract = baselineContract ? requireContractSurface(baselineContract, graphContract.graph.id) : null;
  const deltas = normalizedBaselineContract
    ? freezeObject([buildGraphContractDelta(graphContract, normalizedBaselineContract)])
    : freezeObject([]);

  return freezeObject({
    graphId: graphContract.graph.id,
    current: graphContract,
    baseline: normalizedBaselineContract,
    deltas,
    changedSinceBaseline: deltas.some((delta) => delta.changed),
    restoreMode,
    importedFromGraphId: normalizedBaselineContract?.graph?.id ?? null,
  });
}

export function buildGraphImportPosture(graphId, restoreMode = "SameRuntimeExact") {
  return freezeObject({
    graphId,
    exactRestoreMode: restoreMode,
    portableImport: "Denied",
    portableImportReason: "graph-native import currently requires the exact originating runtime envelope",
    hydrate: "Deferred",
    hydrateReason: "graph-native portable hydrate is not yet admitted on this surface",
  });
}

export function buildGraphExportDefinition(
  graphId,
  graphSummary,
  graphContract,
  graphOperationalContract,
  inputDescriptors,
  outputDescriptors,
  definitionEnvelope,
) {
  const compatibility = buildCompatibilityDefinition(
    graphId,
    graphSummary,
    inputDescriptors,
    outputDescriptors,
    definitionEnvelope,
  );
  const dependencies = buildGraphDependencyExplanationRecord(
    graphSummary,
    inputDescriptors,
    outputDescriptors,
    definitionEnvelope,
  );
  const contractSummary = buildGraphContractSummary(graphSummary, graphContract, dependencies);
  const contractHistory = buildGraphContractHistory(graphContract);
  const importPosture = buildGraphImportPosture(graphId);

  return freezeObject({
    id: graphId,
    summary: graphSummary,
    contract: graphContract,
    operationalContract: graphOperationalContract,
    compatibility,
    dependencies,
    contractSummary,
    contractHistory,
    importPosture,
    inputDescriptors,
    descriptors: outputDescriptors,
  });
}

export function buildGraphExportSnapshot(graphDefinition, signalsFacade) {
  const runtimeEnvelope = signalsFacade.adapters().exportRuntimeEnvelope();
  const snapshotEnvelope = signalsFacade.history().snapshot();
  const importPosture = buildGraphImportPosture(
    graphDefinition.id,
    runtimeEnvelope?.runtimeEnvelopeRestoreMode ?? "SameRuntimeExact",
  );
  return freezeObject({
    id: graphDefinition.id,
    definition: graphDefinition,
    runtimeEnvelope,
    snapshotEnvelope,
    restoreMode: runtimeEnvelope?.runtimeEnvelopeRestoreMode ?? "SameRuntimeExact",
    contractHistory: graphDefinition.contractHistory,
    importPosture,
  });
}

export function requireGraphExportDefinition(definition) {
  if (!isPlainObject(definition)) {
    throw new TypeError("signals.importGraph(...) expects a graph export definition object");
  }
  if (typeof definition.id !== "string" || definition.id.length === 0) {
    throw new TypeError("signals.importGraph(...) requires a graph export definition with a non-empty string id");
  }
  if (!isPlainObject(definition.contract) || !isPlainObject(definition.summary)) {
    throw new TypeError("signals.importGraph(...) requires a graph export definition with contract and summary artifacts");
  }
  if (!Array.isArray(definition.inputDescriptors) || !Array.isArray(definition.descriptors)) {
    throw new TypeError("signals.importGraph(...) requires graph export descriptors for public inputs and outputs");
  }
  return definition;
}

export function requireGraphExportSnapshot(snapshot, graphId) {
  if (!isPlainObject(snapshot)) {
    throw new TypeError(`signals.importGraph(...) expects a graph snapshot artifact for graph \`${graphId}\``);
  }
  if (snapshot.id !== graphId) {
    throw new TypeError(
      `signals.importGraph(...) requires matching graph ids; received definition \`${graphId}\` and snapshot \`${snapshot?.id ?? "<unknown>"}\``,
    );
  }
  if (!isPlainObject(snapshot.runtimeEnvelope)) {
    throw new TypeError(
      `signals.importGraph(...) requires a graph snapshot artifact with a runtimeEnvelope for graph \`${graphId}\``,
    );
  }
  return snapshot;
}

export function requireMatchingGraphExportPair(definition, snapshot) {
  const snapshotDefinition = requireGraphExportDefinition(snapshot.definition);
  if (snapshotDefinition.id !== definition.id) {
    throw new TypeError(
      `signals.importGraph(...) requires snapshot.definition.id to match exported definition id \`${definition.id}\``,
    );
  }

  const expectedContract = JSON.stringify(definition.contract);
  const actualContract = JSON.stringify(snapshotDefinition.contract);
  if (expectedContract !== actualContract) {
    throw new TypeError(
      `signals.importGraph(...) requires snapshot.definition.contract to match the exported graph definition for graph \`${definition.id}\``,
    );
  }

  const expectedSummary = JSON.stringify(definition.summary);
  const actualSummary = JSON.stringify(snapshotDefinition.summary);
  if (expectedSummary !== actualSummary) {
    throw new TypeError(
      `signals.importGraph(...) requires snapshot.definition.summary to match the exported graph definition for graph \`${definition.id}\``,
    );
  }

  const expectedInputs = JSON.stringify(definition.inputDescriptors);
  const actualInputs = JSON.stringify(snapshotDefinition.inputDescriptors);
  if (expectedInputs !== actualInputs) {
    throw new TypeError(
      `signals.importGraph(...) requires snapshot.definition.inputDescriptors to match the exported graph definition for graph \`${definition.id}\``,
    );
  }

  const expectedOutputs = JSON.stringify(definition.descriptors);
  const actualOutputs = JSON.stringify(snapshotDefinition.descriptors);
  if (expectedOutputs !== actualOutputs) {
    throw new TypeError(
      `signals.importGraph(...) requires snapshot.definition.descriptors to match the exported graph definition for graph \`${definition.id}\``,
    );
  }

  return snapshotDefinition;
}

export function createImportedReadableSignal(rawSignals, id) {
  return wrapReadableSignal({
    id,
    get() {
      return rawSignals.read(id);
    },
    peek() {
      return rawSignals.read(id);
    },
    free() {},
  }, rawSignals, "signal");
}

export function buildGraphDiagnosticsSurface(
  signalsFacade,
  graphSummary,
  inputDescriptors,
  outputDescriptors,
  definitionEnvelope,
) {
  const specialist = signalsFacade.specialist();
  const diagnostics = signalsFacade.diagnostics();
  const contract = buildGraphContractSurface(graphSummary, inputDescriptors, outputDescriptors);
  const dependencyRecord = buildGraphDependencyExplanationRecord(
    graphSummary,
    inputDescriptors,
    outputDescriptors,
    definitionEnvelope,
  );
  const contractSummary = buildGraphContractSummary(graphSummary, contract, dependencyRecord);
  const inputVersions = specialist.readVersions(graphSummary.inputSourceIds);
  const outputVersions = specialist.readVersions(graphSummary.publishedOutputIds);
  const versionsByInputName = buildInputVersionMap(inputDescriptors, inputVersions);
  const versionsByOutputName = buildOutputVersionMap(outputDescriptors, outputVersions);

  const inputDiagnostics = nullPrototypeRecord();
  for (const descriptor of inputDescriptors) {
    inputDiagnostics[descriptor.inputName] = freezeObject({
      descriptor,
      version: versionsByInputName[descriptor.inputName] ?? null,
      why: diagnostics.why(descriptor.sourceId),
    });
  }

  const outputDiagnostics = nullPrototypeRecord();
  for (const descriptor of outputDescriptors) {
    outputDiagnostics[descriptor.outputName] = freezeObject({
      descriptor,
      version: versionsByOutputName[descriptor.outputName] ?? null,
      why: diagnostics.why(descriptor.publishedId),
    });
  }

  return freezeObject({
    graph: graphSummary,
    contract,
    dependencies: dependencyRecord,
    inputDescriptors,
    descriptors: outputDescriptors,
    inputVersions,
    outputVersions,
    inputs: freezeObject(inputDiagnostics),
    outputs: freezeObject(outputDiagnostics),
    input(name) {
      return requireKnownInput(inputDiagnostics, graphSummary.id, name);
    },
    output(name) {
      return requireKnownOutput(outputDiagnostics, graphSummary.id, name);
    },
    dependenciesForOutput(name) {
      return requireKnownOutput(dependencyRecord, graphSummary.id, name);
    },
    contractSummary() {
      return contractSummary;
    },
    runtimeGraph: diagnostics.summaryNow(),
    executionHistory: diagnostics.historyNow(),
    latestFlow: diagnostics.latestFlow(),
    latestObservation: diagnostics.latestObservation(),
  });
}

export function buildGraphHistorySurface(
  signalsFacade,
  graphSummary,
  inputDescriptors,
  outputDescriptors,
  definitionEnvelope,
) {
  const diagnostics = signalsFacade.diagnostics();
  const history = signalsFacade.history();
  const contract = buildGraphContractSurface(graphSummary, inputDescriptors, outputDescriptors);
  const dependencyRecord = buildGraphDependencyExplanationRecord(
    graphSummary,
    inputDescriptors,
    outputDescriptors,
    definitionEnvelope,
  );
  const contractSummary = buildGraphContractSummary(graphSummary, contract, dependencyRecord);
  const inputHistory = nullPrototypeRecord();
  for (const descriptor of inputDescriptors) {
    inputHistory[descriptor.inputName] = freezeObject({
      descriptor,
      replay: history.replay_for(descriptor.sourceId),
      lineage: history.lineage_for(descriptor.sourceId),
    });
  }
  const outputHistory = nullPrototypeRecord();
  for (const descriptor of outputDescriptors) {
    outputHistory[descriptor.outputName] = freezeObject({
      descriptor,
      replay: history.replay_for(descriptor.publishedId),
      lineage: history.lineage_for(descriptor.publishedId),
    });
  }

  return freezeObject({
    graph: graphSummary,
    contract,
    dependencies: dependencyRecord,
    inputDescriptors,
    descriptors: outputDescriptors,
    inputs: freezeObject(inputHistory),
    outputs: freezeObject(outputHistory),
    input(name) {
      return requireKnownInput(inputHistory, graphSummary.id, name);
    },
    output(name) {
      return requireKnownOutput(outputHistory, graphSummary.id, name);
    },
    dependenciesForOutput(name) {
      return requireKnownOutput(dependencyRecord, graphSummary.id, name);
    },
    contractSummary() {
      return contractSummary;
    },
    executionHistory: diagnostics.historyNow(),
    recentHistory: diagnostics.recentHistory(),
  });
}

function unknownGraphOutputError(graphId, outputName) {
  return new TypeError(
    `signals.graph output \`${graphId}.${String(outputName)}\` is not part of the published graph`,
  );
}

export function requireKnownOutput(outputs, graphId, outputName) {
  const output = outputs[outputName];
  if (!output) {
    throw unknownGraphOutputError(graphId, outputName);
  }
  return output;
}

export function requireKnownInput(inputs, graphId, inputName) {
  const input = inputs[inputName];
  if (!input) {
    throw new TypeError(
      `signals.graph input \`${graphId}.${String(inputName)}\` is not part of the public input contract`,
    );
  }
  return input;
}
