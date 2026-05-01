import {
  clockCapability,
  type ControllerContract,
  createSignals,
  type GraphMutationRequest,
  type GraphPublicationRequest,
  hostCapabilityPlan,
  onlineCapability,
  persistenceCapability,
  type PublishedGraphTransaction,
  type PublishedSignalGraph,
  type ScopedSignalNamespace,
  type SignalNamespace,
  viewportCapability,
  type ComputedSpec,
  type InputSignalHandle,
  type OutputSpec,
  type Signal,
  visibilityCapability,
} from "./index.js";

let visibilityState: "visible" | "hidden" = "visible";
let viewportState = { width: 1280, height: 720 };
let onlineState: "online" | "offline" = "online";
let clockTick = 0;
let persistedDraft = { mode: "draft", revision: 1 };

const signals = createSignals({
  hostCapabilities: hostCapabilityPlan({
    visibility: visibilityCapability({
      source: {
        current() {
          return visibilityState;
        },
        subscribe() {
          return () => {};
        },
      },
      compatibility: "LiveOnly",
    }),
    viewport: viewportCapability({
      source: {
        current() {
          return viewportState;
        },
        subscribe() {
          return () => {};
        },
      },
    }),
    online: onlineCapability({
      source: {
        current() {
          return onlineState;
        },
        subscribe() {
          return () => {};
        },
      },
    }),
    clock: clockCapability({
      source: {
        current() {
          return clockTick;
        },
      },
      pollMs: 5,
    }),
    persistence: persistenceCapability({
      source: {
        current() {
          return persistedDraft;
        },
      },
    }),
  }),
});

const count: InputSignalHandle<number> = signals.input(1, { id: "count" });
const nameInput: InputSignalHandle<string> = signals.input("Ada", { id: "name" });
const scopedSignals: ScopedSignalNamespace = signals.scope("itemDetail");
const nestedScopedSignals: ScopedSignalNamespace = scopedSignals.scope("editSession");
const scopedDescriptor = nestedScopedSignals.descriptor();
const scopedCanonicalCountId = nestedScopedSignals.canonicalId("count");
const scopedIdentity = nestedScopedSignals.signalIdentity("count");
const scopedIdentityGraphId = scopedIdentity.graphId;
const scopedIdentityRootScopeId = scopedIdentity.rootScopeId;
const scopedIdentityScopePath = scopedIdentity.scopePath;
const scopedDescriptorPath = scopedDescriptor.path;
const scopedDescriptorIdentity = scopedDescriptor.identity;
const scopedDescriptorGraphOwnerId = scopedDescriptor.graphOwnerId;
const scopedCount = nestedScopedSignals.input(1, { id: "count" });
const scopedLabel = nestedScopedSignals.computed("label", () => `${scopedCount()}`);
const scopedOutput = nestedScopedSignals.output("panel", () => ({ count: scopedCount() }));
const viewport = signals.host.viewport;
const visibility = signals.host.visibility;
const online = signals.host.online;
const clock = signals.host.clock;
const persistence = signals.host.persistence;
// @ts-expect-error host capability lifecycle stays framework-owned
viewport?.free();
// @ts-expect-error host capability lifecycle stays framework-owned
visibility?.free();
// @ts-expect-error host capability lifecycle stays framework-owned
online?.free();
// @ts-expect-error host capability lifecycle stays framework-owned
clock?.free();
// @ts-expect-error host capability lifecycle stays framework-owned
persistence?.free();
const next: number = count();
const alsoNext: number = count.get();
const commit = count.set(next + alsoNext);
const viewportSize = viewport?.size() ?? { width: 0, height: 0 };
const viewportWidth = viewport?.width() ?? 0;
const viewportHeight = viewport?.height() ?? 0;
const viewportDescriptor = viewport?.descriptor();
const visibilityStateNow = visibility?.state() ?? "hidden";
const visibilityFlag = visibility?.isVisible() ?? false;
const visibilityDescriptor = visibility?.descriptor();
const onlineStateNow = online?.state() ?? "offline";
const onlineFlag = online?.isOnline() ?? false;
const onlineDescriptor = online?.descriptor();
const clockNow = clock?.now() ?? 0;
const clockDescriptor = clock?.descriptor();
const persistenceValue = (persistence?.value() ?? { mode: "draft", revision: 0 }) as {
  mode: "draft";
  revision: number;
};
const persistenceMode: "draft" = persistenceValue.mode;
const persistenceRevision: number = persistenceValue.revision;
const persistenceDescriptor = persistence?.descriptor();
const persistenceCommit = persistence?.commit();

const doubledSpec: ComputedSpec = {
  reads: ["count"],
  expr: {
    kind: "multiply",
    args: [
      { kind: "read", id: "count" },
      { kind: "value", value: 2 },
    ],
  },
};

const doubled: Signal<number> = signals.computed<number>(doubledSpec, { id: "doubled" });
const doubledFromCallback: Signal<number> = signals.computed<number>(
  "doubledCallback",
  () => count() * 2,
);
const constantFromCallback: Signal<number> = signals.computed<number>(
  "constantCallback",
  () => 2,
);
const generatedFromCallback: Signal<number> = signals.computed<number>(() => 3, { id: "three" });
const gatedFromHostCapability: Signal<string> = signals.computed<string>(() => (
  visibility?.isVisible() ? "onscreen" : "hidden"
), { id: "gatedFromHostCapability" });
const viewportLabel: Signal<string> = signals.computed<string>(() => (
  `${viewport?.width() ?? 0}x${viewport?.height() ?? 0}`
), { id: "viewportLabel" });
const onlineLabel: Signal<string> = signals.computed<string>(() => (
  online?.isOnline() ? "online" : "offline"
), { id: "onlineLabel" });
const clockLabel: Signal<number> = signals.computed<number>(() => (
  (clock?.now() ?? 0) + count()
), { id: "clockLabel" });
const persistenceLabel: Signal<number> = signals.computed<number>(() => (
  persistence?.value().revision ?? 0
), { id: "persistenceLabel" });
const legacyDoubledFromSpecAlias: Signal<number> = signals.computedSpec<number>(
  "legacyDoubled",
  doubledSpec,
);

const panelSpec: OutputSpec = {
  reads: ["count", "doubled"],
  expr: {
    kind: "object",
    fields: [
      ["count", { kind: "read", id: "count" }],
      ["doubled", { kind: "read", id: "doubled" }],
    ],
  },
};

const panel = signals.output<{ count: number; doubled: number }>(panelSpec, { id: "panel" });
const graphDoubledHandle = signals.computed<number>("graphDoubled", () => count() * 2);
const legacyPanelFromSpecAlias = signals.outputSpec<{ count: number; doubled: number }>(
  "legacyPanel",
  panelSpec,
);
const snapshot = panel();
const panelSnapshotFromRead = signals.read<{ count: number; doubled: number }>(panel);
const countSnapshotFromRead = signals.read<number>(count);
const callbackPanel = signals.output<{ count: number; doubled: number }>(() => ({
  count: count(),
  doubled: doubled(),
}), { id: "callbackPanel" });
const callbackPanelSnapshot = callbackPanel();
const namespace: SignalNamespace = signals;
const graphRequest: GraphPublicationRequest<{
  count: InputSignalHandle<number>;
}, {
  count: InputSignalHandle<number>;
  doubled: typeof graphDoubledHandle;
  panel: typeof panel;
}> = {
  inputs: {
    count,
  },
  outputs: {
    count,
    doubled: graphDoubledHandle,
    panel,
  },
};
const graph: PublishedSignalGraph<typeof graphRequest.outputs, NonNullable<typeof graphRequest.inputs>> = signals.graph(
  "itemDetail",
  graphRequest,
);
const graphInputByName = graph.input("count");
const graphCount = graph.outputs.count();
const graphDoubled = graph.outputs.doubled();
const graphPanel = graph.outputs.panel();
const graphDescriptorKind = graph.descriptors()[0]?.publicationKind ?? null;
const graphInputDescriptor = graph.inputDescriptors()[0]?.sourceId ?? null;
const graphInputSnapshot = graph.readInputs();
const graphInputCountValue = graphInputSnapshot.count;
const graphOperationalContract = graph.operationalContract();
const graphOperationalWriteId = graphOperationalContract.writes.count;
const graphOperationalPatchCount = Object.keys(graphOperationalContract.patches).length;
const graphOperationalAuthority = graphOperationalContract.authorities.count.authority;
const graphOperationRequest: GraphMutationRequest<NonNullable<typeof graphRequest.inputs>> = {
  writes: {
    count: 2,
  },
  commands: {},
  reset: ["count"],
};
const graphWriteCommit = graph.writeInputs({
  count: 3,
});
const graphPatchCommit = graph.patchInputs({});
const graphResetCommit = graph.resetInputs(["count"]);
const graphApplyCommit = graph.apply(graphOperationRequest);
const graphTransactionCommit = graph.transaction((
  tx: PublishedGraphTransaction<NonNullable<typeof graphRequest.inputs>>,
) => {
  tx.set("count", 4);
  tx.set(graph.inputs.count, 5);
});
const graphSnapshot = graph.read();
const graphCountValue = graphSnapshot.count;
const graphDoubledValue = graphSnapshot.doubled;
const graphPanelValue = graphSnapshot.panel;
const graphWhy = graph.why("count");
const graphReplay = graph.replayFor("doubled");
const graphLineage = graph.lineageFor("panel");
const graphReadVersions = graph.readVersions();
const graphPublicationSummary = graph.summary();
const graphDiagnosticsSurface = graph.inspectDiagnostics();
const graphHistorySurface = graph.inspectHistory();
const graphCompatibilityDefinition = graph.exportCompatibilityDefinition();
const graphExportDefinition = graph.exportDefinition();
const graphExportSnapshot = graph.exportSnapshot();
const graphImportPosture = graph.importPosture();
const graphCompatibilityCountId = graphCompatibilityDefinition.inputs.count;
const graphContract = graph.contract();
const graphContractDelta = graph.contractDelta(graphContract);
const graphContractHistory = graph.contractHistory();
const graphContractCountId = graphContract.inputs.count;
const graphDiagnosticsWhy = graphDiagnosticsSurface.outputs.count.why;
const graphDiagnosticsInputWhy = graphDiagnosticsSurface.inputs.count.why;
const graphDiagnosticsInputEntry = graphDiagnosticsSurface.input("count");
const graphDiagnosticsOutputEntry = graphDiagnosticsSurface.output("panel");
const graphDiagnosticsDependency = graphDiagnosticsSurface.dependenciesForOutput("panel");
const graphDiagnosticsContractSummary = graphDiagnosticsSurface.contractSummary();
const graphHistoryInputEntry = graphHistorySurface.input("count");
const graphHistoryOutputEntry = graphHistorySurface.output("panel");
const graphHistoryDependency = graphHistorySurface.dependenciesForOutput("panel");
const graphHistoryContractSummary = graphHistorySurface.contractSummary();
const graphDiagnosticsVersion = graphDiagnosticsSurface.outputs.panel.version;
const graphDiagnosticsInputVersion = graphDiagnosticsSurface.inputs.count.version;
const graphHistoryReplay = graphHistorySurface.outputs.doubled.replay;
const graphHistoryInputReplay = graphHistorySurface.inputs.count.replay;
const graphHistoryLineage = graphHistorySurface.outputs.panel.lineage;
const graphCompatibilityPanelId = graphCompatibilityDefinition.outputs.panel;
const graphCompatibilityContractCountId = graphCompatibilityDefinition.contract.inputs.count;
const graphCompatibilityRecipeId = graphCompatibilityDefinition.definitions.recipes[0]?.id ?? null;
const graphDiagnostics = graph.diagnostics();
const graphHistory = graph.history();
const graphSpecialist = graph.specialist();
const graphAdapters = graph.adapters();
const graphOutputByName = graph.output("panel");
const restoredGraph = createSignals().importGraph(graphExportDefinition, graphExportSnapshot);
const restoredGraphContract = restoredGraph.contract();
const restoredGraphContractHistory = restoredGraph.contractHistory();
const restoredGraphImportPosture = restoredGraph.importPosture();
const restoredGraphRead = restoredGraph.read();
const restoredGraphReadInputs = restoredGraph.readInputs();
const restoredGraphCompatibility = restoredGraph.exportCompatibilityDefinition();
const restoredGraphDiagnostics = restoredGraph.inspectDiagnostics();
const restoredGraphHistory = restoredGraph.inspectHistory();

function createEditSessionController(namespace: SignalNamespace) {
  const serverItemData = namespace.input<{
    workflow_target_state_id?: number | null;
  } | null>(null, { id: "serverItemData" });
  const draftEdits = namespace.input<{
    workflow_target_state_id?: number | null;
  }>({}, { id: "draftEdits" });

  const effectiveItemData = namespace.computed(() => ({
    ...(serverItemData() ?? {}),
    ...(draftEdits() ?? {}),
  }), { id: "effectiveItemData" });

  const dirtyState = namespace.computed(() => ({
    isDirty: Object.keys(draftEdits()).length > 0,
  }), { id: "dirtyState" });

  return namespace.controller({
    inputs: {
      serverItemData,
      draftEdits,
    },
    outputs: {
      effectiveItemData,
      dirtyState,
    },
  });
}

function createWorkflowController(
  namespace: SignalNamespace,
  editSession: ReturnType<typeof createEditSessionController>,
) {
  const submitReadiness = namespace.computed(() => {
    const item = editSession.outputs.effectiveItemData();
    const dirty = editSession.outputs.dirtyState();

    return {
      enabled: dirty.isDirty && Boolean(item.workflow_target_state_id),
      targetStateId: item.workflow_target_state_id ?? null,
    };
  }, { id: "submitReadiness" });

  return namespace.controller({
    outputs: {
      submitReadiness,
    },
  });
}

const editSession = createEditSessionController(signals);
const workflow = createWorkflowController(signals, editSession);
const editSessionContract: ControllerContract = editSession;
const itemDetailGraph = signals.graph("itemDetailControllers", (builder) => {
  const scopedEditSession = createEditSessionController(builder.scope("editSession"));
  const scopedWorkflow = createWorkflowController(builder.scope("workflow"), scopedEditSession);
  return builder.expose({
    controllers: [scopedEditSession, scopedWorkflow],
  });
});
const scopedCounterGraph = signals.graph("scopedCounter", {
  outputs: {
    count: scopedCount,
    label: scopedLabel,
    panel: scopedOutput,
  },
});
const itemDetailGraphOutput = itemDetailGraph.output("submitReadiness");
const itemDetailGraphSummary = itemDetailGraph.summary();
const itemDetailGraphDiagnostics = itemDetailGraph.inspectDiagnostics();
const itemDetailGraphHistory = itemDetailGraph.inspectHistory();
const itemDetailGraphCompatibility = itemDetailGraph.exportCompatibilityDefinition();
const itemDetailGraphExportDefinition = itemDetailGraph.exportDefinition();
const itemDetailGraphExportSnapshot = itemDetailGraph.exportSnapshot();
const itemDetailGraphImportPosture = itemDetailGraph.importPosture();
const itemDetailGraphContract = itemDetailGraph.contract();
const itemDetailGraphContractDelta = itemDetailGraph.contractDelta(itemDetailGraphContract);
const itemDetailGraphContractHistory = itemDetailGraph.contractHistory();
const itemDetailGraphInput = itemDetailGraph.input("serverItemData");
const itemDetailGraphInputs = itemDetailGraph.readInputs();
const itemDetailGraphInputDescriptor = itemDetailGraph.inputDescriptors()[0]?.sourceId ?? null;
const itemDetailGraphCompatibilityInputId =
  itemDetailGraphCompatibility.inputs.serverItemData;
const itemDetailGraphCompatibilityContractInputId =
  itemDetailGraphCompatibility.contract.inputs.serverItemData;
const itemDetailGraphCompatibilityOutputId =
  itemDetailGraphCompatibility.outputs.submitReadiness;
const itemDetailGraphContractInputId =
  itemDetailGraphContract.inputs.serverItemData;
const itemDetailGraphHistoryReplay = itemDetailGraphHistory.outputs.submitReadiness.replay;
const itemDetailGraphHistoryInputReplay = itemDetailGraphHistory.inputs.serverItemData.replay;
const itemDetailGraphDiagnosticsWhy = itemDetailGraphDiagnostics.outputs.submitReadiness.why;
const itemDetailGraphDiagnosticsInputWhy = itemDetailGraphDiagnostics.inputs.serverItemData.why;
const itemDetailGraphDependency =
  itemDetailGraphDiagnostics.dependenciesForOutput("submitReadiness");
const itemDetailGraphContractSummary = itemDetailGraphDiagnostics.contractSummary();
const itemDetailGraphHistoryDependency =
  itemDetailGraphHistory.dependenciesForOutput("submitReadiness");
const itemDetailGraphHistoryContractSummary = itemDetailGraphHistory.contractSummary();
const restoredItemDetailGraph = createSignals().importGraph(
  itemDetailGraphExportDefinition,
  itemDetailGraphExportSnapshot,
);
const restoredItemDetailGraphContract = restoredItemDetailGraph.contract();
const restoredItemDetailGraphContractHistory = restoredItemDetailGraph.contractHistory();
const restoredItemDetailGraphImportPosture = restoredItemDetailGraph.importPosture();
const restoredItemDetailGraphRead = restoredItemDetailGraph.read();
const restoredItemDetailGraphReadInputs = restoredItemDetailGraph.readInputs();
const restoredItemDetailGraphDependency =
  restoredItemDetailGraph.inspectDiagnostics().dependenciesForOutput("submitReadiness");

function createFormController(namespace: SignalNamespace) {
  const serverValue = namespace.input<{
    id: string;
    title: string;
    status: string;
  }>({
    id: "task-7",
    title: "Ship docs",
    status: "draft",
  }, { id: "serverValue" });
  const draftValue = namespace.input<{
    title?: string;
    status?: string;
  }>({
    title: "Ship docs",
    status: "ready",
  }, { id: "draftValue" });
  const effectiveValue = namespace.computed(() => ({
    ...serverValue(),
    ...draftValue(),
  }), { id: "effectiveValue" });
  const dirtyState = namespace.computed(() => ({
    isDirty: Object.keys(draftValue()).length > 0,
  }), { id: "dirtyState" });
  const validation = namespace.computed(() => ({
    titleMissing: !effectiveValue().title,
  }), { id: "validation" });

  return namespace.controller({
    inputs: {
      serverValue,
      draftValue,
    },
    outputs: {
      effectiveValue,
      dirtyState,
      validation,
    },
  });
}

function createResourceController(
  namespace: SignalNamespace,
  form: ReturnType<typeof createFormController>,
) {
  const routeParams = namespace.input<{
    taskId: string;
    workspaceId: string;
  }>({
    taskId: "task-7",
    workspaceId: "alpha",
  }, { id: "routeParams" });
  const resourceQuery = namespace.computed(() => ({
    taskId: routeParams().taskId,
    workspaceId: routeParams().workspaceId,
        status: form.outputs.effectiveValue().status,
  }), { id: "resourceQuery" });
  const submitAvailability = namespace.computed(() => ({
    enabled: form.outputs.dirtyState().isDirty && !form.outputs.validation().titleMissing,
    taskId: resourceQuery().taskId,
  }), { id: "submitAvailability" });

  return namespace.controller({
    inputs: {
      routeParams,
    },
    outputs: {
      resourceQuery,
      submitAvailability,
    },
  });
}

function createAuthorityController(namespace: SignalNamespace) {
  const serverValue = namespace.input({
    id: "task-7",
    title: "Ship docs",
  }, { id: "serverValue" });
  const draftValue = namespace.input({
    title: "Ship docs",
  }, { id: "draftValue" });
  const externalParams = namespace.input({
    taskId: "task-7",
  }, { id: "externalParams" });
  const effectiveValue = namespace.computed(() => ({
    ...serverValue(),
    ...draftValue(),
    taskId: externalParams().taskId,
  }), { id: "effectiveValue" });

  return namespace.controller({
    inputs: {
      serverValue: namespace.publicInput(serverValue, { authority: "readOnly" }),
      draftValue: namespace.publicInput(draftValue),
      externalParams: namespace.publicInput(externalParams, { authority: "imported" }),
    },
    outputs: {
      effectiveValue,
    },
  });
}

const taskEditorGraph = createSignals().graph("taskEditor", (graph) => {
  const form = createFormController(graph.scope("form"));
  const resource = createResourceController(graph.scope("resource"), form);
  return graph.expose({
    controllers: [form, resource],
  });
});
const authorityGraph = createSignals().graph("taskAuthority", (graph) => {
  const authority = createAuthorityController(graph.scope("authority"));
  return graph.expose({
    controllers: [authority],
  });
});
const taskEditorGraphContract = taskEditorGraph.contract();
const taskEditorOperationalContract = taskEditorGraph.operationalContract();
const taskEditorContractDelta = taskEditorGraph.contractDelta(taskEditorGraphContract);
const taskEditorGraphInputId = taskEditorGraphContract.inputs.serverValue;
const taskEditorGraphOutputId = taskEditorGraphContract.outputs.submitAvailability;
const taskEditorPatchId = taskEditorOperationalContract.patches.draftValue;
const taskEditorGraphInputWhy = taskEditorGraph.inspectDiagnostics().inputs.serverValue.why;
const taskEditorGraphOutputReplay = taskEditorGraph.inspectHistory().outputs.submitAvailability.replay;
const taskEditorGraphDependency =
  taskEditorGraph.inspectDiagnostics().dependenciesForOutput("submitAvailability");
const taskEditorGraphContractSummary = taskEditorGraph.inspectDiagnostics().contractSummary();
const taskEditorGraphExportDefinition = taskEditorGraph.exportDefinition();
const taskEditorGraphExportSnapshot = taskEditorGraph.exportSnapshot();
const taskEditorGraphImportPosture = taskEditorGraph.importPosture();
const taskEditorGraphContractHistory = taskEditorGraph.contractHistory();
const restoredTaskEditorGraph = createSignals().importGraph(
  taskEditorGraphExportDefinition,
  taskEditorGraphExportSnapshot,
);
const restoredTaskEditorGraphHistory = restoredTaskEditorGraph.contractHistory();
const restoredTaskEditorGraphImportPosture = restoredTaskEditorGraph.importPosture();
const taskEditorGraphPatchCommit = taskEditorGraph.patchInputs({
  draftValue: {
    status: "published",
  },
});
const taskEditorGraphApplyCommit = taskEditorGraph.apply({
  writes: {
    routeParams: {
      taskId: "task-8",
      workspaceId: "beta",
    },
  },
  patches: {
    draftValue: {
      title: "Ship package",
    },
  },
  commands: {},
});
const taskEditorGraphTxCommit = taskEditorGraph.transaction((tx) => {
  tx.set("serverValue", {
    id: "task-7",
    title: "Ship docs",
    status: "ready",
  });
});
const authorityGraphContract = authorityGraph.contract();
const authorityGraphOperationalContract = authorityGraph.operationalContract();
const authorityGraphRead = authorityGraph.read();
const authorityGraphInputRead = authorityGraph.readInputs();
const authorityGraphReadOnlyAuthority =
  authorityGraphOperationalContract.authorities.serverValue.authority;
const authorityGraphImportedAuthority =
  authorityGraphOperationalContract.authorities.externalParams.authority;
const authorityGraphPatchId = authorityGraphOperationalContract.patches.draftValue;
const authorityGraphWriteCommit = authorityGraph.writeInputs({
  draftValue: {
    title: "Ready to ship",
  },
});
const authorityGraphPatchCommit = authorityGraph.patchInputs({
  draftValue: {
    title: "Approved",
  },
});
const authorityGraphResetCommit = authorityGraph.resetInputs(["draftValue"]);
const authorityGraphApplyCommit = authorityGraph.apply({
  writes: {
    draftValue: {
      title: "Ship package",
    },
  },
  commands: {},
});
const authorityGraphTransactionCommit = authorityGraph.transaction((tx) => {
  tx.set("draftValue", {
    title: "Queued",
  });
});
const explicitCallbackPanel = signals.outputCallback<{ count: number; doubled: number }>(
  "callbackPanelExplicit",
  () => snapshot,
);
const adapters = signals.adapters();
const definitions = adapters.exportDefinitions();
const runtimeEnvelope = adapters.exportRuntimeEnvelope();
adapters.restoreExactRuntimeEnvelope(runtimeEnvelope);
const transportReport = adapters.hostCapabilityTransportReport(runtimeEnvelope);
const proof = adapters.runtimeProofReport();
const runtimeEnvelopeRestoreMode = runtimeEnvelope.runtimeEnvelopeRestoreMode;
const restoredBranchId = runtimeEnvelope.snapshot.snapshot.meta.branch_id;
const snapshotExplanationRetention =
  runtimeEnvelope.snapshot.snapshot.meta.artifact_retention.explanation_retention;
const checkpointImage = runtimeEnvelope.snapshot.snapshot.checkpoint_image;
const diagnosticGraph = runtimeEnvelope.snapshot.snapshot.diagnostic_graph;
const proofVersion = proof.proofSchemaVersion;
const proofDigest = proof.registryBundleDigest;
const maybeUnavailable = definitions.unavailableCallbacks.map(
  (artifact) => artifact.signalKind,
);
const diagnostics = signals.diagnostics();
const history = signals.history();
const specialist = signals.specialist();
const currentBranch = history.current_branch();
const previewBranch = history.create_branch("preview");
const branchReplay = history.replay_for_branch(currentBranch.id);
const branchSnapshot = history.branch_snapshot(currentBranch.id);
const branchEnvelope = history.branch_snapshot_envelope(currentBranch.id);
const branchSnapshotRestoreMode = branchSnapshot.snapshotRestoreMode;
const branchEnvelopeRestoreMode = branchEnvelope.snapshotEnvelopeRestoreMode;
history.restore_exact_snapshot(branchEnvelope);
history.restore_exact_branch_snapshot(currentBranch.id, branchSnapshot);
const branchProof = history.branch_state_proof(currentBranch.id);
const parityProof = history.replay_parity_proof(currentBranch.id, currentBranch.id);
const artifactProof = history.replay_artifact_proof({
  proofSchemaVersion: proof.proofSchemaVersion,
  registryBundleDigest: proof.registryBundleDigest,
  loweredStrategyBundleDigest: null,
  mergePlanDigest: null,
  mergeResultDigest: null,
  lineageDigest: null,
  branchStateDigest: branchProof.stateDigest,
}, currentBranch.id);
const previewPlan = history.plan_merge_policy_preview({
  source_branch_id: previewBranch.id,
  target_branch_id: currentBranch.id,
});
const previewPlanProof = history.plan_merge_policy_preview_with_proof({
  source_branch_id: previewBranch.id,
  target_branch_id: currentBranch.id,
});
const previewResult = history.merge_branches_policy_preview({
  source_branch_id: previewBranch.id,
  target_branch_id: currentBranch.id,
});
const previewResultProof = history.merge_branches_policy_preview_with_proof({
  source_branch_id: previewBranch.id,
  target_branch_id: currentBranch.id,
});
const graphSummary = diagnostics.summaryNow();
const specialistGraphSummary = specialist.graphSummary();
const specialistEvaluateDirty = specialist.evaluateDirty();
const performanceSummary = diagnostics.performanceSummary();
const latestFlow = diagnostics.latestFlow();
const latestObservation = diagnostics.latestObservation();
const latestHostCapabilityEvent = diagnostics.latestHostCapabilityEvent();
const recentHostCapabilityEvents = diagnostics.recentHostCapabilityEvents();
const hostCapabilityReport = diagnostics.hostCapabilityReport();
const hostCapabilityLineageDigest = hostCapabilityReport.lineageDigest;
const hostCapabilityBreadthDigest = hostCapabilityReport.breadthDigest;
const hostCapabilityLineageEntry = hostCapabilityReport.lineage[0] ?? null;
const hostCapabilityBreadthFamily = hostCapabilityReport.breadth.families[0] ?? null;
const latestFailure = diagnostics.latestFailure();
const latestFrontierExecution = diagnostics.latestFrontierExecution();
const recentHistory = diagnostics.recentHistory();
const latestHostCapabilityRead =
  latestFlow?.callbackNodes[0]?.hostCapabilityReads[0]?.compatibility ??
  latestObservation?.callbackNodes[0]?.hostCapabilityReads[0]?.compatibility ??
  null;
const unavailableHostCapabilityTransport =
  runtimeEnvelope.definitions.unavailableCallbacks[0]?.hostCapabilityTransports[0] ?? null;
const latestCallbackCurrentReads = latestFlow?.callbackNodes[0]?.currentReads ?? [];

const callbackNodeIds =
  latestFlow?.callbackNodes.map((node) => node.id) ??
  latestObservation?.callbackNodes.map((node) => node.id) ??
  [];
const latestHistoryNode = recentHistory[0]?.nodes[0]?.node ?? null;
const graphProfile = graphSummary.profile;
const specialistGraphProfile = specialistGraphSummary.profile;
const specialistTouchedNodes = specialistEvaluateDirty.touchedNodes;
const latestFailureMessage = latestFailure?.message ?? null;
const latestFrontierSeedCount = latestFrontierExecution?.seed_count ?? 0;
const latestHostCapabilityEventKind = latestHostCapabilityEvent?.kind ?? null;
const latestHostCapabilityEventQueuedCount = latestHostCapabilityEvent?.queuedInvalidationCount ?? 0;
const latestHostCapabilityDeniedIds = latestHostCapabilityEvent?.deniedCallbackIds ?? [];
const hostCapabilityInvalidationCount = performanceSummary.hostCapabilityInvalidationCount ?? 0;
const hostCapabilityReadCount = performanceSummary.hostCapabilityReadCount ?? 0;
const hostCapabilityPollCount = performanceSummary.hostCapabilityPollCount ?? 0;
const hostCapabilityNoOpPollCount = performanceSummary.hostCapabilityNoOpPollCount ?? 0;
const hostCapabilityManualCommitCount = performanceSummary.hostCapabilityManualCommitCount ?? 0;
const hostCapabilityNoOpManualCommitCount =
  performanceSummary.hostCapabilityNoOpManualCommitCount ?? 0;
const hostCapabilityReevaluationCount = performanceSummary.hostCapabilityReevaluationCount ?? 0;
const hostCapabilityCompatibilityDenialCount =
  performanceSummary.hostCapabilityCompatibilityDenialCount ?? 0;
const hostCapabilityUnavailabilityArtifactCount =
  performanceSummary.hostCapabilityUnavailabilityArtifactCount ?? 0;
const hostCapabilityBroadFanoutDenialCount =
  performanceSummary.hostCapabilityBroadFanoutDenialCount ?? 0;
const branchReplayCallback = branchReplay.frames[0]?.callback?.registered ?? null;
const branchSnapshotBranchId = branchSnapshot.meta.branch_id;
const branchEnvelopeSnapshotId = branchEnvelope.snapshot.meta.snapshot_id;
const parityMismatchCount = parityProof.mismatchClasses.length;
const artifactParity = artifactProof.parity;
const previewPlanSource = previewPlan.source_branch_id;
const previewPlanStrategy = previewPlan.selected_semantics.strategy_name;
const previewPlanResolution = previewPlan.resolution_plan?.divergence ?? null;
const previewPlanNodeMapEntry = previewPlan.node_map[0]?.source_node ?? null;
const previewPlanDecision = previewPlan.node_plan[0]?.decision ?? null;
const previewPlanAdoptionSource = previewPlan.adoption_core[0]?.source_node ?? null;
const previewPlanCarryPolicy = previewPlan.adoption_policy[0]?.runtime_artifact ?? null;
const previewPlanDigest = previewPlanProof.proof.planDigest;
const previewResultTarget = previewResult.target_branch;
const previewResultRecordNode = previewResult.records[0]?.source_node ?? null;
const previewResultCounter = previewResult.counters.replay_event_count;
const previewResultDigest = previewResultProof.proof.resultDigest;

signals.transaction((tx) => {
  tx.set(count, snapshot.count + commit.touchedNodes);
  // @ts-expect-error computed handles must stay read-only inside transactions
  tx.set(doubled, 4);
});

// @ts-expect-error branded callable handles must not accept structural forgeries
const forgedSignal: InputSignalHandle<number> = {
  id: "forged",
  get() {
    return 1;
  },
  set() {
    return commit;
  },
};

void constantFromCallback;
void doubledFromCallback;
void generatedFromCallback;
void gatedFromHostCapability;
void viewportLabel;
void onlineLabel;
void clockLabel;
void persistenceLabel;
void legacyDoubledFromSpecAlias;
void legacyPanelFromSpecAlias;
void callbackPanelSnapshot;
void namespace;
void graph;
void graphCount;
void graphDoubled;
void graphPanel;
void graphSnapshot;
void graphCountValue;
void graphDoubledValue;
void graphPanelValue;
void graphDoubledHandle;
void graphDescriptorKind;
void graphInputByName;
void graphInputDescriptor;
void graphInputSnapshot;
void graphInputCountValue;
void graphOperationalContract;
void graphOperationalWriteId;
void graphOperationalPatchCount;
void graphOperationalAuthority;
void graphWriteCommit;
void graphPatchCommit;
void graphResetCommit;
void graphApplyCommit;
void graphTransactionCommit;
void graphWhy;
void graphReplay;
void graphLineage;
void graphReadVersions;
void graphPublicationSummary;
void graphCompatibilityDefinition;
void graphExportDefinition;
void graphExportSnapshot;
void graphImportPosture;
void graphCompatibilityCountId;
void graphContractHistory;
void graphCompatibilityPanelId;
void graphCompatibilityRecipeId;
void graphDiagnostics;
void graphHistory;
void graphSpecialist;
void graphAdapters;
void graphOutputByName;
void restoredGraph;
void restoredGraphContract;
void restoredGraphContractHistory;
void restoredGraphImportPosture;
void restoredGraphRead;
void restoredGraphReadInputs;
void restoredGraphCompatibility;
void restoredGraphDiagnostics;
void restoredGraphHistory;
void itemDetailGraph;
void itemDetailGraphOutput;
void itemDetailGraphSummary;
void itemDetailGraphCompatibility;
void itemDetailGraphExportDefinition;
void itemDetailGraphExportSnapshot;
void itemDetailGraphImportPosture;
void itemDetailGraphInput;
void itemDetailGraphInputs;
void itemDetailGraphInputDescriptor;
void itemDetailGraphCompatibilityInputId;
void itemDetailGraphCompatibilityOutputId;
void itemDetailGraphContractHistory;
void restoredItemDetailGraph;
void restoredItemDetailGraphContract;
void restoredItemDetailGraphContractHistory;
void restoredItemDetailGraphImportPosture;
void restoredItemDetailGraphRead;
void restoredItemDetailGraphReadInputs;
void restoredItemDetailGraphDependency;
void taskEditorOperationalContract;
void taskEditorPatchId;
void taskEditorGraphExportDefinition;
void taskEditorGraphExportSnapshot;
void taskEditorGraphImportPosture;
void taskEditorGraphContractHistory;
void restoredTaskEditorGraph;
void restoredTaskEditorGraphHistory;
void restoredTaskEditorGraphImportPosture;
void taskEditorGraphPatchCommit;
void taskEditorGraphApplyCommit;
void taskEditorGraphTxCommit;
void authorityGraph;
void authorityGraphContract;
void authorityGraphOperationalContract;
void authorityGraphRead;
void authorityGraphInputRead;
void authorityGraphReadOnlyAuthority;
void authorityGraphImportedAuthority;
void authorityGraphPatchId;
void authorityGraphWriteCommit;
void authorityGraphPatchCommit;
void authorityGraphResetCommit;
void authorityGraphApplyCommit;
void authorityGraphTransactionCommit;
void explicitCallbackPanel;
void panelSnapshotFromRead;
void countSnapshotFromRead;
void nameInput;
void scopedSignals;
void nestedScopedSignals;
void scopedDescriptor;
void scopedCanonicalCountId;
void scopedCounterGraph;
void definitions;
void runtimeEnvelope;
void runtimeEnvelopeRestoreMode;
void transportReport;
void restoredBranchId;
void snapshotExplanationRetention;
void checkpointImage;
void diagnosticGraph;
void maybeUnavailable;
void proof;
void proofVersion;
void proofDigest;
void diagnostics;
void history;
void specialist;
void currentBranch;
void previewBranch;
void branchReplay;
void branchSnapshot;
void branchEnvelope;
void branchSnapshotRestoreMode;
void branchEnvelopeRestoreMode;
void branchProof;
void parityProof;
void artifactProof;
void previewPlan;
void previewPlanProof;
void previewResult;
void previewResultProof;
void graphProfile;
void specialistGraphProfile;
void specialistTouchedNodes;
void callbackNodeIds;
void latestHistoryNode;
void latestFailureMessage;
void latestFrontierSeedCount;
void latestHostCapabilityEventKind;
void latestHostCapabilityEventQueuedCount;
void latestHostCapabilityDeniedIds;
void hostCapabilityReport;
void hostCapabilityLineageDigest;
void hostCapabilityBreadthDigest;
void hostCapabilityLineageEntry;
void hostCapabilityBreadthFamily;
void hostCapabilityInvalidationCount;
void hostCapabilityReadCount;
void hostCapabilityPollCount;
void hostCapabilityNoOpPollCount;
void hostCapabilityManualCommitCount;
void hostCapabilityNoOpManualCommitCount;
void hostCapabilityReevaluationCount;
void hostCapabilityCompatibilityDenialCount;
void hostCapabilityUnavailabilityArtifactCount;
void hostCapabilityBroadFanoutDenialCount;
void latestHostCapabilityRead;
void recentHostCapabilityEvents;
void unavailableHostCapabilityTransport;
void latestCallbackCurrentReads;
void branchReplayCallback;
void branchSnapshotBranchId;
void branchEnvelopeSnapshotId;
void parityMismatchCount;
void artifactParity;
void previewPlanSource;
void previewPlanStrategy;
void previewPlanResolution;
void previewPlanNodeMapEntry;
void previewPlanDecision;
void previewPlanAdoptionSource;
void previewPlanCarryPolicy;
void previewPlanDigest;
void previewResultTarget;
void previewResultRecordNode;
void previewResultCounter;
void previewResultDigest;
void viewportState;
void viewportSize;
void viewportWidth;
void viewportHeight;
void viewportDescriptor;
void visibilityState;
void visibilityStateNow;
void visibilityFlag;
void visibilityDescriptor;
void onlineState;
void onlineStateNow;
void onlineFlag;
void onlineDescriptor;
void clockTick;
void clockNow;
void clockDescriptor;
void persistedDraft;
void persistenceValue;
void persistenceMode;
void persistenceRevision;
void persistenceDescriptor;
void persistenceCommit;
void forgedSignal;
