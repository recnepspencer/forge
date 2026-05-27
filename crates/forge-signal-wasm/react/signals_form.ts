import { useMemo, useRef } from "react";

import { useMaybeReactSignalsStore } from "./context.js";
import { useSignalValue } from "./hooks.js";

import type {
  ReactSignalsStore,
  RuntimeFormController,
  SignalsFormActionBinding,
  SignalsFormBinding,
  SignalsFormCheckboxBinding,
  SignalsFormFieldBinding,
  SignalsFormFieldState,
  SignalsFormMultiSelectBinding,
  SignalsFormOption,
  SignalsFormSelectBinding,
} from "./model.js";
import type {
  RuntimeFormDeclaration,
  RuntimeFormFieldHandleFor,
  SignalsWithFormLike,
} from "./form_model.js";

function eventValue(eventOrValue: unknown): unknown {
  if (
    eventOrValue
    && typeof eventOrValue === "object"
    && "currentTarget" in eventOrValue
  ) {
    const currentTarget = (eventOrValue as { currentTarget?: { value?: unknown } }).currentTarget;
    if (currentTarget && "value" in currentTarget) {
      return currentTarget.value;
    }
  }
  if (
    eventOrValue
    && typeof eventOrValue === "object"
    && "target" in eventOrValue
  ) {
    const target = (eventOrValue as { target?: { value?: unknown } }).target;
    if (target && "value" in target) {
      return target.value;
    }
  }
  return eventOrValue;
}

function eventChecked(eventOrChecked: unknown): boolean {
  if (typeof eventOrChecked === "boolean") {
    return eventOrChecked;
  }
  if (
    eventOrChecked
    && typeof eventOrChecked === "object"
    && "currentTarget" in eventOrChecked
  ) {
    const currentTarget = (eventOrChecked as { currentTarget?: { checked?: unknown } }).currentTarget;
    if (currentTarget && typeof currentTarget.checked === "boolean") {
      return currentTarget.checked;
    }
  }
  if (
    eventOrChecked
    && typeof eventOrChecked === "object"
    && "target" in eventOrChecked
  ) {
    const target = (eventOrChecked as { target?: { checked?: unknown } }).target;
    if (target && typeof target.checked === "boolean") {
      return target.checked;
    }
  }
  return Boolean(eventOrChecked);
}

function eventMultiValue(eventOrValue: unknown): unknown[] {
  if (Array.isArray(eventOrValue)) {
    return eventOrValue;
  }
  if (
    eventOrValue
    && typeof eventOrValue === "object"
    && "currentTarget" in eventOrValue
  ) {
    const currentTarget = (eventOrValue as {
      currentTarget?: { selectedOptions?: Iterable<{ value?: unknown }> };
    }).currentTarget;
    if (currentTarget?.selectedOptions) {
      return Array.from(currentTarget.selectedOptions, (entry) => entry.value);
    }
  }
  if (
    eventOrValue
    && typeof eventOrValue === "object"
    && "target" in eventOrValue
  ) {
    const target = (eventOrValue as {
      target?: { selectedOptions?: Iterable<{ value?: unknown }>; value?: unknown };
    }).target;
    if (target?.selectedOptions) {
      return Array.from(target.selectedOptions, (entry) => entry.value);
    }
    if (Array.isArray(target?.value)) {
      return target.value;
    }
  }
  return eventOrValue == null ? [] : [eventOrValue];
}

function readFieldMessages(form: RuntimeFormController, fieldId: string): readonly unknown[] {
  return form.visibleMessages().filter((message) => message.target === fieldId);
}

function readFieldInteraction(form: RuntimeFormController, fieldId: string): unknown | null {
  return form.interaction().fields.find((entry) => entry.field === fieldId) ?? null;
}

function readFieldState<TValue = unknown, TRaw = TValue>(
  form: RuntimeFormController,
  fieldId: string,
): SignalsFormFieldState<TValue, TRaw> {
  const field = form.field<TValue, TRaw>(fieldId);
  const value = field.value();
  const writePosture = form.fieldWritePosture(fieldId);
  const blocked = !Boolean((writePosture as { canWrite?: boolean }).canWrite);
  return Object.freeze({
    name: fieldId,
    value,
    disabled: blocked,
    readOnly: blocked,
    field,
    dirty: field.dirty(),
    diagnostics: field.diagnostics(),
    messages: readFieldMessages(form, fieldId),
    interaction: readFieldInteraction(form, fieldId),
    writePosture,
  });
}

function readFieldBinding<TValue = unknown, TRaw = TValue>(
  form: RuntimeFormController,
  fieldId: string,
  options?: { readonly input?: unknown },
): SignalsFormFieldBinding<TValue, TRaw> {
  const state = readFieldState<TValue, TRaw>(form, fieldId);
  const binding = form.bindInput<TValue, TRaw>(fieldId, options?.input);
  return Object.freeze({
    ...state,
    binding,
    onChange(next: unknown) {
      binding.input(eventValue(next) as TRaw);
    },
    onBlur() {
      binding.blur();
    },
    onFocus() {
      binding.focus();
    },
  });
}

function readCheckboxBinding<TValue = boolean>(
  form: RuntimeFormController,
  fieldId: string,
  options?: { readonly input?: unknown },
): SignalsFormCheckboxBinding<TValue> {
  const field = form.field<TValue, boolean>(fieldId);
  const binding = form.bindInput<TValue, boolean>(fieldId, options?.input);
  const writePosture = form.fieldWritePosture(fieldId);
  const blocked = !Boolean((writePosture as { canWrite?: boolean }).canWrite);
  return Object.freeze({
    name: fieldId,
    checked: Boolean(field.value()),
    disabled: blocked,
    readOnly: blocked,
    dirty: field.dirty(),
    diagnostics: field.diagnostics(),
    messages: readFieldMessages(form, fieldId),
    interaction: readFieldInteraction(form, fieldId),
    writePosture,
    onChange(next: unknown) {
      binding.set(eventChecked(next) as TValue);
    },
    onBlur() {
      binding.blur();
    },
    onFocus() {
      binding.focus();
    },
  });
}

function readSelectBinding<TValue = unknown, TRaw = TValue>(
  form: RuntimeFormController,
  fieldId: string,
  fieldOptions: readonly SignalsFormOption<TValue>[],
  options?: { readonly input?: unknown },
): SignalsFormSelectBinding<TValue, TRaw> {
  return Object.freeze({
    ...readFieldBinding<TValue, TRaw>(form, fieldId, options),
    options: fieldOptions,
  });
}

function readMultiSelectBinding<TValue = string>(
  form: RuntimeFormController,
  fieldId: string,
  fieldOptions: readonly SignalsFormOption<TValue>[],
  options?: { readonly input?: unknown },
): SignalsFormMultiSelectBinding<TValue> {
  const field = form.field<readonly TValue[], readonly TValue[]>(fieldId);
  const binding = form.bindInput<readonly TValue[], readonly TValue[]>(fieldId, options?.input);
  const writePosture = form.fieldWritePosture(fieldId);
  const blocked = !Boolean((writePosture as { canWrite?: boolean }).canWrite);
  const value = field.value() ?? [];
  return Object.freeze({
    name: fieldId,
    value: Array.isArray(value) ? value : [],
    disabled: blocked,
    readOnly: blocked,
    dirty: field.dirty(),
    diagnostics: field.diagnostics(),
    messages: readFieldMessages(form, fieldId),
    interaction: readFieldInteraction(form, fieldId),
    writePosture,
    options: fieldOptions,
    onChange(next: unknown) {
      binding.input(eventMultiValue(next) as readonly TValue[]);
    },
    onBlur() {
      binding.blur();
    },
    onFocus() {
      binding.focus();
    },
  });
}

function readActionBinding(form: RuntimeFormController, actionId: string): SignalsFormActionBinding {
  const plan = form.actionPlan(actionId);
  const debug = form.debugAction(actionId);
  const latestExecution = debug.latestExecution;
  return Object.freeze({
    plan,
    debug,
    disabled: plan.status !== "accepted" || !plan.readiness.canRun || debug.pending,
    pending: debug.pending,
    latestExecution,
    resultKind: readExecutionResultKind(latestExecution),
    execute() {
      return form.executeAction(actionId);
    },
  });
}

function readExecutionResultKind(execution: unknown): string | null {
  if (!execution || typeof execution !== "object" || !("resultKind" in execution)) {
    return null;
  }
  return typeof execution.resultKind === "string" ? execution.resultKind : null;
}

function readActionIds(form: RuntimeFormController): string[] {
  return form.actions().catalog.map((entry) => entry.id);
}

function requireSignalsFormStore<TSignals extends SignalsWithFormLike>(
  explicitStore: ReactSignalsStore<TSignals> | undefined,
  providerStore: ReactSignalsStore | null,
): ReactSignalsStore<TSignals> {
  if (explicitStore) {
    return explicitStore;
  }
  if (providerStore) {
    return providerStore as ReactSignalsStore<TSignals>;
  }
  throw new TypeError(
    "React signals store was not provided. Wrap the tree with <ReactSignalsStoreProvider store={...}> or pass store explicitly.",
  );
}

function requireSignalsFormFactory(signals: SignalsWithFormLike): SignalsWithFormLike["form"] {
  if (typeof signals.form !== "function") {
    throw new TypeError(
      "useSignalsForm(...) requires a forge-signal-wasm signals runtime with signals.form(...) available.",
    );
  }
  return signals.form;
}

export function useSignalsForm<
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
> {
  const providerStore = useMaybeReactSignalsStore();
  const resolvedStore = requireSignalsFormStore(store, providerStore);
  const controllerRef = useRef<{
    readonly remountKey: unknown;
    readonly signals: TSignals;
    readonly controller: RuntimeFormController;
  } | null>(null);

  if (
    controllerRef.current === null
    || controllerRef.current.signals !== resolvedStore.signals
    || !Object.is(controllerRef.current.remountKey, options?.remountKey)
  ) {
    controllerRef.current = Object.freeze({
      remountKey: options?.remountKey,
      signals: resolvedStore.signals,
      controller: requireSignalsFormFactory(resolvedStore.signals)(
        declaration as RuntimeFormDeclaration<TSource>,
      ),
    });
  }

  const controller = controllerRef.current.controller as RuntimeFormController<
    TSource,
    { [TKey in keyof TFields]: RuntimeFormFieldHandleFor<TFields[TKey]> }
  >;
  const summarySnapshot = useSignalValue(controller.summarySignal(), resolvedStore);

  return useMemo(() => {
    const actions = Object.fromEntries(
      readActionIds(controller).map((actionId) => [actionId, readActionBinding(controller, actionId)]),
    ) as Readonly<Record<Extract<keyof TActions, string>, SignalsFormActionBinding>>;

    return Object.freeze({
      controller,
      source: controller.source(),
      draft: controller.draft(),
      effective: controller.effective(),
      dirty: controller.dirty(),
      patchPlan: controller.patchPlan(),
      readiness: controller.readiness(),
      visibleMessages: controller.visibleMessages(),
      actions,
      fieldState<TValue = unknown, TRaw = TValue>(
        fieldId: Extract<keyof TFields, string>,
      ) {
        return readFieldState<TValue, TRaw>(controller, fieldId);
      },
      field<TValue = unknown, TRaw = TValue>(
        fieldId: Extract<keyof TFields, string>,
        fieldOptions?: { readonly input?: unknown },
      ) {
        return readFieldBinding<TValue, TRaw>(controller, fieldId, fieldOptions);
      },
      checkbox<TValue = boolean>(
        fieldId: Extract<keyof TFields, string>,
        fieldOptions?: { readonly input?: unknown },
      ) {
        return readCheckboxBinding<TValue>(controller, fieldId, fieldOptions);
      },
      select<TValue = unknown, TRaw = TValue>(
        fieldId: Extract<keyof TFields, string>,
        selectOptions: readonly SignalsFormOption<TValue>[],
        fieldOptions?: { readonly input?: unknown },
      ) {
        return readSelectBinding<TValue, TRaw>(controller, fieldId, selectOptions, fieldOptions);
      },
      multiSelect<TValue = string>(
        fieldId: Extract<keyof TFields, string>,
        selectOptions: readonly SignalsFormOption<TValue>[],
        fieldOptions?: { readonly input?: unknown },
      ) {
        return readMultiSelectBinding<TValue>(controller, fieldId, selectOptions, fieldOptions);
      },
      action(actionId: Extract<keyof TActions, string>) {
        return readActionBinding(controller, actionId);
      },
      reset(resetOptions?: { readonly reason?: string }) {
        return controller.reset(resetOptions);
      },
    });
  }, [controller, summarySnapshot]);
}
