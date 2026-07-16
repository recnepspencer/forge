import { applyPatchValue } from "../../lines/actions/line_patch_execution.js";
import { createLocalPatchEffectEnvelope } from "../resource_effect_envelope.js";
import { createResourceEffectDependencyIndex } from "../dependencies/resource_effect_dependency_index.js";
import { createResourceEffectBranchAcquisitionPlan } from "../branches/resource_effect_branch_acquisition_plan.js";
import {
  executeResourceEffectBranchAcquisition,
  requireBranchCommandSurface,
} from "../branches/resource_effect_branch_acquisition_execution.js";
import {
  cleanupFailedResourceEffectAdmission,
} from "./admission/resource_effect_admission_cleanup.js";
import { executeResourceEffectConfirmation } from "./settlement/resource_effect_confirmation_execution.js";
import { rebuildResourceEffectProjection } from "../projection/resource_effect_projection_rebuild.js";
import {
  authoredSignalIds,
  dependencyClosure,
  foldEffects,
  publicEffectSummary,
  requireOpenEffect,
  resourceEffectLocusKey,
  stableResourcePatchDigest,
} from "./resource_effect_branch_dag_indexing.js";
import { createResourceEffectSettlementRegistry } from "./settlement/resource_effect_settlement_registry.js";
import {
  drainReadyRecordedConfirmations,
} from "./settlement/resource_effect_recorded_confirmation_drain.js";
import {
  createConfirmedEffectCheckpoint,
  createConfirmedEffectResult,
} from "./settlement/resource_effect_confirmation_checkpoint.js";
import { rejectResourceEffectAndDependents } from "./settlement/resource_effect_rejection_execution.js";
import {
  createRejectedEffectCheckpoint,
  finalizeRejectedEffectCheckpoint,
} from "./settlement/resource_effect_rejection_checkpoint.js";
function createResourceEffectBranchDag(materialization, projectionCoordinator) {
  const history = materialization.history;
  const index = createResourceEffectDependencyIndex();
  const settlements = createResourceEffectSettlementRegistry();
  const confirmedEffects = new Map();
  const confirmedEffectsByLocus = new Map();
  let canonicalValueAuthority;
  let canonicalRevisionAuthority;
  const lineId = materialization.lineIdentity.runtimeLineId;

  return Object.freeze({
    admit(effectPlan, patch, currentState) {
      return enqueue(async () => {
      adoptCanonicalEraIfQuiescent(currentState);
      requireBranchCommandSurface(history);
      const admissionCanonicalValue = canonicalValueAuthority;
      const canonicalBasis = await readCanonicalBasis(history);
      const retriedEffect = index.effectForRetryLineage(
        effectPlan.retryLineageId,
      );
      if (retriedEffect !== null) {
        return admitRetryDuplicate(retriedEffect, patch);
      }
      const dependencySet = index.plan(
        effectPlan.planId,
        effectPlan.dependencies,
        canonicalBasis.authoredGraphGeneration,
        effectPlan.dependencyCloseoutPolicy,
      );
      let registered = null;
      try {
        const dependencyEffects = dependencyClosure(index, dependencySet);
        const dependencyBasisValue = foldEffects(
          materialization,
          admissionCanonicalValue,
          dependencyEffects,
        );
        const baseValue = dependencySet.cardinality === 0
          ? admissionCanonicalValue
          : dependencyBasisValue;
        const patchOutcome = applyPatchValue(materialization, patch, baseValue);
        const acquisitionPlan = createResourceEffectBranchAcquisitionPlan({
          effectId: effectPlan.planId,
          canonicalBasis,
          dependencySet,
          dependencyBasisValue,
          dependencyTraversalCount: dependencyEffects.length,
        });
        const acquisition = await executeResourceEffectBranchAcquisition(
          history,
          acquisitionPlan,
          authoredSignalIds(materialization),
          patchOutcome.nextValue,
        );
        const envelope = createLocalPatchEffectEnvelope(
          effectPlan,
          patch,
          patchOutcome.diagnostics,
          acquisition,
        );
        registered = index.register({
          effectId: envelope.effectId,
          envelope,
          patchIntent: patch,
          locusKey: resourceEffectLocusKey(envelope.locus),
          dependencySet,
          branch: acquisition,
          baseValue,
          effectValue: patchOutcome.nextValue,
          canonicalGeneration: canonicalBasis.authoredGraphGeneration,
          lifecycle: "Pending",
          serverRevision: null,
          retryLineageId: envelope.plan.retryLineageId,
          patchResult: patchOutcome.result,
        });
        const rebuilt = await rebuildProjection(
          canonicalValueAuthority,
          [registered.effectId],
        );
        return Object.freeze({ envelope, patchOutcome, projection: rebuilt });
      } catch (error) {
        index.cancelReservation(effectPlan.planId);
        if (registered !== null) {
          await cleanupFailedResourceEffectAdmission(
            history,
            index,
            registered,
            error,
          );
        }
        throw error;
      }
      });
    },
    settle(effectId, settlement, currentState) {
      return enqueue(async () => {
      adoptCanonicalEraIfQuiescent(currentState);
      const settlementAdmission = settlements.begin(effectId, settlement);
      if (settlementAdmission.kind === "duplicate") {
        return settlementAdmission.receipt;
      }
      const settlementToken = settlementAdmission.token;
      try {
      if (settlementAdmission.kind === "resume") {
        return await resumeSettlement(
          settlementAdmission.checkpoint,
          settlementToken,
        );
      }
      const effect = requireOpenEffect(index, effectId);
      if (settlement.kind === "rejected") {
        const retired = await rejectResourceEffectAndDependents(
          history,
          index,
          effect,
        );
        const checkpoint = settlements.checkpoint(
          settlementToken,
          createRejectedEffectCheckpoint(
            effectId,
            retired,
            canonicalValueAuthority,
          ),
        );
        return await finalizeRejectedCheckpoint(checkpoint, settlementToken);
      }
      const unresolvedDependencies = effect.dependencySet.dependencyIds.filter(
        (dependencyId) => !confirmedEffects.has(dependencyId),
      );
      if (unresolvedDependencies.length > 0) {
        index.replace(effectId, {
          ...effect,
          lifecycle: "ResponseRecorded",
          recordedSettlement: settlement,
        });
        const recorded = Object.freeze({
          kind: "responseRecorded",
          effectId,
          waitingOnDependencyEffectIds: Object.freeze(unresolvedDependencies),
          canonicalValue: canonicalValueAuthority,
          projection: projectionCoordinator.projectionFor(lineId),
        });
        index.replace(effectId, {
          ...index.get(effectId),
          settlementToken,
        });
        return settlements.record(settlementToken, recorded);
      }
      const merged = await confirmEffect(effect, settlement, settlementToken);
      const automaticallySettled = await drainRecordedConfirmations([
        effect.effectId,
      ]);
      const result = Object.freeze({
        ...merged,
        automaticallySettled: Object.freeze(automaticallySettled),
        projection: projectionCoordinator.projectionFor(lineId),
      });
      return settlements.terminal(settlementToken, result);
      } catch (error) {
        settlements.cancel(settlementToken);
        throw error;
      }
      });
    },
    openEffects() {
      return Object.freeze(index.open().map(publicEffectSummary));
    },
    effect(effectId) {
      const effect = index.get(effectId);
      return effect === null ? null : publicEffectSummary(effect);
    },
    lastOpenEffect() {
      const effect = index.lastOpen();
      return effect === null ? null : publicEffectSummary(effect);
    },
    projection() {
      return projectionCoordinator.projectionFor(lineId);
    },
    rebuildProjection(currentState) {
      return enqueue(() => {
        adoptCanonicalEraIfQuiescent(currentState);
        return rebuildProjection(canonicalValueAuthority, [], true);
      });
    },
    counters() {
      return index.counters();
    },
  });

  function enqueue(operation) {
    return projectionCoordinator.enqueue(operation);
  }

  function adoptCanonicalEraIfQuiescent(currentState) {
    if (canonicalValueAuthority === undefined) {
      canonicalValueAuthority = currentState.canonicalValue;
      canonicalRevisionAuthority = currentState.canonicalRevision;
      return;
    }
    // Server deliveries (refresh, revalidate, restore) advance canonical truth
    // outside effect settlement. Once the DAG is quiescent, a changed line
    // canonical value supersedes the fold committed by prior merges — and the
    // prior era's confirmed-effect reconciliation history with it. Without
    // this, admission keeps judging patches against a retired canonical era
    // (e.g. denying an insert as duplicate after a delivery removed the item).
    if (
      index.projectionIdentity().openEffectCount === 0
      && canonicalRevisionAuthority !== currentState.canonicalRevision
    ) {
      canonicalValueAuthority = currentState.canonicalValue;
      canonicalRevisionAuthority = currentState.canonicalRevision;
      confirmedEffects.clear();
      confirmedEffectsByLocus.clear();
    }
  }

  async function readCanonicalBasis(runtimeHistory) {
    return projectionCoordinator.canonicalBasis(runtimeHistory);
  }

  async function confirmEffect(effect, settlement, settlementToken) {
    const sameLocus = [...(confirmedEffectsByLocus.get(effect.locusKey) ?? [])];
    const confirmation = await executeResourceEffectConfirmation({
      history,
      effect,
      settlement,
      canonicalBranchId: (await readCanonicalBasis(history)).branchId,
      canonicalValue: canonicalValueAuthority,
      sameLocusConfirmedEffects: sameLocus,
      materialization,
      authoredSignalIds: authoredSignalIds(materialization),
    });
    const checkpoint = settlements.checkpoint(
      settlementToken,
      createConfirmedEffectCheckpoint(effect, settlement, confirmation),
    );
    return finalizeConfirmedCheckpoint(checkpoint);
  }

  async function finalizeConfirmedCheckpoint(checkpoint) {
    const { effect, settlement, confirmation } = checkpoint;
    const current = index.get(effect.effectId);
    if (current?.lifecycle !== "Retired") {
      commitConfirmedEffect(effect, settlement, confirmation);
    }
    await rebuildProjection(confirmation.canonicalValue, [effect.effectId]);
    return createConfirmedEffectResult(effect, confirmation);
  }

  function commitConfirmedEffect(effect, settlement, confirmation) {
    canonicalValueAuthority = confirmation.canonicalValue;
    const confirmedEffect = Object.freeze({
      ...confirmation.settlingEffect,
      patchIntent: settlement.serverPatch ?? effect.patchIntent,
      reconciliation: confirmation.reconciliation,
    });
    confirmedEffects.set(effect.effectId, confirmedEffect);
    const sameLocus = confirmedEffectsByLocus.get(effect.locusKey) ?? new Set();
    sameLocus.add(confirmedEffect);
    confirmedEffectsByLocus.set(effect.locusKey, sameLocus);
    index.retire(effect.effectId, Object.freeze({
      kind: confirmation.reconciliation.conflict.kind === "superseded"
        ? "supersededAndRetired"
        : "merged",
      closeout: confirmation.closeout,
    }));
  }

  async function resumeSettlement(checkpoint, settlementToken) {
    if (checkpoint.kind === "rejectedNativeRetirement") {
      return finalizeRejectedCheckpoint(checkpoint, settlementToken);
    }
    if (checkpoint.kind !== "confirmedNativeCloseout") {
      throw new TypeError("resource effect settlement checkpoint is unsupported");
    }
    const merged = await finalizeConfirmedCheckpoint(checkpoint);
    const automaticallySettled = await drainRecordedConfirmations([
      checkpoint.effect.effectId,
    ]);
    return settlements.terminal(settlementToken, Object.freeze({
      ...merged,
      automaticallySettled: Object.freeze(automaticallySettled),
      projection: projectionCoordinator.projectionFor(lineId),
    }));
  }

  function finalizeRejectedCheckpoint(checkpoint, settlementToken) {
    return finalizeRejectedEffectCheckpoint({
      checkpoint,
      settlementToken,
      settlements,
      rebuildProjection,
    });
  }

  async function drainRecordedConfirmations(confirmedEffectIds) {
    return drainReadyRecordedConfirmations({
      index,
      settlements,
      confirmedEffects,
      confirmedEffectIds,
      confirmEffect,
    });
  }

  async function rebuildProjection(
    canonicalValue,
    affectedEffectIds,
    forceBroadRebuild = false,
  ) {
    return rebuildResourceEffectProjection({
      history,
      index,
      materialization,
      projectionCoordinator,
      lineId,
      canonicalValue,
      affectedEffectIds,
      forceBroadRebuild,
    });
  }

  function admitRetryDuplicate(effect, patch) {
    if (
      stableResourcePatchDigest(effect.patchIntent)
        !== stableResourcePatchDigest(patch)
    ) {
      const error = new TypeError(
        `resource effect retry lineage ${effect.retryLineageId} cannot carry different patch intent`,
      );
      error.name = "ResourceEffectRetryDenial";
      error.code = "retryIntentConflict";
      throw error;
    }
    return Object.freeze({
      kind: "duplicateAdmission",
      effectId: effect.effectId,
      retryLineageId: effect.retryLineageId,
      originalResult: effect.patchResult,
      projection: projectionCoordinator.projectionFor(lineId),
    });
  }
}

export { createResourceEffectBranchDag };
