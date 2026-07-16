import assert from "node:assert/strict";

const ITEM_COUNT = 32;

export function generateConcurrentEffectScenario(seed, count = 10) {
  const random = seededRandom(seed);
  const effects = Array.from({ length: count }, (_, index) => ({
    index,
    locus: index % 4,
    value: `seed-${seed}:effect-${index}`,
    accepted: index === 0 ? true : index === 1 ? false : random() >= 0.45,
    serverRevision: index % 3 === 0 ? 1 + Math.floor(random() * 20) : null,
    dependencyIndexes: dependenciesFor(index),
  }));
  return Object.freeze({
    seed,
    effects: Object.freeze(effects.map(Object.freeze)),
    settlementOrder: Object.freeze(shuffle(
      effects.map((effect) => effect.index),
      random,
    )),
  });
}

export async function runConcurrentEffectScenario(runtime, scenario) {
  const { signals, resourcePatch } = runtime;
  const family = createCertificationFamily(signals, scenario.seed);
  const line = family.line({ runId: String(scenario.seed) });
  await line.awaitSettlement();
  const baselineBranchCount = (await signals.history().branches()).length;
  const effectIds = [];
  const effectIndexById = new Map();

  for (const effect of scenario.effects) {
    const basePatch = family.patch.itemAspect({
      itemId: `item:${effect.locus}`,
      aspect: "title",
      value: effect.value,
    });
    const patch = effect.dependencyIndexes.length === 0
      ? basePatch
      : resourcePatch.dependsOn(
          basePatch,
          effect.dependencyIndexes.map((index) => effectIds[index]),
        );
    const admission = await line.patch(patch, {
      idempotencyKey: `cert:${scenario.seed}:${effect.index}`,
    });
    assert.equal(typeof admission.effectId, "string");
    effectIds.push(admission.effectId);
    effectIndexById.set(admission.effectId, effect.index);
  }

  const settlementKinds = [];
  for (const effectIndex of scenario.settlementOrder) {
    const effect = scenario.effects[effectIndex];
    const effectId = effectIds[effectIndex];
    const summary = line.effects().get(effectId);
    if (summary?.lifecycle === "Retired") continue;
    const options = {
      responseId: `cert:${scenario.seed}:response:${effectIndex}`,
      ...(effect.serverRevision === null
        ? {}
        : { serverRevision: effect.serverRevision }),
    };
    const settled = effect.accepted
      ? await line.effects().confirm(effectId, options)
      : await line.effects().reject(effectId, options);
    settlementKinds.push(settled.kind);
    if (effectIndex % 4 === 0) {
      const duplicate = effect.accepted
        ? await line.effects().confirm(effectId, options)
        : await line.effects().reject(effectId, options);
      assert.equal(duplicate.kind, "duplicateSettlement");
    }
  }

  const expected = referenceCanonicalTitles(scenario);
  const actual = line.value().items.map((item) => item.title);
  assert.deepEqual(actual, expected);
  assert.equal(line.effects().open().length, 0);
  assert.equal(line.effects().projection().kind, "canonical");
  const counters = line.effects().counters();
  assert.equal(counters.openEffectCount, 0);
  assert.equal(counters.pendingAdmissionCount, 0);
  assert.equal(counters.dependencyIndexKeyCount, 0);
  assert.equal(counters.locusIndexKeyCount, 0);
  assert.equal((await signals.history().branches()).length, baselineBranchCount);

  const effectProof = effectIds.map((effectId) => {
    const summary = line.effects().get(effectId);
    const index = effectIndexById.get(effectId);
    const dependencies = summary.dependencyEffectIds.map(
      (dependencyId) => effectIndexById.get(dependencyId),
    );
    return Object.freeze({
      index,
      lifecycle: summary.lifecycle,
      terminalKind: summary.terminal?.kind ?? null,
      dependencies,
      dependencyPolicy: summary.dependencyCloseoutPolicy,
      dependencyProofDigest: JSON.stringify([
        index,
        summary.dependencyCloseoutPolicy,
        dependencies,
      ]),
      runtimeDependencyProofDigestPresent:
        typeof summary.envelope.plan.branch.semanticDependencyProof.proofDigest
          === "string",
    });
  });
  const diagnostics = line.diagnostics();
  const history = line.history();
  const lastHistoryEntry = history.lifecycle.at(-1) ?? null;
  const verification = history.verificationPackage();
  const proof = Object.freeze({
    seed: scenario.seed,
    effectCount: scenario.effects.length,
    canonicalTitles: Object.freeze(actual),
    projectionKind: line.effects().projection().kind,
    effectProof: Object.freeze(effectProof),
    settlementKinds: Object.freeze([...settlementKinds].sort()),
    diagnosticsProof: Object.freeze({
      lastOperation: diagnostics.lastOperation,
      lastOutcome: diagnostics.lastOutcome,
      visibleSelectionKind: diagnostics.visibleSelection.kind,
    }),
    historyProof: Object.freeze({
      lastEvent: lastHistoryEntry?.event ?? null,
      lastOperation: lastHistoryEntry?.lastOperation ?? null,
      replayAvailability: history.availability.replay.kind,
      restoreAvailability: history.availability.restoreExact.kind,
      lastEffectIndex: effectIndexById.get(
        verification.lifecycle.lastEffect?.effectId,
      ) ?? null,
    }),
    counters,
    branchResidue: (await signals.history().branches()).length - baselineBranchCount,
  });
  line.free();
  return proof;
}

export function semanticScenarioDigest(proof) {
  return JSON.stringify({
    effectCount: proof.effectCount,
    canonicalTitles: proof.canonicalTitles,
    projectionKind: proof.projectionKind,
    effectProof: proof.effectProof,
    settlementKinds: proof.settlementKinds,
    diagnosticsProof: proof.diagnosticsProof,
    historyProof: {
      lastEvent: proof.historyProof.lastEvent,
      lastOperation: proof.historyProof.lastOperation,
      lastEffectIndex: proof.historyProof.lastEffectIndex,
    },
    counters: proof.counters,
    branchResidue: proof.branchResidue,
  });
}

export async function runDenialParityProbe(runtime, seed) {
  const family = createCertificationFamily(runtime.signals, `denial-${seed}`);
  const line = family.line({ runId: String(seed) });
  await line.awaitSettlement();
  const baseline = (await runtime.signals.history().branches()).length;
  const denials = [];
  const firstPatch = family.patch.itemAspect({
    itemId: "item:0",
    aspect: "title",
    value: "first",
  });
  try {
    await line.patch(runtime.resourcePatch.dependsOn(
      firstPatch,
      ["missing-effect"],
    ));
  } catch (error) {
    denials.push(Object.freeze({ name: error.name, code: error.code }));
  }
  const admitted = await line.patch(firstPatch, {
    idempotencyKey: `denial:${seed}:retry-lineage`,
  });
  const summary = line.effects().get(admitted.effectId);
  try {
    await line.patch(family.patch.itemAspect({
      itemId: "item:0",
      aspect: "title",
      value: "conflict",
    }), {
      idempotencyKey: `denial:${seed}:retry-lineage`,
    });
  } catch (error) {
    denials.push(Object.freeze({ name: error.name, code: error.code }));
  }
  await line.effects().reject(admitted.effectId);
  assert.equal((await runtime.signals.history().branches()).length, baseline);
  const proof = Object.freeze({
    denials: Object.freeze(denials),
    dependencyProofAuthority:
      summary.envelope.plan.branch.semanticDependencyProof.authority,
    dependencyProofDigest: JSON.stringify(["independent", 0]),
    runtimeDependencyProofDigestPresent:
      typeof summary.envelope.plan.branch.semanticDependencyProof.proofDigest
        === "string",
    counters: line.effects().counters(),
  });
  line.free();
  return proof;
}

export async function runBoundednessProbe(runtime, population) {
  const family = createCertificationFamily(runtime.signals, `slope-${population}`);
  const line = family.line({ runId: String(population) });
  await line.awaitSettlement();
  const baselineBranchCount = (await runtime.signals.history().branches()).length;
  const effectIds = [];
  for (let index = 0; index < population; index += 1) {
    const admission = await line.patch(family.patch.itemAspect({
      itemId: `item:${index}`,
      aspect: "title",
      value: `population-${population}:${index}`,
    }));
    effectIds.push(admission.effectId);
  }
  const settled = await line.effects().confirm(effectIds[0], {
    responseId: `slope:${population}:confirmed`,
  });
  const counters = settled.projection.plan.counters;
  for (const effectId of effectIds.slice(1).reverse()) {
    await line.effects().reject(effectId, {
      responseId: `slope:${population}:reject:${effectId}`,
    });
  }
  assert.equal(line.effects().open().length, 0);
  assert.equal(
    (await runtime.signals.history().branches()).length,
    baselineBranchCount,
  );
  line.free();
  return Object.freeze({ population, counters });
}

export function referenceCanonicalTitles(scenario) {
  const survives = new Map();
  const isConfirmed = (effect) => {
    if (survives.has(effect.index)) return survives.get(effect.index);
    const confirmed = effect.accepted && effect.dependencyIndexes.every(
      (index) => isConfirmed(scenario.effects[index]),
    );
    survives.set(effect.index, confirmed);
    return confirmed;
  };
  const winners = new Map();
  for (const effect of scenario.effects) {
    if (!isConfirmed(effect)) continue;
    const previous = winners.get(effect.locus);
    if (previous === undefined || compareAuthority(previous, effect) < 0) {
      winners.set(effect.locus, effect);
    }
  }
  return Array.from({ length: ITEM_COUNT }, (_, index) =>
    winners.get(index)?.value ?? `loaded-${index}`,
  );
}

function createCertificationFamily(signals, seed) {
  return signals.api({ effects: signals.resource.effects.branchNative() })
    .url(`/certification/${seed}/:runId`)
    .response(signals.resource.response.objectItems()({
      field: "items",
      itemId: (item) => item.id,
      aspects: signals.resource.response.objectAspects()({ title: "title" }),
    }))
    .list({
      load: () => ({
        items: Array.from({ length: ITEM_COUNT }, (_, index) => ({
          id: `item:${index}`,
          title: `loaded-${index}`,
        })),
      }),
    });
}

function dependenciesFor(index) {
  if (index === 3) return [0];
  if (index === 5) return [2, 3];
  if (index >= 6 && index % 3 === 0) return [index - 2];
  return [];
}

function compareAuthority(left, right) {
  return (left.serverRevision ?? -1) - (right.serverRevision ?? -1)
    || left.index - right.index;
}

function shuffle(values, random) {
  const shuffled = [...values];
  for (let index = shuffled.length - 1; index > 0; index -= 1) {
    const other = Math.floor(random() * (index + 1));
    [shuffled[index], shuffled[other]] = [shuffled[other], shuffled[index]];
  }
  return shuffled;
}

function seededRandom(seed) {
  let state = seed >>> 0;
  return () => {
    state = (Math.imul(state, 1_664_525) + 1_013_904_223) >>> 0;
    return state / 0x1_0000_0000;
  };
}
