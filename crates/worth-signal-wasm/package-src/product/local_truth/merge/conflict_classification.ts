import { aspectsEquivalent } from "../schema/schema_declaration.js";
import { canonicalDigest, deepFreeze } from "../support/canonical.js";

export function classifyLocalTruthDeltas(schema, basis, rawPolicy) {
  const policy = normalizeMergePolicy(rawPolicy);
  const classifications = basis.deltas.map((delta) => classifyDelta(schema, basis, policy, delta));
  const conflicts = classifications
    .filter((classification) => classification.kind === "ResolutionRequired")
    .map((classification) => classification.conflict);
  return deepFreeze({
    artifactFamily: "LocalTruthMergeClassification",
    policy,
    classifications,
    conflicts,
    counters: {
      lociClassified: classifications.length,
      conflictsIssued: conflicts.length,
      automaticDecisions: classifications.length - conflicts.length,
    },
    digest: canonicalDigest({ basis: basis.identityDigest, policy, classifications, conflicts }),
  });
}

function classifyDelta(schema, basis, policy, delta) {
  const sourceChanged = !aspectsEquivalent(schema, delta.aspectId, delta.baseValue, delta.sourceValue);
  const targetChanged = !aspectsEquivalent(schema, delta.aspectId, delta.baseValue, delta.targetValue);
  let kind;
  let selectionBasis;
  if (!sourceChanged && !targetChanged) {
    kind = "Unchanged";
    selectionBasis = "equivalence";
  } else if (sourceChanged && !targetChanged) {
    kind = "AdoptSource";
    selectionBasis = "sourceOnlyChange";
  } else if (!sourceChanged && targetChanged) {
    kind = "PreserveTarget";
    selectionBasis = "targetOnlyChange";
  } else if (aspectsEquivalent(schema, delta.aspectId, delta.sourceValue, delta.targetValue)) {
    kind = "Equivalent";
    selectionBasis = "declaredAspectEquivalence";
  } else if (policy.overlap === "preferSource") {
    kind = "AdoptSource";
    selectionBasis = "declaredMergePolicy";
  } else if (policy.overlap === "preferTarget") {
    kind = "PreserveTarget";
    selectionBasis = "declaredMergePolicy";
  } else {
    kind = "ResolutionRequired";
    selectionBasis = "manualReview";
  }
  const common = {
    artifactFamily: "LocalTruthMergeLocusClassification",
    entityId: delta.entityId,
    aspectId: delta.aspectId,
    effectiveBaseCommitId: delta.effectiveBaseCommitId,
    sourceLocusCommitId: delta.sourceLocusCommitId,
    targetLocusCommitId: delta.targetLocusCommitId,
    kind,
    selectionBasis,
    baseValue: delta.baseValue,
    sourceValue: delta.sourceValue,
    targetValue: delta.targetValue,
  };
  if (kind !== "ResolutionRequired") {
    return deepFreeze(common);
  }
  return deepFreeze({ ...common, conflict: buildConflict(basis, policy, delta) });
}

function buildConflict(basis, policy, delta) {
  const evidence = {
    authorityId: basis.authorityId,
    schemaIdentity: basis.schemaIdentity,
    sourceHead: basis.sourceBasis.headCommitId,
    targetHead: basis.targetBasis.headCommitId,
    effectiveBaseCommitId: delta.effectiveBaseCommitId,
    entityId: delta.entityId,
    aspectId: delta.aspectId,
    baseDigest: canonicalDigest(delta.baseValue),
    sourceDigest: canonicalDigest(delta.sourceValue),
    targetDigest: canonicalDigest(delta.targetValue),
    policyIdentity: policy.identity,
  };
  const conflictId = `truth-conflict:${canonicalDigest(evidence)}`;
  return deepFreeze({
    artifactFamily: "LocalTruthConflictRecord",
    id: conflictId,
    ...evidence,
    alternatives: [
      alternative(conflictId, "source", delta.sourceValue),
      alternative(conflictId, "target", delta.targetValue),
    ],
  });
}

function alternative(conflictId, choice, value) {
  return deepFreeze({
    artifactFamily: "LocalTruthConflictAlternative",
    id: `truth-alternative:${canonicalDigest({ conflictId, choice, value })}`,
    choice,
    value,
  });
}

function normalizeMergePolicy(rawPolicy) {
  const overlap = rawPolicy?.overlap ?? "review";
  if (!new Set(["review", "preferSource", "preferTarget"]).has(overlap)) {
    throw new TypeError(`unsupported local truth overlap policy ${String(overlap)}`);
  }
  const normalized = { artifactFamily: "LocalTruthMergePolicyDeclaration", overlap };
  return deepFreeze({ ...normalized, identity: `truth-policy:${canonicalDigest(normalized)}` });
}
