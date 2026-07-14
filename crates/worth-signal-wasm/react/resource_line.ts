import { useMemo } from "react";

import { useOptionalSignalValue } from "./hooks.js";
import { useOptionalResourceLine } from "./hooks.js";
import { useReactSignalsStore } from "./context.js";

import type {
  ReactSignalsStore,
  ResourceLineFamilyReactLike,
  ResourceLineReactLike,
  ResourceLineSelection,
} from "./model.js";

function isDisabledResourceLineSelection(
  selection: unknown,
): selection is { readonly enabled: false } {
  return Boolean(
    selection
      && typeof selection === "object"
      && Object.keys(selection).length === 1
      && "enabled" in selection
      && (selection as { enabled?: unknown }).enabled === false,
  );
}

export function optionalResourceLine<
  TParams,
  TLine extends ResourceLineReactLike<any, TParams>,
>(
  family: ResourceLineFamilyReactLike<TParams, TLine>,
  selection: ResourceLineSelection<TParams>,
): TLine | null {
  if (selection == null || isDisabledResourceLineSelection(selection)) {
    return null;
  }
  return family.line(selection);
}

export function useResourceLine<
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
) {
  const resolvedStore = store ?? useReactSignalsStore();
  const line = useMemo(
    () => family.optionalLine?.(selection) ?? optionalResourceLine(family, selection),
    [family, selection],
  );
  return useOptionalResourceLine<TValue, TInactive, TParams, TLine>(
    line,
    resolvedStore,
    options,
  );
}

export function useOptionalResourceLineValue<
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
) {
  const resolvedStore = store ?? useReactSignalsStore();
  return useOptionalSignalValue<TValue, TInactive>(
    line?.signal(),
    resolvedStore,
    options,
  );
}
