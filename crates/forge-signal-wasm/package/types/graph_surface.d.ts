import type { RunSummary, SignalValue, VersionSummary, AspectId } from "./model.js";
import type { ControllerContract, ControllerContractDefinition } from "./controller_surface.js";
import type {
  ExecutionHistorySummary,
  ExecutionHistorySurfaceSummary,
  FlowSurfaceSummary,
  GraphSummary,
  LineageSummary,
  ObservationSurfaceSummary,
  ReplaySummary,
  RuntimeDefinitionEnvelope,
  WhySummary,
} from "./diagnostics.js";

export type GraphReadableHandle<T = SignalValue> =
  | import("./callable_surface.js").InputSignalHandle<T>
  | import("./callable_surface.js").ComputedSignalHandle<T>
  | import("./callable_surface.js").OutputSignalHandle<T>;

export type GraphInputHandle<T = SignalValue> =
  import("./callable_surface.js").InputSignalHandle<T>;

export type GraphPublicInputAuthority = "writable" | "readOnly" | "imported";

export interface PublicGraphInputOptions {
  authority?: GraphPublicInputAuthority;
}

export interface PublicGraphInputContractEntry<
  THandle extends GraphInputHandle = GraphInputHandle,
> {
  handle: THandle;
  authority: GraphPublicInputAuthority;
}

export interface PublishedGraphInputDescriptor {
  inputName: string;
  sourceId: string;
  sourceKind: "input";
  authority: GraphPublicInputAuthority;
}

export interface PublishedGraphDescriptor {
  outputName: string;
  sourceId: string;
  sourceKind: "input" | "computed" | "output";
  publishedId: string;
  publicationKind: "existingOutput" | "synthesizedOutput";
}

export interface PublishedGraphSummary {
  id: string;
  inputCount: number;
  inputNames: ReadonlyArray<string>;
  inputSourceIds: ReadonlyArray<string>;
  outputCount: number;
  outputNames: ReadonlyArray<string>;
  publishedOutputIds: ReadonlyArray<string>;
  sourceIds: ReadonlyArray<string>;
  synthesizedOutputCount: number;
}

export interface PublishedGraphCompatibilityDefinition {
  id: string;
  contract: PublishedGraphContractSurface;
  inputs: Readonly<Record<string, string>>;
  outputs: Readonly<Record<string, string>>;
  inputSourceIds: ReadonlyArray<string>;
  publishedOutputIds: ReadonlyArray<string>;
  sourceIds: ReadonlyArray<string>;
  inputDescriptors: ReadonlyArray<PublishedGraphInputDescriptor>;
  descriptors: ReadonlyArray<PublishedGraphDescriptor>;
  definitions: RuntimeDefinitionEnvelope;
}

export interface PublishedGraphContractSurface {
  graph: PublishedGraphSummary;
  inputs: Readonly<Record<string, string>>;
  outputs: Readonly<Record<string, string>>;
  inputDescriptors: ReadonlyArray<PublishedGraphInputDescriptor>;
  descriptors: ReadonlyArray<PublishedGraphDescriptor>;
}

export interface PublishedGraphOutputDependencyExplanation {
  graphId: string;
  outputName: string;
  publishedId: string;
  sourceId: string;
  publicInputNames: ReadonlyArray<string>;
  publicInputSourceIds: ReadonlyArray<string>;
  transitiveSignalIds: ReadonlyArray<string>;
}

export interface PublishedGraphContractSummary {
  graph: PublishedGraphSummary;
  contract: PublishedGraphContractSurface;
  inputCount: number;
  outputCount: number;
  inputNames: ReadonlyArray<string>;
  outputNames: ReadonlyArray<string>;
  dependencies: Readonly<Record<string, PublishedGraphOutputDependencyExplanation>>;
}

export interface PublishedGraphContractNameRemap {
  name: string;
  previousId: string;
  currentId: string;
}

export interface PublishedGraphContractNameDelta {
  added: ReadonlyArray<string>;
  removed: ReadonlyArray<string>;
  remapped: ReadonlyArray<PublishedGraphContractNameRemap>;
}

export interface PublishedGraphInputDescriptorDeltaEntry {
  inputName: string;
  previousSourceId: string;
  currentSourceId: string;
  previousAuthority: GraphPublicInputAuthority;
  currentAuthority: GraphPublicInputAuthority;
}

export interface PublishedGraphOutputDescriptorDeltaEntry {
  outputName: string;
  previousSourceId: string;
  currentSourceId: string;
  previousPublishedId: string;
  currentPublishedId: string;
  previousSourceKind: "input" | "computed" | "output";
  currentSourceKind: "input" | "computed" | "output";
  previousPublicationKind: "existingOutput" | "synthesizedOutput";
  currentPublicationKind: "existingOutput" | "synthesizedOutput";
}

export interface PublishedGraphContractDelta {
  graphId: string;
  previousGraphId: string | null;
  changed: boolean;
  inputs: PublishedGraphContractNameDelta;
  outputs: PublishedGraphContractNameDelta;
  inputDescriptorsChanged: ReadonlyArray<PublishedGraphInputDescriptorDeltaEntry>;
  outputDescriptorsChanged: ReadonlyArray<PublishedGraphOutputDescriptorDeltaEntry>;
}

export type GraphRestoreMode =
  | "LiveRuntime"
  | "SameRuntimeExact";

export interface PublishedGraphContractHistory {
  graphId: string;
  current: PublishedGraphContractSurface;
  baseline: PublishedGraphContractSurface | null;
  deltas: ReadonlyArray<PublishedGraphContractDelta>;
  changedSinceBaseline: boolean;
  restoreMode: GraphRestoreMode;
  importedFromGraphId: string | null;
}

export interface GraphImportPosture {
  graphId: string;
  exactRestoreMode: GraphRestoreMode;
  portableImport: "Denied";
  portableImportReason: string;
  hydrate: "Deferred";
  hydrateReason: string;
}

export interface PublishedGraphInputAuthorityDescriptor {
  inputName: string;
  sourceId: string;
  authority: GraphPublicInputAuthority;
  supportsWrite: boolean;
  supportsPatch: boolean;
  supportsReset: boolean;
}

export interface ExportedSignalGraphDefinition<
  TOutputs extends GraphOutputDefinitions = GraphOutputDefinitions,
  TInputs extends GraphInputDefinitions = Record<string, never>,
> {
  id: string;
  summary: PublishedGraphSummary;
  contract: PublishedGraphContractSurface;
  operationalContract: PublishedGraphOperationalContractSurface<TInputs>;
  compatibility: PublishedGraphCompatibilityDefinition;
  dependencies: Readonly<Record<string, PublishedGraphOutputDependencyExplanation>>;
  contractSummary: PublishedGraphContractSummary;
  contractHistory: PublishedGraphContractHistory;
  importPosture: GraphImportPosture;
  inputDescriptors: ReadonlyArray<PublishedGraphInputDescriptor>;
  descriptors: ReadonlyArray<PublishedGraphDescriptor>;
}

export interface ExportedSignalGraphSnapshot<
  TOutputs extends GraphOutputDefinitions = GraphOutputDefinitions,
  TInputs extends GraphInputDefinitions = Record<string, never>,
> {
  id: string;
  definition: ExportedSignalGraphDefinition<TOutputs, TInputs>;
  runtimeEnvelope: RuntimeEnvelope;
  snapshotEnvelope: RuntimeSnapshotEnvelope;
  restoreMode: GraphRestoreMode;
  contractHistory: PublishedGraphContractHistory;
  importPosture: GraphImportPosture;
}

export interface PublishedGraphInputDiagnosticsEntry {
  descriptor: PublishedGraphInputDescriptor;
  version: VersionSummary | null;
  why: WhySummary;
}

export interface PublishedGraphOutputDiagnosticsEntry {
  descriptor: PublishedGraphDescriptor;
  version: VersionSummary | null;
  why: WhySummary;
}

export interface PublishedGraphDiagnosticsSurface<
  TOutputDefinitions extends GraphOutputDefinitions,
  TInputDefinitions extends GraphInputDefinitions = Record<string, never>,
> {
  graph: PublishedGraphSummary;
  contract: PublishedGraphContractSurface;
  dependencies: Readonly<Record<string, PublishedGraphOutputDependencyExplanation>>;
  inputDescriptors: ReadonlyArray<PublishedGraphInputDescriptor>;
  descriptors: ReadonlyArray<PublishedGraphDescriptor>;
  inputVersions: ReadonlyArray<VersionSummary>;
  outputVersions: ReadonlyArray<VersionSummary>;
  inputs: {
    readonly [TName in keyof NormalizeGraphRecord<TInputDefinitions>]:
      UnwrapGraphInputHandle<TInputDefinitions[TName]> extends GraphInputHandle<unknown>
        ? PublishedGraphInputDiagnosticsEntry
        : never;
  };
  outputs: {
    readonly [TName in keyof NormalizeGraphRecord<TOutputDefinitions>]:
      TOutputDefinitions[TName] extends GraphReadableHandle<unknown>
        ? PublishedGraphOutputDiagnosticsEntry
        : never;
  };
  input<TName extends keyof NormalizeGraphRecord<TInputDefinitions>>(
    name: TName,
  ): UnwrapGraphInputHandle<TInputDefinitions[TName]> extends GraphInputHandle<unknown>
    ? PublishedGraphInputDiagnosticsEntry
    : never;
  output<TName extends keyof NormalizeGraphRecord<TOutputDefinitions>>(
    name: TName,
  ): TOutputDefinitions[TName] extends GraphReadableHandle<unknown>
    ? PublishedGraphOutputDiagnosticsEntry
    : never;
  dependenciesForOutput<TName extends keyof NormalizeGraphRecord<TOutputDefinitions>>(
    name: TName,
  ): PublishedGraphOutputDependencyExplanation;
  contractSummary(): PublishedGraphContractSummary;
  runtimeGraph: GraphSummary;
  executionHistory: ExecutionHistorySurfaceSummary;
  latestFlow: FlowSurfaceSummary | null;
  latestObservation: ObservationSurfaceSummary | null;
}

export interface PublishedGraphOutputHistoryEntry {
  descriptor: PublishedGraphDescriptor;
  replay: ReplaySummary;
  lineage: LineageSummary;
}

export interface PublishedGraphInputHistoryEntry {
  descriptor: PublishedGraphInputDescriptor;
  replay: ReplaySummary;
  lineage: LineageSummary;
}

export interface PublishedGraphHistorySurface<
  TOutputDefinitions extends GraphOutputDefinitions,
  TInputDefinitions extends GraphInputDefinitions = Record<string, never>,
> {
  graph: PublishedGraphSummary;
  contract: PublishedGraphContractSurface;
  dependencies: Readonly<Record<string, PublishedGraphOutputDependencyExplanation>>;
  inputDescriptors: ReadonlyArray<PublishedGraphInputDescriptor>;
  descriptors: ReadonlyArray<PublishedGraphDescriptor>;
  inputs: {
    readonly [TName in keyof NormalizeGraphRecord<TInputDefinitions>]:
      UnwrapGraphInputHandle<TInputDefinitions[TName]> extends GraphInputHandle<unknown>
        ? PublishedGraphInputHistoryEntry
        : never;
  };
  outputs: {
    readonly [TName in keyof NormalizeGraphRecord<TOutputDefinitions>]:
      TOutputDefinitions[TName] extends GraphReadableHandle<unknown>
        ? PublishedGraphOutputHistoryEntry
        : never;
  };
  input<TName extends keyof NormalizeGraphRecord<TInputDefinitions>>(
    name: TName,
  ): UnwrapGraphInputHandle<TInputDefinitions[TName]> extends GraphInputHandle<unknown>
    ? PublishedGraphInputHistoryEntry
    : never;
  output<TName extends keyof NormalizeGraphRecord<TOutputDefinitions>>(
    name: TName,
  ): TOutputDefinitions[TName] extends GraphReadableHandle<unknown>
    ? PublishedGraphOutputHistoryEntry
    : never;
  dependenciesForOutput<TName extends keyof NormalizeGraphRecord<TOutputDefinitions>>(
    name: TName,
  ): PublishedGraphOutputDependencyExplanation;
  contractSummary(): PublishedGraphContractSummary;
  executionHistory: ExecutionHistorySurfaceSummary;
  recentHistory: ReadonlyArray<ExecutionHistorySummary>;
}

export type GraphOutputDefinitions = Record<string, GraphReadableHandle>;
export type GraphInputDefinition = GraphInputHandle | PublicGraphInputContractEntry;
export type GraphInputDefinitions = Record<string, GraphInputDefinition>;
export type GraphInternalDefinitions = Record<string, unknown>;

type UnionToIntersection<T> = (
  T extends unknown ? (value: T) => void : never
) extends (value: infer TIntersection) => void
  ? TIntersection
  : never;

type NormalizeRecord<T> = T extends Record<string, unknown> ? T : Record<string, never>;
type StripIndexSignature<T> = {
  [TKey in keyof T as string extends TKey
    ? never
    : number extends TKey
      ? never
      : symbol extends TKey
        ? never
        : TKey]: T[TKey];
};
type NormalizeGraphRecord<T> = StripIndexSignature<NormalizeRecord<T>>;

export type ControllerContractInputs<TController> =
  TController extends ControllerContract<infer TInputs, any, any>
    ? NormalizeGraphRecord<TInputs>
    : Record<string, never>;

export type ControllerContractOutputs<TController> =
  TController extends ControllerContract<any, infer TOutputs, any>
    ? NormalizeGraphRecord<TOutputs>
    : Record<string, never>;

export type ControllerContractInternals<TController> =
  TController extends ControllerContract<any, any, infer TInternal>
    ? NormalizeGraphRecord<TInternal>
    : Record<string, never>;

export type MergedControllerInputs<TControllers extends ReadonlyArray<ControllerContract>> =
  NormalizeGraphRecord<UnionToIntersection<ControllerContractInputs<TControllers[number]>>>;

export type MergedControllerOutputs<TControllers extends ReadonlyArray<ControllerContract>> =
  NormalizeGraphRecord<UnionToIntersection<ControllerContractOutputs<TControllers[number]>>>;

export type MergedControllerInternals<TControllers extends ReadonlyArray<ControllerContract>> =
  NormalizeGraphRecord<UnionToIntersection<ControllerContractInternals<TControllers[number]>>>;

export type PublishedGraphOutputs<TDefinitions extends GraphOutputDefinitions> = {
  readonly [TName in keyof NormalizeGraphRecord<TDefinitions>]:
    TDefinitions[TName] extends GraphReadableHandle<infer TValue>
      ? import("./callable_surface.js").OutputSignalHandle<TValue>
      : never;
};

type UnwrapGraphInputHandle<TDefinition> =
  TDefinition extends PublicGraphInputContractEntry<infer THandle>
    ? THandle
    : TDefinition extends GraphInputHandle
      ? TDefinition
      : never;

export type PublishedGraphInputs<TDefinitions extends GraphInputDefinitions> = {
  readonly [TName in keyof NormalizeGraphRecord<TDefinitions>]:
    UnwrapGraphInputHandle<TDefinitions[TName]> extends GraphInputHandle<infer TValue>
      ? import("./callable_surface.js").InputSignalHandle<TValue>
      : never;
};

export type PublishedGraphInputValues<TDefinitions extends GraphInputDefinitions> = {
  readonly [TName in keyof NormalizeGraphRecord<TDefinitions>]:
    UnwrapGraphInputHandle<TDefinitions[TName]> extends GraphInputHandle<infer TValue>
      ? TValue
      : never;
};

export type PublishedGraphPatchValues<TDefinitions extends GraphInputDefinitions> = Partial<{
  readonly [TName in keyof NormalizeGraphRecord<TDefinitions>]:
    UnwrapGraphInputHandle<TDefinitions[TName]> extends GraphInputHandle<infer TValue>
      ? TValue extends Record<string, unknown>
        ? Partial<TValue>
        : never
      : never;
}>;

export type PublishedGraphValues<TDefinitions extends GraphOutputDefinitions> = {
  readonly [TName in keyof NormalizeGraphRecord<TDefinitions>]:
    TDefinitions[TName] extends GraphReadableHandle<infer TValue>
      ? TValue
      : never;
};

export type ImportedGraphSignals<TDefinitions extends Record<string, GraphReadableHandle | GraphInputHandle>> = {
  readonly [TName in keyof NormalizeGraphRecord<TDefinitions>]:
    TDefinitions[TName] extends GraphReadableHandle<infer TValue> | GraphInputHandle<infer TValue>
      ? import("./callable_surface.js").Signal<TValue>
      : never;
};

export interface GraphPublicationRequest<
  TInputs extends GraphInputDefinitions = Record<string, never>,
  TOutputs extends GraphOutputDefinitions = GraphOutputDefinitions,
> {
  inputs?: TInputs;
  outputs: TOutputs;
}

export interface GraphExposureRequest<
  TControllers extends ReadonlyArray<ControllerContract> = ReadonlyArray<never>,
  TInputs extends GraphInputDefinitions = Record<string, never>,
  TOutputs extends GraphOutputDefinitions = GraphOutputDefinitions,
  TInternal extends GraphInternalDefinitions = Record<string, never>,
> {
  controllers?: TControllers;
  inputs?: TInputs;
  outputs?: TOutputs;
  internal?: TInternal;
}

export interface PublishedGraphContract<
  TInputs extends GraphInputDefinitions = Record<string, never>,
  TOutputs extends GraphOutputDefinitions = GraphOutputDefinitions,
> {
  inputs?: TInputs;
  outputs: TOutputs;
}

export interface GraphOperationalContract<
  TWrites extends Record<string, unknown> = Record<string, never>,
  TPatches extends Record<string, unknown> = Record<string, never>,
  TCommands extends Record<string, unknown> = Record<string, never>,
> {
  writes: TWrites;
  patches: TPatches;
  commands: TCommands;
}

export interface PublishedGraphOperationalContractSurface<
  TInputs extends GraphInputDefinitions = Record<string, never>,
> extends GraphOperationalContract<
  Partial<PublishedGraphInputValues<TInputs>>,
  PublishedGraphPatchValues<TInputs>,
  Record<string, never>
> {
  graph: PublishedGraphSummary;
  authorities: {
    readonly [TName in keyof NormalizeGraphRecord<TInputs>]:
      UnwrapGraphInputHandle<TInputs[TName]> extends GraphInputHandle<unknown>
        ? PublishedGraphInputAuthorityDescriptor
        : never;
  };
  resettableInputNames: ReadonlyArray<Extract<keyof NormalizeGraphRecord<TInputs>, string>>;
}

export interface GraphMutationRequest<
  TInputs extends GraphInputDefinitions = Record<string, never>,
> {
  writes?: Partial<PublishedGraphInputValues<TInputs>>;
  patches?: PublishedGraphPatchValues<TInputs>;
  commands?: Record<string, never>;
  reset?: ReadonlyArray<Extract<keyof NormalizeGraphRecord<TInputs>, string>>;
}

export type GraphTransactionInputTarget<
  TInputs extends GraphInputDefinitions = GraphInputDefinitions,
> =
  | Extract<keyof NormalizeGraphRecord<TInputs>, string>
  | PublishedGraphInputs<TInputs>[keyof PublishedGraphInputs<TInputs>];

export interface PublishedGraphTransaction<
  TInputs extends GraphInputDefinitions = GraphInputDefinitions,
> {
  set(input: GraphTransactionInputTarget<TInputs>, value: SignalValue): void;
  setWithAspects(
    input: GraphTransactionInputTarget<TInputs>,
    value: SignalValue,
    aspects: ReadonlyArray<AspectId>,
  ): void;
  setWithRegions(
    input: GraphTransactionInputTarget<TInputs>,
    value: SignalValue,
    changedRegions: unknown,
  ): void;
  setWithRegionsAndAspects(
    input: GraphTransactionInputTarget<TInputs>,
    value: SignalValue,
    changedRegions: unknown,
    aspects: ReadonlyArray<AspectId>,
  ): void;
  free(): void;
  [Symbol.dispose](): void;
}

export type GraphScope<TPersistence = SignalValue> =
  import("./callable_surface.js").ScopedSignalNamespace<TPersistence>;

export interface GraphConstructionSurface<TPersistence = SignalValue> {
  readonly id: string;
  scope(localScopeId: string): GraphScope<TPersistence>;
  controller<
    TInputs extends GraphInputDefinitions = Record<string, never>,
    TOutputs extends GraphOutputDefinitions = Record<string, never>,
    TInternal extends Record<string, unknown> = Record<string, never>,
  >(definition: ControllerContractDefinition<TInputs, TOutputs, TInternal>): ControllerContract<TInputs, TOutputs, TInternal>;
  publicInput<THandle extends GraphInputHandle>(
    handle: THandle,
    options?: PublicGraphInputOptions,
  ): PublicGraphInputContractEntry<THandle>;
  expose<
    TControllers extends ReadonlyArray<ControllerContract> = ReadonlyArray<never>,
    TInputs extends GraphInputDefinitions = Record<string, never>,
    TOutputs extends GraphOutputDefinitions = GraphOutputDefinitions,
    TInternal extends GraphInternalDefinitions = Record<string, never>,
  >(
    definition: GraphExposureRequest<TControllers, TInputs, TOutputs, TInternal>,
  ): PublishedGraphContract<
    MergedControllerInputs<TControllers> & TInputs,
    MergedControllerOutputs<TControllers> & TOutputs
  >;
}

export type GraphBuilder<
  TPersistence = SignalValue,
  TInputs extends GraphInputDefinitions = Record<string, never>,
  TOutputs extends GraphOutputDefinitions = GraphOutputDefinitions,
> = (graph: GraphConstructionSurface<TPersistence>) => PublishedGraphContract<TInputs, TOutputs>;

export interface PublishedSignalGraph<
  TOutputs extends GraphOutputDefinitions = GraphOutputDefinitions,
  TInputs extends GraphInputDefinitions = Record<string, never>,
> {
  readonly id: string;
  readonly inputs: PublishedGraphInputs<TInputs>;
  readonly outputs: PublishedGraphOutputs<TOutputs>;
  contract(): PublishedGraphContractSurface;
  contractDelta(previousContract: PublishedGraphContractSurface): PublishedGraphContractDelta;
  contractHistory(): PublishedGraphContractHistory;
  importPosture(): GraphImportPosture;
  operationalContract(): PublishedGraphOperationalContractSurface<TInputs>;
  input<TName extends keyof NormalizeGraphRecord<TInputs>>(name: TName): PublishedGraphInputs<TInputs>[TName];
  output<TName extends keyof NormalizeGraphRecord<TOutputs>>(name: TName): PublishedGraphOutputs<TOutputs>[TName];
  read(): PublishedGraphValues<TOutputs>;
  readInputs(): PublishedGraphInputValues<TInputs>;
  writeInputs(values: Partial<PublishedGraphInputValues<TInputs>>): RunSummary;
  patchInputs(patches: PublishedGraphPatchValues<TInputs>): RunSummary;
  resetInputs(inputNames?: ReadonlyArray<Extract<keyof NormalizeGraphRecord<TInputs>, string>>): RunSummary;
  apply(mutation: GraphMutationRequest<TInputs>): RunSummary;
  transaction(callback: (tx: PublishedGraphTransaction<TInputs>) => void): RunSummary;
  why<TName extends keyof NormalizeGraphRecord<TOutputs>>(name: TName): WhySummary;
  replayFor<TName extends keyof NormalizeGraphRecord<TOutputs>>(name: TName): ReplaySummary;
  lineageFor<TName extends keyof NormalizeGraphRecord<TOutputs>>(name: TName): LineageSummary;
  readVersions(): ReadonlyArray<VersionSummary>;
  summary(): PublishedGraphSummary;
  inputDescriptors(): ReadonlyArray<PublishedGraphInputDescriptor>;
  descriptors(): ReadonlyArray<PublishedGraphDescriptor>;
  inspectDiagnostics(): PublishedGraphDiagnosticsSurface<TOutputs, TInputs>;
  inspectHistory(): PublishedGraphHistorySurface<TOutputs, TInputs>;
  exportCompatibilityDefinition(): PublishedGraphCompatibilityDefinition;
  exportDefinition(): ExportedSignalGraphDefinition<TOutputs, TInputs>;
  exportSnapshot(): ExportedSignalGraphSnapshot<TOutputs, TInputs>;
  diagnostics(): import("./callable_surface.js").CallableSignalDiagnostics;
  history(): import("./callable_surface.js").CallableSignalHistory;
  specialist(): import("./callable_surface.js").CallableSignalSpecialist;
  adapters(): import("./callable_surface.js").CallableSignalAdapters;
  compatibilityApp(): import("./raw_surface.js").SignalApp;
  compatibilityRuntime(): import("./raw_surface.js").SignalRuntime;
}

export interface ImportedSignalGraph<
  TOutputs extends GraphOutputDefinitions = GraphOutputDefinitions,
  TInputs extends GraphInputDefinitions = Record<string, never>,
> {
  readonly id: string;
  readonly inputs: ImportedGraphSignals<TInputs>;
  readonly outputs: ImportedGraphSignals<TOutputs>;
  contract(): PublishedGraphContractSurface;
  contractHistory(): PublishedGraphContractHistory;
  importPosture(): GraphImportPosture;
  input<TName extends keyof NormalizeGraphRecord<TInputs>>(name: TName): ImportedGraphSignals<TInputs>[TName];
  output<TName extends keyof NormalizeGraphRecord<TOutputs>>(name: TName): ImportedGraphSignals<TOutputs>[TName];
  read(): PublishedGraphValues<TOutputs>;
  readInputs(): PublishedGraphInputValues<TInputs>;
  summary(): PublishedGraphSummary;
  inputDescriptors(): ReadonlyArray<PublishedGraphInputDescriptor>;
  descriptors(): ReadonlyArray<PublishedGraphDescriptor>;
  inspectDiagnostics(): PublishedGraphDiagnosticsSurface<TOutputs, TInputs>;
  inspectHistory(): PublishedGraphHistorySurface<TOutputs, TInputs>;
  exportCompatibilityDefinition(): PublishedGraphCompatibilityDefinition;
  exportDefinition(): ExportedSignalGraphDefinition<TOutputs, TInputs>;
  exportSnapshot(): ExportedSignalGraphSnapshot<TOutputs, TInputs>;
}

export type SignalNamespace<TPersistence = SignalValue> = Pick<
  import("./callable_surface.js").CallableSignals<TPersistence>,
  | "host"
  | "scope"
  | "controller"
  | "publicInput"
  | "input"
  | "computedSpec"
  | "computed"
  | "outputSpec"
  | "output"
  | "outputCallback"
  | "graph"
>;
