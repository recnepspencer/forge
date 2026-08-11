import { freezeObject } from "../graph_support.js";

export async function runWorkerFirstAsyncTransaction(rootSession, callback, operation) {
  if (typeof callback !== "function") {
    throw new TypeError(`${operation}(...) requires a callback`);
  }
  const tx = createWorkerFirstAsyncTransaction(rootSession, operation);
  try {
    callback(tx);
    // applyActiveTransaction stages host tip + notify before the worker queue.
    return await rootSession.applyActiveTransaction(tx.drain());
  } finally {
    tx.free();
  }
}

function createWorkerFirstAsyncTransaction(rootSession, operation) {
  const stagedValues = new Map();
  let freed = false;

  function requireInputHandle(input, method) {
    if (
      typeof input !== "function"
      || typeof input.id !== "string"
      || !rootSession.hasMutableInputId(input.id)
      || typeof input.get !== "function"
    ) {
      throw new TypeError(
        `${operation}(...).${method}(...) requires a worker-first input handle`,
      );
    }
    return input;
  }

  function requireActive(method) {
    if (!freed) {
      return;
    }
    throw new TypeError(`${operation}(...).${method}(...) cannot be used after free()`);
  }

  function readCurrentValue(input) {
    if (stagedValues.has(input.id)) {
      return stagedValues.get(input.id);
    }
    return input.get();
  }

  function remember(id, value) {
    stagedValues.set(id, value);
  }

  function mergePatchValue(currentValue, patchValue, method) {
    const currentIsObject = currentValue !== null && typeof currentValue === "object";
    const patchIsObject = patchValue !== null && typeof patchValue === "object";
    if (!currentIsObject || !patchIsObject) {
      throw new TypeError(`${operation}(...).${method}(...) requires object or array values`);
    }
    if (Array.isArray(currentValue) || Array.isArray(patchValue)) {
      return patchValue;
    }
    return {
      ...currentValue,
      ...patchValue,
    };
  }

  const transactionOps = [];

  return freezeObject({
    set(input, value) {
      requireActive("set");
      const handle = requireInputHandle(input, "set");
      transactionOps.push({ kind: "set", id: handle.id, value });
      remember(handle.id, value);
    },
    patch(input, value) {
      requireActive("patch");
      const handle = requireInputHandle(input, "patch");
      const nextValue = mergePatchValue(readCurrentValue(handle), value, "patch");
      transactionOps.push({ kind: "set", id: handle.id, value: nextValue });
      remember(handle.id, nextValue);
    },
    setWithAspects(input, value, aspects) {
      requireActive("setWithAspects");
      const handle = requireInputHandle(input, "setWithAspects");
      transactionOps.push({ kind: "set", id: handle.id, value, aspects });
      remember(handle.id, value);
    },
    setWithRegions(input, value, changedRegions) {
      requireActive("setWithRegions");
      const handle = requireInputHandle(input, "setWithRegions");
      transactionOps.push({ kind: "setWithRegions", id: handle.id, value, changedRegions });
      remember(handle.id, value);
    },
    setWithRegionsAndAspects(input, value, changedRegions, aspects) {
      requireActive("setWithRegionsAndAspects");
      const handle = requireInputHandle(input, "setWithRegionsAndAspects");
      transactionOps.push({
        kind: "setWithRegions",
        id: handle.id,
        value,
        changedRegions,
        aspects,
      });
      remember(handle.id, value);
    },
    drain() {
      requireActive("drain");
      return freezeObject(transactionOps.slice());
    },
    free() {
      freed = true;
      stagedValues.clear();
      transactionOps.length = 0;
    },
    [Symbol.dispose]() {
      this.free();
    },
  });
}
