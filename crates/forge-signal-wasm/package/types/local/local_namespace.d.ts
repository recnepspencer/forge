import type { AspectId, RunSummary, SignalValue } from "../model.js";
import type { InputSignalHandle, ScopedSignalNamespace } from "../callable_surface.js";
import type { FormSourceDeclaration } from "../forms/sources.js";

export interface LocalDialogStateOptions {
  readonly identity: string;
  readonly initialOpen?: boolean;
  readonly debugName?: string;
}

export interface LocalDialogState {
  readonly scope: ScopedSignalNamespace;
  readonly scopeId: string;
  readonly signal: InputSignalHandle<boolean>;
  open(): RunSummary | Promise<RunSummary>;
  close(): RunSummary | Promise<RunSummary>;
  toggle(): RunSummary | Promise<RunSummary>;
  free(): void;
  [Symbol.dispose](): void;
}

export interface LocalListStateOptions<TItem = SignalValue> {
  readonly identity: string;
  readonly initial: readonly TItem[] | TItem[];
  readonly aspects?: ReadonlyArray<AspectId>;
  readonly debugName?: string;
}

export interface LocalListState<TItem = SignalValue> {
  readonly scope: ScopedSignalNamespace;
  readonly scopeId: string;
  readonly items: InputSignalHandle<readonly TItem[] | TItem[]>;
  reset(): RunSummary | Promise<RunSummary>;
  free(): void;
  [Symbol.dispose](): void;
}

export interface LocalFormSourceStateOptions<TValue = SignalValue> {
  readonly identity: string;
  readonly initial: TValue;
  readonly debugName?: string;
  readonly sourceId?: string;
  readonly contract?: string;
}

export interface LocalFormSourceState<TValue = SignalValue> {
  readonly scope: ScopedSignalNamespace;
  readonly scopeId: string;
  readonly signal: InputSignalHandle<TValue>;
  readonly source: FormSourceDeclaration<TValue>;
  reset(): RunSummary | Promise<RunSummary>;
  free(): void;
  [Symbol.dispose](): void;
}

export interface LocalNamespace {
  dialogState(options: LocalDialogStateOptions): LocalDialogState;
  listState<TItem = SignalValue>(options: LocalListStateOptions<TItem>): LocalListState<TItem>;
  formSource<TValue = SignalValue>(
    options: LocalFormSourceStateOptions<TValue>,
  ): LocalFormSourceState<TValue>;
}
