import { freezeObject } from "../graph_support.js";

export function createWorkerFirstScopeDescriptor(path) {
  const scopeId = path.join(".");
  const localScopeId = path.at(-1) ?? null;
  const parentScopeId = path.length > 1 ? path.slice(0, -1).join(".") : null;
  const segments = path.map((segment, index) => freezeObject({
    id: path.slice(0, index + 1).join("."),
    localScopeId: segment,
    depth: index + 1,
  }));
  return freezeObject({
    id: scopeId,
    localScopeId,
    parentScopeId,
    depth: segments.length,
    path: freezeObject(segments),
    graphOwnerId: null,
    identity: freezeObject({
      scopeId,
      parentScopeId,
      path: freezeObject(segments),
      depth: segments.length,
    }),
  });
}

export function createWorkerFirstScopedSignalIdentity(path, localId) {
  const descriptor = createWorkerFirstScopeDescriptor(path);
  const rootScopeId = descriptor.path[0]?.localScopeId ?? null;
  return freezeObject({
    localId,
    canonicalId: canonicalWorkerFirstScopedSignalId(path, localId),
    scopeId: descriptor.id,
    graphOwnerId: null,
    graphId: null,
    rootScopeId,
    scopePath: descriptor.path,
  });
}

export function canonicalWorkerFirstScopedSignalId(path, localId) {
  requireWorkerFirstScopeLocalId(localId);
  return path.length === 0 ? localId : `${path.join(".")}.${localId}`;
}

export function requireWorkerFirstScopeLocalId(localId) {
  if (typeof localId !== "string" || localId.length === 0) {
    throw new TypeError("worker-first scoped authoring requires a non-empty local id");
  }
}
