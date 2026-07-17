interface FormInputBinding<TValue = unknown, TRaw = TValue> {
  input(
    rawValue: TRaw,
    options?: { readonly commit?: boolean },
  ): void;
  set(value: TValue): void;
}

export function commitTextInput<TRaw>(
  binding: FormInputBinding<unknown, TRaw>,
  eventOrValue: unknown,
): void {
  binding.input(eventValue(eventOrValue) as TRaw, { commit: true });
}

export function setCheckboxInput<TValue>(
  binding: FormInputBinding<TValue, boolean>,
  eventOrChecked: unknown,
): void {
  binding.set(eventChecked(eventOrChecked) as TValue);
}

export function commitMultiSelectInput<TRaw>(
  binding: FormInputBinding<unknown, readonly TRaw[]>,
  eventOrValue: unknown,
): void {
  binding.input(eventMultiValue(eventOrValue) as readonly TRaw[], { commit: true });
}

function eventValue(eventOrValue: unknown): unknown {
  const currentTarget = eventTarget(eventOrValue, "currentTarget");
  if (currentTarget && "value" in currentTarget) {
    return currentTarget.value;
  }
  const target = eventTarget(eventOrValue, "target");
  return target && "value" in target ? target.value : eventOrValue;
}

function eventChecked(eventOrChecked: unknown): boolean {
  if (typeof eventOrChecked === "boolean") {
    return eventOrChecked;
  }
  const currentTarget = eventTarget(eventOrChecked, "currentTarget");
  if (currentTarget && typeof currentTarget.checked === "boolean") {
    return currentTarget.checked;
  }
  const target = eventTarget(eventOrChecked, "target");
  return target && typeof target.checked === "boolean"
    ? target.checked
    : Boolean(eventOrChecked);
}

function eventMultiValue(eventOrValue: unknown): unknown[] {
  if (Array.isArray(eventOrValue)) {
    return eventOrValue;
  }
  const currentTarget = eventTarget(eventOrValue, "currentTarget");
  if (currentTarget?.selectedOptions) {
    return selectedValues(currentTarget.selectedOptions);
  }
  const target = eventTarget(eventOrValue, "target");
  if (target?.selectedOptions) {
    return selectedValues(target.selectedOptions);
  }
  if (Array.isArray(target?.value)) {
    return target.value;
  }
  return eventOrValue == null ? [] : [eventOrValue];
}

function eventTarget(
  event: unknown,
  key: "currentTarget" | "target",
): {
  readonly value?: unknown;
  readonly checked?: unknown;
  readonly selectedOptions?: Iterable<{ readonly value?: unknown }>;
} | null {
  if (!event || typeof event !== "object" || !(key in event)) {
    return null;
  }
  const target = (event as Record<string, unknown>)[key];
  return target && typeof target === "object" ? target : null;
}

function selectedValues(
  options: Iterable<{ readonly value?: unknown }>,
): unknown[] {
  return Array.from(options, (entry) => entry.value);
}
