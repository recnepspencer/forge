export function buildRuntimeSmokeSource(packageName) {
  return `import init, { clockCapability, createSignals, hostCapabilityPlan, onlineCapability, persistenceCapability, viewportCapability, visibilityCapability } from "${packageName}";
import * as reactApi from "${packageName}/react";

await init();
let visibilityState = "visible";
let visibilityListener = null;
let viewportState = { width: 1280, height: 720 };
let viewportListener = null;
let onlineState = "online";
let onlineListener = null;
let clockTick = 0;
let persistedDraft = { mode: "draft", revision: 1 };
const signals = createSignals({
  hostCapabilities: hostCapabilityPlan({
    visibility: visibilityCapability({
      source: {
        current() {
          return visibilityState;
        },
        subscribe(listener) {
          visibilityListener = listener;
          return () => {
            visibilityListener = null;
          };
        },
      },
      compatibility: "LiveOnly",
    }),
    viewport: viewportCapability({
      source: {
        current() {
          return viewportState;
        },
        subscribe(listener) {
          viewportListener = listener;
          return () => {
            viewportListener = null;
          };
        },
      },
    }),
    online: onlineCapability({
      source: {
        current() {
          return onlineState;
        },
        subscribe(listener) {
          onlineListener = listener;
          return () => {
            onlineListener = null;
          };
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
const count = signals.input(1, { id: "count" });
const doubled = signals.computed(() => count() * 2, { id: "doubled" });
const visibleLabel = signals.computed(
  () => (signals.host.visibility?.isVisible() ? "visible" : "hidden"),
  { id: "visibleLabel" },
);
const viewportLabel = signals.computed(
  () => (signals.host.viewport?.width() ?? 0) + "x" + (signals.host.viewport?.height() ?? 0),
  { id: "viewportLabel" },
);
const onlineLabel = signals.computed(
  () => (signals.host.online?.isOnline() ? "online" : "offline"),
  { id: "onlineLabel" },
);
const clockLabel = signals.computed(
  () => (signals.host.clock?.now() ?? 0) + count(),
  { id: "clockLabel" },
);
const persistenceLabel = signals.computed(
  () => signals.host.persistence?.value().revision ?? 0,
  { id: "persistenceLabel" },
);
const name = signals.input("Ada", { id: "name" });
const displayLabel = signals.computed(
  () => name().toUpperCase(),
  { id: "displayLabel" },
);
const namingGraph = signals.graph("naming", {
  inputs: {
    name,
  },
  outputs: {
    publicDisplayName: displayLabel,
  },
});
function createEditSessionController(namespace) {
  const serverItemData = namespace.input(null, { id: "serverItemData" });
  const draftEdits = namespace.input({}, { id: "draftEdits" });

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
function createWorkflowController(namespace, editSession) {
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
function createFormController(namespace) {
  const serverValue = namespace.input({
    id: "task-7",
    title: "Ship docs",
    status: "draft",
  }, { id: "serverValue" });
  const draftValue = namespace.input({
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
function createResourceController(namespace, form) {
  const routeParams = namespace.input({
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
function createAuthorityController(namespace) {
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
const itemDetailGraph = signals.graph("itemDetail", (graph) => {
  const editSession = createEditSessionController(graph.scope("editSession"));
  const workflow = createWorkflowController(graph.scope("workflow"), editSession);
  return graph.expose({
    controllers: [editSession, workflow],
  });
});
const pageModalGraph = signals.graph("itemWorkspace", (graph) => {
  const page = createEditSessionController(graph.scope("page"));
  const modal = createEditSessionController(graph.scope("modal"));
  return graph.expose({
    inputs: {
      pageServerItemData: page.inputs.serverItemData,
      modalServerItemData: modal.inputs.serverItemData,
    },
    outputs: {
      pageEffectiveItemData: page.outputs.effectiveItemData,
      modalEffectiveItemData: modal.outputs.effectiveItemData,
    },
  });
});
const repeatedRows = signals.scope("rows");
const row0 = repeatedRows.scope("row-0");
const row1 = repeatedRows.scope("row-1");
const row0Descriptor = row0.descriptor();
const row0Identity = row0.signalIdentity("count");
const row1Identity = row1.signalIdentity("count");
const row0Count = row0.input(0, { id: "count" });
const row1Count = row1.input(1, { id: "count" });
const taskEditorGraph = signals.graph("taskEditor", (graph) => {
  const form = createFormController(graph.scope("form"));
  const resource = createResourceController(graph.scope("resource"), form);
  return graph.expose({
    controllers: [form, resource],
  });
});
const authorityGraph = signals.graph("taskAuthority", (graph) => {
  const authority = createAuthorityController(graph.scope("authority"));
  return graph.expose({
    controllers: [authority],
  });
});
  visibleLabel();
  viewportLabel();
  onlineLabel();
  clockLabel();
  viewportState = { width: 1440, height: 900 };
  viewportListener?.();
  visibilityState = "hidden";
  visibilityListener?.();
  onlineState = "offline";
  onlineListener?.();
  clockTick = 5;
  await new Promise((resolve) => setTimeout(resolve, 15));
  persistedDraft = { mode: "draft", revision: 2 };
  signals.host.persistence?.commit();
  signals.host.persistence?.commit();
  await Promise.resolve();
signals.transaction((tx) => {
  tx.set(count, 2);
});
const history = signals.history();
const branch = history.current_branch();
const previewBranch = history.create_branch("preview");
const replay = history.replay_for_branch(branch.id);
const snapshot = history.snapshot();
const branchSnapshot = history.branch_snapshot(branch.id);
const branchEnvelope = history.branch_snapshot_envelope(branch.id);
const adapters = signals.adapters();
const runtimeEnvelope = adapters.exportRuntimeEnvelope();
const transportReport = adapters.hostCapabilityTransportReport(runtimeEnvelope);
const restoredExact = createSignals();
restoredExact.adapters().restoreExactRuntimeEnvelope(runtimeEnvelope);
const portableImport = createSignals();
let portableImportError = null;
try {
  portableImport.adapters().replaceRuntimeEnvelope(runtimeEnvelope);
} catch (error) {
  portableImportError = {
    code: error?.code ?? null,
    message: error?.message ?? String(error),
  };
}
const portableImportDiagnostics = portableImport.diagnostics();
const portableImportLatestHostCapabilityEvent =
  portableImportDiagnostics.latestHostCapabilityEvent();
const portableImportRecentHostCapabilityEvents =
  portableImportDiagnostics.recentHostCapabilityEvents();
const portableImportHostCapabilityReport =
  portableImportDiagnostics.hostCapabilityReport();
const portableImportDeniedCallbackIds =
  portableImportRecentHostCapabilityEvents.flatMap((event) => event?.deniedCallbackIds ?? []);
const portableImportPerformanceSummary = portableImportDiagnostics.performanceSummary();
const unavailableVisibleLabel =
  runtimeEnvelope.definitions.unavailableCallbacks.find((artifact) => artifact.id === "visibleLabel");
const unavailableViewportLabel =
  runtimeEnvelope.definitions.unavailableCallbacks.find((artifact) => artifact.id === "viewportLabel");
const unavailableOnlineLabel =
  runtimeEnvelope.definitions.unavailableCallbacks.find((artifact) => artifact.id === "onlineLabel");
const unavailableClockLabel =
  runtimeEnvelope.definitions.unavailableCallbacks.find((artifact) => artifact.id === "clockLabel");
const unavailablePersistenceLabel =
  runtimeEnvelope.definitions.unavailableCallbacks.find((artifact) => artifact.id === "persistenceLabel");
const specialist = signals.specialist();
const specialistGraphSummary = specialist.graphSummary();
const specialistEvaluateDirty = specialist.evaluateDirty();
const diagnostics = signals.diagnostics();
const performanceSummary = diagnostics.performanceSummary();
const hostCapabilityReport = diagnostics.hostCapabilityReport();
const latestHostCapabilityEvent = diagnostics.latestHostCapabilityEvent();
const recentHostCapabilityEvents = diagnostics.recentHostCapabilityEvents();
const previewPlan = history.plan_merge_policy_preview({
  source_branch_id: previewBranch.id,
  target_branch_id: branch.id,
});
const previewPlanProof = history.plan_merge_policy_preview_with_proof({
  source_branch_id: previewBranch.id,
  target_branch_id: branch.id,
});
const previewResult = history.merge_branches_policy_preview({
  source_branch_id: previewBranch.id,
  target_branch_id: branch.id,
});
const itemDetailGraphSummary = itemDetailGraph.summary();
const itemDetailGraphRead = itemDetailGraph.read();
const itemDetailGraphInputs = itemDetailGraph.readInputs();
const itemDetailGraphOutputId = itemDetailGraph.output("submitReadiness").id;
const itemDetailGraphContract = itemDetailGraph.contract();
const itemDetailGraphOperationalContract = itemDetailGraph.operationalContract();
itemDetailGraph.writeInputs({
  serverItemData: {
    workflow_target_state_id: "ready",
  },
});
itemDetailGraph.patchInputs({
  draftEdits: {
    title: "Ship docs",
  },
});
itemDetailGraph.transaction((tx) => {
  tx.set("draftEdits", {
    title: "Ready to ship",
    workflow_target_state_id: "ready",
  });
});
itemDetailGraph.apply({
  writes: {
    serverItemData: {
      workflow_target_state_id: "done",
    },
  },
  patches: {
    draftEdits: {
      reviewState: "approved",
    },
  },
  commands: {},
});
const itemDetailGraphOperationalSnapshot = itemDetailGraph.readInputs();
itemDetailGraph.resetInputs();
const itemDetailGraphResetSnapshot = itemDetailGraph.readInputs();
const itemDetailGraphDiagnostics = itemDetailGraph.inspectDiagnostics();
const itemDetailGraphHistory = itemDetailGraph.inspectHistory();
const itemDetailGraphCompatibility = itemDetailGraph.exportCompatibilityDefinition();
const itemDetailGraphExportDefinition = itemDetailGraph.exportDefinition();
const itemDetailGraphExportSnapshot = itemDetailGraph.exportSnapshot();
const itemDetailGraphImportPosture = itemDetailGraph.importPosture();
const importedItemDetailGraph = createSignals().importGraph(
  itemDetailGraphExportDefinition,
  itemDetailGraphExportSnapshot,
);
const importedItemDetailGraphContract = importedItemDetailGraph.contract();
const importedItemDetailGraphContractHistory = importedItemDetailGraph.contractHistory();
const importedItemDetailGraphImportPosture = importedItemDetailGraph.importPosture();
const importedItemDetailGraphRead = importedItemDetailGraph.read();
const importedItemDetailGraphInputs = importedItemDetailGraph.readInputs();
const itemDetailGraphDependency =
  itemDetailGraphDiagnostics.dependenciesForOutput("submitReadiness");
const itemDetailGraphContractSummary = itemDetailGraphDiagnostics.contractSummary();
const itemDetailGraphContractDelta = itemDetailGraph.contractDelta({
  ...itemDetailGraphContract,
  outputs: {},
});
const taskEditorGraphContract = taskEditorGraph.contract();
const taskEditorGraphOperationalContract = taskEditorGraph.operationalContract();
taskEditorGraph.patchInputs({
  draftValue: {
    status: "published",
  },
});
taskEditorGraph.apply({
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
const taskEditorGraphDiagnostics = taskEditorGraph.inspectDiagnostics();
const taskEditorGraphHistory = taskEditorGraph.inspectHistory();
const taskEditorGraphCompatibility = taskEditorGraph.exportCompatibilityDefinition();
const authorityGraphContract = authorityGraph.contract();
const authorityGraphOperationalContract = authorityGraph.operationalContract();
authorityGraph.writeInputs({
  draftValue: {
    title: "Ready to ship",
  },
});
authorityGraph.patchInputs({
  draftValue: {
    title: "Approved",
  },
});
authorityGraph.transaction((tx) => {
  tx.set("draftValue", {
    title: "Queued",
  });
});
const authorityGraphRead = authorityGraph.read();
const authorityGraphInputs = authorityGraph.readInputs();
const authorityGraphDiagnostics = authorityGraph.inspectDiagnostics();
const authorityGraphHistory = authorityGraph.inspectHistory();
const authorityGraphCompatibility = authorityGraph.exportCompatibilityDefinition();

const summary = {
  hasInit: typeof init === "function",
  hasCreateSignals: typeof createSignals === "function",
  reactKeys: Object.keys(reactApi).sort(),
  doubled: doubled(),
  visibleLabel: visibleLabel(),
  viewportLabel: viewportLabel(),
  onlineLabel: onlineLabel(),
  clockLabel: clockLabel(),
  persistenceLabel: persistenceLabel(),
  visibilityState: signals.host.visibility?.state() ?? null,
  visibilityCompatibility: signals.host.visibility?.descriptor().compatibility ?? null,
  viewportSize: signals.host.viewport?.size() ?? null,
  viewportCompatibility: signals.host.viewport?.descriptor().compatibility ?? null,
  onlineState: signals.host.online?.state() ?? null,
  onlineCompatibility: signals.host.online?.descriptor().compatibility ?? null,
  clockNow: signals.host.clock?.now() ?? null,
  clockCompatibility: signals.host.clock?.descriptor().compatibility ?? null,
  persistenceValue: signals.host.persistence?.value() ?? null,
  persistenceCompatibility: signals.host.persistence?.descriptor().compatibility ?? null,
  nameValue: name(),
  namingGraphInputId: namingGraph.contract().inputs.name,
  namingGraphOutputId: namingGraph.output("publicDisplayName").id,
  namingGraphDescriptor: namingGraph.descriptors()[0],
  namingGraphCompatibilityOutputId:
    namingGraph.exportCompatibilityDefinition().contract.outputs.publicDisplayName,
  runtimeEnvelopeRestoreMode: runtimeEnvelope.runtimeEnvelopeRestoreMode,
  runtimeEnvelopeUnavailableHostCapabilityCompatibility:
    unavailableVisibleLabel?.hostCapabilityReads[0]?.compatibility ?? null,
  runtimeEnvelopeUnavailablePortableOutcome:
    unavailableVisibleLabel?.hostCapabilityTransports[0]?.portableImportOutcome ?? null,
  runtimeEnvelopeUnavailableExactRestoreOutcome:
    unavailableVisibleLabel?.hostCapabilityTransports[0]?.exactRestoreOutcome ?? null,
  runtimeEnvelopeUnavailablePortableReason:
    unavailableVisibleLabel?.hostCapabilityTransports[0]?.portableImportReason ?? null,
  runtimeEnvelopeUnavailableOnlineCompatibility:
    unavailableOnlineLabel?.hostCapabilityReads[0]?.compatibility ?? null,
  runtimeEnvelopeUnavailableViewportCompatibility:
    unavailableViewportLabel?.hostCapabilityReads[0]?.compatibility ?? null,
  runtimeEnvelopeUnavailableViewportPortableOutcome:
    unavailableViewportLabel?.hostCapabilityTransports[0]?.portableImportOutcome ?? null,
  runtimeEnvelopeUnavailableViewportPortableReason:
    unavailableViewportLabel?.hostCapabilityTransports[0]?.portableImportReason ?? null,
  runtimeEnvelopeUnavailableOnlinePortableOutcome:
    unavailableOnlineLabel?.hostCapabilityTransports[0]?.portableImportOutcome ?? null,
  runtimeEnvelopeUnavailableOnlinePortableReason:
    unavailableOnlineLabel?.hostCapabilityTransports[0]?.portableImportReason ?? null,
  runtimeEnvelopeUnavailableClockCompatibility:
    unavailableClockLabel?.hostCapabilityReads[0]?.compatibility ?? null,
  runtimeEnvelopeUnavailableClockPortableOutcome:
    unavailableClockLabel?.hostCapabilityTransports[0]?.portableImportOutcome ?? null,
  runtimeEnvelopeUnavailableClockPortableReason:
    unavailableClockLabel?.hostCapabilityTransports[0]?.portableImportReason ?? null,
  runtimeEnvelopeUnavailablePersistenceCompatibility:
    unavailablePersistenceLabel?.hostCapabilityReads[0]?.compatibility ?? null,
  runtimeEnvelopeUnavailablePersistencePortableOutcome:
    unavailablePersistenceLabel?.hostCapabilityTransports[0]?.portableImportOutcome ?? null,
  runtimeEnvelopeUnavailableCallbackCount:
    runtimeEnvelope.definitions.unavailableCallbacks.length,
  runtimeEnvelopeUnavailableCurrentReads: unavailableVisibleLabel?.currentReads ?? [],
  latestHostCapabilityEventKind: latestHostCapabilityEvent?.kind ?? null,
  latestHostCapabilityEventQueuedCount: latestHostCapabilityEvent?.queuedInvalidationCount ?? null,
  recentHostCapabilityEventCount: recentHostCapabilityEvents.length,
  hostCapabilityReadCount: performanceSummary.hostCapabilityReadCount ?? null,
  hostCapabilityPollCount: performanceSummary.hostCapabilityPollCount ?? null,
  hostCapabilityNoOpPollCount: performanceSummary.hostCapabilityNoOpPollCount ?? null,
  hostCapabilityManualCommitCount: performanceSummary.hostCapabilityManualCommitCount ?? null,
  hostCapabilityNoOpManualCommitCount: performanceSummary.hostCapabilityNoOpManualCommitCount ?? null,
  hostCapabilityInvalidationCount: performanceSummary.hostCapabilityInvalidationCount ?? null,
  hostCapabilityInvalidationBatchFlushCount: performanceSummary.hostCapabilityInvalidationBatchFlushCount ?? null,
  hostCapabilityReevaluationCount: performanceSummary.hostCapabilityReevaluationCount ?? null,
  hostCapabilityInvalidationTouchedNodeCount: performanceSummary.hostCapabilityInvalidationTouchedNodeCount ?? null,
  hostCapabilityCompatibilityDenialCount: performanceSummary.hostCapabilityCompatibilityDenialCount ?? null,
  hostCapabilityUnavailabilityArtifactCount: performanceSummary.hostCapabilityUnavailabilityArtifactCount ?? null,
  hostCapabilityBroadFanoutDenialCount: performanceSummary.hostCapabilityBroadFanoutDenialCount ?? null,
  hostCapabilityReportDigest: hostCapabilityReport.digest,
  hostCapabilityReportLineageDigest: hostCapabilityReport.lineageDigest,
  hostCapabilityReportBreadthDigest: hostCapabilityReport.breadthDigest,
  hostCapabilityReportFamilyCount: hostCapabilityReport.families.length,
  hostCapabilityReportLineageCount: hostCapabilityReport.lineage.length,
  hostCapabilityReportMaxTouchedNodes: hostCapabilityReport.breadth.maxTouchedNodes,
  hostCapabilityReportMaxReevaluatedNodes: hostCapabilityReport.breadth.maxReevaluatedNodes,
  transportReportDigest: transportReport.digest,
  transportReportUnavailableArtifactCount: transportReport.totals.unavailableArtifactCount,
  transportReportDeniedFamilyCount: transportReport.totals.deniedFamilyCount,
  transportReportUnavailableFamilyCount: transportReport.totals.unavailableFamilyCount,
  portableImportLatestHostCapabilityEventKind:
    portableImportLatestHostCapabilityEvent?.kind ?? null,
  portableImportLatestHostCapabilityEventQueuedCount:
    portableImportLatestHostCapabilityEvent?.queuedInvalidationCount ?? null,
  portableImportLatestHostCapabilityEventDeniedIds:
    portableImportLatestHostCapabilityEvent?.deniedCallbackIds ?? [],
  portableImportDeniedCallbackIds,
  portableImportRecentHostCapabilityEventCount:
    portableImportRecentHostCapabilityEvents.length,
  portableImportHostCapabilityCompatibilityDenialCount:
    portableImportPerformanceSummary.hostCapabilityCompatibilityDenialCount ?? null,
  portableImportHostCapabilityReportDigest:
    portableImportHostCapabilityReport.digest,
  portableImportHostCapabilityReportLineageDigest:
    portableImportHostCapabilityReport.lineageDigest,
  portableImportHostCapabilityReportFamilyCount:
    portableImportHostCapabilityReport.families.length,
  branchIdType: typeof branch.id,
  replayFrameCount: replay.frames.length,
  replayHasCallback: replay.frames.some((frame) => frame.callback?.id === "doubled"),
  snapshotBranchId: snapshot.snapshot.meta.branch_id,
  branchSnapshotBranchId: branchSnapshot.meta.branch_id,
  branchSnapshotRestoreMode: branchSnapshot.snapshotRestoreMode,
  branchEnvelopeRestoreMode: branchEnvelope.snapshotEnvelopeRestoreMode,
  exportedPolicyPreset: runtimeEnvelope.definitions.policy.preset,
  snapshotPolicyTier: snapshot.snapshot.meta.runtime_policy.tier,
  snapshotReplayHead: snapshot.snapshot.meta.replay_head,
  snapshotExplanationRetention: snapshot.snapshot.meta.artifact_retention.explanation_retention,
  restoredExactDoubled: restoredExact.read("doubled"),
  portableImportErrorCode: portableImportError?.code ?? null,
  portableImportErrorMessage: portableImportError?.message ?? null,
  specialistGraphProfile: specialistGraphSummary.profile,
  specialistTouchedNodes: specialistEvaluateDirty.touchedNodes,
  previewBranchId: previewBranch.id,
  previewPlanSource: previewPlan.source_branch_id,
  previewPlanStrategy: previewPlan.selected_semantics.strategy_name,
  previewPlanResolution: previewPlan.resolution_plan?.divergence ?? null,
  previewPlanNodeMapIsArray: Array.isArray(previewPlan.node_map),
  previewPlanNodePlansAreTyped:
    Array.isArray(previewPlan.node_plan) &&
    previewPlan.node_plan.every((entry) => typeof entry.decision === "string"),
  previewPlanAdoptionCoreIsTyped:
    Array.isArray(previewPlan.adoption_core) &&
    previewPlan.adoption_core.every((entry) => typeof entry.source_node === "string"),
  previewPlanAdoptionPolicyIsTyped:
    Array.isArray(previewPlan.adoption_policy) &&
    previewPlan.adoption_policy.every((entry) => typeof entry.runtime_artifact === "string"),
  previewPlanDigest: previewPlanProof.proof.planDigest,
  previewResultCounter: previewResult.counters.replay_event_count,
  previewResultRecordsAreTyped:
    Array.isArray(previewResult.records) &&
    previewResult.records.every(
      (record) => typeof record.source_node === "string" && typeof record.action === "string",
    ),
  itemDetailGraphId: itemDetailGraphSummary.id,
  itemDetailGraphInputNames: itemDetailGraphSummary.inputNames,
  itemDetailGraphOutputNames: itemDetailGraphSummary.outputNames,
  itemDetailGraphInputKeys: Object.keys(itemDetailGraphInputs).sort(),
  itemDetailGraphReadKeys: Object.keys(itemDetailGraphRead).sort(),
  itemDetailGraphSubmitOutputId: itemDetailGraphOutputId,
  itemDetailGraphOperationalWriteId:
    itemDetailGraphOperationalContract.writes.serverItemData,
  itemDetailGraphOperationalPatchId:
    itemDetailGraphOperationalContract.patches.draftEdits,
  itemDetailGraphOperationalAuthority:
    itemDetailGraphOperationalContract.authorities.draftEdits.authority,
  itemDetailGraphOperationalSupportsPatch:
    itemDetailGraphOperationalContract.authorities.draftEdits.supportsPatch,
  itemDetailGraphOperationalServerState:
    itemDetailGraphOperationalSnapshot.serverItemData?.workflow_target_state_id ?? null,
  itemDetailGraphOperationalDraftTitle:
    itemDetailGraphOperationalSnapshot.draftEdits?.title ?? null,
  itemDetailGraphOperationalDraftReviewState:
    itemDetailGraphOperationalSnapshot.draftEdits?.reviewState ?? null,
  itemDetailGraphResetDraftKeys:
    Object.keys(itemDetailGraphResetSnapshot.draftEdits ?? {}).sort(),
  itemDetailGraphResetServerValue:
    itemDetailGraphResetSnapshot.serverItemData,
  row0ScopePath: row0Descriptor.path.map((segment) => segment.id),
  row0ScopeParent: row0Descriptor.identity.parentScopeId,
  row0SignalCanonicalId: row0Identity.canonicalId,
  row0SignalGraphId: row0Identity.graphId,
  row0SignalRootScopeId: row0Identity.rootScopeId,
  row1SignalCanonicalId: row1Identity.canonicalId,
  row0HandleId: row0Count.id,
  row1HandleId: row1Count.id,
  itemDetailGraphContractInputId: itemDetailGraphContract.inputs.serverItemData,
  itemDetailGraphWhyId: itemDetailGraphDiagnostics.outputs.submitReadiness.why.id,
  itemDetailGraphInputWhyId: itemDetailGraphDiagnostics.inputs.serverItemData.why.id,
  itemDetailGraphReplayFrameCount:
    itemDetailGraphHistory.outputs.submitReadiness.replay.frames.length,
  itemDetailGraphInputReplayFrameCount:
    itemDetailGraphHistory.inputs.serverItemData.replay.frames.length,
  itemDetailGraphCompatibilityInputId:
    itemDetailGraphCompatibility.inputs.serverItemData,
  itemDetailGraphCompatibilityContractInputId:
    itemDetailGraphCompatibility.contract.inputs.serverItemData,
  itemDetailGraphCompatibilityOutputId:
    itemDetailGraphCompatibility.outputs.submitReadiness,
  itemDetailGraphDependencyInputNames:
    [...itemDetailGraphDependency.publicInputNames].sort(),
  itemDetailGraphDependencySourceIds:
    [...itemDetailGraphDependency.publicInputSourceIds].sort(),
  itemDetailGraphContractSummaryOutputCount:
    itemDetailGraphContractSummary.outputCount,
  itemDetailGraphContractDeltaAddedOutputs:
    itemDetailGraphContractDelta.outputs.added,
  importedItemDetailGraphOutputId:
    importedItemDetailGraph.exportCompatibilityDefinition().outputs.submitReadiness,
  importedItemDetailGraphReadiness:
    importedItemDetailGraphRead.submitReadiness?.targetStateId ?? null,
  importedItemDetailGraphInputCount:
    importedItemDetailGraphInputs.serverItemData?.workflow_target_state_id ?? null,
  importedItemDetailGraphContractInputId:
    importedItemDetailGraphContract.inputs.serverItemData,
  importedItemDetailGraphHistoryChanged:
    importedItemDetailGraphContractHistory.changedSinceBaseline,
  importedItemDetailGraphHistoryRestoreMode:
    importedItemDetailGraphContractHistory.restoreMode,
  itemDetailGraphImportPortableMode:
    itemDetailGraphImportPosture.portableImport,
  itemDetailGraphImportHydrateMode:
    itemDetailGraphImportPosture.hydrate,
  importedItemDetailGraphExactRestoreMode:
    importedItemDetailGraphImportPosture.exactRestoreMode,
  pageModalPageInputId: pageModalGraph.contract().inputs.pageServerItemData,
  pageModalModalInputId: pageModalGraph.contract().inputs.modalServerItemData,
  pageModalPageOutputId: pageModalGraph.contract().outputs.pageEffectiveItemData,
  pageModalModalOutputId: pageModalGraph.contract().outputs.modalEffectiveItemData,
  taskEditorGraphInputId: taskEditorGraphContract.inputs.serverValue,
  taskEditorGraphRouteParamsId: taskEditorGraphContract.inputs.routeParams,
  taskEditorGraphOutputId: taskEditorGraphContract.outputs.submitAvailability,
  taskEditorGraphPatchId: taskEditorGraphOperationalContract.patches.draftValue,
  taskEditorGraphPatchedStatus:
    taskEditorGraph.readInputs().draftValue.status ?? null,
  taskEditorGraphRouteParamTaskId:
    taskEditorGraph.readInputs().routeParams.taskId,
  taskEditorGraphInputWhyId: taskEditorGraphDiagnostics.inputs.routeParams.why.id,
  taskEditorGraphOutputWhyId: taskEditorGraphDiagnostics.outputs.submitAvailability.why.id,
  taskEditorGraphInputReplayFrameCount:
    taskEditorGraphHistory.inputs.routeParams.replay.frames.length,
  taskEditorGraphOutputReplayFrameCount:
    taskEditorGraphHistory.outputs.submitAvailability.replay.frames.length,
  taskEditorGraphCompatibilityInputId:
    taskEditorGraphCompatibility.contract.inputs.routeParams,
  taskEditorGraphCompatibilityOutputId:
    taskEditorGraphCompatibility.contract.outputs.submitAvailability,
  authorityGraphInputId: authorityGraphContract.inputs.serverValue,
  authorityGraphOutputId: authorityGraphContract.outputs.effectiveValue,
  authorityGraphWritablePatchId: authorityGraphOperationalContract.patches.draftValue,
  authorityGraphReadOnlyAuthority:
    authorityGraphOperationalContract.authorities.serverValue.authority,
  authorityGraphImportedAuthority:
    authorityGraphOperationalContract.authorities.externalParams.authority,
  authorityGraphDraftTitle: authorityGraphInputs.draftValue.title ?? null,
  authorityGraphTaskId: authorityGraphRead.effectiveValue.taskId ?? null,
  authorityGraphInputWhyId: authorityGraphDiagnostics.inputs.externalParams.why.id,
  authorityGraphOutputReplayFrameCount:
    authorityGraphHistory.outputs.effectiveValue.replay.frames.length,
  authorityGraphCompatibilityInputId:
    authorityGraphCompatibility.contract.inputs.externalParams,
};

console.log(JSON.stringify(summary));
`;
}
