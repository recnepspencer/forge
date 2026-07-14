import type { SignalValue } from "../model.js";
import type { FormInteractionInputSource } from "./interaction.js";

export interface FormBoundInput<TValue = SignalValue, TRaw = TValue> {
  input(rawValue: TRaw, options?: {
    readonly commit?: boolean;
    readonly source?: FormInteractionInputSource;
  }): void;
  compose(rawValue: TRaw): void;
  commit(rawValue?: TRaw): void;
  focus(): void;
  blur(): void;
  touch(): void;
  visit(): void;
  set(value: TValue): void;
  clearDraft(): void;
}

export interface FormBoundInputOptions<TValue = SignalValue, TRaw = TValue> {
  readonly parse?: (rawValue: TRaw) => TValue;
  readonly source?: FormInteractionInputSource;
}
