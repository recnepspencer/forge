import type {
  FormControllerReactLike,
  FormFieldHandleReactLike,
  ReactSignalsStore,
  RuntimeFormController,
  RuntimeFormDeclaration,
  RuntimeFormFieldHandleFor,
  SignalsFormBinding,
  SignalsWithFormLike,
} from "./model.js";

export declare function useFormField<
  TValue = unknown,
  TRaw = TValue,
  TForm extends FormControllerReactLike = FormControllerReactLike,
>(
  form: TForm,
  fieldId: string,
  store: ReactSignalsStore,
  options?: {
    input?: unknown;
  },
): {
  field: FormFieldHandleReactLike<TValue, TRaw>;
  binding: ReturnType<TForm["bindInput"]>;
  value: TValue;
  dirty: ReturnType<FormFieldHandleReactLike<TValue, TRaw>["dirty"]>;
  diagnostics: ReturnType<FormFieldHandleReactLike<TValue, TRaw>["diagnostics"]>;
  messages: readonly unknown[];
  interaction: unknown | null;
  writePosture: unknown;
  textInput(): {
    readonly name: string;
    readonly value: TValue;
    onChange(next: unknown): void;
    onBlur(): void;
    onFocus(): void;
  };
  checkboxInput(): {
    readonly name: string;
    readonly checked: boolean;
    onChange(next: unknown): void;
    onBlur(): void;
    onFocus(): void;
  };
};

export declare function useFormAction<
  TForm extends FormControllerReactLike = FormControllerReactLike,
>(
  form: TForm,
  actionId: string,
  store: ReactSignalsStore,
): {
  plan: ReturnType<TForm["actionPlan"]>;
  debug: ReturnType<TForm["debugAction"]>;
  disabled: boolean;
  pending: boolean;
  latestExecution: ReturnType<TForm["debugAction"]>["latestExecution"];
  resultKind: string | null;
  execute(): ReturnType<TForm["executeAction"]>;
};

export declare function useSignalsForm<
  TSource = unknown,
  TFields extends Record<string, unknown> = Record<string, unknown>,
  TActions extends Record<string, unknown> = Record<string, unknown>,
  TSignals extends SignalsWithFormLike = SignalsWithFormLike,
>(
  declaration: RuntimeFormDeclaration<TSource, TFields> & {
    readonly actions?: TActions | ((...args: never[]) => TActions);
  },
  store?: ReactSignalsStore<TSignals>,
  options?: {
    readonly remountKey?: unknown;
  },
): SignalsFormBinding<
  Extract<keyof TFields, string>,
  Extract<keyof TActions, string>,
  RuntimeFormController<
    TSource,
    { [TKey in keyof TFields]: RuntimeFormFieldHandleFor<TFields[TKey]> }
  >
>;
