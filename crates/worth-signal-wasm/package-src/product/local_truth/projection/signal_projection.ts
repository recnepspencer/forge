import { canonicalDigest, deepFreeze } from "../support/canonical.js";

export function createLocalTruthSignalProjection({ schema, bindings, driver }) {
  const bindingByEntity = normalizeBindings(bindings);
  const postureByBranch = new Map();
  const counters = {
    projections: 0,
    rebuilds: 0,
    projectedEntities: 0,
    invalidatedAspects: 0,
  };

  return deepFreeze({
    async initialize(branch, snapshot) {
      return runProjection("initialize", branch, snapshot, allOperations(schema, snapshot));
    },
    async fork(branch, parentBranch) {
      try {
        const binding = await driver.fork({ branch, parentBranch });
        const receipt = currentReceipt("fork", branch.id, null, [], binding);
        postureByBranch.set(branch.id, receipt);
        return receipt;
      } catch (error) {
        return rememberFailure(branch.id, "fork", null, error);
      }
    },
    async project(commit, snapshot) {
      return runProjection("commit", { id: commit.branchId }, snapshot, commit.operations, commit);
    },
    async rebuild(branch, snapshot) {
      counters.rebuilds += 1;
      try {
        const plan = buildPlan(schema, bindingByEntity, branch.id, null, snapshot, allOperations(schema, snapshot));
        const binding = await driver.rebuild(plan);
        const receipt = currentReceipt("rebuild", branch.id, null, plan.updates, binding);
        recordProjection(plan);
        postureByBranch.set(branch.id, receipt);
        return receipt;
      } catch (error) {
        return rememberFailure(branch.id, "rebuild", null, error);
      }
    },
    async destroy(branchId) {
      try {
        await driver.destroy({ branchId });
        const receipt = deepFreeze({
          artifactFamily: "LocalTruthSignalProjectionReceipt",
          branchId,
          commitId: null,
          posture: "RebuildRequired",
          reason: "projectionDestroyed",
        });
        postureByBranch.set(branchId, receipt);
        return receipt;
      } catch (error) {
        return rememberFailure(branchId, "destroy", null, error);
      }
    },
    posture(branchId) {
      return postureByBranch.get(branchId) ?? deepFreeze({
        artifactFamily: "LocalTruthSignalProjectionReceipt",
        branchId,
        commitId: null,
        posture: "CommittedDerivationPending",
        reason: "projectionNotInitialized",
      });
    },
    counters() {
      return deepFreeze({ ...counters });
    },
  });

  async function runProjection(kind, branch, snapshot, operations, commit = null) {
    const plan = buildPlan(schema, bindingByEntity, branch.id, commit?.id ?? null, snapshot, operations);
    const pending = deepFreeze({
      artifactFamily: "LocalTruthSignalProjectionReceipt",
      branchId: branch.id,
      commitId: commit?.id ?? null,
      posture: "CommittedDerivationPending",
      planDigest: plan.digest,
    });
    postureByBranch.set(branch.id, pending);
    try {
      const binding = kind === "initialize"
        ? await driver.initialize(plan)
        : await driver.apply(plan);
      const receipt = currentReceipt(kind, branch.id, commit?.id ?? null, plan.updates, binding);
      recordProjection(plan);
      postureByBranch.set(branch.id, receipt);
      return receipt;
    } catch (error) {
      return rememberFailure(branch.id, kind, commit?.id ?? null, error, plan.digest);
    }
  }

  function recordProjection(plan) {
    counters.projections += 1;
    counters.projectedEntities += plan.counters.projectedEntities;
    counters.invalidatedAspects += plan.counters.invalidatedAspects;
  }

  function rememberFailure(branchId, operation, commitId, error, planDigest = null) {
    const receipt = deepFreeze({
      artifactFamily: "LocalTruthSignalProjectionReceipt",
      branchId,
      commitId,
      posture: "RebuildRequired",
      operation,
      planDigest,
      reason: projectionFailureReason(error),
    });
    postureByBranch.set(branchId, receipt);
    return receipt;
  }
}

function projectionFailureReason(error) {
  if (error instanceof Error) return error.message;
  if (error && typeof error === "object") {
    const code = typeof error.code === "string" ? error.code : null;
    const message = typeof error.message === "string"
      ? error.message
      : typeof error.detail === "string"
        ? error.detail
        : null;
    if (code || message) return [code, message].filter(Boolean).join(": ");
    try {
      return JSON.stringify(
        error,
        (key, value) => (typeof value === "bigint" ? value.toString() : value),
      );
    } catch {
      return String(error);
    }
  }
  return String(error);
}

export function buildPlan(schema, bindingByEntity, branchId, commitId, snapshot, operations) {
  const aspectsByEntity = new Map();
  for (const operation of operations) {
    const aspects = aspectsByEntity.get(operation.entityId) ?? new Set();
    aspects.add(operation.aspectId);
    aspectsByEntity.set(operation.entityId, aspects);
  }
  const updates = [...aspectsByEntity]
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([entityId, aspects]) => {
      const binding = bindingByEntity.get(entityId);
      if (!binding) {
        throw new TypeError(`local truth entity ${entityId} has no Signal projection binding`);
      }
      return deepFreeze({
        entityId,
        signalId: binding.signalId,
        value: snapshot.values[entityId],
        truthAspects: [...aspects].sort(),
        aspects: [...aspects]
          .sort()
          .map((aspectId) => requireSignalAspect(binding, aspectId)),
      });
    });
  return deepFreeze({
    artifactFamily: "LocalTruthSignalProjectionPlan",
    schemaIdentity: schema.identity,
    branchId,
    commitId,
    updates,
    counters: {
      committedLoci: operations.length,
      projectedEntities: updates.length,
      invalidatedAspects: updates.reduce((total, update) => total + update.aspects.length, 0),
    },
    digest: canonicalDigest({ schemaIdentity: schema.identity, branchId, commitId, updates }),
  });
}

function allOperations(schema, snapshot) {
  return Object.keys(snapshot.values).flatMap((entityId) => schema.aspects.map((aspect) => ({
    entityId,
    aspectId: aspect.id,
  })));
}

function normalizeBindings(bindings) {
  if (!Array.isArray(bindings) || bindings.length === 0) {
    throw new TypeError("local truth Signal projection requires at least one entity binding");
  }
  const map = new Map();
  for (const binding of bindings) {
    if (!binding || typeof binding.entityId !== "string" || typeof binding.signalId !== "string") {
      throw new TypeError("local truth Signal projection bindings require entityId and signalId strings");
    }
    if (map.has(binding.entityId)) {
      throw new TypeError(`duplicate local truth Signal projection binding for ${binding.entityId}`);
    }
    if (!binding.aspectMap || typeof binding.aspectMap !== "object" || Array.isArray(binding.aspectMap)) {
      throw new TypeError(`local truth Signal projection binding ${binding.entityId} requires an aspectMap`);
    }
    map.set(binding.entityId, deepFreeze({ ...binding, aspectMap: { ...binding.aspectMap } }));
  }
  return map;
}

function requireSignalAspect(binding, aspectId) {
  const signalAspect = binding.aspectMap[aspectId];
  if (!Number.isInteger(signalAspect) || signalAspect < 0 || signalAspect > 255) {
    throw new TypeError(`local truth aspect ${aspectId} has no valid native Signal aspect binding`);
  }
  return signalAspect;
}

function currentReceipt(operation, branchId, commitId, updates, binding) {
  return deepFreeze({
    artifactFamily: "LocalTruthSignalProjectionReceipt",
    branchId,
    commitId,
    posture: "Current",
    operation,
    binding,
    projectedEntities: updates.length,
    invalidatedAspects: updates.reduce((total, update) => total + update.aspects.length, 0),
    digest: canonicalDigest({ operation, branchId, commitId, updates }),
  });
}
