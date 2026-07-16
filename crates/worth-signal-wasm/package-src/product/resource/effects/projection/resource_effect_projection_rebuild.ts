import { applyPatchValue } from "../../lines/actions/line_patch_execution.js";
import {
  patchLineBindingState,
  readLineBindingState,
} from "../../lines/state/line_binding_state.js";
import { rebindLineProjectionDiagnostics } from "../../lines/state/line_projection_rebinding.js";
import {
  createLocusRefreshes,
  expandAffectedEffects,
} from "../runtime/resource_effect_branch_dag_indexing.js";
import { planResourceOptimisticProjection } from "./resource_optimistic_projection.js";
import { materializeResourceProjection } from "./resource_projection_materialization.js";

async function rebuildResourceEffectProjection(options) {
  const canonicalBasis = await options.projectionCoordinator.canonicalBasis(
    options.history,
  );
  const affected = expandAffectedEffects(options.index, options.affectedEffectIds);
  const locusRefreshes = createLocusRefreshes(options.index, affected);
  const previousProjection = options.projectionCoordinator.projectionFor(
    options.lineId,
  );
  const openEffectIdentity = options.index.projectionIdentity();
  const plan = planResourceOptimisticProjection({
    canonicalBasis,
    canonicalValue: options.canonicalValue,
    openEffectIdentity,
    openEffectCount: openEffectIdentity.openEffectCount,
    affectedEffectIds: affected.map((effect) => effect.effectId),
    affectedLocusKeys: locusRefreshes.map((refresh) => refresh.locusKey),
    materializeProjection: () => materializeResourceProjection({
      materialization: options.materialization,
      canonicalValue: options.canonicalValue,
      loadAllOpenEffects: () => options.index.open(),
      locusRefreshes,
      affectedEffectCount: affected.length,
      forceBroadRebuild: options.forceBroadRebuild,
      previousProjection:
        previousProjection?.kind === "derivedEffectProjectionBranch"
          ? previousProjection
          : null,
      applyPatch: (candidate, value) =>
        applyPatchValue(options.materialization, candidate, value),
    }),
  });
  return options.projectionCoordinator.updateLine({
    lineId: options.lineId,
    history: options.history,
    plan,
    binding: options.materialization.binding,
    publish(receipt) {
      const state = readLineBindingState(options.materialization.binding);
      patchLineBindingState(options.materialization.binding, {
        value: receipt.projectedValue ?? state.canonicalValue,
        diagnostics: rebindLineProjectionDiagnostics(
          state.diagnostics,
          receipt,
        ),
      });
    },
  });
}

export { rebuildResourceEffectProjection };
