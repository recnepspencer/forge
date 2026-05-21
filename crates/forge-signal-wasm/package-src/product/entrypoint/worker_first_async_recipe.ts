import {
  forbidOpaqueIdOption,
  requireAuthoringOptions,
  requireOptionalDebugName,
} from "../authoring_option_validation.js";
import { createWorkerFirstAsyncReadableHandle } from "./worker_first_async_readable.js";
import { evaluateWorkerFirstDeclarativeSpec } from "./worker_first_declarative_expr.js";

export async function createWorkerFirstAsyncRecipeHandle(
  rootSession,
  family,
  id,
  specOrCompute,
  options,
) {
  const debugName = options ? requireOptionalDebugName(family, options) : null;
  if (typeof specOrCompute === "function") {
    await rootSession.createStandaloneCallbackReadable(id, family, specOrCompute);
    return createWorkerFirstAsyncReadableHandle(rootSession, id, family, debugName);
  }
  const spec = normalizeWorkerFirstAsyncRecipeArgs(family, specOrCompute, options);
  await rootSession.createStandaloneReadable(id, family, spec);
  return createWorkerFirstAsyncReadableHandle(rootSession, id, family, debugName);
}

export function createWorkerFirstSyncCallbackRecipeHandle(
  rootSession,
  family,
  id,
  callback,
  options,
) {
  const debugName = options ? requireOptionalDebugName(family, options) : null;
  if (typeof callback !== "function") {
    throw new TypeError(`worker-first ${family}(...) callback form requires a function`);
  }
  rootSession.createEagerStandaloneCallbackReadable(id, family, callback);
  return createWorkerFirstAsyncReadableHandle(rootSession, id, family, debugName);
}

export function createWorkerFirstSyncDeclarativeRecipeHandle(
  rootSession,
  family,
  id,
  spec,
  options,
  operation,
) {
  const debugName = options ? requireOptionalDebugName(family, options) : null;
  const evaluation = evaluateWorkerFirstDeclarativeSpec(
    rootSession,
    family,
    spec,
    operation,
  );
  rootSession.createEagerStandaloneReadable(
    id,
    family,
    spec,
    evaluation.value,
    evaluation.dependencyIds,
  );
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
  return specOrCompute;
}
