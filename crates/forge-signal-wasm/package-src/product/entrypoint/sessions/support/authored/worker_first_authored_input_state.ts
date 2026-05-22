import { materializeWorkerCachedValue } from "../worker_cached_value.js";

export function nextGeneratedStandaloneSignalId(counters, family, scopeId = null) {
  const counterKey = scopeId ?? "__root__";
  const next = (counters.get(counterKey) ?? 0) + 1;
  counters.set(counterKey, next);
  if (typeof scopeId === "string" && scopeId.length > 0) {
    return `__forgeSignalScoped.${scopeId}.${family}.${next}`;
  }
  return `__forgeSignal.${family}.${next}`;
}

export function createAuthoredInputPublication(id, initial, options = {}) {
  const echoId = `${id}.__workerFirstInputEcho`;
  return {
    policy: { preset: "operational" },
    sources: [
      {
        id,
        initial,
        ...(options.producesAspects === undefined
          ? {}
          : { producesAspects: options.producesAspects }),
      },
    ],
    recipes: [
      {
        id: echoId,
        reads: [id],
        expr: { kind: "read", id },
        identity: { kind: "exact" },
      },
    ],
    outputIds: [echoId],
  };
}

export function createWorkerFirstAuthoredInputState(initial) {
  return {
    baselineValue: materializeWorkerCachedValue(initial),
    currentValue: materializeWorkerCachedValue(initial),
    invalidatedMessage: null,
  };
}

export function invalidateWorkerFirstAuthoredInputs(authoredInputs, message) {
  for (const authoredInput of authoredInputs.values()) {
    authoredInput.invalidatedMessage = message;
  }
}

export function hasMutableWorkerFirstAuthoredInputId(authoredInputs, id) {
  return authoredInputs.get(id)?.invalidatedMessage === null;
}

export function readWorkerFirstAuthoredInputValue(authoredInputs, id) {
  const authoredInput = authoredInputs.get(id);
  if (!authoredInput) {
    return undefined;
  }
  if (authoredInput.invalidatedMessage !== null) {
    throw new TypeError(
      `worker-first authored input \`${id}\` cannot be used because ${authoredInput.invalidatedMessage}`,
    );
  }
  return authoredInput.currentValue;
}

export function applyCommittedWorkerFirstAuthoredInputs(authoredInputs, transactionOps) {
  for (const transactionOp of transactionOps) {
    if (!transactionOp || transactionOp.kind !== "set" || typeof transactionOp.id !== "string") {
      continue;
    }
    const authoredInput = authoredInputs.get(transactionOp.id);
    if (!authoredInput || authoredInput.invalidatedMessage !== null) {
      continue;
    }
    authoredInput.currentValue = materializeWorkerCachedValue(transactionOp.value);
  }
}

export function readWorkerFirstAuthoredInputBaseline(authoredInputs, id) {
  const authoredInput = authoredInputs.get(id);
  if (!authoredInput || authoredInput.invalidatedMessage !== null) {
    throw new TypeError(
      `worker-first authored input \`${id}\` cannot read its baseline because it is not currently available`,
    );
  }
  return authoredInput.baselineValue;
}

export function writeWorkerFirstAuthoredInputBaseline(authoredInputs, id, value) {
  const authoredInput = authoredInputs.get(id);
  if (!authoredInput || authoredInput.invalidatedMessage !== null) {
    throw new TypeError(
      `worker-first authored input \`${id}\` cannot update its baseline because it is not currently available`,
    );
  }
  authoredInput.baselineValue = materializeWorkerCachedValue(value);
}

export function buildAuthoredInputMutationOperation(id, mutation, authoredInput) {
  if (!mutation || typeof mutation !== "object") {
    throw new TypeError("worker-first inputAsync mutation requires an operation object");
  }
  switch (mutation.kind) {
    case "set":
      return { kind: "set", id, value: mutation.value };
    case "reset":
      return { kind: "set", id, value: authoredInput.baselineValue };
    case "patch":
      return { kind: "set", id, value: mergeWorkerFirstPatchValue(authoredInput.currentValue, mutation.value) };
    default:
      throw new TypeError("worker-first inputAsync mutation kind is unsupported");
  }
}

function mergeWorkerFirstPatchValue(currentValue, patchValue) {
  const currentIsObject = currentValue !== null && typeof currentValue === "object";
  const patchIsObject = patchValue !== null && typeof patchValue === "object";
  if (!currentIsObject || !patchIsObject) {
    throw new TypeError("worker-first inputAsync patch(...) requires object or array values");
  }
  if (Array.isArray(currentValue) || Array.isArray(patchValue)) {
    return patchValue;
  }
  return {
    ...currentValue,
    ...patchValue,
  };
}
