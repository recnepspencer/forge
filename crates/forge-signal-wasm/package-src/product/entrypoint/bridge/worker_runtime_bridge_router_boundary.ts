import { createCanonicalDigest } from "../../router/url_authority/router_verification_packages.js";
import { createBrowserHistoryStory } from "../../router/projection/ingress/router_browser_history_story.js";
import {
  isRouterBrowserHistoryIngress,
  normalizeWorkerBrowserHistoryIngress,
} from "../../router/projection/ingress/router_browser_history_ingress.js";
import {
  createBrowserAuthorityBoundaryArtifact,
  createRouterBrowserAuthorityCoherence,
  normalizeOptionalBrowserAuthorityCoherence,
} from "../../router/projection/ingress/router_browser_authority_coherence.js";
import { createRawLocationAuthority } from "../../router/url_authority/router_url_authority.js";
import {
  isRouterBrowserHistoryWriteback,
  requireRouterBrowserHistoryWriteback,
} from "../../router/projection/ingress/router_browser_history_writeback.js";

function normalizeBridgeBrowserHistoryIngress(ingress, operation) {
  if (isRouterBrowserHistoryIngress(ingress)) {
    const normalizedIngress = normalizeWorkerBrowserHistoryIngress(ingress, operation);
    return Object.freeze({
      ...normalizedIngress,
      ...serializeBridgeBrowserAuthorityCoherence(ingress.coherence ?? null),
    });
  }
  const normalizedIngress = normalizeWorkerBrowserHistoryIngress(ingress, operation);
  const coherence = normalizeLegacyWorkerBrowserAuthorityCoherence(ingress, operation);
  return Object.freeze({
    ...normalizedIngress,
    ...serializeBridgeBrowserAuthorityCoherence(coherence),
  });
}

function createBridgeBrowserHistoryIngressReport(ingress, runtimeReport) {
  const routeWasAdmitted = runtimeReport.runtimeAdmittedRouteCount > 0;
  const coherence = restoreBridgeBrowserAuthorityCoherence(ingress);
  const diagnostics = Object.freeze({
    boundarySource: "browserHistoryIngress",
    boundaryArtifact: createBrowserAuthorityBoundaryArtifact({ coherence }, {
      kind: routeWasAdmitted ? "admitted" : "notFound",
      routeId: routeWasAdmitted ? ingress.routeIdentity : null,
    }),
    navigationKind: ingress.navigationKind,
    rawLocationHref: ingress.rawLocation,
    routeIdentity: ingress.routeIdentity,
    coherenceKind: coherence?.coherenceKind ?? null,
    outcomeKind: routeWasAdmitted ? "admitted" : "notFound",
    routeId: routeWasAdmitted ? ingress.routeIdentity : null,
    href: routeWasAdmitted ? ingress.rawLocation : null,
  });
  return Object.freeze({
    ...runtimeReport,
    navigationKind: ingress.navigationKind,
    rawLocationHref: ingress.rawLocation,
    routeIdentity: ingress.routeIdentity,
    runtimeRouteSourceId: ingress.runtimeRouteSourceId ?? null,
    runtimeContinuitySourceId: ingress.runtimeContinuitySourceId ?? null,
    coherence() {
      return coherence;
    },
    outcome() {
      return Object.freeze({
        kind: routeWasAdmitted ? "admitted" : "notFound",
        routeIdentity: ingress.routeIdentity,
        href: ingress.rawLocation,
      });
    },
    diagnostics() {
      return diagnostics;
    },
    verification() {
      return Object.freeze({
        browserHistoryEnvelopeDigest: runtimeReport.browserHistoryEnvelopeDigest,
        routeTruthDigest: runtimeReport.routeTruthDigest,
        continuityDigest: runtimeReport.continuityDigest,
        replayRestoreDigest: runtimeReport.replayRestoreDigest,
        workerFirstTruthDigest: runtimeReport.workerFirstTruthDigest,
      });
    },
  });
}

function normalizeBridgeBrowserHistoryWriteback(writeback, operation) {
  if (isRouterBrowserHistoryWriteback(writeback)) {
    return requireRouterBrowserHistoryWriteback(writeback, operation);
  }
  if (!writeback || typeof writeback !== "object" || Array.isArray(writeback)) {
    throw new TypeError(`${operation} expects a browser-history writeback object`);
  }
  const {
    navigationKind,
    targetKind,
    targetHref,
    routeIdentity = null,
    runtimeRouteSourceId = null,
    routeValue,
    runtimeContinuitySourceId = null,
    continuityValue,
    coherence = null,
    ...unknownOptions
  } = writeback;
  const unknownKeys = Object.keys(unknownOptions);
  if (unknownKeys.length > 0) {
    throw new TypeError(`${operation} does not support: ${unknownKeys.join(", ")}`);
  }
  const normalizedNavigationKind = requireWritebackNavigationKind(navigationKind, operation);
  const normalizedTargetKind = requireWritebackTargetKind(targetKind, operation);
  if (normalizedNavigationKind === "external" && normalizedTargetKind !== "external") {
    throw new TypeError(`${operation} external navigation requires targetKind \"external\"`);
  }
  if (normalizedNavigationKind !== "external" && normalizedTargetKind !== "local") {
    throw new TypeError(`${operation} local writeback requires targetKind \"local\"`);
  }
  const normalizedTargetHref = requireNonEmptyString(targetHref, `${operation} targetHref`);
  if (normalizedTargetKind === "local" && routeIdentity == null) {
    throw new TypeError(`${operation} requires routeIdentity for local graph-issued writeback`);
  }
  return Object.freeze({
    kind: "routerBrowserHistoryWriteback",
    navigationKind: normalizedNavigationKind,
    targetKind: normalizedTargetKind,
    targetHref: normalizedTargetHref,
    rawLocation: normalizedTargetKind === "local"
      ? createRawLocationAuthority(normalizedTargetHref, {
          navigationType: normalizedNavigationKind === "pushstate" ? "push" : "replace",
        })
      : null,
    routeIdentity: normalizeOptionalString(routeIdentity, `${operation} routeIdentity`),
    runtimeRouteSourceId: normalizeOptionalString(
      runtimeRouteSourceId,
      `${operation} runtimeRouteSourceId`,
    ),
    routeValue,
    runtimeContinuitySourceId: normalizeOptionalString(
      runtimeContinuitySourceId,
      `${operation} runtimeContinuitySourceId`,
    ),
    continuityValue,
    ...serializeBridgeBrowserAuthorityCoherence(
      normalizeOptionalBrowserAuthorityCoherence(
        coherence,
        `${operation} coherence`,
      ),
    ),
    verification() {
      const normalizedCoherence = normalizeOptionalBrowserAuthorityCoherence(
        coherence,
        `${operation} coherence`,
      );
      return Object.freeze({
        browserHistoryWritebackDigest: createCanonicalDigest("worker-bridge-browser-history-writeback", {
          navigationKind: normalizedNavigationKind,
          targetKind: normalizedTargetKind,
          targetHref: normalizedTargetHref,
          routeIdentity: routeIdentity ?? null,
          runtimeRouteSourceId: runtimeRouteSourceId ?? null,
          runtimeContinuitySourceId: runtimeContinuitySourceId ?? null,
          routeValue,
          continuityValue,
          coherenceDigest:
            normalizedCoherence?.verification().browserAuthorityCoherenceDigest ?? null,
        }),
      });
    },
  });
}

function createBridgeBrowserHistoryStory(initialReport) {
  return createBrowserHistoryStory(initialReport);
}

async function createBridgeBrowserHistoryWritebackReport(
  writeback,
  admitBrowserHistoryIngress,
  readDiagnosticsSummary,
) {
  if (writeback.rawLocation !== null) {
    const ingressReport = await admitBrowserHistoryIngress({
      navigationKind: writeback.navigationKind,
      rawLocation: writeback.rawLocation.href,
      routeIdentity: writeback.routeIdentity,
      ...(writeback.runtimeRouteSourceId === null
        ? {}
        : { runtimeRouteSourceId: writeback.runtimeRouteSourceId }),
      ...(writeback.routeValue === undefined
        ? {}
        : { routeValue: writeback.routeValue }),
      ...(writeback.runtimeContinuitySourceId === null
        ? {}
        : { runtimeContinuitySourceId: writeback.runtimeContinuitySourceId }),
      ...(writeback.continuityValue === undefined
        ? {}
        : { continuityValue: writeback.continuityValue }),
      ...(writeback.coherence == null
        ? {}
        : { coherence: writeback.coherence }),
    });
    return createWorkerBridgeLocalWritebackReport(writeback, ingressReport);
  }
  const diagnosticsSummary = await readDiagnosticsSummary();
  return createWorkerBridgeExternalWritebackReport(writeback, diagnosticsSummary);
}

function createWorkerBridgeLocalWritebackReport(writeback, ingressReport) {
  const routeWasAdmitted = ingressReport.runtimeAdmittedRouteCount > 0;
  const coherence = restoreBridgeBrowserAuthorityCoherence(writeback);
  const diagnostics = Object.freeze({
    boundarySource: "browserHistoryWriteback",
    boundaryArtifact: createBrowserAuthorityBoundaryArtifact({ coherence }, {
      kind: routeWasAdmitted ? "admitted" : "notFound",
      routeId: routeWasAdmitted ? writeback.routeIdentity : null,
    }),
    navigationKind: writeback.navigationKind,
    targetKind: writeback.targetKind,
    targetHref: writeback.targetHref,
    routeIdentity: writeback.routeIdentity,
    coherenceKind: coherence?.coherenceKind ?? null,
    outcomeKind: routeWasAdmitted ? "admitted" : "notFound",
    routeId: routeWasAdmitted ? writeback.routeIdentity : null,
    href: routeWasAdmitted ? writeback.targetHref : null,
  });
  return Object.freeze({
    envelopeFamily: "browserHistoryWriteback",
    causality: ingressReport.causality,
    navigationKind: writeback.navigationKind,
    targetKind: writeback.targetKind,
    targetHref: writeback.targetHref,
    routeIdentity: writeback.routeIdentity,
    runtimeRouteSourceId: writeback.runtimeRouteSourceId,
    runtimeContinuitySourceId: writeback.runtimeContinuitySourceId,
    coherence() {
      return coherence;
    },
    boundaryArtifact: diagnostics.boundaryArtifact,
    browserHistoryWritebackDigest: writeback.verification().browserHistoryWritebackDigest,
    routeTruthDigest: createCanonicalDigest("worker-browser-history-writeback-route-truth", {
      browserHistoryWritebackDigest: writeback.verification().browserHistoryWritebackDigest,
      ingressRouteTruthDigest: ingressReport.routeTruthDigest,
      targetHref: writeback.targetHref,
      routeIdentity: writeback.routeIdentity,
      coherenceDigest:
        coherence?.verification().browserAuthorityCoherenceDigest ?? null,
      boundaryArtifact: diagnostics.boundaryArtifact,
    }),
    boundaryStoryDigest: createCanonicalDigest("worker-browser-history-writeback-boundary-story", {
      browserHistoryWritebackDigest: writeback.verification().browserHistoryWritebackDigest,
      targetKind: writeback.targetKind,
      targetHref: writeback.targetHref,
      routeIdentity: writeback.routeIdentity,
      coherenceKind: coherence?.coherenceKind ?? null,
      boundaryArtifact: diagnostics.boundaryArtifact,
    }),
    runtimeAdmittedRouteCount: ingressReport.runtimeAdmittedRouteCount,
    runtimeMutationBreadth: ingressReport.runtimeMutationBreadth,
    workerFirstTruthDigest: ingressReport.workerFirstTruthDigest,
    performance: ingressReport.performance,
    ambientLocationReadDenied: ingressReport.ambientLocationReadDenied,
    outcome() {
      return Object.freeze({
        kind: routeWasAdmitted ? "admitted" : "notFound",
        routeIdentity: writeback.routeIdentity,
        href: writeback.targetHref,
      });
    },
    diagnostics() {
      return diagnostics;
    },
    verification() {
      return Object.freeze({
        browserHistoryWritebackDigest: writeback.verification().browserHistoryWritebackDigest,
        routeTruthDigest: createCanonicalDigest("worker-browser-history-writeback-route-truth-verification", {
          ingressRouteTruthDigest: ingressReport.routeTruthDigest,
          targetHref: writeback.targetHref,
          routeIdentity: writeback.routeIdentity,
          coherenceDigest:
            coherence?.verification().browserAuthorityCoherenceDigest ?? null,
          boundaryArtifact: diagnostics.boundaryArtifact,
        }),
        boundaryStoryDigest: createCanonicalDigest("worker-browser-history-writeback-boundary-story-verification", {
          boundaryArtifact: diagnostics.boundaryArtifact,
          targetHref: writeback.targetHref,
          routeIdentity: writeback.routeIdentity,
          coherenceKind: coherence?.coherenceKind ?? null,
        }),
      });
    },
  });
}

function createWorkerBridgeExternalWritebackReport(writeback, diagnosticsSummary) {
  const performance = createZeroBoundaryPerformance(
    writeback.verification().browserHistoryWritebackDigest,
  );
  const coherence = restoreBridgeBrowserAuthorityCoherence(writeback);
  return Object.freeze({
    envelopeFamily: "browserHistoryWriteback",
    causality: null,
    navigationKind: writeback.navigationKind,
    targetKind: writeback.targetKind,
    targetHref: writeback.targetHref,
    routeIdentity: writeback.routeIdentity,
    runtimeRouteSourceId: writeback.runtimeRouteSourceId,
    runtimeContinuitySourceId: writeback.runtimeContinuitySourceId,
    coherence() {
      return coherence;
    },
    boundaryArtifact: "externalNavigationEscaped",
    browserHistoryWritebackDigest: writeback.verification().browserHistoryWritebackDigest,
    routeTruthDigest: createCanonicalDigest("worker-browser-history-external-route-truth", {
      browserHistoryWritebackDigest: writeback.verification().browserHistoryWritebackDigest,
      workerFirstTruthDigest: diagnosticsSummary.workerFirstTruthDigest,
      targetHref: writeback.targetHref,
      routeIdentity: writeback.routeIdentity,
      coherenceDigest:
        coherence?.verification().browserAuthorityCoherenceDigest ?? null,
    }),
    boundaryStoryDigest: createCanonicalDigest("worker-browser-history-external-boundary-story", {
      browserHistoryWritebackDigest: writeback.verification().browserHistoryWritebackDigest,
      targetHref: writeback.targetHref,
      routeIdentity: writeback.routeIdentity,
      coherenceKind: coherence?.coherenceKind ?? null,
      boundaryArtifact: "externalNavigationEscaped",
    }),
    runtimeAdmittedRouteCount: 0,
    runtimeMutationBreadth: 0,
    workerFirstTruthDigest: diagnosticsSummary.workerFirstTruthDigest,
    performance,
    ambientLocationReadDenied: true,
    outcome() {
      return null;
    },
    diagnostics() {
      return Object.freeze({
        boundarySource: "browserHistoryWriteback",
        boundaryArtifact: "externalNavigationEscaped",
        navigationKind: writeback.navigationKind,
        targetKind: writeback.targetKind,
        targetHref: writeback.targetHref,
        routeIdentity: writeback.routeIdentity,
        coherenceKind: coherence?.coherenceKind ?? null,
        outcomeKind: null,
        routeId: null,
        href: null,
      });
    },
    verification() {
      return Object.freeze({
        browserHistoryWritebackDigest: writeback.verification().browserHistoryWritebackDigest,
        routeTruthDigest: createCanonicalDigest("worker-browser-history-external-route-truth-verification", {
          workerFirstTruthDigest: diagnosticsSummary.workerFirstTruthDigest,
          targetHref: writeback.targetHref,
          coherenceDigest:
            coherence?.verification().browserAuthorityCoherenceDigest ?? null,
        }),
        boundaryStoryDigest: createCanonicalDigest("worker-browser-history-external-boundary-story-verification", {
          boundaryArtifact: "externalNavigationEscaped",
          targetHref: writeback.targetHref,
          coherenceKind: coherence?.coherenceKind ?? null,
        }),
      });
    },
  });
}

function createZeroBoundaryPerformance(identity) {
  return Object.freeze({
    bridgeEnvelopeCount: 1,
    submittedItemCount: 0,
    coalescedItemCount: 0,
    runtimeAdmittedItemCount: 0,
    runtimeMutationBreadth: 0,
    ambientWorkerReadCount: 0,
    diagnosticsColdReconstructionCount: 0,
    payloadIdentityByteCount: 0,
    performanceDigest: createCanonicalDigest("worker-browser-history-zero-performance", {
      identity,
    }),
  });
}

function requireWritebackNavigationKind(value, operation) {
  if (value === "pushstate" || value === "replacestate" || value === "external") {
    return value;
  }
  throw new TypeError(
    `${operation} navigationKind must be one of pushstate, replacestate, external`,
  );
}

function requireWritebackTargetKind(value, operation) {
  if (value === "local" || value === "external") {
    return value;
  }
  throw new TypeError(`${operation} targetKind must be one of local, external`);
}

function requireNonEmptyString(value, label) {
  if (typeof value === "string" && value.trim().length > 0) {
    return value;
  }
  throw new TypeError(`${label} must be a non-empty string`);
}

function normalizeOptionalString(value, label) {
  if (value === null || value === undefined) {
    return null;
  }
  return requireNonEmptyString(value, label);
}

function normalizeLegacyWorkerBrowserAuthorityCoherence(ingress, operation) {
  if (!ingress || typeof ingress !== "object" || Array.isArray(ingress)) {
    return null;
  }
  if ("coherence" in ingress) {
    return normalizeOptionalBrowserAuthorityCoherence(ingress.coherence, `${operation} coherence`);
  }
  const {
    coherenceKind = null,
    coherenceChannelId = null,
    coherenceSourceTabId = null,
    expectedRouteId = null,
  } = ingress;
  if (coherenceKind == null) {
    return null;
  }
  return normalizeOptionalBrowserAuthorityCoherence(
    createRouterBrowserAuthorityCoherence(
      coherenceKind,
      coherenceChannelId,
      {
        sourceTabId: coherenceSourceTabId,
        expectedRouteId,
      },
    ),
    `${operation} coherence`,
  );
}

function serializeBridgeBrowserAuthorityCoherence(coherence) {
  return Object.freeze({
    coherenceKind: coherence?.coherenceKind ?? null,
    coherenceChannelId: coherence?.channelId ?? null,
    coherenceSourceTabId: coherence?.sourceTabId ?? null,
    expectedRouteId: coherence?.expectedRouteId ?? null,
  });
}

function restoreBridgeBrowserAuthorityCoherence(value) {
  return normalizeLegacyWorkerBrowserAuthorityCoherence(value, "worker browser-history coherence");
}

export {
  createBridgeBrowserHistoryIngressReport,
  createBridgeBrowserHistoryStory,
  createBridgeBrowserHistoryWritebackReport,
  normalizeBridgeBrowserHistoryIngress,
  normalizeBridgeBrowserHistoryWriteback,
};
