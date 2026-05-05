import { loadSignalsModule } from "../../signals_runtime/module_loading/load_signals_module.mjs";
import { loadResourceModule } from "../module_loading/load_resource_module.mjs";

export async function createRealResourceSignals() {
  const mod = await loadSignalsModule({ rawSurface: "real" });
  const signals = mod.createSignals();
  return {
    mod,
    signals,
    async cleanup() {
      try {
        signals.free();
      } finally {
        await mod.cleanup();
      }
    },
  };
}

export async function createRealResourceRuntime() {
  const [signalsRuntime, resourceMod] = await Promise.all([
    createRealResourceSignals(),
    loadResourceModule(),
  ]);
  return {
    ...signalsRuntime,
    resourceMod,
    async cleanup() {
      try {
        await resourceMod.cleanup();
      } finally {
        await signalsRuntime.cleanup();
      }
    },
  };
}

export function createRealResourceDetail(mod, signals, options = {}) {
  return signals.resource.detail({
    params: mod.resourceParams(),
    normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
    ...options,
  });
}

export function createRealResourceCollection(mod, signals, options = {}) {
  return signals.resource.collection({
    params: mod.resourceParams(),
    normalizeParams: ({ workspaceId }) =>
      mod.resourceParamIdentity({ workspaceId }, workspaceId),
    requestContext: mod.resourceRequestContext({ basisId: "basis-1" }),
    itemIdentity: (item) => item.id,
    reconcile: mod.resourceCollectionShape({
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      aspects: mod.resourceItemAspects({
        title: {
          read: (item) => item.title,
          write: (item, title) => ({ ...item, title: String(title) }),
        },
      }),
    }),
    ...options,
  });
}

export function createBranchHead(signals, branchName) {
  const history = signals.history();
  const branch = history.create_branch(branchName);
  history.switch_branch(branch.id);
  const marker = signals.input(0, { debugName: `${branchName}.marker` });
  signals.transaction((tx) => {
    tx.set(marker, 1);
  });
  return history.current_branch();
}

export function installHistoryOverrides(signals, overrides) {
  const originalHistory = signals.history.bind(signals);
  signals.history = () => {
    const history = originalHistory();
    return wrapHistoryObject(history, overrides);
  };
  return () => {
    signals.history = originalHistory;
  };
}

export function createCapabilityRestrictedSignalNamespace(signalNamespace, overrides) {
  return Object.freeze({
    ...signalNamespace,
    history() {
      return wrapHistoryObject(signalNamespace.history(), overrides);
    },
    scope(scopeId) {
      return createCapabilityRestrictedSignalNamespace(
        signalNamespace.scope(scopeId),
        overrides,
      );
    },
  });
}

export function createRealResourceNamespace(resourceMod, signals, overrides = null) {
  const signalNamespace =
    overrides === null
      ? signals
      : createCapabilityRestrictedSignalNamespace(signals, overrides);
  return resourceMod.createResourceNamespace(signalNamespace, {});
}

function wrapHistoryObject(history, overrides) {
  const wrapped = { ...history };
  for (const [key, value] of Object.entries(overrides)) {
    if (!(key in history)) {
      throw new TypeError(
        `history override \`${key}\` is invalid because the real runtime does not expose that capability`,
      );
    }
    wrapped[key] = typeof value === "function"
      ? (...args) => value(history, ...args)
      : value;
  }
  return Object.freeze(wrapped);
}
