import { requireResourceDelivery } from "../../delivery/resource_delivery.js";
import { createDeliveryEffectEnvelope } from "../../effects/resource_effect_envelope.js";
import { createDeliveryEffectPlan } from "../../effects/resource_effect_plan.js";
import { assertLinePatchRecordAdmitsPatch } from "./line_patch_admission.js";
import {
  createBasisRefreshDeliveryEffectSummary,
  createInvalidateDeliveryEffectSummary,
  createPatchDeliveryEffectSummary,
} from "./line_delivery_effect_summary.js";
import { applyPatchValue } from "./line_patch_execution.js";
import { recordLineHistoryEntry } from "../history/record_line_history_entry.js";
import { executeLineReload } from "./line_reload_execution.js";
import {
  createDeliveredDiagnostics,
  createInvalidatedDiagnostics,
} from "../state/line_diagnostics_value.js";
import {
  createFreshnessFromPolicy,
  createInvalidatedFreshness,
} from "../state/line_freshness_value.js";
import { createFulfilledLineStatus } from "../state/line_status_value.js";

function executeLineDelivery(materialization, packet) {
  const packetValue = requireResourceDelivery(
    packet,
    materialization.patch.familyKind,
  );
  const deliveryState = materialization.delivery;
  if (deliveryState.has(packetValue.packetId)) {
    return Object.freeze({
      kind: "duplicateIgnored",
      packetId: packetValue.packetId,
      deliveryKind: packetValue.kind,
    });
  }
  const expectedBasisId = materialization.requestState.currentBasisId();
  if (
    packetValue.basisId !== null
    && packetValue.basisId !== expectedBasisId
  ) {
    return Object.freeze({
      kind: "basisRejected",
      packetId: packetValue.packetId,
      expectedBasisId,
      actualBasisId: packetValue.basisId,
    });
  }
  const currentValue = materialization.binding.valueSignal();
  if (currentValue === null) {
    throw new TypeError(
      `${materialization.patch.familyKind} resource lines do not admit deliver(...) before visible value exists`,
    );
  }

  const patchValue = createPatchValueForDelivery(packetValue);
  if (patchValue !== null) {
    assertLinePatchRecordAdmitsPatch(materialization.patch, patchValue);
  }

  const supersededOperation = materialization.lifecycle.supersedePendingReload();
  if (supersededOperation !== null) {
    recordLineHistoryEntry(
      materialization.lifecycleHistory,
      materialization.binding,
      "superseded",
      { supersededOperation },
    );
  }
  const resolvedNextBasisId =
    packetValue.nextBasisId === undefined
      ? expectedBasisId
      : packetValue.nextBasisId;
  if (packetValue.kind === "basisRefresh") {
    const stagedBasis = materialization.requestState.stageDescriptor(
      resolvedNextBasisId,
    );
    const deliveryEffectSummary = createBasisRefreshDeliveryEffectSummary(
      packetValue,
      resolvedNextBasisId,
      supersededOperation,
    );
    const effectPlan = createDeliveryEffectPlan(
      materialization,
      stagedBasis.descriptor,
      materialization.binding.diagnosticsSignal(),
      deliveryEffectSummary,
    );
    deliveryState.remember(packetValue.packetId);
    const reloadStatus = executeLineReload(
      materialization,
      "delivery",
      {
        fulfilledEvent: "delivered",
        requestDescriptorOverride: stagedBasis.descriptor,
        finalizeFulfilledDiagnostics(nextDiagnostics) {
          const effectEnvelope = createDeliveryEffectEnvelope(
            effectPlan,
            deliveryEffectSummary,
          );
          return createDeliveredDiagnostics(
            nextDiagnostics,
            Object.freeze({
              ...deliveryEffectSummary,
              effectEnvelope,
            }),
          );
        },
        onFulfilled() {
          stagedBasis.commit();
        },
      },
    );
    return Object.freeze({
      kind: "basisRefreshed",
      packetId: packetValue.packetId,
      basisId: packetValue.basisId,
      nextBasisId: resolvedNextBasisId,
      reloadStatus,
    });
  }

  let applied;
  if (packetValue.kind === "invalidate") {
    const status = createFulfilledLineStatus("delivery");
    const freshness = createInvalidatedFreshness("deliveryInvalidate");
    const priorDiagnostics = materialization.binding.diagnosticsSignal();
    const previousDiagnostics = createInvalidatedDiagnostics(
      priorDiagnostics,
      "deliveryInvalidate",
      "line",
    );
    const deliveryEffectSummary = createInvalidateDeliveryEffectSummary(
      packetValue,
      resolvedNextBasisId,
      supersededOperation,
    );
    const effectEnvelope = createDeliveryEffectEnvelope(
      createDeliveryEffectPlan(
        materialization,
        materialization.requestState.readDescriptor(),
        priorDiagnostics,
        deliveryEffectSummary,
      ),
      deliveryEffectSummary,
    );
    const diagnostics = createDeliveredDiagnostics(
      previousDiagnostics,
      Object.freeze({
        ...deliveryEffectSummary,
        effectEnvelope,
      }),
    );
    materialization.binding.statusSignal.set(status);
    materialization.binding.freshnessSignal.set(freshness);
    materialization.binding.diagnosticsSignal.set(diagnostics);
    applied = Object.freeze({
      kind: "applied",
      deliveryKind: "invalidate",
      scope: "invalidate",
      packetId: packetValue.packetId,
      basisId: packetValue.basisId,
      nextBasisId: resolvedNextBasisId,
      supersededOperation,
    });
  } else {
    const effectPlan = createDeliveryEffectPlan(
      materialization,
      materialization.requestState.readDescriptor(),
      materialization.binding.diagnosticsSignal(),
      packetValue,
    );
    const patchOutcome = applyPatchValue(materialization, patchValue, currentValue);
    const status = createFulfilledLineStatus("delivery");
    const freshness = createFreshnessFromPolicy(materialization.reload.policy);
    const previousDiagnostics = materialization.binding.diagnosticsSignal();
    const deliveryEffectSummary = createPatchDeliveryEffectSummary(
      packetValue,
      patchValue,
      patchOutcome.diagnostics,
      resolvedNextBasisId,
      supersededOperation,
    );
    const effectEnvelope = createDeliveryEffectEnvelope(
      effectPlan,
      deliveryEffectSummary,
    );
    const diagnostics = createDeliveredDiagnostics(
      previousDiagnostics,
      Object.freeze({
        ...deliveryEffectSummary,
        effectEnvelope,
      }),
    );
    materialization.binding.statusSignal.set(status);
    materialization.binding.freshnessSignal.set(freshness);
    materialization.binding.diagnosticsSignal.set(diagnostics);
    applied = Object.freeze({
      kind: "applied",
      deliveryKind: packetValue.kind,
      scope: patchOutcome.diagnostics.scope,
      packetId: packetValue.packetId,
      basisId: packetValue.basisId,
      nextBasisId: resolvedNextBasisId,
      supersededOperation,
    });
  }

  materialization.requestState.advanceBasis(resolvedNextBasisId);
  deliveryState.remember(packetValue.packetId);
  recordLineHistoryEntry(
    materialization.lifecycleHistory,
    materialization.binding,
    "delivered",
  );
  return applied;
}

function createPatchValueForDelivery(packetValue) {
  if (packetValue.kind === "replace") {
    return Object.freeze({
      kind: "replace",
      nextValue: packetValue.nextValue,
    });
  }
  if (packetValue.kind === "patch") {
    return packetValue.patch;
  }
  return null;
}

export { executeLineDelivery };
