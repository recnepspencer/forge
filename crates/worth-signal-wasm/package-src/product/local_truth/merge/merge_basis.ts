import {
  locusKey,
  readBranchSnapshot,
  requireCurrentBasis,
} from "../authority/authority_state.js";
import { extractAspect } from "../schema/schema_declaration.js";
import { canonicalDigest, deepFreeze } from "../support/canonical.js";
import { denied, success, unavailable } from "../support/outcomes.js";

export function resolveLocalTruthMergeBasis(state, schema, request) {
  try {
    const { branch: source, snapshot: sourceSnapshot } = readBranchSnapshot(state, request?.sourceBranchId);
    const { branch: target, snapshot: targetSnapshot } = readBranchSnapshot(state, request?.targetBranchId);
    if (source.id === target.id) {
      return denied("identicalMergeBranches", "source and target local truth branches must differ");
    }
    const sourceBasis = requireCurrentBasis(state, source, request?.expectedSourceBasis);
    const targetBasis = requireCurrentBasis(state, target, request?.expectedTargetBasis);
    if (!sourceBasis.ok || !targetBasis.ok) {
      return denied(
        sourceBasis.ok ? targetBasis.code : sourceBasis.code,
        "local truth merge requires current source and target bases",
      );
    }
    const ancestry = resolveStructuralAncestor(state, source, target);
    if (ancestry.posture !== "success") {
      return ancestry;
    }
    const scope = canonicalizeScope(schema, sourceSnapshot, targetSnapshot, request?.scope);
    const deltas = scope.loci.map((locus) => buildLocusDelta(
      state,
      schema,
      source,
      target,
      sourceSnapshot,
      targetSnapshot,
      ancestry.value.commitId,
      ancestry.value.snapshotId,
      locus,
    ));
    const counters = deepFreeze({
      ancestryNodesVisited: ancestry.value.nodesVisited,
      commitSegmentsVisited: deltas.reduce((total, delta) => total + delta.commitSegmentsVisited, 0),
      entitiesVisited: scope.entityIds.length,
      aspectsVisited: deltas.length,
    });
    const value = deepFreeze({
      artifactFamily: "ResolvedLocalTruthMergeBasis",
      authorityId: state.authorityId,
      schemaIdentity: state.schemaIdentity,
      sourceBasis: source.basis,
      targetBasis: target.basis,
      structuralAncestorCommitId: ancestry.value.commitId,
      scope,
      deltas,
      counters,
      identityDigest: canonicalDigest({
        authorityId: state.authorityId,
        schemaIdentity: state.schemaIdentity,
        sourceBasis: source.basis.identityDigest,
        targetBasis: target.basis.identityDigest,
        structuralAncestorCommitId: ancestry.value.commitId,
        scope,
        deltas,
        counters,
      }),
    });
    return success(value);
  } catch (error) {
    return unavailable(
      "localTruthMergeBasisUnavailable",
      error instanceof Error ? error.message : String(error),
    );
  }
}

function buildLocusDelta(
  state,
  schema,
  source,
  target,
  sourceSnapshot,
  targetSnapshot,
  structuralBaseCommitId,
  structuralBaseSnapshotId,
  locus,
) {
  const key = locusKey(locus.entityId, locus.aspectId);
  const integrationKey = sourceIntegrationKey(source.id, key);
  const integratedSource = state.lineageByBranch.get(target.id).get(integrationKey) ?? null;
  const integratedSourceCommitId = integratedSource?.sourceCommitId ?? null;
  const effectiveBaseCommitId = integratedSourceCommitId ?? structuralBaseCommitId;
  const effectiveBaseSnapshot = state.snapshots.get(structuralBaseSnapshotId);
  if (!integratedSource && !effectiveBaseSnapshot) {
    throw new Error(`structural base snapshot ${structuralBaseSnapshotId} is unavailable`);
  }
  const sourceLocusCommitId = state.locusHeadByBranch.get(source.id).get(key);
  const targetLocusCommitId = state.locusHeadByBranch.get(target.id).get(key);
  return deepFreeze({
    artifactFamily: "LocalTruthAspectDelta",
    entityId: locus.entityId,
    aspectId: locus.aspectId,
    effectiveBaseCommitId,
    integratedSourceCommitId,
    sourceLocusCommitId,
    targetLocusCommitId,
    baseValue: integratedSource
      ? integratedSource.sourceValue
      : extractAspect(schema, effectiveBaseSnapshot.values[locus.entityId], locus.aspectId),
    sourceValue: extractAspect(schema, sourceSnapshot.values[locus.entityId], locus.aspectId),
    targetValue: extractAspect(schema, targetSnapshot.values[locus.entityId], locus.aspectId),
    commitSegmentsVisited: integratedSourceCommitId ? 1 : 2,
  });
}

function resolveStructuralAncestor(state, source, target) {
  const sourcePath = branchPath(state, source);
  const targetPath = branchPath(state, target);
  if (!sourcePath || !targetPath) {
    return unavailable("ambiguousLocalTruthAncestry", "branch ancestry is cyclic or incomplete");
  }
  const targetIndexes = new Map(targetPath.map((branch, index) => [branch.id, index]));
  const sourceIndex = sourcePath.findIndex((branch) => targetIndexes.has(branch.id));
  if (sourceIndex < 0) {
    return denied("unrelatedLocalTruthBranches", "branches do not share a structural ancestor");
  }
  const common = sourcePath[sourceIndex];
  const targetIndex = targetIndexes.get(common.id);
  const sourceChild = sourceIndex === 0 ? null : sourcePath[sourceIndex - 1];
  const targetChild = targetIndex === 0 ? null : targetPath[targetIndex - 1];
  const anchor = sourceChild && targetChild
    ? (sourceChild.forkRevision <= targetChild.forkRevision ? sourceChild : targetChild)
    : sourceChild ?? targetChild ?? common;
  return success({
    commitId: anchor === common ? common.headCommitId : anchor.forkCommitId,
    snapshotId: anchor === common ? common.snapshotId : anchor.forkSnapshotId,
    nodesVisited: sourcePath.length + targetPath.length,
  });
}

function branchPath(state, branch) {
  const path = [];
  const seen = new Set();
  let cursor = branch;
  while (cursor) {
    if (seen.has(cursor.id)) return null;
    seen.add(cursor.id);
    path.push(cursor);
    cursor = cursor.parentBranchId ? state.branches.get(cursor.parentBranchId) : null;
    if (path.at(-1).parentBranchId && !cursor) return null;
  }
  return path;
}

function canonicalizeScope(schema, sourceSnapshot, targetSnapshot, rawScope) {
  const allEntityIds = Object.keys(sourceSnapshot.values)
    .filter((entityId) => Object.hasOwn(targetSnapshot.values, entityId))
    .sort();
  const requestedEntities = normalizeStringList(rawScope?.entityIds ?? allEntityIds, "entityIds");
  const requestedAspects = normalizeStringList(
    rawScope?.aspectIds ?? schema.aspects.map((aspect) => aspect.id),
    "aspectIds",
  );
  const entityIds = [...new Set(requestedEntities)].sort();
  const aspectIds = [...new Set(requestedAspects)].sort();
  if (entityIds.length !== requestedEntities.length || aspectIds.length !== requestedAspects.length) {
    throw new TypeError("local truth merge scope contains duplicate entities or aspects");
  }
  for (const entityId of entityIds) {
    if (!allEntityIds.includes(entityId)) {
      throw new TypeError(`merge scope entity ${entityId} is not present on both branches`);
    }
  }
  for (const aspectId of aspectIds) {
    if (!schema.aspects.some((aspect) => aspect.id === aspectId)) {
      throw new TypeError(`merge scope aspect ${aspectId} is not declared by the schema`);
    }
  }
  const loci = entityIds.flatMap((entityId) => aspectIds.map((aspectId) => ({ entityId, aspectId })));
  return deepFreeze({ artifactFamily: "LocalTruthMergeScope", entityIds, aspectIds, loci });
}

function normalizeStringList(value, label) {
  if (!Array.isArray(value) || value.some((entry) => typeof entry !== "string" || entry === "")) {
    throw new TypeError(`local truth merge scope ${label} must be an array of non-empty strings`);
  }
  return value;
}

export function sourceIntegrationKey(sourceBranchId, locus) {
  return `${sourceBranchId.length}:${sourceBranchId}${locus}`;
}
