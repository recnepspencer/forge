import type { InputSignalHandle } from "../callable_surface.js";
import type { GraphReadableHandle, PublicGraphInputContractEntry } from "../graph_surface.js";
import type { SignalValue } from "../model.js";
import type { ResourceLine } from "../resource/resource_lifecycle.js";

export type FormSourceKind =
  | "signal"
  | "graphPublicInput"
  | "resourceLine"
  | "externalBoundary";

export interface FormSourceDeclaration<TValue = SignalValue> {
  readonly kind: FormSourceKind;
  readonly __formSourceValue?: TValue;
}

export interface FormSourceOptions {
  readonly id?: string;
  readonly contract?: string;
}

export interface FormSourceFactory {
  signal<TValue = SignalValue>(
    handle: GraphReadableHandle<TValue>,
    options?: FormSourceOptions,
  ): FormSourceDeclaration<TValue>;
  graphPublicInput<THandle extends InputSignalHandle>(
    entry: PublicGraphInputContractEntry<THandle>,
    options?: FormSourceOptions,
  ): FormSourceDeclaration<ReturnType<THandle>>;
  resourceLine<TParams = unknown, TValue = SignalValue>(
    line: ResourceLine<TParams, TValue>,
    options?: FormSourceOptions,
  ): FormSourceDeclaration<TValue>;
  external<TValue = SignalValue>(
    readable: TValue | (() => TValue) | { get(): TValue },
    options?: FormSourceOptions,
  ): FormSourceDeclaration<TValue>;
}

export interface FormSourceAuthorityDiagnostics {
  readonly kind: FormSourceKind;
  readonly sourceId: string;
  readonly explicit: boolean;
  readonly contract: string;
  readonly sourceValueDigest: string;
  readonly sourceAuthorityDigest: string;
  readonly identity: Readonly<Record<string, unknown>>;
}
