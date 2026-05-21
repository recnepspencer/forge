import {
  buildGraphContractSummary,
  buildGraphContractSurface,
  buildGraphDependencyExplanationRecord,
} from "../../../graph_support.js";

export async function inspectWorkerFirstGraphDiagnostics(session) {
  const definitionEnvelope = await session.bridge.exportDefinitions();
  const graphSummary = session.definition.summary;
  const contract = buildGraphContractSurface(
    graphSummary,
    session.definition.inputDescriptors,
    session.definition.descriptors,
  );
  const dependencies = buildGraphDependencyExplanationRecord(
    graphSummary,
    session.definition.inputDescriptors,
    session.definition.descriptors,
    definitionEnvelope,
  );
  const contractSummary = buildGraphContractSummary(
    graphSummary,
    contract,
    dependencies,
  );
  const [inputVersions, outputVersions, latestFlow, latestObservation, inputWhys, outputWhys] =
    await Promise.all([
      session.bridge.readVersions(session.trackedInputIds),
      session.bridge.readVersions(session.trackedOutputIds),
      session.bridge.latestFlow(),
      session.bridge.latestObservation(),
      Promise.all(
        session.definition.inputDescriptors.map((descriptor) =>
          session.bridge.why(descriptor.sourceId),
        ),
      ),
      Promise.all(
        session.definition.descriptors.map((descriptor) =>
          session.bridge.why(descriptor.publishedId),
        ),
      ),
    ]);
  const inputs = freezeInspectionEntries(
    session.definition.inputDescriptors,
    "inputName",
    inputVersions,
    inputWhys,
  );
  const outputs = freezeInspectionEntries(
    session.definition.descriptors,
    "outputName",
    outputVersions,
    outputWhys,
  );

  return Object.freeze({
    graph: graphSummary,
    contract,
    dependencies,
    inputDescriptors: session.definition.inputDescriptors,
    descriptors: session.definition.descriptors,
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
    latestFlow,
    latestObservation,
  });
}

export async function inspectWorkerFirstGraphHistory(session) {
  const definitionEnvelope = await session.bridge.exportDefinitions();
  const graphSummary = session.definition.summary;
  const contract = buildGraphContractSurface(
    graphSummary,
    session.definition.inputDescriptors,
    session.definition.descriptors,
  );
  const dependencies = buildGraphDependencyExplanationRecord(
    graphSummary,
    session.definition.inputDescriptors,
    session.definition.descriptors,
    definitionEnvelope,
  );
  const contractSummary = buildGraphContractSummary(
    graphSummary,
    contract,
    dependencies,
  );
  const [recentHistory, inputEntries, outputEntries] = await Promise.all([
    session.bridge.recentHistory(),
    Promise.all(
      session.definition.inputDescriptors.map(async (descriptor) => ({
        descriptor,
        replay: await session.bridge.replayFor(descriptor.sourceId),
        lineage: await session.bridge.lineageFor(descriptor.sourceId),
      })),
    ),
    Promise.all(
      session.definition.descriptors.map(async (descriptor) => ({
        descriptor,
        replay: await session.bridge.replayFor(descriptor.publishedId),
        lineage: await session.bridge.lineageFor(descriptor.publishedId),
      })),
    ),
  ]);
  const inputs = freezeHistoryEntries(inputEntries, "inputName");
  const outputs = freezeHistoryEntries(outputEntries, "outputName");

  return Object.freeze({
    graph: graphSummary,
    contract,
    dependencies,
    inputDescriptors: session.definition.inputDescriptors,
    descriptors: session.definition.descriptors,
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
    recentHistory,
  });
}

function freezeInspectionEntries(descriptors, nameField, versions, whys) {
  const versionsById = new Map((versions ?? []).map((entry) => [entry.id, entry]));
  const entries = Object.create(null);
  descriptors.forEach((descriptor, index) => {
    entries[descriptor[nameField]] = Object.freeze({
      descriptor,
      version:
        versionsById.get(
          nameField === "inputName" ? descriptor.sourceId : descriptor.publishedId,
        ) ?? null,
      why: whys[index],
    });
  });
  return Object.freeze(entries);
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
  throw new TypeError(
    `worker-first graph ${graphId} does not expose ${family} \`${name}\``,
  );
}
