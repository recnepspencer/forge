import {
  requireOptionalDebugName,
} from "../authoring_option_validation.js";
import { createWorkerFirstAsyncInputHandle } from "./worker_first_async_input.js";
import { createWorkerFirstLinkedHandle } from "./worker_first_async_linked.js";
import {
  createWorkerFirstSyncDeclarativeRecipeHandle,
  createWorkerFirstSyncCallbackRecipeHandle,
  normalizeWorkerFirstAsyncRecipeOptions,
} from "./worker_first_async_recipe.js";

export function createWorkerFirstSyncInputHandle(rootSession, id, initial, options) {
  rootSession.createEagerStandaloneInput(id, initial, options);
  return createWorkerFirstAsyncInputHandle(
    rootSession,
    id,
    options ? requireOptionalDebugName("input", options) : null,
  );
}

export function createWorkerFirstSyncLinkedHandle(
  rootSession,
  id,
  sourceOrDefinition,
  options,
) {
  return createWorkerFirstLinkedHandle(
    rootSession,
    id,
    sourceOrDefinition,
    options,
  );
}

export function createWorkerFirstSyncRecipeHandle(
  rootSession,
  family,
  id,
  specOrCompute,
  options,
  operation,
) {
  if (typeof specOrCompute !== "function") {
    return createWorkerFirstSyncDeclarativeRecipeHandle(
      rootSession,
      family,
      id,
      specOrCompute,
      options,
      operation,
    );
  }
  const normalizedOptions = normalizeWorkerFirstAsyncRecipeOptions(family, options);
  return createWorkerFirstSyncCallbackRecipeHandle(
    rootSession,
    family,
    id,
    specOrCompute,
    normalizedOptions,
  );
}

export function createWorkerFirstSyncComputedCallbackHandle(
  rootSession,
  id,
  compute,
  options,
) {
  const normalizedOptions = normalizeWorkerFirstAsyncRecipeOptions("computed", options);
  return createWorkerFirstSyncCallbackRecipeHandle(
    rootSession,
    "computed",
    id,
    compute,
    normalizedOptions,
  );
}

export function createWorkerFirstSyncOutputCallbackHandle(
  rootSession,
  id,
  compute,
  options,
) {
  const normalizedOptions = normalizeWorkerFirstAsyncRecipeOptions("output", options);
  return createWorkerFirstSyncCallbackRecipeHandle(
    rootSession,
    "output",
    id,
    compute,
    normalizedOptions,
  );
}
