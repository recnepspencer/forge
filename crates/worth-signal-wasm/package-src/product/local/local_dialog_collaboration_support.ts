import { stableDigest } from "./local_dialog_state_projection_support.js";

export const COLLABORATION_POSTURE_RANK = Object.freeze({
  notDeclared: 0,
  active: 1,
  settling: 2,
  blocked: 3,
  unavailable: 4,
});

export function createNativeCollaborationState(declaration) {
  if (!declaration) {
    return Object.freeze({
      declared: false,
      mode: "notDeclared",
      actorId: null,
      posture: "notDeclared",
      reason: "dialog collaboration is not declared",
      lockOwnerId: null,
      leasedModes: Object.freeze([]),
      branchId: null,
      readOnly: false,
      remoteUpdateDigest: null,
      presence: Object.freeze([]),
      comments: Object.freeze([]),
      digest: stableDigest({ declared: false }),
    });
  }
  return Object.freeze({
    declared: true,
    mode: declaration.mode,
    actorId: declaration.actorId ?? null,
    posture: "active",
    reason: "dialog collaboration is active",
    lockOwnerId: null,
    leasedModes: Object.freeze([]),
    branchId: null,
    readOnly: false,
    remoteUpdateDigest: null,
    presence: Object.freeze([]),
    comments: Object.freeze([]),
    digest: stableDigest({ declared: true, mode: declaration.mode, actorId: declaration.actorId ?? null }),
  });
}

export function dialogCollaborationConflicts(nativeCollaboration, formCollaboration) {
  if (
    !nativeCollaboration?.declared
    || !formCollaboration?.declared
    || nativeCollaboration.mode === "notDeclared"
    || formCollaboration.mode === "notDeclared"
    || nativeCollaboration.mode === formCollaboration.mode
  ) {
    return Object.freeze([]);
  }
  return Object.freeze([
    Object.freeze({
      kind: "modeConflict",
      reason: `dialog collaboration mode "${nativeCollaboration.mode}" conflicts with bound form collaboration mode "${formCollaboration.mode}"`,
      nativeMode: nativeCollaboration.mode,
      boundFormMode: formCollaboration.mode,
    }),
  ]);
}

export function dialogCollaborationEventKind(previous, next) {
  if (previous?.posture !== next.posture) {
    return "postureChange";
  }
  if (previous?.lockOwnerId !== next.lockOwnerId) {
    return "lockChange";
  }
  if (stableDigest(previous?.leasedModes ?? []) !== stableDigest(next.leasedModes ?? [])) {
    return "leaseChange";
  }
  if (previous?.branchId !== next.branchId) {
    return "branchChange";
  }
  if (Boolean(previous?.readOnly) !== Boolean(next.readOnly)) {
    return "readOnlyChange";
  }
  if ((previous?.remoteUpdateDigest ?? null) !== (next.remoteUpdateDigest ?? null)) {
    return "remoteUpdateChange";
  }
  if (stableDigest(previous?.presence ?? []) !== stableDigest(next.presence ?? [])) {
    return "presenceChange";
  }
  if (stableDigest(previous?.comments ?? []) !== stableDigest(next.comments ?? [])) {
    return "commentChange";
  }
  return "remoteUpdateChange";
}

export function isSuccessfulBoundFormExecution(execution) {
  return execution?.resultKind === "fulfilled";
}
