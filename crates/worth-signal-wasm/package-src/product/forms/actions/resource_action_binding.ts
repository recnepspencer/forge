import { readResourceLineHandle } from "../sources/form_sources.js";
import { stageResourcePatchLowering } from "./resource_patch_lowering.js";

export function resolveResourceActionBinding(declaration, source, actionId, fieldDeclarations, patch) {
  const declared = declaration.resourceAction ?? null;
  const patchAdmissionBlocker = patch.blocked?.find((blocker) => (
    blocker.kind === "resource:actionUnavailable" && blocker.action === actionId
  )) ?? null;
  const line = readResourceLineHandle(source);
  const history = typeof line?.history === "function" ? line.history() : null;
  const patchCapable = line !== null
    && typeof line.patch === "function"
    && typeof line.reconciliation === "function";
  if (patchAdmissionBlocker !== null) {
    return unavailableBinding(
      actionId,
      declared,
      declaration.id === "submit"
        ? "submitWithoutResourcePatchAdmission"
        : "declaredWithoutResourcePatchAdmission",
      patchAdmissionBlocker.reason,
    );
  }
  if (declaration.id === "submit") {
    if (line !== null && !patchCapable) {
      return unavailableBinding(
        actionId,
        null,
        "submitWithoutPatchCapability",
        "resource-line submit requires a patch-capable resource line",
      );
    }
    if (line !== null) {
      const staged = stageResourcePatchLowering(line, fieldDeclarations, patch, actionId);
      if (staged.kind === "denied") {
        return unavailableBinding(
          actionId,
          null,
          "submitWithoutResourcePatchAdmission",
          staged.reason,
        );
      }
    }
    return Object.freeze({
      declared: false,
      action: null,
      source: patchCapable ? "submitPatchPlan" : "none",
      blockers: Object.freeze([]),
    });
  }
  if (declared === null) {
    return Object.freeze({
      declared: false,
      action: null,
      source: "none",
      blockers: Object.freeze([]),
    });
  }
  if (line === null) {
    return unavailableBinding(
      actionId,
      declared,
      "declaredWithoutResourceLine",
      "declared resource-line action requires a resource line form source",
    );
  }
  if (declared.kind === "refresh" || declared.kind === "revalidate") {
    return Object.freeze({
      declared: true,
      action: declared,
      source: declared.kind === "refresh" ? "declaredRefresh" : "declaredRevalidate",
      blockers: Object.freeze([]),
    });
  }
  if (declared.kind === "replayExact") {
    if (typeof history?.replayExact !== "function") {
      return unavailableBinding(
        actionId,
        declared,
        "declaredWithoutReplayCapability",
        "declared replayExact action requires resource line exact replay capability",
      );
    }
    return Object.freeze({
      declared: true,
      action: declared,
      source: "declaredReplayExact",
      blockers: Object.freeze([]),
    });
  }
  if (declared.kind === "restoreExact") {
    if (typeof history?.restoreExact !== "function") {
      return unavailableBinding(
        actionId,
        declared,
        "declaredWithoutRestoreCapability",
        "declared restoreExact action requires resource line exact restore capability",
      );
    }
    return Object.freeze({
      declared: true,
      action: declared,
      source: "declaredRestoreExact",
      blockers: Object.freeze([]),
    });
  }
  if (declared.kind === "rollbackLastEffect") {
    if (typeof history?.rollbackLastEffect !== "function") {
      return unavailableBinding(
        actionId,
        declared,
        "declaredWithoutRollbackCapability",
        "declared rollbackLastEffect action requires resource line rollback capability",
      );
    }
    return Object.freeze({
      declared: true,
      action: declared,
      source: "declaredRollbackLastEffect",
      blockers: Object.freeze([]),
    });
  }
  if (!patchCapable) {
    return unavailableBinding(
      actionId,
      declared,
      "declaredWithoutPatchCapability",
      "declared resource-line action requires a patch-capable resource line",
    );
  }
  const staged = stageResourcePatchLowering(line, fieldDeclarations, patch, actionId);
  if (staged.kind === "denied") {
    return unavailableBinding(
      actionId,
      declared,
      "declaredWithoutResourcePatchAdmission",
      staged.reason,
    );
  }
  return Object.freeze({
    declared: true,
    action: declared,
    source: "declaredPatchPlan",
    blockers: Object.freeze([]),
  });
}

export function isResolvedResourceActionBinding(resourceAction) {
  return (
    resourceAction !== null &&
    typeof resourceAction === "object" &&
    Object.hasOwn(resourceAction, "declared") &&
    Object.hasOwn(resourceAction, "source") &&
    Object.hasOwn(resourceAction, "blockers")
  );
}

function unavailableBinding(actionId, action, source, reason) {
  return Object.freeze({
    declared: action !== null,
    action,
    source,
    blockers: Object.freeze([Object.freeze({
      kind: "resource:actionUnavailable",
      action: actionId,
      reason,
    })]),
  });
}
