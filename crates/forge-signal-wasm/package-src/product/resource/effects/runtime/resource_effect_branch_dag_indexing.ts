import { applyPatchValue } from "../../lines/actions/line_patch_execution.js";

function expandAffectedEffects(index, effectIds) {
  const affected = new Map();
  const visit = (effectId) => {
    if (affected.has(effectId)) {
      return;
    }
    const effect = index.get(effectId);
    if (effect === null) {
      return;
    }
    affected.set(effectId, effect);
    for (const dependentId of index.reverseDependents(effectId)) {
      visit(dependentId);
    }
  };
  for (const effectId of effectIds) {
    visit(effectId);
  }
  return [...affected.values()];
}

function createLocusRefreshes(index, affectedEffects) {
  const byLocus = new Map();
  for (const effect of affectedEffects) {
    if (!byLocus.has(effect.locusKey)) {
      byLocus.set(effect.locusKey, effect);
    }
  }
  return [...byLocus].map(([affectedLocusKey, template]) => Object.freeze({
    locusKey: affectedLocusKey,
    templatePatch: template.patchIntent,
    openEffects: Object.freeze(
      index.effectsAtLocus(affectedLocusKey)
        .map((effectId) => index.get(effectId))
        .filter((effect) => effect?.lifecycle !== "Retired")
        .sort((left, right) =>
          left.admissionSequence - right.admissionSequence),
    ),
  }));
}

function dependencyClosure(index, dependencySet) {
  const requiredIds = new Set();
  const ordered = [];
  const visit = (effectId) => {
    if (requiredIds.has(effectId)) {
      return;
    }
    const effect = requireOpenEffect(index, effectId);
    for (const dependencyId of effect.dependencySet.dependencyIds) {
      visit(dependencyId);
    }
    requiredIds.add(effectId);
    ordered.push(effect);
  };
  for (const effectId of dependencySet.dependencyIds) {
    visit(effectId);
  }
  return ordered;
}

function foldEffects(materialization, canonicalValue, effects) {
  let value = canonicalValue;
  for (const effect of effects) {
    value = applyPatchValue(materialization, effect.patchIntent, value).nextValue;
  }
  return value;
}

function requireOpenEffect(index, effectId) {
  const effect = index.get(effectId);
  if (effect === null || effect.lifecycle === "Retired") {
    throw new TypeError(`unknown open resource effect ${effectId}`);
  }
  return effect;
}

function resourceEffectLocusKey(locus) {
  return JSON.stringify(locus, Object.keys(locus).sort());
}

function publicEffectSummary(effect) {
  return Object.freeze({
    effectId: effect.effectId,
    envelope: effect.envelope,
    lifecycle: effect.lifecycle,
    branchId: Number(effect.branch.branch.branch.id),
    nativeParentBranchId: Number(effect.branch.nativeAncestryProof.parentBranchId),
    dependencyBasisBranchId:
      effect.branch.dependencyBasisBranch === null
        ? null
        : Number(effect.branch.dependencyBasisBranch.branch.id),
    dependencyEffectIds: effect.dependencySet.dependencyIds,
    dependencyCloseoutPolicy: effect.dependencySet.closeoutPolicy,
    locus: effect.envelope.locus,
    admissionSequence: effect.admissionSequence,
    terminal: effect.terminal ?? null,
  });
}

function stableResourcePatchDigest(value) {
  return JSON.stringify(canonicalize(value));
}

function authoredSignalIds(materialization) {
  return [
    materialization.binding.canonicalValueSignal.id,
    materialization.binding.valueSignal.id,
  ];
}

function canonicalize(value) {
  if (Array.isArray(value)) {
    return value.map(canonicalize);
  }
  if (!value || typeof value !== "object") {
    return value;
  }
  const canonical = {};
  for (const key of Object.keys(value).sort()) {
    canonical[key] = canonicalize(value[key]);
  }
  return canonical;
}

export {
  authoredSignalIds,
  createLocusRefreshes,
  dependencyClosure,
  expandAffectedEffects,
  foldEffects,
  publicEffectSummary,
  requireOpenEffect,
  resourceEffectLocusKey,
  stableResourcePatchDigest,
};
