import type {
  OutputSignalHandle,
  ScopedSignalNamespace,
} from "../callable_surface.js";
export type FeatureStoreStateCompatible<TValue> =
  TValue extends null | boolean | number | string
    ? TValue
    : TValue extends readonly (infer TItem)[]
      ? readonly FeatureStoreStateCompatible<TItem>[]
      : TValue extends (infer TItem)[]
        ? FeatureStoreStateCompatible<TItem>[]
        : TValue extends object
          ? { [TKey in keyof TValue]: FeatureStoreStateCompatible<TValue[TKey]> }
          : never;

export type FeatureStoreStateDefinition = Record<string, unknown>;

export type FeatureStoreStateConstraint<
  TState extends FeatureStoreStateDefinition,
> = {
  readonly [TKey in keyof TState]: FeatureStoreStateCompatible<TState[TKey]>;
};

export type FeatureStoreSetValue<TValue> =
  TValue extends null
    ? null
    : TValue extends string
      ? string
      : TValue extends number
        ? number
        : TValue extends boolean
          ? boolean
          : TValue extends readonly (infer TItem)[]
            ? readonly FeatureStoreSetValue<TItem>[]
            : TValue extends (infer TItem)[]
              ? FeatureStoreSetValue<TItem>[]
              : TValue extends object
                ? { [TKey in keyof TValue]: FeatureStoreSetValue<TValue[TKey]> }
                : never;

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
    value: FeatureStoreSetValue<TState[TKey]>,
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
    readonly state: FeatureStoreStateConstraint<TState>;
    readonly actions: (
      context: FeatureStoreActionContext<TState>,
    ) => TActions;
  }): FeatureStore<TState, TActions>;
}
