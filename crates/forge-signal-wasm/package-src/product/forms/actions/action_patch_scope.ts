import { deletePath } from "../values/value_paths.js";
import { stableValueDigest } from "../values/value_semantics.js";

export function resolveActionPatchArtifact(declaration, fieldDeclarations, patchPlan) {
  if (declaration.patchPolicy === "ignore") {
    return Object.freeze({
      patch: Object.freeze({
        policy: "ignore",
        empty: true,
        operations: Object.freeze([]),
        equivalenceDigest: stableValueDigest({ ignoredPatchDigest: patchPlan.equivalenceDigest }),
      }),
      resourceDeniedReason: null,
    });
  }
  const scopedFields = declaration.resourceAction?.kind === "patchPlan"
    ? declaration.resourceAction.fields ?? null
    : null;
  if (scopedFields === null) {
    return Object.freeze({
      patch: fullPatchArtifact(declaration.patchPolicy, patchPlan),
      resourceDeniedReason: null,
    });
  }
  const declaredFieldIds = new Set(fieldDeclarations.map((field) => field.id));
  const missingField = scopedFields.find((fieldId) => !declaredFieldIds.has(fieldId));
  if (missingField !== undefined) {
    const reason = `resource-line action "${declaration.id}" declares scoped patch field "${missingField}" but that field is not declared on the form`;
    return deniedScopedPatchArtifact(declaration, patchPlan, reason);
  }
  const scopedFieldSet = new Set(scopedFields);
  if (patchPlan.replacement !== null) {
    const outOfScopeFields = patchPlan.replacement.fields.filter((fieldId) => !scopedFieldSet.has(fieldId));
    if (outOfScopeFields.length > 0) {
      const reason = `resource-line action "${declaration.id}" cannot lower a whole-resource replace because it would also consume out-of-scope fields: ${outOfScopeFields.join(", ")}`;
      return deniedScopedPatchArtifact(declaration, patchPlan, reason);
    }
    return Object.freeze({
      patch: Object.freeze({
        policy: declaration.patchPolicy,
        empty: false,
        semanticDirty: true,
        operations: Object.freeze([]),
        blocked: filterScopedPatchBlockers(patchPlan.blocked, scopedFieldSet),
        broadReplacement: true,
        replacement: patchPlan.replacement,
        equivalenceDigest: stableValueDigest(patchPlan.replacement),
      }),
      resourceDeniedReason: null,
    });
  }
  const operations = Object.freeze(
    patchPlan.operations.filter((operation) => scopedFieldSet.has(operation.field)),
  );
  const semanticDirty = operations.length > 0;
  return Object.freeze({
    patch: Object.freeze({
      policy: declaration.patchPolicy,
      empty: !semanticDirty,
      semanticDirty,
      operations,
      blocked: filterScopedPatchBlockers(patchPlan.blocked, scopedFieldSet),
      broadReplacement: false,
      replacement: null,
      equivalenceDigest: stableValueDigest(
        operations.map((operation) => (
          operation.kind === "removeItem"
            ? [operation.kind, operation.field, operation.itemId]
            : [operation.kind, operation.field, operation.itemId ?? null, operation.valueDigest ?? null]
        )),
      ),
    }),
    resourceDeniedReason: null,
  });
}

export function consumeDraftFields(previousDraft, patch, fieldDeclarations) {
  const consumedFieldIds = patch.replacement !== null
    ? patch.replacement.fields
    : [...new Set(patch.operations.map((operation) => operation.field))];
  if (consumedFieldIds.length === 0) {
    return Object.freeze({
      nextDraft: previousDraft,
      clearedFields: Object.freeze([]),
      draftReset: Object.keys(previousDraft).length === 0,
    });
  }
  const declarationsById = new Map(fieldDeclarations.map((field) => [field.id, field]));
  let nextDraft = previousDraft;
  const clearedFields = [];
  for (const fieldId of consumedFieldIds) {
    const declaration = declarationsById.get(fieldId);
    if (!declaration) {
      continue;
    }
    nextDraft = deletePath(nextDraft, declaration.segments);
    clearedFields.push(fieldId);
  }
  return Object.freeze({
    nextDraft,
    clearedFields: Object.freeze(clearedFields),
    draftReset: Object.keys(nextDraft).length === 0,
  });
}

function fullPatchArtifact(policy, patchPlan) {
  return Object.freeze({
    policy,
    empty: patchPlan.empty,
    semanticDirty: patchPlan.semanticDirty,
    operations: patchPlan.operations,
    blocked: patchPlan.blocked,
    broadReplacement: patchPlan.broadReplacement,
    replacement: patchPlan.replacement,
    equivalenceDigest: patchPlan.equivalenceDigest,
  });
}

function deniedScopedPatchArtifact(declaration, patchPlan, reason) {
  return Object.freeze({
    patch: Object.freeze({
      policy: declaration.patchPolicy,
      empty: false,
      semanticDirty: patchPlan.semanticDirty,
      operations: Object.freeze([]),
      blocked: Object.freeze([Object.freeze({
        kind: "resource:actionUnavailable",
        action: declaration.id,
        reason,
      })]),
      broadReplacement: patchPlan.broadReplacement,
      replacement: patchPlan.replacement,
      equivalenceDigest: stableValueDigest({
        kind: "scopedPatchDenied",
        action: declaration.id,
        reason,
        patchDigest: patchPlan.equivalenceDigest,
      }),
    }),
    resourceDeniedReason: reason,
  });
}

function filterScopedPatchBlockers(blockers, scopedFieldSet) {
  return Object.freeze(
    blockers.filter((blocker) => {
      if (blocker.field !== undefined) {
        return scopedFieldSet.has(blocker.field);
      }
      if (Array.isArray(blocker.fields)) {
        return blocker.fields.some((fieldId) => scopedFieldSet.has(fieldId));
      }
      return blocker.action !== undefined || blocker.section !== undefined || blocker.group !== undefined
        ? false
        : true;
    }),
  );
}
