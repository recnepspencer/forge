import type {
  ManagedResourceWriteExecution,
  ManagedResourceWriteFeedback,
  ManagedResourceWriteFeedbackMessages,
  ManagedResourceRecoveryLineLike,
  ManagedResourceWriteHookOptions,
  ManagedResourceWriteLineLike,
  ManagedResourceWriteOptions,
  ManagedResourceWriteRecoveryDeclaration,
  ManagedResourceWriteRecoveryPolicy,
  ManagedResourceWriteResult,
  ManagedResourceWriteState,
  OptionalResourceLineResult,
  OptionalSignalValueResult,
  ReactSignalsStore,
  ResourceCatalogDefinition,
  ResourceLineFamilyReactLike,
  ResourceLineReactLike,
  ResourceOperationExecutionReactLike,
  ResourceOperationView,
  ResourceLineSelection,
  ResourceViewResult,
} from "./model.js";

export declare function createResourceCatalog<
  TSignals extends object,
  TCatalog,
>(options: {
  id: string;
  build(signals: TSignals): TCatalog;
}): ResourceCatalogDefinition<TSignals, TCatalog>;

export declare function createResourceCatalog<
  TSignals extends object,
  TScope,
  TDomains extends Record<string, (scope: TScope, signals: TSignals) => unknown>,
>(options: {
  id: string;
  scope(signals: TSignals): TScope;
  domains: TDomains;
}): ResourceCatalogDefinition<
  TSignals,
  Readonly<{
    scope: TScope;
    domains: {
      readonly [TKey in keyof TDomains]: ReturnType<TDomains[TKey]>;
    };
  } & {
    readonly [TKey in keyof TDomains]: ReturnType<TDomains[TKey]>;
  }>
>;

export declare function getResourceCatalog<
  TSignals extends object,
  TCatalog,
>(
  signals: TSignals,
  definition: ResourceCatalogDefinition<TSignals, TCatalog>,
): TCatalog;

export declare function useResourceCatalog<
  TSignals extends object,
  TCatalog,
>(
  store: ReactSignalsStore<TSignals>,
  definition: ResourceCatalogDefinition<TSignals, TCatalog>,
): TCatalog;

export declare function useResourceCatalog<
  TSignals extends object,
  TCatalog,
>(
  definition: ResourceCatalogDefinition<TSignals, TCatalog>,
  store?: ReactSignalsStore<TSignals>,
): TCatalog;

export declare function optionalResourceLine<
  TParams,
  TLine extends ResourceLineReactLike<any, TParams>,
>(
  family: ResourceLineFamilyReactLike<TParams, TLine>,
  selection: ResourceLineSelection<TParams>,
): TLine | null;

export declare function useOptionalResourceLineValue<
  TValue = unknown,
  TInactive = undefined,
  TParams = unknown,
  TLine extends ResourceLineReactLike<TValue, TParams> = ResourceLineReactLike<TValue, TParams>,
>(
  line: TLine | null | undefined,
  store?: ReactSignalsStore,
  options?: {
    inactiveValue?: TInactive;
  },
): OptionalSignalValueResult<TValue, TInactive>;

export declare function useResourceLine<
  TValue = unknown,
  TInactive = undefined,
  TParams = unknown,
  TLine extends ResourceLineReactLike<TValue, TParams> = ResourceLineReactLike<TValue, TParams>,
>(
  family: ResourceLineFamilyReactLike<TParams, TLine>,
  selection: ResourceLineSelection<TParams>,
  store?: ReactSignalsStore,
  options?: {
    inactiveValue?: TInactive;
  },
): OptionalResourceLineResult<TLine, TValue, TInactive>;

export declare function useResourceOperation<
  TValue = unknown,
  TParams = unknown,
  TLine extends ResourceLineReactLike<TValue, TParams> = ResourceLineReactLike<TValue, TParams>,
  TExecution extends ResourceOperationExecutionReactLike<TValue, TParams, TLine> = ResourceOperationExecutionReactLike<TValue, TParams, TLine>,
>(
  execution: TExecution,
  store?: ReactSignalsStore,
): ResourceOperationView<TLine, TValue, TParams>;

export declare function useResourceView<
  TValue = unknown,
  TInactive = undefined,
  TLine extends ResourceLineReactLike<TValue> = ResourceLineReactLike<TValue>,
>(
  line: TLine | null | undefined,
  store: ReactSignalsStore,
  options?: {
    inactiveValue?: TInactive;
    emptyWhen?(value: TValue): boolean;
    errorMessage?: string;
  },
): ResourceViewResult<TLine, TValue, TInactive>;

export declare function createManagedResourceWriteExecution<
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
>(
  line: TLine,
  options?: ManagedResourceWriteOptions<TLine>,
): ManagedResourceWriteExecution<TLine>;

export declare function executeManagedResourceWrite<
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
>(
  line: TLine,
  options?: ManagedResourceWriteOptions<TLine>,
): Promise<ManagedResourceWriteResult<TLine>>;

export declare const managedResourceWriteFeedback: {
  create<TLine extends ManagedResourceWriteLineLike>(
    result: ManagedResourceWriteResult<TLine>,
    messages?: ManagedResourceWriteFeedbackMessages,
  ): ManagedResourceWriteFeedback<TLine>;
};

export declare const managedResourceWriteRecovery: {
  refresh<TLine extends ManagedResourceRecoveryLineLike>(
    line: TLine | (() => TLine),
    reason?: string,
  ): ManagedResourceWriteRecoveryDeclaration<TLine>;
  revalidate<TLine extends ManagedResourceRecoveryLineLike>(
    line: TLine | (() => TLine),
    reason?: string,
  ): ManagedResourceWriteRecoveryDeclaration<TLine>;
  apply<TLine extends ManagedResourceWriteLineLike>(
    result: ManagedResourceWriteResult<TLine>,
    policy?: ManagedResourceWriteRecoveryPolicy,
  ): Promise<ManagedResourceWriteResult<TLine>>;
};

export declare function useManagedResourceWrite<
  TArgs,
  TLine extends ManagedResourceWriteLineLike,
>(
  options: ManagedResourceWriteHookOptions<TArgs, TLine>,
): ManagedResourceWriteState<TArgs, TLine>;
