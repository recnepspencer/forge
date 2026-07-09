declare const WORTHSignalControllerContractBrand: unique symbol;

export interface ControllerAuthoringSurface<TPersistence = import("./model.js").SignalValue> {
  readonly host: import("./callable_surface.js").CallableSignalsHost<TPersistence>;
  readonly spec: import("./callable_surface.js").ExplicitSignalSpecNamespace;
  scope(localScopeId: string): ControllerAuthoringSurface<TPersistence>;
  controller<
    TInputs extends Record<string, unknown> = Record<string, unknown>,
    TOutputs extends Record<string, unknown> = Record<string, unknown>,
    TInternal extends Record<string, unknown> = Record<string, unknown>,
  >(
    definition: ControllerContractDefinition<TInputs, TOutputs, TInternal>,
  ): ControllerContract<TInputs, TOutputs, TInternal>;
  controller<
    TInputs extends Record<string, unknown> = Record<string, unknown>,
    TOutputs extends Record<string, unknown> = Record<string, unknown>,
    TInternal extends Record<string, unknown> = Record<string, unknown>,
  >(
    builder: ControllerContractBuilder<TPersistence, TInputs, TOutputs, TInternal>,
  ): ControllerContract<TInputs, TOutputs, TInternal>;
  publicInput<THandle extends import("./callable_surface.js").InputSignalHandle>(
    handle: THandle,
    options?: import("./graph_surface.js").PublicGraphInputOptions,
  ): import("./graph_surface.js").PublicGraphInputContractEntry<THandle>;
  input<T = import("./model.js").SignalValue>(
    initial: T,
    options?: import("./callable_surface.js").InputAuthoringOptions,
  ): import("./callable_surface.js").InputSignalHandle<T>;
  linked<T = import("./model.js").SignalValue>(
    source: () => T,
    options?: import("./callable_surface.js").LinkedSignalOptions,
  ): import("./callable_surface.js").LinkedSignalHandle<T, T>;
  linked<TSource = import("./model.js").SignalValue>(
    definition: import("./callable_surface.js").LinkedIdentitySignalDefinition<TSource>,
  ): import("./callable_surface.js").LinkedSignalHandle<TSource, TSource>;
  linked<TSource = import("./model.js").SignalValue, TValue = TSource>(
    definition: import("./callable_surface.js").LinkedComputedSignalDefinition<TSource, TValue>,
  ): import("./callable_surface.js").LinkedSignalHandle<TValue, TSource>;
  computedSpec<T = import("./model.js").SignalValue>(
    id: string,
    spec: import("./model.js").ComputedSpec,
  ): import("./callable_surface.js").ComputedSignalHandle<T>;
  computed<T = import("./model.js").SignalValue>(
    spec: import("./model.js").ComputedSpec,
    options?: import("./callable_surface.js").SignalAuthoringOptions,
  ): import("./callable_surface.js").ComputedSignalHandle<T>;
  computed<T = import("./model.js").SignalValue>(
    compute: () => T,
    options?: import("./callable_surface.js").SignalAuthoringOptions,
  ): import("./callable_surface.js").ComputedSignalHandle<T>;
  outputSpec<T = import("./model.js").SignalValue>(
    id: string,
    spec: import("./model.js").OutputSpec,
  ): import("./callable_surface.js").OutputSignalHandle<T>;
  output<T = import("./model.js").SignalValue>(
    spec: import("./model.js").OutputSpec,
    options?: import("./callable_surface.js").SignalAuthoringOptions,
  ): import("./callable_surface.js").OutputSignalHandle<T>;
  output<T = import("./model.js").SignalValue>(
    compute: () => T,
    options?: import("./callable_surface.js").SignalAuthoringOptions,
  ): import("./callable_surface.js").OutputSignalHandle<T>;
  outputCallback<T = import("./model.js").SignalValue>(
    id: string,
    compute: () => T,
    options?: import("./callable_surface.js").CallbackSignalAuthoringOptions,
  ): import("./callable_surface.js").OutputSignalHandle<T>;
}

export type ControllerContractBuilder<
  TPersistence = import("./model.js").SignalValue,
  TInputs extends Record<string, unknown> = Record<string, unknown>,
  TOutputs extends Record<string, unknown> = Record<string, unknown>,
  TInternal extends Record<string, unknown> = Record<string, unknown>,
> = (
  namespace: ControllerAuthoringSurface<TPersistence>,
) => ControllerContractDefinition<TInputs, TOutputs, TInternal>;

export interface ControllerContract<
  TInputs extends Record<string, unknown> = Record<string, unknown>,
  TOutputs extends Record<string, unknown> = Record<string, unknown>,
  TInternal extends Record<string, unknown> = Record<string, unknown>,
> {
  readonly inputs: TInputs;
  readonly outputs: TOutputs;
  readonly internal: TInternal;
  readonly [WORTHSignalControllerContractBrand]: "controllerContract";
}

export interface ControllerContractDefinition<
  TInputs extends Record<string, unknown> = Record<string, unknown>,
  TOutputs extends Record<string, unknown> = Record<string, unknown>,
  TInternal extends Record<string, unknown> = Record<string, unknown>,
> {
  inputs?: TInputs;
  outputs?: TOutputs;
  internal?: TInternal;
}
