import {
  HOST_CAPABILITY_PLAN_BRAND,
  requirePlainObject,
} from "../../host_capability_declarations.js";
import { wrapSignals } from "../../signals.js";
import { workerFirstHostCapabilitiesUnsupportedReason } from "../worker_first_host_capabilities.js";
import { createWorkerFirstCallableSignals } from "../worker_first_callable_signals.js";
import { normalizeCreateSignalsAssets } from "./create_signals_assets.js";

const DEPLOYMENT_VALUES = new Set([
  "workerFirst",
  "mainThreadCompatibility",
]);

export function createCallableSignals(options) {
  return createSignals({
    ...options,
    deployment: "mainThreadCompatibility",
  });
}

export async function createSignals(options) {
  const plan = planCreateSignalsDeployment(options);
  if (plan.family === "mainThreadCompatibility") {
    return createMainThreadCompatibilitySignals(plan.request);
  }
  if (plan.family === "workerFirst") {
    return createWorkerFirstCallableSignals(plan.request);
  }
  throw createSignalsConstructionError(plan);
}

export function explainCreateSignalsConstruction(options) {
  return planCreateSignalsDeployment(options).explanation;
}

export function planCreateSignalsDeployment(options) {
  const request = normalizeCreateSignalsOptions(options);
  if (request.deployment === "mainThreadCompatibility") {
    return Object.freeze({
      family: "mainThreadCompatibility",
      request,
      explanation: freezeExplanation({
        requestedDeployment: request.deployment,
        selectedFamily: "mainThreadCompatibility",
        selectedDeployment: "mainThreadCompatibility",
        reason: "explicitCompatibilityDeployment",
        compatibilityRecovery: null,
      }),
    });
  }
  if (typeof globalThis.Worker !== "function") {
    return workerUnavailablePlan(
      request,
      "workerConstructorUnavailable",
      "Dedicated worker construction is unavailable in this environment.",
    );
  }
  const unsupportedWorkerFirstHostCapabilities =
    workerFirstHostCapabilitiesUnsupportedReason(request.hostCapabilities);
  if (unsupportedWorkerFirstHostCapabilities !== null) {
    return deniedPlan(
      request,
      unsupportedWorkerFirstHostCapabilities.reason,
      unsupportedWorkerFirstHostCapabilities.message,
    );
  }
  return workerFirstPlan(request);
}

function normalizeCreateSignalsOptions(options) {
  if (options === undefined) {
    return Object.freeze({
      deployment: "workerFirst",
      hostCapabilities: null,
      assets: null,
    });
  }
  const normalizedOptions = requirePlainObject(
    options,
    "createSignals options must be an object when provided",
  );
  const {
    deployment = "workerFirst",
    hostCapabilities,
    assets,
    ...unknownOptions
  } = normalizedOptions;
  const unknownKeys = Object.keys(unknownOptions);
  if (unknownKeys.length > 0) {
    throw new TypeError(
      `createSignals options do not support: ${unknownKeys.join(", ")}`,
    );
  }
  if (!DEPLOYMENT_VALUES.has(deployment)) {
    throw new TypeError(
      `createSignals deployment must be one of ${[...DEPLOYMENT_VALUES].join(", ")}`,
    );
  }
  if (
    hostCapabilities !== undefined &&
    (!hostCapabilities || hostCapabilities[HOST_CAPABILITY_PLAN_BRAND] !== true)
  ) {
    throw new TypeError(
      "createSignals hostCapabilities must be created with hostCapabilityPlan(...)",
    );
  }
  return Object.freeze({
    deployment,
    hostCapabilities: hostCapabilities ?? null,
    assets: normalizeCreateSignalsAssets(assets, deployment),
  });
}

async function createMainThreadCompatibilitySignals(request) {
  const rawSurface = await import("../../../raw_surface.js");
  const wasmInput = request.assets?.wasmUrl;
  await rawSurface.default(wasmInput === null || wasmInput === undefined
    ? undefined
    : wasmInput);
  return request.hostCapabilities === null
    ? wrapSignals(rawSurface.createRawSignals())
    : wrapSignals(rawSurface.createRawSignals(), {
      hostCapabilities: request.hostCapabilities,
    });
}

function workerFirstPlan(request) {
  return Object.freeze({
    family: "workerFirst",
    request,
    explanation: freezeExplanation({
      requestedDeployment: request.deployment,
      selectedFamily: "workerFirst",
      selectedDeployment: "workerFirst",
      reason: "workerFirstImportedGraphCallableSurface",
      compatibilityRecovery: null,
    }),
  });
}

function deniedPlan(request, reason, message) {
  const compatibilityRecovery = Object.freeze({
    deployment: "mainThreadCompatibility",
    message: "Retry with deployment: \"mainThreadCompatibility\" to construct the explicit main-thread runtime lane.",
  });
  return Object.freeze({
    family: "denied",
    request,
    artifact: freezeArtifact({
      artifactFamily: "signalsConstructionDenied",
      requestedDeployment: request.deployment,
      reason,
      message,
      compatibilityRecovery,
    }),
    explanation: freezeExplanation({
      requestedDeployment: request.deployment,
      selectedFamily: "denied",
      selectedDeployment: null,
      reason,
      compatibilityRecovery,
    }),
  });
}

function workerUnavailablePlan(request, reason, message) {
  const compatibilityRecovery = Object.freeze({
    deployment: "mainThreadCompatibility",
    message: "Retry with deployment: \"mainThreadCompatibility\" to construct the explicit main-thread runtime lane.",
  });
  return Object.freeze({
    family: "workerUnavailable",
    request,
    artifact: freezeArtifact({
      artifactFamily: "workerUnavailableConstruction",
      requestedDeployment: request.deployment,
      reason,
      message,
      compatibilityRecovery,
    }),
    explanation: freezeExplanation({
      requestedDeployment: request.deployment,
      selectedFamily: "workerUnavailable",
      selectedDeployment: null,
      reason,
      compatibilityRecovery,
    }),
  });
}

function freezeArtifact(artifact) {
  return Object.freeze(artifact);
}

function freezeExplanation(explanation) {
  return Object.freeze(explanation);
}

function createSignalsConstructionError(plan) {
  const error = new Error(plan.artifact.message);
  error.name = plan.artifact.artifactFamily;
  error.artifactFamily = plan.artifact.artifactFamily;
  error.requestedDeployment = plan.artifact.requestedDeployment;
  error.reason = plan.artifact.reason;
  error.compatibilityRecovery = plan.artifact.compatibilityRecovery;
  error.explanation = plan.explanation;
  return error;
}
