import { useMemo } from "react";

import type { FormFieldWritePosture } from "../package/types/forms/core.js";
import {
  commitTextInput,
  setCheckboxInput,
} from "./form_input_events.js";
import { useSignalValue, useSignalsDiagnostics } from "./hooks.js";

import type {
  FormControllerReactLike,
  FormFieldHandleReactLike,
  FormInteractionFieldReactLike,
  FormVisibleMessageReactLike,
  ReactSignalsStore,
} from "./model.js";

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
  messages: readonly FormVisibleMessageReactLike[];
  interaction: FormInteractionFieldReactLike | null;
  writePosture: FormFieldWritePosture;
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
    const field = form.field(fieldId) as FormFieldHandleReactLike<TValue, TRaw>;
    const binding = form.bindInput<TValue, TRaw>(fieldId, options?.input) as ReturnType<TForm["bindInput"]>;
    const value = field.value();
    const messages = form
      .visibleMessages()
      .filter((message) => message.target === fieldId);
    const interaction = form
      .interaction()
      .fields
      .find((entry) => entry.field === fieldId) ?? null;

    const fieldBinding = {
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
            commitTextInput(binding, next);
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
            setCheckboxInput(binding, next);
          },
          onBlur() {
            binding.blur();
          },
          onFocus() {
            binding.focus();
          },
        });
      },
    };
    return Object.freeze(fieldBinding) as {
      field: FormFieldHandleReactLike<TValue, TRaw>;
      binding: ReturnType<TForm["bindInput"]>;
      value: TValue;
      dirty: ReturnType<FormFieldHandleReactLike<TValue, TRaw>["dirty"]>;
      diagnostics: ReturnType<FormFieldHandleReactLike<TValue, TRaw>["diagnostics"]>;
      messages: readonly FormVisibleMessageReactLike[];
      interaction: FormInteractionFieldReactLike | null;
      writePosture: FormFieldWritePosture;
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
    const actionBinding = {
      plan,
      debug,
      disabled: plan.status !== "accepted" || !plan.readiness.canRun || debug.pending,
      pending: debug.pending,
      latestExecution,
      resultKind: readExecutionResultKind(latestExecution),
      execute() {
        return form.executeAction(actionId);
      },
    };
    return Object.freeze(actionBinding) as {
      plan: ReturnType<TForm["actionPlan"]>;
      debug: ReturnType<TForm["debugAction"]>;
      disabled: boolean;
      pending: boolean;
      latestExecution: ReturnType<TForm["debugAction"]>["latestExecution"];
      resultKind: string | null;
      execute(): ReturnType<TForm["executeAction"]>;
    };
  }, [actionId, form, summarySnapshot]);
}

function readExecutionResultKind(execution: unknown): string | null {
  if (!execution || typeof execution !== "object" || !("resultKind" in execution)) {
    return null;
  }
  return typeof execution.resultKind === "string" ? execution.resultKind : null;
}
