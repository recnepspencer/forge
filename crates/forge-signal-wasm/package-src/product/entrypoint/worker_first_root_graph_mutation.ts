import { freezeObject } from "../graph_support.js";
import { planWorkerFirstPublishedGraphMutation } from "./sessions/support/worker_first_published_graph_mutation.js";

export async function applyWorkerFirstRootGraphMutation(rootSession, graphId, mutationContext, mutation) {
  const transactionOps = planWorkerFirstPublishedGraphMutation({
    definition: { id: graphId },
    inputDescriptorsByName: mutationContext.inputDescriptorsByName,
    inputAuthoritiesByName: mutationContext.inputAuthoritiesByName,
    initialValuesBySourceId: mutationContext.initialValuesBySourceId,
    cachedInputs: readCurrentInputValues(rootSession, mutationContext.inputDescriptorsByName),
    mutation,
  });
  return rootSession.applyActiveTransaction(transactionOps);
}

export async function runWorkerFirstRootGraphTransaction(
  rootSession,
  graphId,
  mutationContext,
  callback,
  operation,
) {
  if (typeof callback !== "function") {
    throw new TypeError(`signals.graph \`${graphId}\` ${operation}(...) requires a callback`);
  }
  const tx = createWorkerFirstRootGraphTransaction(graphId, mutationContext, operation);
  try {
    callback(tx);
    return rootSession.applyActiveTransaction(tx.drain());
  } finally {
    tx.free();
  }
}

function createWorkerFirstRootGraphTransaction(graphId, mutationContext, operation) {
  const stagedInputs = new Map(mutationContext.currentInputValues);
  let freed = false;
  const transactionOps = [];

  function requireActive(method) {
    if (!freed) {
      return;
    }
    throw new TypeError(`signals.graph \`${graphId}\` ${operation}(...).${method}(...) cannot be used after free()`);
  }

  function resolveInput(input, method) {
    if (typeof input === "string") {
      const descriptor = mutationContext.inputDescriptorsByName[input];
      if (!descriptor) {
        throw new TypeError(
          `signals.graph \`${graphId}\` ${operation}(...).${method}(...) cannot use unknown public input \`${input}\``,
        );
      }
      return descriptor;
    }
    if (
      typeof input !== "function"
      || typeof input.id !== "string"
      || !mutationContext.inputDescriptorById.has(input.id)
    ) {
      throw new TypeError(
        `signals.graph \`${graphId}\` ${operation}(...).${method}(...) requires a published graph input handle or public input name`,
      );
    }
    return mutationContext.inputDescriptorById.get(input.id);
  }

  function requireAuthority(inputName, capability, method) {
    const authority = mutationContext.inputAuthoritiesByName[inputName];
    if (!authority?.[capability]) {
      throw new TypeError(
        `signals.graph \`${graphId}\` ${operation}(...).${method}(...) cannot mutate public input \`${inputName}\` because the graph operational contract denies ${capability}`,
      );
    }
  }

  function rememberValue(sourceId, value) {
    stagedInputs.set(sourceId, value);
  }

  function mergePatchValue(currentValue, patchValue, method) {
    const currentIsObject = currentValue !== null && typeof currentValue === "object";
    const patchIsObject = patchValue !== null && typeof patchValue === "object";
    if (!currentIsObject || !patchIsObject) {
      throw new TypeError(
        `signals.graph \`${graphId}\` ${operation}(...).${method}(...) requires object or array values`,
      );
    }
    if (Array.isArray(currentValue) || Array.isArray(patchValue)) {
      return patchValue;
    }
    return {
      ...currentValue,
      ...patchValue,
    };
  }

  return freezeObject({
    set(input, value) {
      requireActive("set");
      const descriptor = resolveInput(input, "set");
      requireAuthority(descriptor.inputName, "supportsWrite", "set");
      transactionOps.push({ kind: "set", id: descriptor.sourceId, value });
      rememberValue(descriptor.sourceId, value);
    },
    patch(input, value) {
      requireActive("patch");
      const descriptor = resolveInput(input, "patch");
      requireAuthority(descriptor.inputName, "supportsPatch", "patch");
      const nextValue = mergePatchValue(
        stagedInputs.get(descriptor.sourceId),
        value,
        "patch",
      );
      transactionOps.push({ kind: "set", id: descriptor.sourceId, value: nextValue });
      rememberValue(descriptor.sourceId, nextValue);
    },
    setWithAspects(input, value, aspects) {
      requireActive("setWithAspects");
      const descriptor = resolveInput(input, "setWithAspects");
      requireAuthority(descriptor.inputName, "supportsWrite", "setWithAspects");
      transactionOps.push({ kind: "set", id: descriptor.sourceId, value, aspects });
      rememberValue(descriptor.sourceId, value);
    },
    setWithRegions(input, value, changedRegions) {
      requireActive("setWithRegions");
      const descriptor = resolveInput(input, "setWithRegions");
      requireAuthority(descriptor.inputName, "supportsWrite", "setWithRegions");
      transactionOps.push({ kind: "setWithRegions", id: descriptor.sourceId, value, changedRegions });
      rememberValue(descriptor.sourceId, value);
    },
    setWithRegionsAndAspects(input, value, changedRegions, aspects) {
      requireActive("setWithRegionsAndAspects");
      const descriptor = resolveInput(input, "setWithRegionsAndAspects");
      requireAuthority(descriptor.inputName, "supportsWrite", "setWithRegionsAndAspects");
      transactionOps.push({
        kind: "setWithRegions",
        id: descriptor.sourceId,
        value,
        changedRegions,
        aspects,
      });
      rememberValue(descriptor.sourceId, value);
    },
    free() {
      freed = true;
      transactionOps.length = 0;
    },
    [Symbol.dispose]() {
      this.free();
    },
    drain() {
      requireActive("drain");
      if (transactionOps.length === 0) {
        throw new TypeError(
          `signals.graph \`${graphId}\` ${operation}(...) requires at least one staged mutation`,
        );
      }
      return freezeObject(transactionOps.slice());
    },
  });
}

function readCurrentInputValues(rootSession, inputDescriptorsByName) {
  const context = rootSession.currentImportContext();
  const currentValues = new Map();
  for (const descriptor of Object.values(inputDescriptorsByName)) {
    currentValues.set(descriptor.sourceId, context.signalValueById.get(descriptor.sourceId));
  }
  return currentValues;
}
