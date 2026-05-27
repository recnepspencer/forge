import { useMemo } from "react";

import { useSignalValue, useSignalsDiagnostics } from "./hooks.js";

import type {
  FormControllerReactLike,
  FormFieldHandleReactLike,
  ReactSignalsStore,
} from "./model.js";

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

export function useFormField<
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
} {
  const diagnostics = useSignalsDiagnostics(store);

  return useMemo(() => {
    const field = form.field<TValue, TRaw>(fieldId);
    const binding = form.bindInput<TValue, TRaw>(fieldId, options?.input);
    const value = field.value();
    const messages = form
      .visibleMessages()
      .filter((message) => message.target === fieldId);
    const interaction = form
      .interaction()
      .fields
      .find((entry) => entry.field === fieldId) ?? null;

    return Object.freeze({
      field,
      binding,
      value,
      dirty: field.dirty(),
      diagnostics: field.diagnostics(),
      messages,
      interaction,
      writePosture: form.fieldWritePosture(fieldId),
      textInput() {
        return Object.freeze({
          name: fieldId,
          value,
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
      },
      checkboxInput() {
        return Object.freeze({
          name: fieldId,
          checked: Boolean(value),
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
      },
    });
  }, [diagnostics, fieldId, form, options?.input, store]);
}

export function useFormAction<
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
} {
  const summarySnapshot = useSignalValue(form.summarySignal(), store);

  return useMemo(() => {
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
  }, [actionId, form, summarySnapshot]);
}

function readExecutionResultKind(execution: unknown): string | null {
  if (!execution || typeof execution !== "object" || !("resultKind" in execution)) {
    return null;
  }
  return typeof execution.resultKind === "string" ? execution.resultKind : null;
}
