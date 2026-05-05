import { requireResourceDelivery } from "../../delivery/resource_delivery.js";
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
    deliveryState.remember(packetValue.packetId);
    const reloadStatus = executeLineReload(
      materialization,
      "delivery",
      {
        fulfilledEvent: "delivered",
        requestDescriptorOverride: stagedBasis.descriptor,
        finalizeFulfilledDiagnostics(nextDiagnostics) {
          return createDeliveredDiagnostics(
            nextDiagnostics,
            Object.freeze({
              deliveryKind: "basisRefresh",
              deliveryScope: "basis",
              packetId: packetValue.packetId,
              basisId: packetValue.basisId,
              nextBasisId: resolvedNextBasisId,
              supersededOperation,
              patchKind: null,
              patchScope: null,
              patchedItemId: null,
              patchedAspect: null,
              patchedSummary: null,
              valueChanged: false,
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
    const diagnostics = createDeliveredDiagnostics(
      createInvalidatedDiagnostics(
        materialization.binding.diagnosticsSignal(),
        "deliveryInvalidate",
        "line",
      ),
      Object.freeze({
        deliveryKind: "invalidate",
        deliveryScope: "invalidate",
        packetId: packetValue.packetId,
        basisId: packetValue.basisId,
        nextBasisId: resolvedNextBasisId,
        supersededOperation,
        patchKind: null,
        patchScope: null,
        patchedItemId: null,
        patchedAspect: null,
        patchedSummary: null,
        valueChanged: false,
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
    const patchValue =
      packetValue.kind === "replace"
        ? Object.freeze({
            kind: "replace",
            nextValue: packetValue.nextValue,
          })
        : packetValue.patch;
    const patchOutcome = applyPatchValue(materialization, patchValue, currentValue);
    const status = createFulfilledLineStatus("delivery");
    const freshness = createFreshnessFromPolicy(materialization.reload.policy);
    const diagnostics = createDeliveredDiagnostics(
      materialization.binding.diagnosticsSignal(),
      Object.freeze({
        deliveryKind: packetValue.kind,
        deliveryScope: patchOutcome.diagnostics.scope,
        packetId: packetValue.packetId,
        basisId: packetValue.basisId,
        nextBasisId: resolvedNextBasisId,
        supersededOperation,
        patchKind: patchValue.kind,
        patchScope: patchOutcome.diagnostics.scope,
        patchedItemId: patchOutcome.diagnostics.itemId,
        patchedAspect: patchOutcome.diagnostics.aspect,
        patchedSummary: patchOutcome.diagnostics.summary,
        valueChanged: patchOutcome.diagnostics.valueChanged,
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

export { executeLineDelivery };
