import React from "react";

import {
  COUNTRY_OPTIONS,
  SOURCE_DRAFT,
  currency,
  selectedRegionSummary,
  validateCarrierEmail,
  validatePrice,
  validateShippingSelection,
} from "./formsSectionSupport";
import type { RolloutDraft } from "./formsSectionSupport";

type CarrierErrorMap = Partial<Record<string, string>>;

interface FormsSectionState {
  draft: RolloutDraft;
  regionsOpen: boolean;
  regionsRef: React.RefObject<HTMLDivElement | null>;
  priceError?: string;
  shippingError?: string;
  carrierErrors: CarrierErrorMap;
  visibleMessages: string[];
  isDirty: boolean;
  diagnosticsOutput: string;
  patchDraft: (next: Partial<RolloutDraft>) => void;
  resetDraft: () => void;
  setRegionsOpen: React.Dispatch<React.SetStateAction<boolean>>;
  toggleRegion: (code: string) => void;
  updateCarrierEmail: (code: string, value: string) => void;
}

function formatArray(values: string[]): string {
  return values.length > 0 ? `[${values.map((value) => `"${value}"`).join(", ")}]` : "[]";
}

function createCarrierErrors(draft: RolloutDraft): CarrierErrorMap {
  const next: CarrierErrorMap = {};
  for (const option of COUNTRY_OPTIONS) {
    const error = validateCarrierEmail(option.code, draft.shippingRegions, draft.carrierEmails);
    if (error) {
      next[option.code] = error;
    }
  }
  return next;
}

function createDiagnosticsOutput(
  draft: RolloutDraft,
  dirtyFields: Array<keyof RolloutDraft>,
  patchPlanSummary: string[],
  visibleMessages: string[],
): string {
  const inspectorLines = [
    "const source = form.source();",
    `> ${JSON.stringify(
      {
        price: SOURCE_DRAFT.price,
        shippingRegions: SOURCE_DRAFT.shippingRegions,
        carrierEmails: Object.fromEntries(
          Object.entries(SOURCE_DRAFT.carrierEmails).filter(([, value]) => value.length > 0),
        ),
      },
      null,
      2,
    )}`,
    "",
    "const dirty = form.dirty();",
    `> ${JSON.stringify({ isDirty: dirtyFields.length > 0, fields: dirtyFields }, null, 2)}`,
    "",
    "const patchPlan = form.patchPlan();",
    `> ${JSON.stringify({ empty: patchPlanSummary.length === 0, operations: patchPlanSummary }, null, 2)}`,
    "",
    'const readiness = form.readiness();',
    `> ${JSON.stringify({ canSubmit: visibleMessages.length === 0, blockers: visibleMessages }, null, 2)}`,
    "",
    'const savePlan = form.actionPlan("save");',
    `> ${JSON.stringify({ disabled: visibleMessages.length > 0, reason: visibleMessages[0] ?? null }, null, 2)}`,
    "",
    "const draft = form.draft;",
    `> ${JSON.stringify(
      {
        price: draft.price,
        shippingRegions: draft.shippingRegions,
        carrierEmails: Object.fromEntries(
          Object.entries(draft.carrierEmails).filter(([, value]) => value.length > 0),
        ),
      },
      null,
      2,
    )}`,
  ];
  return inspectorLines.join("\n");
}

export function useFormsSectionState(): FormsSectionState {
  const [draft, setDraft] = React.useState<RolloutDraft>(SOURCE_DRAFT);
  const [regionsOpen, setRegionsOpen] = React.useState(false);
  const regionsRef = React.useRef<HTMLDivElement | null>(null);

  const priceError = validatePrice(draft.price, draft.baseCost, draft.targetMargin) ?? undefined;
  const shippingError = validateShippingSelection(draft.shippingRegions) ?? undefined;
  const carrierErrors = React.useMemo(() => createCarrierErrors(draft), [draft]);

  const visibleMessages = React.useMemo(
    () => [priceError, shippingError, ...Object.values(carrierErrors)].filter(Boolean) as string[],
    [carrierErrors, priceError, shippingError],
  );

  const dirtyFields = React.useMemo(
    () =>
      (Object.keys(SOURCE_DRAFT) as Array<keyof RolloutDraft>).filter(
        (key) => JSON.stringify(SOURCE_DRAFT[key]) !== JSON.stringify(draft[key]),
      ),
    [draft],
  );

  const patchPlanSummary = React.useMemo(() => {
    const ops: string[] = [];
    if (draft.price !== SOURCE_DRAFT.price) {
      ops.push(`replace price -> ${currency.format(draft.price)}`);
    }
    if (JSON.stringify(draft.shippingRegions) !== JSON.stringify(SOURCE_DRAFT.shippingRegions)) {
      ops.push(`replace shippingRegions -> ${formatArray(draft.shippingRegions)}`);
    }
    for (const code of draft.shippingRegions) {
      const current = draft.carrierEmails[code] ?? "";
      const source = SOURCE_DRAFT.carrierEmails[code] ?? "";
      if (current !== source) {
        ops.push(`set carrierEmails.${code} -> "${current}"`);
      }
    }
    return ops;
  }, [draft]);

  const diagnosticsOutput = React.useMemo(
    () => createDiagnosticsOutput(draft, dirtyFields, patchPlanSummary, visibleMessages),
    [dirtyFields, draft, patchPlanSummary, visibleMessages],
  );

  React.useEffect(() => {
    function handlePointerDown(event: PointerEvent): void {
      if (!regionsRef.current?.contains(event.target as Node)) {
        setRegionsOpen(false);
      }
    }

    if (!regionsOpen) {
      return;
    }

    window.addEventListener("pointerdown", handlePointerDown);
    return () => window.removeEventListener("pointerdown", handlePointerDown);
  }, [regionsOpen]);

  const patchDraft = React.useCallback((next: Partial<RolloutDraft>): void => {
    setDraft((current) => ({ ...current, ...next }));
  }, []);

  const toggleRegion = React.useCallback((code: string): void => {
    setDraft((current) => {
      const shippingRegions = current.shippingRegions.includes(code)
        ? current.shippingRegions.filter((value) => value !== code)
        : [...current.shippingRegions, code];
      return { ...current, shippingRegions };
    });
  }, []);

  const updateCarrierEmail = React.useCallback((code: string, value: string): void => {
    setDraft((current) => ({
      ...current,
      carrierEmails: { ...current.carrierEmails, [code]: value },
    }));
  }, []);

  const resetDraft = React.useCallback(() => {
    setDraft(SOURCE_DRAFT);
  }, []);

  return {
    draft,
    regionsOpen,
    regionsRef,
    priceError,
    shippingError,
    carrierErrors,
    visibleMessages,
    isDirty: dirtyFields.length > 0,
    diagnosticsOutput,
    patchDraft,
    resetDraft,
    setRegionsOpen,
    toggleRegion,
    updateCarrierEmail,
  };
}

export { selectedRegionSummary };
