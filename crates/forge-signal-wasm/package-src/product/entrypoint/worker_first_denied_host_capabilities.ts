import {
  HOST_CLOCK_HANDLE_BRAND,
  HOST_ONLINE_HANDLE_BRAND,
  HOST_PERSISTENCE_HANDLE_BRAND,
  HOST_VIEWPORT_HANDLE_BRAND,
  HOST_VISIBILITY_HANDLE_BRAND,
} from "../host_capability_declarations.js";
import { recordHostCapabilityRead } from "../callback_frames.js";
import { freezeObject } from "../graph_support.js";
import { recordHostCapabilityReadDenied } from "./worker_first_host_capability_events.js";

const KNOWN_DENIED_HOST_CAPABILITY_FACTORIES = Object.freeze({
  viewport: createDeniedViewportCapability,
  visibility: createDeniedVisibilityCapability,
  online: createDeniedOnlineCapability,
  clock: createDeniedClockCapability,
  persistence: createDeniedPersistenceCapability,
});

export function createWorkerFirstHostSurface(
  registrations,
  performanceSummary,
  diagnosticsRecorder,
) {
  const host = Object.fromEntries(
    registrations.map((registration) => [
      registration.handle.descriptor().family,
      registration.handle,
    ]),
  );
  for (const [family, createDeniedCapability] of Object.entries(
    KNOWN_DENIED_HOST_CAPABILITY_FACTORIES,
  )) {
    host[family] ??= createDeniedCapability(performanceSummary, diagnosticsRecorder);
  }
  return freezeObject(host);
}

function createDeniedViewportCapability(performanceSummary, diagnosticsRecorder) {
  const denyRead = createDeniedHostCapabilityRead("viewport", performanceSummary, diagnosticsRecorder);
  return freezeObject({
    size: denyRead,
    width: denyRead,
    height: denyRead,
    descriptor: () => createDeniedDescriptor("viewport"),
    [HOST_VIEWPORT_HANDLE_BRAND]: true,
  });
}

function createDeniedVisibilityCapability(performanceSummary, diagnosticsRecorder) {
  const denyRead = createDeniedHostCapabilityRead("visibility", performanceSummary, diagnosticsRecorder);
  return freezeObject({
    state: denyRead,
    isVisible: denyRead,
    descriptor: () => createDeniedDescriptor("visibility"),
    [HOST_VISIBILITY_HANDLE_BRAND]: true,
  });
}

function createDeniedOnlineCapability(performanceSummary, diagnosticsRecorder) {
  const denyRead = createDeniedHostCapabilityRead("online", performanceSummary, diagnosticsRecorder);
  return freezeObject({
    state: denyRead,
    isOnline: denyRead,
    descriptor: () => createDeniedDescriptor("online"),
    [HOST_ONLINE_HANDLE_BRAND]: true,
  });
}

function createDeniedClockCapability(performanceSummary, diagnosticsRecorder) {
  const denyRead = createDeniedHostCapabilityRead("clock", performanceSummary, diagnosticsRecorder);
  return freezeObject({
    now: denyRead,
    descriptor: () => createDeniedDescriptor("clock"),
    [HOST_CLOCK_HANDLE_BRAND]: true,
  });
}

function createDeniedPersistenceCapability(performanceSummary, diagnosticsRecorder) {
  const denyRead = createDeniedHostCapabilityRead("persistence", performanceSummary, diagnosticsRecorder);
  return freezeObject({
    value: denyRead,
    commit: denyRead,
    descriptor: () => createDeniedDescriptor("persistence"),
    [HOST_PERSISTENCE_HANDLE_BRAND]: true,
  });
}

function createDeniedHostCapabilityRead(family, performanceSummary, diagnosticsRecorder) {
  const descriptor = createDeniedDescriptor(family);
  return () => {
    const error = new TypeError(
      `worker-first host capability \`${family}\` was not admitted for this runtime`,
    );
    error.code = "computeCallbackMissingHostCapabilityReadDenied";
    recordHostCapabilityReadDenied(
      performanceSummary,
      diagnosticsRecorder,
      descriptor,
      error,
      "missing-host-capability",
      true,
    );
    throw error;
  };
}

export function throwDetachedWorkerFirstHostCapabilityRead(
  performanceSummary,
  diagnosticsRecorder,
  descriptor,
) {
  const error = new TypeError(
    `worker-first host capability \`${descriptor.family}\` was detached from this runtime`,
  );
  error.code = "computeCallbackDetachedHostCapabilityReadDenied";
  recordHostCapabilityReadDenied(
    performanceSummary,
    diagnosticsRecorder,
    descriptor,
    error,
    "detached-host-capability",
    false,
  );
  throw error;
}

export function recordWorkerFirstAdmittedHostCapabilityRead(
  rootSession,
  descriptor,
  performanceSummary,
  diagnosticsRecorder,
  disposed,
) {
  if (disposed) {
    throwDetachedWorkerFirstHostCapabilityRead(
      performanceSummary,
      diagnosticsRecorder,
      descriptor,
    );
  }
  performanceSummary.hostCapabilityReadCount += 1;
  recordHostCapabilityRead(rootSession, descriptor);
}

function createDeniedDescriptor(family) {
  return freezeObject({
    family,
    registrationId: family,
    compatibility: "Unavailable",
  });
}
