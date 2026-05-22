import {
  buildGraphContractSummary,
  buildGraphContractSurface,
  buildGraphDependencyExplanationRecord,
} from "../graph_support.js";

export function inspectWorkerFirstRootGraphDiagnostics(session) {
  const context = session.rootSession.currentImportContext();
  const definition = session.definition;
  const graphSummary = definition.summary;
  const contract = buildGraphContractSurface(
    graphSummary,
    definition.inputDescriptors,
    definition.descriptors,
  );
  const dependencies = buildGraphDependencyExplanationRecord(
    graphSummary,
    definition.inputDescriptors,
    definition.descriptors,
    context.runtimeDefinitionEnvelope,
  );
  const contractSummary = buildGraphContractSummary(graphSummary, contract, dependencies);
  const inputVersions = definition.inputDescriptors.map((descriptor) => context.versionById.get(descriptor.sourceId) ?? null);
  const outputVersions = definition.descriptors.map((descriptor) => context.versionById.get(descriptor.publishedId) ?? null);
  const inputWhys = definition.inputDescriptors.map((descriptor) => context.whyById.get(descriptor.sourceId));
  const outputWhys = definition.descriptors.map((descriptor) => context.whyById.get(descriptor.publishedId));
  const inputs = freezeInspectionEntries(definition.inputDescriptors, "inputName", inputVersions, inputWhys);
  const outputs = freezeInspectionEntries(definition.descriptors, "outputName", outputVersions, outputWhys);

  return Object.freeze({
    graph: graphSummary,
    contract,
    dependencies,
    inputDescriptors: definition.inputDescriptors,
    descriptors: definition.descriptors,
    inputVersions,
    outputVersions,
    inputs,
    outputs,
    input(name) {
      return requireKnownInspectionEntry(inputs, graphSummary.id, "input", name);
    },
    output(name) {
      return requireKnownInspectionEntry(outputs, graphSummary.id, "output", name);
    },
    dependenciesForOutput(name) {
      return requireKnownInspectionEntry(dependencies, graphSummary.id, "output", name);
    },
    contractSummary() {
      return contractSummary;
    },
    runtimeGraph: session.diagnosticsSummary(),
    executionHistory: session.diagnosticsHistory(),
    latestFlow: context.latestFlow,
    latestObservation: context.latestObservation,
  });
}

export function inspectWorkerFirstRootGraphHistory(session) {
  const context = session.rootSession.currentImportContext();
  const definition = session.definition;
  const graphSummary = definition.summary;
  const contract = buildGraphContractSurface(
    graphSummary,
    definition.inputDescriptors,
    definition.descriptors,
  );
  const dependencies = buildGraphDependencyExplanationRecord(
    graphSummary,
    definition.inputDescriptors,
    definition.descriptors,
    context.runtimeDefinitionEnvelope,
  );
  const contractSummary = buildGraphContractSummary(graphSummary, contract, dependencies);
  const inputEntries = definition.inputDescriptors.map((descriptor) => ({
    descriptor,
    replay: context.replayById.get(descriptor.sourceId),
    lineage: context.lineageById.get(descriptor.sourceId),
  }));
  const outputEntries = definition.descriptors.map((descriptor) => ({
    descriptor,
    replay: context.replayById.get(descriptor.publishedId),
    lineage: context.lineageById.get(descriptor.publishedId),
  }));
  const inputs = freezeHistoryEntries(inputEntries, "inputName");
  const outputs = freezeHistoryEntries(outputEntries, "outputName");

  return Object.freeze({
    graph: graphSummary,
    contract,
    dependencies,
    inputDescriptors: definition.inputDescriptors,
    descriptors: definition.descriptors,
    inputs,
    outputs,
    input(name) {
      return requireKnownInspectionEntry(inputs, graphSummary.id, "input", name);
    },
    output(name) {
      return requireKnownInspectionEntry(outputs, graphSummary.id, "output", name);
    },
    dependenciesForOutput(name) {
      return requireKnownInspectionEntry(dependencies, graphSummary.id, "output", name);
    },
    contractSummary() {
      return contractSummary;
    },
    executionHistory: session.diagnosticsHistory(),
    recentHistory: context.recentHistory,
  });
}

function freezeInspectionEntries(descriptors, nameField, versions, whys) {
  const record = Object.create(null);
  descriptors.forEach((descriptor, index) => {
    record[descriptor[nameField]] = Object.freeze({
      descriptor,
      version: versions[index],
      why: whys[index],
    });
  });
  return Object.freeze(record);
}

function freezeHistoryEntries(entries, nameField) {
  const record = Object.create(null);
  for (const entry of entries) {
    record[entry.descriptor[nameField]] = Object.freeze(entry);
  }
  return Object.freeze(record);
}

function requireKnownInspectionEntry(record, graphId, family, name) {
  const entry = record[name];
  if (entry) {
    return entry;
  }
  throw new TypeError(`worker-first graph ${graphId} does not expose ${family} \`${name}\``);
}
