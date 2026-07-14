import { useMemo } from "react";

import { useMaybeReactSignalsStore } from "./context.js";

import type {
  ReactSignalsStore,
  ResourceCatalogDefinition,
  SignalsLike,
} from "./model.js";

type ResourceCatalogCacheEntry = {
  definition: ResourceCatalogDefinition<SignalsLike, unknown>;
  value: unknown;
};

const catalogCacheBySignals = new WeakMap<object, Map<string, ResourceCatalogCacheEntry>>();

function cacheForSignals(signals: object): Map<string, ResourceCatalogCacheEntry> {
  const cached = catalogCacheBySignals.get(signals);
  if (cached) {
    return cached;
  }
  const created = new Map<string, ResourceCatalogCacheEntry>();
  catalogCacheBySignals.set(signals, created);
  return created;
}

export function createResourceCatalog<
  TSignals extends SignalsLike,
  TCatalog,
>(options: {
  id: string;
  build(signals: TSignals): TCatalog;
}): ResourceCatalogDefinition<TSignals, TCatalog>;

export function createResourceCatalog<
  TSignals extends SignalsLike,
  TScope,
  TDomains extends Record<string, (scope: TScope, signals: TSignals) => unknown>,
>(options: {
  id: string;
  scope(signals: TSignals): TScope;
  domains: TDomains;
}): ResourceCatalogDefinition<
  TSignals,
  Readonly<{
    scope: TScope;
    domains: {
      readonly [TKey in keyof TDomains]: ReturnType<TDomains[TKey]>;
    };
  } & {
    readonly [TKey in keyof TDomains]: ReturnType<TDomains[TKey]>;
  }>
>;

export function createResourceCatalog(options: {
  id: string;
  build?(signals: SignalsLike): unknown;
  scope?(signals: SignalsLike): unknown;
  domains?: Record<string, (scope: unknown, signals: SignalsLike) => unknown>;
}): ResourceCatalogDefinition<SignalsLike, unknown> {
  if (typeof options.build === "function") {
    return Object.freeze({
      id: options.id,
      build: options.build,
    });
  }
  if (typeof options.scope !== "function" || !options.domains) {
    throw new TypeError(
      `createResourceCatalog("${options.id}") requires either build(signals) or scope(signals) plus domains`,
    );
  }
  const domainEntries = Object.entries(options.domains);
  return Object.freeze({
    id: options.id,
    build(signals: SignalsLike) {
      const scope = options.scope!(signals);
      const domains = Object.fromEntries(
        domainEntries.map(([name, build]) => [name, build(scope, signals)]),
      );
      return Object.freeze({
        scope,
        domains,
        ...domains,
      });
    },
  });
}

export function getResourceCatalog<
  TSignals extends SignalsLike,
  TCatalog,
>(
  signals: TSignals,
  definition: ResourceCatalogDefinition<TSignals, TCatalog>,
): TCatalog {
  const cache = cacheForSignals(signals);
  const cached = cache.get(definition.id);
  if (cached) {
    if (cached.definition !== definition) {
      throw new TypeError(
        `resource catalog id "${definition.id}" was registered with more than one definition for the same signals runtime`,
      );
    }
    return cached.value as TCatalog;
  }
  const created = definition.build(signals);
  cache.set(definition.id, {
    definition: definition as unknown as ResourceCatalogDefinition<SignalsLike, unknown>,
    value: created,
  });
  return created;
}

export function useResourceCatalog<
  TSignals extends SignalsLike,
  TCatalog,
>(
  store: ReactSignalsStore<TSignals>,
  definition: ResourceCatalogDefinition<TSignals, TCatalog>,
): TCatalog;

export function useResourceCatalog<
  TSignals extends SignalsLike,
  TCatalog,
>(
  definition: ResourceCatalogDefinition<TSignals, TCatalog>,
  store?: ReactSignalsStore<TSignals>,
): TCatalog;

export function useResourceCatalog<
  TSignals extends SignalsLike,
  TCatalog,
>(
  storeOrDefinition: ReactSignalsStore<TSignals> | ResourceCatalogDefinition<TSignals, TCatalog>,
  maybeDefinition?: ResourceCatalogDefinition<TSignals, TCatalog> | ReactSignalsStore<TSignals>,
): TCatalog {
  const providerStore = useMaybeReactSignalsStore();
  const resolved = resolveResourceCatalogHookInputs(
    storeOrDefinition,
    maybeDefinition,
    providerStore,
  );
  return useMemo(
    () => getResourceCatalog(resolved.store.signals, resolved.definition),
    [resolved.definition, resolved.store],
  );
}

function resolveResourceCatalogHookInputs<
  TSignals extends SignalsLike,
  TCatalog,
>(
  storeOrDefinition: ReactSignalsStore<TSignals> | ResourceCatalogDefinition<TSignals, TCatalog>,
  maybeDefinition: ResourceCatalogDefinition<TSignals, TCatalog> | ReactSignalsStore<TSignals> | undefined,
  providerStore: ReactSignalsStore<TSignals> | null,
): {
  readonly store: ReactSignalsStore<TSignals>;
  readonly definition: ResourceCatalogDefinition<TSignals, TCatalog>;
} {
  if (maybeDefinition === undefined) {
    return Object.freeze({
      store: requireProviderStore(providerStore),
      definition: requireResourceCatalogDefinition(storeOrDefinition),
    });
  }
  if (isReactSignalsStore(storeOrDefinition) && isResourceCatalogDefinition(maybeDefinition)) {
    return Object.freeze({
      store: storeOrDefinition,
      definition: maybeDefinition,
    });
  }
  if (isResourceCatalogDefinition(storeOrDefinition) && isReactSignalsStore(maybeDefinition)) {
    return Object.freeze({
      store: maybeDefinition,
      definition: storeOrDefinition,
    });
  }
  throw new TypeError(
    "useResourceCatalog(...) requires either (catalog, store?) or (store, catalog)",
  );
}

function isReactSignalsStore(value: unknown): value is ReactSignalsStore {
  const candidate = value as {
    signals?: unknown;
    subscribeSignal?: unknown;
    getSignalSnapshot?: unknown;
  } | null;
  return Boolean(
    candidate
    && typeof candidate === "object"
    && "signals" in candidate
    && typeof candidate.subscribeSignal === "function"
    && typeof candidate.getSignalSnapshot === "function",
  );
}

function isResourceCatalogDefinition<TSignals extends SignalsLike, TCatalog>(
  value: unknown,
): value is ResourceCatalogDefinition<TSignals, TCatalog> {
  const candidate = value as {
    id?: unknown;
    build?: unknown;
  } | null;
  return Boolean(
    candidate
    && typeof candidate === "object"
    && typeof candidate.id === "string"
    && typeof candidate.build === "function",
  );
}

function requireResourceCatalogDefinition<TSignals extends SignalsLike, TCatalog>(
  value: ReactSignalsStore<TSignals> | ResourceCatalogDefinition<TSignals, TCatalog>,
): ResourceCatalogDefinition<TSignals, TCatalog> {
  if (isResourceCatalogDefinition(value)) {
    return value;
  }
  throw new TypeError(
    "useResourceCatalog(catalog, store?) requires a catalog definition as its first argument when no explicit second argument is provided",
  );
}

function requireProviderStore<TSignals extends SignalsLike>(
  providerStore: ReactSignalsStore<TSignals> | null,
): ReactSignalsStore<TSignals> {
  if (providerStore !== null) {
    return providerStore;
  }
  throw new TypeError(
    "React signals store was not provided. Wrap the tree with <ReactSignalsStoreProvider store={...}> or pass store explicitly.",
  );
}
