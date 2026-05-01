declare const forgeSignalControllerContractBrand: unique symbol;

export interface ControllerContract<
  TInputs extends Record<string, unknown> = Record<string, unknown>,
  TOutputs extends Record<string, unknown> = Record<string, unknown>,
  TInternal extends Record<string, unknown> = Record<string, unknown>,
> {
  readonly inputs: TInputs;
  readonly outputs: TOutputs;
  readonly internal: TInternal;
  readonly [forgeSignalControllerContractBrand]: "controllerContract";
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
