import {
  forbidOpaqueIdOption,
  requireAuthoringOptions,
  requireOptionalDebugName,
} from "../authoring_option_validation.js";
import { createWorkerFirstAsyncReadableHandle } from "./worker_first_async_readable.js";

export async function createWorkerFirstAsyncRecipeHandle(
  rootSession,
  family,
  id,
  specOrCompute,
  options,
) {
  const spec = normalizeWorkerFirstAsyncRecipeArgs(family, specOrCompute, options);
  const debugName = options ? requireOptionalDebugName(family, options) : null;
  await rootSession.createStandaloneReadable(id, family, spec);
  return createWorkerFirstAsyncReadableHandle(rootSession, id, family, debugName);
}

export function normalizeWorkerFirstAsyncRecipeOptions(family, options) {
  if (options === undefined) {
    return undefined;
  }
  const normalized = requireAuthoringOptions(family, options);
  forbidOpaqueIdOption(family, normalized);
  return normalized;
}

function normalizeWorkerFirstAsyncRecipeArgs(family, specOrCompute, options) {
  if (typeof specOrCompute === "string") {
    throw new TypeError(
      `worker-first ${family}Async(...) does not accept an explicit id in app authoring form; use the generated async lane or deployment: "mainThreadCompatibility" for explicit structural names`,
    );
  }
  if (typeof specOrCompute === "function") {
    throwWorkerFirstAsyncRecipeCallbackUnavailable(family);
  }
  return specOrCompute;
}

function throwWorkerFirstAsyncRecipeCallbackUnavailable(family) {
  const error = new Error(
    `worker-first ${family}Async(...) does not support callback authoring; use a declarative ${family} spec or deployment: "mainThreadCompatibility" for callback authoring`,
  );
  error.name = "WorkerFirstAsyncRecipeCallbackUnavailable";
  error.code = "workerFirstAsyncRecipeCallbackUnavailable";
  error.compatibilityRecovery = Object.freeze({
    deployment: "mainThreadCompatibility",
    message:
      'Retry with deployment: "mainThreadCompatibility" to use callback authoring.',
  });
  throw error;
}
