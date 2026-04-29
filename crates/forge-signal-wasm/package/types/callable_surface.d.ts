import type {
  AspectId,
  InputOptions,
  ComputedSpec,
  OutputSpec,
  RunSummary,
  SignalValue,
  WebObservationNotice,
} from "./model.js";
import type { RuntimeDefinitionEnvelope } from "./diagnostics.js";
import type {
  ComputedSignal,
  DisposableHandle,
  InputSignal,
  OutputSignal,
  SignalAdapters,
  SignalApp,
  SignalDiagnostics,
  SignalHistory,
  SignalRuntime,
  SignalSpecialist,
  Signals,
} from "./raw_surface.js";

declare const forgeSignalBrand: unique symbol;
declare const forgeSignalInputBrand: unique symbol;
declare const forgeSignalComputedBrand: unique symbol;
declare const forgeSignalOutputBrand: unique symbol;

export interface Signal<T = SignalValue> {
  (): T;
  get(): T;
  free(): void;
  [Symbol.dispose](): void;
  readonly id: string;
  readonly [forgeSignalBrand]: "signal";
}

export interface InputSignalHandle<T = SignalValue> extends Signal<T> {
  set(value: T): RunSummary;
  readonly [forgeSignalInputBrand]: "input";
}

export interface ComputedSignalHandle<T = SignalValue> extends Signal<T> {
  readonly [forgeSignalComputedBrand]: "computed";
}

export interface OutputSignalHandle<T = SignalValue> extends Signal<T> {
  readonly [forgeSignalOutputBrand]: "output";
}

export type CallableSignalTarget =
  | string
  | InputSignalHandle
  | ComputedSignalHandle
  | OutputSignalHandle;

export interface CallableSignalsTransaction {
  set(input: InputSignalHandle, value: SignalValue): void;
  setWithAspects(input: InputSignalHandle, value: SignalValue, aspects: ReadonlyArray<AspectId>): void;
  setWithRegions(input: InputSignalHandle, value: SignalValue, changedRegions: unknown): void;
  setWithRegionsAndAspects(
    input: InputSignalHandle,
    value: SignalValue,
    changedRegions: unknown,
    aspects: ReadonlyArray<AspectId>,
  ): void;
  free(): void;
  [Symbol.dispose](): void;
}

export interface CallableSignalAdapters {
  exportDefinitions(): RuntimeDefinitionEnvelope;
  exportRuntimeEnvelope(): never;
  replaceRuntimeEnvelope(envelope: unknown): never;
  runtimeProofReport(): unknown;
  free(): void;
  [Symbol.dispose](): void;
}

export interface CallableSignals {
  input<T = SignalValue>(id: string, initial: T, options?: InputOptions): InputSignalHandle<T>;
  computedSpec<T = SignalValue>(id: string, spec: ComputedSpec): ComputedSignalHandle<T>;
  computed<T = SignalValue>(id: string, spec: ComputedSpec): ComputedSignalHandle<T>;
  computed<T = SignalValue>(id: string, compute: () => T): ComputedSignalHandle<T>;
  computed<T = SignalValue>(compute: () => T, options?: { id?: string }): ComputedSignalHandle<T>;
  outputSpec<T = SignalValue>(id: string, spec: OutputSpec): OutputSignalHandle<T>;
  output<T = SignalValue>(id: string, spec: OutputSpec): OutputSignalHandle<T>;
  output<T = SignalValue>(id: string, compute: () => T): never;
  output<T = SignalValue>(compute: () => T, options?: { id?: string }): never;
  outputCallback<T = SignalValue>(id: string, compute: () => T): never;
  transaction(callback: (tx: CallableSignalsTransaction) => void): RunSummary;
  batch(callback: (tx: CallableSignalsTransaction) => void): RunSummary;
  watch(target: CallableSignalTarget, callback: (notice: WebObservationNotice) => void): DisposableHandle;
  effect(target: CallableSignalTarget, callback: () => void): DisposableHandle;
  nuke(handle: DisposableHandle): boolean;
  diagnostics(): SignalDiagnostics;
  history(): SignalHistory;
  specialist(): SignalSpecialist;
  adapters(): CallableSignalAdapters;
  compatibilityApp(): SignalApp;
  compatibilityRuntime(): SignalRuntime;
  free(): void;
  [Symbol.dispose](): void;
}

export function createSignals(): CallableSignals;
export function createCallableSignals(): CallableSignals;
export function wrapSignals(signals: Signals): CallableSignals;

export {
  Signals as RawSignals,
  InputSignal as RawInputSignal,
  ComputedSignal as RawComputedSignal,
  OutputSignal as RawOutputSignal,
  DisposableHandle as RawDisposableHandle,
  SignalAdapters,
  SignalApp,
  SignalDiagnostics,
  SignalHistory,
  SignalRuntime,
  SignalSpecialist,
};
