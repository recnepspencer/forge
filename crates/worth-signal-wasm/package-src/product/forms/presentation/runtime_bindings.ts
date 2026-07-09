import { normalizeCollaborationUpdate } from "../collaboration/artifacts.js";
import { requireDeclaredPresentationScopeTarget } from "./scope_registry.js";
import { normalizeScopedPresentationUpdate } from "./scope_updates.js";

export function createPresentationRuntimeBindings({
  presentationPolicy,
  presentationBindings,
  exits,
  handoffs,
  attachments,
  media,
  messages,
  scopeRegistry,
  collaborationDeclaration,
  collaborations,
}) {
  return Object.freeze({
    reportExit(update) {
      const scoped = normalizeScopedPresentationUpdate(update, presentationPolicy.exit, "exit", scopeRegistry);
      const artifact = exits.report({ ...update, ...scoped });
      presentationBindings.reportPresentationLane("exit", { ...update, ...scoped, __alreadyTracked: true });
      return artifact;
    },
    clearExit(options = {}) {
      const artifact = exits.clear(options.reason ?? null);
      presentationBindings.clearPresentationLane("exit", { ...options, __alreadyTracked: true });
      return artifact;
    },
    reportHandoff(update) {
      const scoped = normalizeScopedPresentationUpdate(update, presentationPolicy.handoff, "handoff", scopeRegistry);
      const artifact = handoffs.report({ ...update, ...scoped });
      presentationBindings.reportPresentationLane("handoff", { ...update, ...scoped, __alreadyTracked: true });
      return artifact;
    },
    clearHandoff(options = {}) {
      const artifact = handoffs.clear(options.reason ?? null);
      presentationBindings.clearPresentationLane("handoff", { ...options, __alreadyTracked: true });
      return artifact;
    },
    reportAttachments(update) {
      const scoped = normalizeScopedPresentationUpdate(update, presentationPolicy.attachments, "attachments", scopeRegistry);
      const artifact = attachments.report({ ...update, ...scoped });
      presentationBindings.reportPresentationLane("attachments", { ...update, ...scoped, __alreadyTracked: true });
      return artifact;
    },
    clearAttachments(options = {}) {
      const artifact = attachments.clear(options.reason ?? null);
      presentationBindings.clearPresentationLane("attachments", { ...options, __alreadyTracked: true });
      return artifact;
    },
    reportMedia(update) {
      const scoped = normalizeScopedPresentationUpdate(update, presentationPolicy.media, "media", scopeRegistry);
      const artifact = media.report({ ...update, ...scoped });
      presentationBindings.reportPresentationLane("media", { ...update, ...scoped, __alreadyTracked: true });
      return artifact;
    },
    clearMedia(options = {}) {
      const artifact = media.clear(options.reason ?? null);
      presentationBindings.clearPresentationLane("media", { ...options, __alreadyTracked: true });
      return artifact;
    },
    reportMessages(update) {
      const scope = update.scope ?? "wholeForm";
      requireDeclaredPresentationScopeTarget(scopeRegistry, scope, update.target ?? null, "message target");
      const artifact = messages.report(update);
      presentationBindings.trackPresentationLane("messages", "messages", scope, {
        status: update.status,
        target: artifact.target,
        reason: String(update.reason),
        token: artifact.messageDigest,
        section: null,
        scopeKind: null,
        surfaceId: null,
        supersessionHandoff: "replace",
      });
      return artifact;
    },
    clearMessages(options = {}) {
      const artifact = messages.clear(options.reason ?? null);
      presentationBindings.clearTrackedPresentationLane(
        "messages",
        "messages",
        "wholeForm",
        options.reason ?? null,
      );
      return artifact;
    },
    reportCollaboration(update) {
      return collaborations.report(normalizeCollaborationUpdate(collaborationDeclaration, update));
    },
    clearCollaboration(options = {}) {
      return collaborations.clear(options.reason ?? undefined);
    },
  });
}
