export function createWorkerFirstHostDependencyId(descriptor) {
  assertHostDependencyDescriptor(descriptor);
  return [
    "__forgeSignal.hostCapability",
    encodeURIComponent(descriptor.family),
    encodeURIComponent(descriptor.registrationId),
    encodeURIComponent(descriptor.compatibility),
  ].join(":");
}

export function createWorkerFirstHostDependencyRecords(hostCapabilityReads) {
  const recordsById = new Map();
  for (const descriptor of hostCapabilityReads) {
    const dependencyId = createWorkerFirstHostDependencyId(descriptor);
    recordsById.set(dependencyId, Object.freeze({
      dependencyId,
      family: descriptor.family,
      registrationId: descriptor.registrationId,
      compatibility: descriptor.compatibility,
    }));
  }
  return Object.freeze([...recordsById.values()]);
}

export function createWorkerFirstHostDependencyIds(hostCapabilityReads) {
  return Object.freeze(
    createWorkerFirstHostDependencyRecords(hostCapabilityReads)
      .map((record) => record.dependencyId),
  );
}

export function hostDependenciesIntersect(dependencyIds, changedDependencyIds) {
  if (!Array.isArray(dependencyIds) || dependencyIds.length === 0) {
    return false;
  }
  for (const dependencyId of dependencyIds) {
    if (changedDependencyIds.has(dependencyId)) {
      return true;
    }
  }
  return false;
}

function assertHostDependencyDescriptor(descriptor) {
  if (!descriptor || typeof descriptor !== "object") {
    throw new TypeError("worker-first callback host dependency requires a descriptor object");
  }
  for (const key of ["family", "registrationId", "compatibility"]) {
    if (typeof descriptor[key] !== "string" || descriptor[key].length === 0) {
      throw new TypeError(
        `worker-first callback host dependency requires a non-empty ${key}`,
      );
    }
  }
}
