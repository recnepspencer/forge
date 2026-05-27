import type {
  OutputSignalHandle,
  ScopedSignalNamespace,
} from "../callable_surface.js";
import type { SignalValue } from "../model.js";

export type FeatureStoreStateDefinition = Record<string, SignalValue>;

export type FeatureStoreStateHandles<TState extends FeatureStoreStateDefinition> = {
  readonly [TKey in keyof TState]: import("../callable_surface.js").InputSignalHandle<TState[TKey]>;
};

export interface FeatureStoreActionContext<
  TState extends FeatureStoreStateDefinition,
> {
  readonly scope: ScopedSignalNamespace;
  readonly state: FeatureStoreStateHandles<TState>;
  set<TKey extends keyof TState>(
    key: TKey,
    value: TState[TKey],
  ): unknown;
  reset<TKey extends keyof TState>(
    key?: TKey,
  ): unknown;
  read(): Readonly<TState>;
}

export interface FeatureStore<
  TState extends FeatureStoreStateDefinition = FeatureStoreStateDefinition,
  TActions extends Record<string, (...args: never[]) => unknown> = Record<
    string,
    (...args: never[]) => unknown
  >,
> {
  readonly scope: ScopedSignalNamespace;
  readonly scopeId: string;
  readonly state: FeatureStoreStateHandles<TState>;
  readonly snapshot: OutputSignalHandle<Readonly<TState>>;
  read(): Readonly<TState>;
  readonly actions: Readonly<TActions>;
  free(): void;
  [Symbol.dispose](): void;
}

export interface FeatureStoreFactory {
  <
    TState extends FeatureStoreStateDefinition,
    TActions extends Record<string, (...args: never[]) => unknown>,
  >(options: {
    readonly id: string;
    readonly state: TState;
    readonly actions: (
      context: FeatureStoreActionContext<TState>,
    ) => TActions;
  }): FeatureStore<TState, TActions>;
}
