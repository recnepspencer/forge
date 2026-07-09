import {
  createDiagnosticsRecorder,
  emptyPerformanceSummary,
  hostCapabilityPlan,
  onlineCapability,
  parseHostCapabilityPlan,
  persistenceCapability,
  unavailableArtifactKey,
  viewportCapability,
  visibilityCapability,
  clockCapability,
} from "./host_capability_declarations.js";
import {
  registerClockCapability,
  registerOnlineCapability,
  registerPersistenceCapability,
  registerViewportCapability,
  registerVisibilityCapability,
} from "./host_capability_registrations.js";

export {
  clockCapability,
  hostCapabilityPlan,
  onlineCapability,
  persistenceCapability,
  viewportCapability,
  visibilityCapability,
};

export function createHostCapabilities(rawSignals, options) {
  const plan = parseHostCapabilityPlan(options);
  const diagnosticsRecorder = createDiagnosticsRecorder();
  const globalMetrics = emptyPerformanceSummary();
  const exportedUnavailableArtifactKeys = new Set();

  function recordPortableImportDenial(unavailableCallbacks) {
    const callbacksByFamily = new Map();
    for (const artifact of unavailableCallbacks) {
      if (!artifact || typeof artifact.id !== "string") {
        continue;
      }
      const transports = Array.isArray(artifact.hostCapabilityTransports)
        ? artifact.hostCapabilityTransports
        : [];
      for (const transport of transports) {
        if (!transport || typeof transport.family !== "string") {
          continue;
        }
        const ids = callbacksByFamily.get(transport.family) ?? [];
        ids.push(artifact.id);
        callbacksByFamily.set(transport.family, ids);
      }
    }
    const transports = unavailableCallbacks.flatMap((artifact) =>
      Array.isArray(artifact?.hostCapabilityTransports) ? artifact.hostCapabilityTransports : []);
    for (const transport of transports) {
      if (transport?.portableImportOutcome !== "Denied" && transport?.portableImportOutcome !== "Incompatible") {
        continue;
      }
      const callbackIds = callbacksByFamily.get(transport.family) ?? [];
      globalMetrics.hostCapabilityCompatibilityDenialCount += 1;
      diagnosticsRecorder.push({
        kind: "PortableImportDenied",
        family: transport.family,
        registrationId: transport.registrationId,
        compatibility: transport.compatibility,
        invalidationMode: null,
        queuedInvalidationCount: 0,
        previousState: null,
        nextState: null,
        touchedNodes: 0,
        reevaluatedNodes: 0,
        portableImportOutcome: transport.portableImportOutcome,
        portableImportReason: transport.portableImportReason,
        deniedCallbackIds: callbackIds,
      });
    }
  }

  function recordExportedUnavailableCallbacks(unavailableCallbacks) {
    const artifacts = Array.isArray(unavailableCallbacks)
      ? unavailableCallbacks.filter((artifact) =>
        Array.isArray(artifact?.hostCapabilityTransports) &&
          artifact.hostCapabilityTransports.length > 0)
      : [];
    for (const artifact of artifacts) {
      exportedUnavailableArtifactKeys.add(unavailableArtifactKey(artifact));
    }
    globalMetrics.hostCapabilityUnavailabilityArtifactCount = exportedUnavailableArtifactKeys.size;
  }

  if (!plan) {
    return {
      host: Object.freeze({}),
      dispose() {},
      performanceSummary() {
        return { ...globalMetrics };
      },
      latestDiagnosticsEvent() {
        return diagnosticsRecorder.latest();
      },
      recentDiagnosticsEvents() {
        return diagnosticsRecorder.recent();
      },
      recordExportedUnavailableCallbacks,
      recordPortableImportDenial,
    };
  }

  const registrations = [
    plan.viewport && registerViewportCapability(rawSignals, plan.viewport, diagnosticsRecorder),
    plan.visibility && registerVisibilityCapability(rawSignals, plan.visibility, diagnosticsRecorder),
    plan.online && registerOnlineCapability(rawSignals, plan.online, diagnosticsRecorder),
    plan.clock && registerClockCapability(rawSignals, plan.clock, diagnosticsRecorder),
    plan.persistence && registerPersistenceCapability(rawSignals, plan.persistence, diagnosticsRecorder),
  ].filter(Boolean);
  const activeRegistrations = registrations.slice();

  const host = Object.freeze(Object.fromEntries(
    registrations.map((registration) => [registration.hostEntry.descriptor().family, registration.hostEntry]),
  ));

  return {
    host,
    dispose() {
      for (const registration of activeRegistrations.splice(0)) {
        registration.dispose();
      }
    },
    performanceSummary() {
      return registrations.reduce((summary, registration) => {
        const metrics = registration.performanceSummary();
        for (const [key, value] of Object.entries(metrics)) {
          summary[key] = (summary[key] ?? 0) + value;
        }
        return summary;
      }, { ...globalMetrics });
    },
    latestDiagnosticsEvent() {
      return diagnosticsRecorder.latest();
    },
    recentDiagnosticsEvents() {
      return diagnosticsRecorder.recent();
    },
    recordExportedUnavailableCallbacks,
    recordPortableImportDenial,
  };
}
