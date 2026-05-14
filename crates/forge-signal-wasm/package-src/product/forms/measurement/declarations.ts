import { FormDeclarationError } from "../form_errors.js";

const SUPPORTED_CAUSES = new Set([
  "resizeObserver",
  "fontLoad",
  "viewport",
  "contentGrowth",
  "asyncMessage",
  "textareaGrowth",
  "animationFrame",
]);

const DEFAULT_OBSERVE = Object.freeze([
  "resizeObserver",
  "fontLoad",
  "viewport",
  "contentGrowth",
  "asyncMessage",
  "textareaGrowth",
  "animationFrame",
]);

export function materializeMeasurementDeclaration(declaration) {
  const declared = declaration.measurement ?? {};
  if (declared == null || typeof declared !== "object" || Array.isArray(declared)) {
    throw new FormDeclarationError("form measurement metadata must be an object", {
      measurement: declared,
    });
  }
  const observe = declared.observe ?? DEFAULT_OBSERVE;
  if (!Array.isArray(observe) || observe.length === 0) {
    throw new FormDeclarationError("form measurement observe must be a non-empty array", {
      observe,
    });
  }
  for (const cause of observe) {
    if (!SUPPORTED_CAUSES.has(cause)) {
      throw new FormDeclarationError("form measurement cause is not supported", {
        cause,
      });
    }
  }
  const maxRetainedSnapshots = declared.maxRetainedSnapshots ?? 20;
  if (!Number.isInteger(maxRetainedSnapshots) || maxRetainedSnapshots <= 0) {
    throw new FormDeclarationError("form measurement maxRetainedSnapshots must be a positive integer", {
      maxRetainedSnapshots,
    });
  }
  return Object.freeze({
    observe: Object.freeze([...new Set(observe)]),
    batching: "animationFrameCoalesced",
    maxRetainedSnapshots,
  });
}
