import { FormDeclarationError } from "../form_errors.js";
import { readResourceLineHandle } from "../sources/form_sources.js";
import { stableValueDigest } from "../values/value_paths.js";

export function createResourceMergeProjectionRegistry(fieldDeclarations, stepDeclarations, availabilityDeclarations) {
  const fieldsByPath = new Map();
  for (const declaration of fieldDeclarations) {
    const ids = fieldsByPath.get(declaration.path) ?? [];
    ids.push(declaration.id);
    fieldsByPath.set(declaration.path, ids);
  }
  const sectionsByField = new Map();
  for (const step of stepDeclarations) {
    for (const fieldId of step.fields) {
      appendUniqueMappedValue(sectionsByField, fieldId, step.id);
    }
  }
  for (const declaration of availabilityDeclarations) {
    if (declaration.scope !== "section") {
      continue;
    }
    for (const fieldId of declaration.fields) {
      appendUniqueMappedValue(sectionsByField, fieldId, declaration.ownerId);
    }
  }
  return Object.freeze({
    fieldsByPath,
    sectionsByField,
  });
}

export function previewResourceMerge(signalNamespace, source, store, registry, request) {
  const line = readResourceLineHandle(source);
  if (line === null) {
    return store.report(unavailablePreview("form source is not resource-backed", request));
  }
  if (!isMergePreviewRequest(request)) {
    throw new FormDeclarationError("resource merge preview requires source_branch_id and target_branch_id", {
      request,
    });
  }
  const latestEffect = line.diagnosticsSummary().latest.effect;
  if (latestEffect === null) {
    return store.report(unavailablePreview(
      "resource merge preview requires a current resource-backed effect",
      request,
    ));
  }
  const branchNamespace = signalNamespace?.resource?.branch;
  if (!branchNamespace || typeof branchNamespace.planEffectMerge !== "function") {
    return store.report(unavailablePreview(
      "resource merge preview is unavailable because the resource branch namespace is not present",
      request,
    ));
  }
  const result = branchNamespace.planEffectMerge({
    merge: request,
    effect: latestEffect,
  });
  return store.report(normalizeMergePreview(result, latestEffect, registry, request));
}

export function currentResourceMergeEffectDigest(source) {
  const line = readResourceLineHandle(source);
  if (line === null) {
    return null;
  }
  const latestEffect = line.diagnosticsSummary().latest.effect;
  return latestEffect === null ? null : stableValueDigest(latestEffect);
}

function normalizeMergePreview(result, effect, registry, request) {
  const effectDigest = stableValueDigest(effect);
  if (result.kind === "denied") {
    return unavailablePreview(result.detail, request, {
      effectDigest,
      proofDigest: null,
    });
  }
  const artifact = result.resourceEffect?.rebaseArtifact;
  if (!artifact) {
    return unavailablePreview("resource merge preview did not expose a resource effect artifact", request, {
      effectDigest,
      proofDigest: null,
    });
  }
  if (artifact.kind === "rebaseAvailable") {
    return Object.freeze({
      kind: "resourceMergePreview",
      sourceKind: "resourceLine",
      status: "ready",
      stale: false,
      request: Object.freeze({ ...request }),
      effectDigest,
      sourceBranchId: request.source_branch_id,
      targetBranchId: request.target_branch_id,
      reason: "resource merge preview found no projected conflicts",
      conflictCount: 0,
      projectedFields: Object.freeze([]),
      projectedSections: Object.freeze([]),
      blockers: Object.freeze([]),
      messages: Object.freeze([]),
      proofDigest: artifact.proof.nativeMergePlanDigest,
      resultDigest: stableValueDigest({
        request,
        effectDigest,
        rebaseArtifact: artifact,
      }),
    });
  }
  if (artifact.kind === "mappingUnavailable") {
    const blocker = Object.freeze({
      kind: "resource:mergeMappingUnavailable",
      action: "submit",
      reason: artifact.detail,
    });
    const message = Object.freeze({
      code: "resource.merge.mapping_unavailable",
      message: artifact.detail,
      severity: "error",
      audience: "user",
      visibility: "blocked",
    });
    return Object.freeze({
      kind: "resourceMergePreview",
      sourceKind: "resourceLine",
      status: "unavailable",
      stale: false,
      request: Object.freeze({ ...request }),
      effectDigest,
      sourceBranchId: request.source_branch_id,
      targetBranchId: request.target_branch_id,
      reason: artifact.detail,
      conflictCount: artifact.conflictCount,
      projectedFields: Object.freeze([]),
      projectedSections: Object.freeze([]),
      blockers: Object.freeze([blocker]),
      messages: Object.freeze([message]),
      proofDigest: artifact.proof.nativeMergePlanDigest,
      resultDigest: stableValueDigest({
        request,
        effectDigest,
        rebaseArtifact: artifact,
        blocker,
        message,
      }),
    });
  }
  const projected = projectConflictArtifacts(artifact.conflicts, registry);
  return Object.freeze({
    kind: "resourceMergePreview",
    sourceKind: "resourceLine",
    status: "conflict",
    stale: false,
    request: Object.freeze({ ...request }),
    effectDigest,
    sourceBranchId: request.source_branch_id,
    targetBranchId: request.target_branch_id,
    reason: "resource merge preview found projected merge conflicts",
    conflictCount: artifact.conflictCount,
    projectedFields: projected.fields,
    projectedSections: projected.sections,
    blockers: projected.blockers,
    messages: projected.messages,
    proofDigest: artifact.proof.nativeMergePlanDigest,
    resultDigest: stableValueDigest({
      request,
      effectDigest,
      rebaseArtifact: artifact,
      projected,
    }),
  });
}

function projectConflictArtifacts(conflicts, registry) {
  const fieldSet = new Set();
  const sectionSet = new Set();
  const blockers = [];
  const messages = [];
  for (const conflict of conflicts) {
    const projectedFields = projectedFieldIds(conflict.resource.locus, registry);
    for (const fieldId of projectedFields) {
      fieldSet.add(fieldId);
      for (const sectionId of registry.sectionsByField.get(fieldId) ?? []) {
        sectionSet.add(sectionId);
      }
      blockers.push(Object.freeze({
        kind: "resource:mergeConflict",
        action: "submit",
        field: fieldId,
        section: (registry.sectionsByField.get(fieldId) ?? [])[0],
        reason: `resource merge preview reports a conflict for field "${fieldId}"`,
      }));
      messages.push(Object.freeze({
        code: "resource.merge.conflict",
        message: `Remote resource changes conflict with local changes for "${fieldId}"`,
        severity: "error",
        target: fieldId,
        audience: "user",
        visibility: "visible",
        accessibility: Object.freeze({
          announce: "assertive",
          focusTarget: fieldId,
        }),
      }));
    }
  }
  if (blockers.length === 0) {
    blockers.push(Object.freeze({
      kind: "resource:mergeConflict",
      action: "submit",
      reason: "resource merge preview reports conflicts that could not be projected onto declared form fields",
    }));
    messages.push(Object.freeze({
      code: "resource.merge.conflict",
      message: "Remote resource changes conflict with local changes",
      severity: "error",
      audience: "user",
      visibility: "blocked",
    }));
  }
  return Object.freeze({
    fields: Object.freeze([...fieldSet]),
    sections: Object.freeze([...sectionSet]),
    blockers: Object.freeze(blockers),
    messages: Object.freeze(messages),
  });
}

function projectedFieldIds(locus, registry) {
  const path = pathForResourceLocus(locus);
  if (path === null) {
    return [];
  }
  return Object.freeze([...(registry.fieldsByPath.get(path) ?? [])]);
}

function pathForResourceLocus(locus) {
  switch (locus.kind) {
    case "detailField":
      return locus.field;
    case "detailRegion":
      return locus.region;
    case "detailJsonPath":
      return locus.path;
    case "itemAspect":
    case "jsonItemAspect":
      return locus.aspect;
    case "summary":
      return locus.summary;
    default:
      return null;
  }
}

function unavailablePreview(reason, request, options = {}) {
  return Object.freeze({
    kind: "resourceMergePreview",
    sourceKind: "form",
    status: "unavailable",
    stale: false,
    request: request && isMergePreviewRequest(request) ? Object.freeze({ ...request }) : null,
    effectDigest: options.effectDigest ?? null,
    sourceBranchId: request?.source_branch_id ?? null,
    targetBranchId: request?.target_branch_id ?? null,
    reason,
    conflictCount: 0,
    projectedFields: Object.freeze([]),
    projectedSections: Object.freeze([]),
    blockers: Object.freeze([]),
    messages: Object.freeze([]),
    proofDigest: options.proofDigest ?? null,
    resultDigest: stableValueDigest({
      reason,
      request: request ?? null,
      effectDigest: options.effectDigest ?? null,
      proofDigest: options.proofDigest ?? null,
    }),
  });
}

function isMergePreviewRequest(request) {
  return !!request
    && Number.isSafeInteger(request.source_branch_id)
    && request.source_branch_id >= 0
    && Number.isSafeInteger(request.target_branch_id)
    && request.target_branch_id >= 0;
}

function appendUniqueMappedValue(map, key, value) {
  const existing = map.get(key);
  if (!existing) {
    map.set(key, [value]);
    return;
  }
  if (!existing.includes(value)) {
    existing.push(value);
  }
}
