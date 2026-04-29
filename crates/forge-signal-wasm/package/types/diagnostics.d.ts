import type {
  AspectId,
  KeyedRecipeFamilySpec,
  KeyedSourceFamilySpec,
  RecipeSpec,
  SourceSpec,
} from "./model.js";

export interface HealthSummary {
  activeNodeCount: number;
  cleanNodeCount: number;
  maybeStaleNodeCount: number;
  dirtyNodeCount: number;
  dependencyEdgeCount: number;
  subscriberEdgeCount: number;
}

export interface WhySummary {
  id: string;
  node: string;
  apiFamily: string | null;
  recipeFamily: string | null;
  state: string;
  upstream: ReadonlyArray<string>;
  changedRegions: ReadonlyArray<string>;
  propagationSuppressed: boolean;
  outputChange: string | null;
  outputIdentity: string | null;
  callback: CallbackWhySummary | null;
}

export interface CallbackWhySummary {
  purityPosture: string;
  currentReads: ReadonlyArray<string>;
  registered: boolean;
  unavailableReason?: string;
  tokenSlot?: number;
  tokenGeneration?: number;
  lastRuntimeReadBreadth: number;
  lastDependencyPatch: CallbackDependencyPatchSummary | null;
  lastFailure: CallbackFailureSummary | null;
}

export interface CallbackDependencyPatchSummary {
  previousReads: ReadonlyArray<string>;
  currentReads: ReadonlyArray<string>;
  addedCount: number;
  removedCount: number;
  retainedCount: number;
  runtimeReadBreadth: number;
}

export interface CallbackFailureSummary {
  class: string;
  message: string;
  code: string | null;
}

export interface UnavailableCallbackArtifact {
  id: string;
  signalKind: string;
  reason: string;
  currentReads: ReadonlyArray<string>;
}

export interface RuntimeDefinitionEnvelope {
  policy: unknown;
  sources: ReadonlyArray<SourceSpec>;
  recipes: ReadonlyArray<RecipeSpec>;
  sourceFamilies: ReadonlyArray<KeyedSourceFamilySpec>;
  recipeFamilies: ReadonlyArray<KeyedRecipeFamilySpec>;
  unavailableCallbacks: ReadonlyArray<UnavailableCallbackArtifact>;
}

export interface WebPerformanceSummary {
  activeHandleCount: number;
  activeCallbackCount: number;
  activeComputeCallbackCount: number;
  activeComputeCollectorCount: number;
  matchedWatcherBreadth: number;
  deliveredObservationCount: number;
  rollbackSuppressedDeliveryCount: number;
  serialExecutorUsageCount: number;
  parallelExecutorUsageCount: number;
  outputSerializationCount: number;
  outputSerializationBreadth: number;
  jsCallbackInvocationCount: number;
  jsCallbackFailureCount: number;
  computeCallbackRegistrationCount: number;
  computeCallbackDisposalCount: number;
  computeCallbackInvocationCount: number;
  computeCallbackFailureCount: number;
  computeCallbackGenerationMismatchDenialCount: number;
  computeCallbackSelfReadDenialCount: number;
  computeCallbackDynamicCycleDenialCount: number;
  computeCallbackPromiseReturnDenialCount: number;
  computeCallbackInvalidReturnDenialCount: number;
  computeCallbackCollectorInstallationCount: number;
  computeCallbackCaptureCount: number;
  computeCallbackCapturedReadCount: number;
  computeCallbackReturnSerializationBreadth: number;
  computeCallbackAllocationCount: number;
  computeCallbackReuseCount: number;
  computeCallbackDependencyPatchCount: number;
  computeCallbackDependencyPatchAddedCount: number;
  computeCallbackDependencyPatchRemovedCount: number;
  computeCallbackDependencyPatchRetainedCount: number;
  computeCallbackRuntimeReadBreadth: number;
  computeCallbackConstantNoSignalReadClassificationCount: number;
  computeCallbackSignalTrackedClassificationCount: number;
  computeCallbackMissingUnavailabilityCount: number;
  compatibilityReadCount: number;
  compatibilityReadBreadth: number;
}
