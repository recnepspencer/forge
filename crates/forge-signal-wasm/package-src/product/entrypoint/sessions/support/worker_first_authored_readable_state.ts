import { materializeWorkerCachedValue } from "./worker_cached_value.js";

export function createAuthoredReadablePublication(id, family, spec) {
  return {
    policy: { preset: "operational" },
    sources: [],
    recipes: [
      {
        id,
        reads: spec.reads ?? [],
        expr: spec.expr ?? null,
        when: spec.when ?? null,
        identity: spec.identity ?? null,
        ...(spec.producesAspects === undefined
          ? {}
          : { producesAspects: spec.producesAspects }),
      },
    ],
    outputIds: [id],
  };
}

export function createWorkerFirstAuthoredReadableState(family, value) {
  return {
    family,
    currentValue: materializeWorkerCachedValue(value),
    invalidatedMessage: null,
  };
}

export function invalidateWorkerFirstAuthoredReadables(authoredReadables, message) {
  for (const authoredReadable of authoredReadables.values()) {
    authoredReadable.invalidatedMessage = message;
  }
}

export function hasWorkerFirstAuthoredReadableId(authoredReadables, id) {
  return authoredReadables.get(id)?.invalidatedMessage === null;
}

export function readWorkerFirstAuthoredReadableValue(authoredReadables, id) {
  const authoredReadable = authoredReadables.get(id);
  if (!authoredReadable) {
    return undefined;
  }
  if (authoredReadable.invalidatedMessage !== null) {
    throw new TypeError(
      `worker-first authored ${authoredReadable.family} \`${id}\` cannot be used because ${authoredReadable.invalidatedMessage}`,
    );
  }
  return authoredReadable.currentValue;
}

export function updateWorkerFirstAuthoredReadables(authoredReadables, signals) {
  for (const signal of signals) {
    const authoredReadable = authoredReadables.get(signal.id);
    if (!authoredReadable || authoredReadable.invalidatedMessage !== null) {
      continue;
    }
    authoredReadable.currentValue = materializeWorkerCachedValue(signal.value);
  }
}
