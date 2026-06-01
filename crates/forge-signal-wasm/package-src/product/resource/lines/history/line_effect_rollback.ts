import { executeLineExactBranchRestore } from "./line_history_restore.js";
import { recordLineHistoryEntry } from "./record_line_history_entry.js";
import { applyPatchValue } from "../actions/line_patch_execution.js";
import { createInverseRollbackDiagnostics } from "../state/line_diagnostics_value.js";
import { createFulfilledLineStatus } from "../state/line_status_value.js";
import {
  patchLineBindingState,
  readLineBindingState,
} from "../state/line_binding_state.js";

function executeLineEffectRollback(materialization, historyRead) {
  const basis = historyRead.basis;
  const effect = readLastEffect(materialization);
  if (effect === null) {
    return createUnavailableRollbackResult({
      reason: "noEffect",
      detail:
        "resource effect rollback is unavailable because the line has no recorded resource effect",
      effectId: null,
      rollback: null,
      basis,
    });
  }
  const rollback = effect.optimistic?.rollback ?? null;
  if (rollback?.kind === "compactInverseAvailable") {
    return executeCompactInverseRollback(
      materialization,
      basis,
      effect.effectId,
      rollback,
    );
  }
  if (rollback?.kind !== "exactBranchRestoreAvailable") {
    return createUnavailableRollbackResult({
      reason: classifyUnavailableRollbackReason(rollback),
      detail: readUnavailableRollbackDetail(effect, rollback),
      effectId: effect.effectId,
      rollback,
      basis,
    });
  }
  const restoreResult = executeLineExactBranchRestore(
    materialization,
    {
      kind: "available",
      mode: rollback.mode,
      branchId: rollback.branchId,
      snapshotId: rollback.snapshotId,
    },
    basis,
  );
  if (isPromiseLike(restoreResult)) {
    return restoreResult.then((settledRestoreResult) => {
      if (settledRestoreResult.kind === "unavailable") {
        return createUnavailableRollbackResult({
          reason: settledRestoreResult.reason,
          detail: settledRestoreResult.detail,
          effectId: effect.effectId,
          rollback,
          basis,
        });
      }
      return Object.freeze({
        kind: "rolledBack",
        mode: settledRestoreResult.mode,
        effectId: effect.effectId,
        branchId: settledRestoreResult.branchId,
        snapshotId: settledRestoreResult.snapshotId,
        basisCurrentId: settledRestoreResult.basisCurrentId,
        basisAdvanceCount: settledRestoreResult.basisAdvanceCount,
        rollback,
        reloadStatus: settledRestoreResult.reloadStatus,
      });
    });
  }
  if (restoreResult.kind === "unavailable") {
    return createUnavailableRollbackResult({
      reason: restoreResult.reason,
      detail: restoreResult.detail,
      effectId: effect.effectId,
      rollback,
      basis,
    });
  }
  return Object.freeze({
    kind: "rolledBack",
    mode: restoreResult.mode,
    effectId: effect.effectId,
    branchId: restoreResult.branchId,
    snapshotId: restoreResult.snapshotId,
    basisCurrentId: restoreResult.basisCurrentId,
    basisAdvanceCount: restoreResult.basisAdvanceCount,
    rollback,
    reloadStatus: restoreResult.reloadStatus,
  });
}

function isPromiseLike(value) {
  return value !== null && typeof value === "object" && typeof value.then === "function";
}

function executeCompactInverseRollback(
  materialization,
  basis,
  effectId,
  rollback,
) {
  const previousState = readLineBindingState(materialization.binding);
  const previousDiagnostics = previousState.diagnostics;
  const currentValue = previousState.value;
  if (currentValue === null) {
    return createUnavailableRollbackResult({
      reason: "rollbackUnavailable",
      detail:
        "resource effect compact inverse rollback is unavailable because the line has no visible value",
      effectId,
      rollback,
      basis,
    });
  }
  const inverseOutcome = applyPatchValue(
    materialization,
    rollback.inverse.patch,
    currentValue,
  );
  const status = createFulfilledLineStatus("restore");
  const diagnostics = createInverseRollbackDiagnostics(
    previousDiagnostics,
    rollback,
    inverseOutcome.diagnostics,
  );
  patchLineBindingState(materialization.binding, {
    value: inverseOutcome.nextValue,
    status,
    diagnostics,
  });
  recordLineHistoryEntry(
    materialization.lifecycleHistory,
    materialization.binding,
    "restored",
  );
  return Object.freeze({
    kind: "rolledBack",
    mode: rollback.mode,
    effectId,
    branchId: rollback.branchId,
    snapshotId: rollback.snapshotId,
    basisCurrentId: basis.currentBasisId,
    basisAdvanceCount: basis.advanceCount,
    rollback,
    reloadStatus: status,
  });
}

function readLastEffect(materialization) {
  try {
    return readLineBindingState(materialization.binding).diagnostics.lastEffect ?? null;
  } catch {
    return null;
  }
}

function classifyUnavailableRollbackReason(rollback) {
  if (rollback === null) {
    return "notApplicable";
  }
  if (rollback.kind === "unavailable") {
    return rollback.reason;
  }
  if (rollback.kind === "notApplicable") {
    return "notApplicable";
  }
  return "rollbackUnavailable";
}

function readUnavailableRollbackDetail(effect, rollback) {
  if (rollback?.detail) {
    return rollback.detail;
  }
  return `resource effect rollback is unavailable for effect "${effect.effectId}" because the effect does not carry exact branch restore proof`;
}

function createUnavailableRollbackResult(options) {
  return Object.freeze({
    kind: "unavailable",
    reason: options.reason,
    detail: options.detail,
    effectId: options.effectId,
    basisCurrentId: options.basis.currentBasisId,
    basisAdvanceCount: options.basis.advanceCount,
    rollback: options.rollback,
  });
}

export { executeLineEffectRollback };
