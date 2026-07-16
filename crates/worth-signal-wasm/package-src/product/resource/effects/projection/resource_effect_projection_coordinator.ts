function createResourceEffectProjectionCoordinator() {
  const lines = new Map();
  let canonicalBranchId = null;
  let projectionBranch = null;
  let operationTail = Promise.resolve();

  return Object.freeze({
    enqueue(operation) {
      const result = operationTail.then(operation, operation);
      operationTail = result.catch(() => {});
      return result;
    },
    async canonicalBasis(history) {
      if (canonicalBranchId === null) {
        const current = await history.current_branch();
        canonicalBranchId = Number(current.id);
      }
      return history.worker_branch_basis(canonicalBranchId);
    },
    projectionFor(lineId) {
      return lines.get(lineId)?.receipt ?? null;
    },
    async updateLine(request) {
      const previousEntry = lines.get(request.lineId) ?? null;
      const previousPlan = previousEntry?.plan ?? null;
      const entry = previousEntry ?? {
        lineId: request.lineId,
        binding: request.binding,
        publish: request.publish,
        plan: null,
        receipt: null,
      };
      entry.plan = request.plan;
      lines.set(request.lineId, entry);
      try {
        const canonicalBasis = await this.canonicalBasis(request.history);
        const activeEntries = [...lines.values()].filter(hasOpenProjection);
        if (activeEntries.length === 0) {
          const current = await request.history.current_branch();
          if (Number(current.id) !== Number(canonicalBasis.branchId)) {
            await request.history.switch_branch(canonicalBasis.branchId);
          }
          const retiredProjection = await retirePreviousProjection(
            request.history,
            projectionBranch,
          );
          projectionBranch = null;
          publishLineReceipts(canonicalBasis, null, retiredProjection);
          return lines.get(request.lineId).receipt;
        }

        const fork = await request.history.fork_branch({
          name: "resource-effect-projection",
          parentBranchId: canonicalBasis.branchId,
          expectedParentBasis: canonicalBasis,
        });
        const applied = await request.history.apply_transaction_to_branch({
          branchId: fork.branch.id,
          expectedBasis: fork.createdBasis,
          transactionOps: projectionTransactionOps([...lines.values()]),
        });
        await request.history.switch_branch(fork.branch.id);
        const nextProjectionBranch = Object.freeze({
          branch: fork.branch,
          basis: applied.afterBasis,
        });
        let retiredProjection;
        try {
          retiredProjection = await retirePreviousProjection(
            request.history,
            projectionBranch,
          );
        } catch (error) {
          await restorePreviousProjection(
            request.history,
            projectionBranch,
            nextProjectionBranch,
          );
          throw error;
        }
        projectionBranch = nextProjectionBranch;
        publishLineReceipts(
          canonicalBasis,
          projectionBranch,
          retiredProjection,
        );
        return lines.get(request.lineId).receipt;
      } catch (error) {
        if (previousEntry === null) {
          lines.delete(request.lineId);
        } else {
          entry.plan = previousPlan;
        }
        throw error;
      }
    },
    unregisterLine(lineId) {
      lines.delete(lineId);
    },
  });

  async function retirePreviousProjection(history, previousProjection) {
    if (previousProjection === null) {
      return null;
    }
    const current = await history.current_branch();
    if (Number(current.id) === Number(previousProjection.branch.id)) {
      throw new TypeError(
        "resource projection retirement requires another visible branch to be active",
      );
    }
    const liveBasis = await history.worker_branch_basis(
      previousProjection.branch.id,
    );
    return history.retire_branch({
      branchId: previousProjection.branch.id,
      expectedBasis: liveBasis,
      reason: "projectionRebuild",
    });
  }

  async function restorePreviousProjection(history, previous, replacement) {
    if (previous !== null) {
      await history.switch_branch(previous.branch.id);
    }
    const liveReplacementBasis = await history.worker_branch_basis(
      replacement.branch.id,
    );
    await history.retire_branch({
      branchId: replacement.branch.id,
      expectedBasis: liveReplacementBasis,
      reason: "projectionRebuild",
    });
  }

  function publishLineReceipts(canonicalBasis, globalProjection, retired) {
    for (const entry of lines.values()) {
      entry.receipt = createLineReceipt(
        entry.plan,
        canonicalBasis,
        globalProjection,
        retired,
      );
      entry.publish(entry.receipt);
    }
  }
}

function hasOpenProjection(entry) {
  return entry.plan !== null && entry.plan.openEffectCount > 0;
}

function projectionTransactionOps(entries) {
  const operations = new Map();
  for (const entry of entries) {
    const state = entry.binding.state.current;
    if (hasOpenProjection(entry)) {
      addSet(
        operations,
        entry.binding.valueSignal.id,
        entry.plan.projectedValue,
      );
    }
    addSet(
      operations,
      entry.binding.diagnosticsSignal.id,
      state.diagnostics,
    );
    addSet(operations, entry.binding.processingSignal.id, state.processing);
    addSet(operations, entry.binding.uploadSignal.id, state.upload);
    addSet(operations, entry.binding.downloadSignal.id, state.download);
    addSet(operations, entry.binding.statusSignal.id, state.status);
    addSet(operations, entry.binding.freshnessSignal.id, state.freshness);
  }
  return [...operations.values()];
}

function addSet(operations, id, value) {
  operations.set(id, Object.freeze({ kind: "set", id, value }));
}

function createLineReceipt(plan, canonicalBasis, globalProjection, retired) {
  if (plan === null || plan.openEffectCount === 0) {
    return Object.freeze({
      kind: "canonical",
      branch: null,
      basis: canonicalBasis,
      projectedValue: plan?.projectedValue ?? null,
      projectionDigest: plan?.projectionDigest ?? null,
      retiredProjection: retired,
      plan,
      canonicalAuthority: false,
    });
  }
  return Object.freeze({
    kind: "derivedEffectProjectionBranch",
    branch: globalProjection.branch,
    basis: globalProjection.basis,
    projectedValue: plan.projectedValue,
    projectionDigest: plan.projectionDigest,
    affectedEffectIds: plan.affectedEffectIds,
    retiredProjection: retired,
    plan,
    canonicalAuthority: false,
    detail:
      "visible resource truth is a disposable projection rebuilt from canonical truth and open effects",
  });
}

export { createResourceEffectProjectionCoordinator };
