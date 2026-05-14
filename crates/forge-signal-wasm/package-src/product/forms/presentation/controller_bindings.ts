import { FormDeclarationError } from "../form_errors.js";
import { readPresentationReport } from "./artifacts.js";

export function createPresentationBindings(
  formRef,
  ensureCurrent,
  policy,
  actionDeclarations,
  stepDeclarations,
  settlements,
  handoff,
  exits,
  attachments,
  media,
) {
  function presentation() {
    ensureCurrent();
    return readPresentationReport(
      policy,
      formRef(),
      actionDeclarations,
      stepDeclarations,
      settlements,
    );
  }

  function presentationLifecycle(laneId) {
    const report = presentation();
    if (laneId === undefined) {
      return report;
    }
    return report.lanes.find((lane) => lane.id === laneId) ?? null;
  }

  function acknowledgePresentation(laneId) {
    const lane = presentationLifecycle(laneId);
    if (!lane) {
      throw new FormDeclarationError("presentation lane is not declared", { laneId });
    }
    return settlements.acknowledge(lane);
  }

  function timeoutPresentation(laneId, options = {}) {
    const lane = presentationLifecycle(laneId);
    if (!lane) {
      throw new FormDeclarationError("presentation lane is not declared", { laneId });
    }
    return settlements.timeout(lane, options.reason ?? null);
  }

  function presentationHistory() {
    return settlements.history();
  }

  function reportPresentationLane(laneId, update) {
    const lane = externalLaneDefinition(laneId);
    if (laneId === "handoff" && update.__alreadyTracked !== true) {
      handoff.report({
        status: update.status,
        target: update.target,
        reason: update.reason,
        token: update.token,
        operation: "generic",
      });
    }
    if (laneId === "exit" && update.__alreadyTracked !== true) {
      exits.report({
        status: update.status,
        target: update.target,
        reason: update.reason,
        token: update.token,
        operation: "generic",
      });
    }
    if (laneId === "attachments" && update.__alreadyTracked !== true) {
      attachments.report({
        status: update.status,
        target: update.target,
        reason: update.reason,
        token: update.token,
        operation: "generic",
      });
    }
    if (laneId === "media" && update.__alreadyTracked !== true) {
      media.report({
        status: update.status,
        target: update.target,
        reason: update.reason,
        token: update.token,
        operation: "generic",
      });
    }
    return settlements.reportExternalLane(
      laneId,
      lane.id,
      lane.scope,
      normalizeExternalUpdate(update, lane.policy),
    );
  }

  function clearPresentationLane(laneId, options = {}) {
    const lane = externalLaneDefinition(laneId);
    if (laneId === "handoff" && options.__alreadyTracked !== true) {
      handoff.clear(options.reason ?? null);
    }
    if (laneId === "exit" && options.__alreadyTracked !== true) {
      exits.clear(options.reason ?? null);
    }
    if (laneId === "attachments" && options.__alreadyTracked !== true) {
      attachments.clear(options.reason ?? null);
    }
    if (laneId === "media" && options.__alreadyTracked !== true) {
      media.clear(options.reason ?? null);
    }
    return settlements.clearExternalLane(laneId, lane.id, lane.scope, options.reason ?? null);
  }

  return Object.freeze({
    presentation,
    presentationLifecycle,
    reportPresentationLane,
    clearPresentationLane,
    acknowledgePresentation,
    timeoutPresentation,
    presentationHistory,
  });

  function externalLaneDefinition(laneId) {
    const report = presentation();
    const lane = report.lanes.find((entry) => entry.id === laneId);
    if (laneId === "collaboration" && formRef().collaboration().declared) {
      throw new FormDeclarationError(
        "declared collaboration must be updated through reportCollaboration/clearCollaboration",
        { laneId },
      );
    }
    if (
      !lane ||
      (lane.lane !== "collaboration" &&
        lane.lane !== "exit" &&
        lane.lane !== "attachments" &&
        lane.lane !== "media" &&
        lane.lane !== "handoff")
    ) {
      throw new FormDeclarationError("presentation lane is not a declared external lane", { laneId });
    }
    return lane;
  }
}

function normalizeExternalUpdate(update, policy) {
  if (!update || typeof update !== "object" || Array.isArray(update)) {
    throw new FormDeclarationError("presentation lane update must be an object", { update });
  }
  if (
    update.status !== "pending" &&
    update.status !== "busy" &&
    update.status !== "settling" &&
    update.status !== "ready" &&
    update.status !== "failed" &&
    update.status !== "unavailable"
  ) {
    throw new FormDeclarationError("presentation lane status is not supported", {
      status: update.status,
    });
  }
  return Object.freeze({
    status: update.status,
    target: update.target === undefined ? null : String(update.target),
    reason: String(update.reason),
    token: update.token === undefined || update.token === null ? null : String(update.token),
    supersessionHandoff: policy.supersessionHandoff,
  });
}
