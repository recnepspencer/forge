import { createResourceEffectDependencySet } from "./resource_effect_dependency_set.js";

function createResourceEffectDependencyIndex() {
  const effects = new Map();
  const reservations = new Map();
  const openIds = new Set();
  const dependents = new Map();
  const loci = new Map();
  const retryLineages = new Map();
  const previousOpenEffect = new Map();
  const nextOpenEffect = new Map();
  let lastOpenEffectId = null;
  let nextAdmissionSequence = 1;
  let openEffectGeneration = 0;

  return Object.freeze({
    plan(effectId, dependencies, canonicalGeneration, closeoutPolicy) {
      if (effects.has(effectId) || reservations.has(effectId)) {
        throw new TypeError(`resource effect ${effectId} is already admitted`);
      }
      const dependencySet = createResourceEffectDependencySet({
        effectId,
        dependencies,
        canonicalGeneration,
        closeoutPolicy,
        lookupEffect: (id) => effects.get(id) ?? null,
        dependsTransitivelyOn,
      });
      reservations.set(effectId, nextAdmissionSequence++);
      return dependencySet;
    },
    register(entry) {
      if (effects.has(entry.effectId)) {
        throw new TypeError(`resource effect ${entry.effectId} is already registered`);
      }
      const admissionSequence = reservations.get(entry.effectId);
      if (admissionSequence === undefined) {
        throw new TypeError(
          `resource effect ${entry.effectId} has no admission reservation`,
        );
      }
      const registered = Object.freeze({
        ...entry,
        admissionSequence,
      });
      reservations.delete(entry.effectId);
      effects.set(entry.effectId, registered);
      openIds.add(entry.effectId);
      openEffectGeneration += 1;
      previousOpenEffect.set(entry.effectId, lastOpenEffectId);
      if (lastOpenEffectId !== null) {
        nextOpenEffect.set(lastOpenEffectId, entry.effectId);
      }
      lastOpenEffectId = entry.effectId;
      for (const dependencyId of entry.dependencySet.dependencyIds) {
        indexValue(dependents, dependencyId, entry.effectId);
      }
      indexValue(loci, entry.locusKey, entry.effectId);
      if (entry.retryLineageId !== null) {
        retryLineages.set(entry.retryLineageId, entry.effectId);
      }
      return registered;
    },
    cancelReservation(effectId) {
      return reservations.delete(effectId);
    },
    replace(effectId, entry) {
      if (!effects.has(effectId)) {
        throw new TypeError(`unknown resource effect ${effectId}`);
      }
      const registered = Object.freeze({
        ...entry,
        admissionSequence: effects.get(effectId).admissionSequence,
      });
      effects.set(effectId, registered);
      return registered;
    },
    retire(effectId, terminal = null) {
      const entry = effects.get(effectId);
      if (entry === undefined) {
        return null;
      }
      effects.set(effectId, Object.freeze({
        ...entry,
        lifecycle: "Retired",
        terminal,
      }));
      if (openIds.delete(effectId)) openEffectGeneration += 1;
      unlinkOpenEffect(effectId);
      unindexRetiredEffect(entry);
      return entry;
    },
    withdraw(effectId) {
      const entry = effects.get(effectId);
      if (entry === undefined) return null;
      if (openIds.delete(effectId)) openEffectGeneration += 1;
      unlinkOpenEffect(effectId);
      unindexRetiredEffect(entry);
      if (entry.retryLineageId !== null) {
        retryLineages.delete(entry.retryLineageId);
      }
      effects.delete(effectId);
      return entry;
    },
    get(effectId) {
      return effects.get(effectId) ?? null;
    },
    open() {
      return topologicalOpenEffects(effects, openIds);
    },
    lastOpen() {
      return lastOpenEffectId === null
        ? null
        : effects.get(lastOpenEffectId) ?? null;
    },
    reverseDependents(effectId) {
      return Object.freeze([...(dependents.get(effectId) ?? [])].sort());
    },
    effectsAtLocus(locusKey) {
      return Object.freeze([...(loci.get(locusKey) ?? [])].sort());
    },
    effectForRetryLineage(retryLineageId) {
      if (retryLineageId === null) {
        return null;
      }
      const effectId = retryLineages.get(retryLineageId);
      return effectId === undefined ? null : effects.get(effectId) ?? null;
    },
    dependsTransitivelyOn,
    projectionIdentity() {
      return Object.freeze({
        openEffectCount: openIds.size,
        openEffectGeneration,
      });
    },
    counters() {
      return Object.freeze({
        effectLookupCount: effects.size,
        pendingAdmissionCount: reservations.size,
        openEffectCount: openIds.size,
        dependencyIndexKeyCount: dependents.size,
        locusIndexKeyCount: loci.size,
        retryLineageIndexKeyCount: retryLineages.size,
      });
    },
  });

  function dependsTransitivelyOn(effectId, candidateAncestorId) {
    const pending = [effectId];
    const visited = new Set();
    while (pending.length > 0) {
      const current = pending.pop();
      if (current === candidateAncestorId) {
        return true;
      }
      if (visited.has(current)) {
        continue;
      }
      visited.add(current);
      const effect = effects.get(current);
      if (effect !== undefined) {
        pending.push(...effect.dependencySet.dependencyIds);
      }
    }
    return false;
  }

  function unlinkOpenEffect(effectId) {
    const previousId = previousOpenEffect.get(effectId) ?? null;
    const nextId = nextOpenEffect.get(effectId) ?? null;
    if (previousId !== null) {
      if (nextId === null) {
        nextOpenEffect.delete(previousId);
      } else {
        nextOpenEffect.set(previousId, nextId);
      }
    }
    if (nextId !== null) {
      previousOpenEffect.set(nextId, previousId);
    }
    if (lastOpenEffectId === effectId) {
      lastOpenEffectId = previousId;
    }
    previousOpenEffect.delete(effectId);
    nextOpenEffect.delete(effectId);
  }

  function unindexRetiredEffect(entry) {
    deleteIndexValue(loci, entry.locusKey, entry.effectId);
    for (const dependencyId of entry.dependencySet.dependencyIds) {
      deleteIndexValue(dependents, dependencyId, entry.effectId);
    }
    if ((dependents.get(entry.effectId)?.size ?? 0) === 0) {
      dependents.delete(entry.effectId);
    }
  }
}

function indexValue(index, key, value) {
  const values = index.get(key) ?? new Set();
  values.add(value);
  index.set(key, values);
}

function deleteIndexValue(index, key, value) {
  const values = index.get(key);
  if (values === undefined) return;
  values.delete(value);
  if (values.size === 0) index.delete(key);
}

function topologicalOpenEffects(effects, openIds) {
  const open = [...openIds].map((effectId) => effects.get(effectId));
  const byId = new Map(open.map((effect) => [effect.effectId, effect]));
  const visited = new Set();
  const ordered = [];
  function visit(effect) {
    if (visited.has(effect.effectId)) {
      return;
    }
    visited.add(effect.effectId);
    for (const dependencyId of effect.dependencySet.dependencyIds) {
      const dependency = byId.get(dependencyId);
      if (dependency !== undefined) {
        visit(dependency);
      }
    }
    ordered.push(effect);
  }
  for (const effect of open.sort(
    (left, right) => left.admissionSequence - right.admissionSequence,
  )) {
    visit(effect);
  }
  return Object.freeze(ordered);
}

export { createResourceEffectDependencyIndex };
