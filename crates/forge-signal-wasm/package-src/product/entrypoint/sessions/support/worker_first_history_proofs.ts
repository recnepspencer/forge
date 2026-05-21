import { freezeObject } from "../../../graph_support.js";

export function normalizeWorkerFirstBranchId(branchId, operation) {
  if (typeof branchId === "bigint") {
    if (branchId < 0n) {
      throw new RangeError(`${operation} expects a non-negative branch id`);
    }
    return branchId;
  }
  if (!Number.isSafeInteger(branchId) || branchId < 0) {
    throw new TypeError(`${operation} expects a non-negative safe integer branch id`);
  }
  return BigInt(branchId);
}

export function createWorkerFirstSnapshotArtifact(snapshotArtifact) {
  return freezeObject({
    ...snapshotArtifact.snapshot,
    snapshotRestoreToken: snapshotArtifact.snapshotRestoreToken,
    snapshotPortableWire: snapshotArtifact.snapshotPortableWire,
    snapshotRestoreMode: "SameRuntimeExact",
  });
}

export function createWorkerFirstSnapshotEnvelopeArtifact(snapshotArtifact) {
  return freezeObject({
    ...snapshotArtifact.snapshotEnvelope,
    snapshotEnvelopeRestoreToken: snapshotArtifact.snapshotEnvelopeRestoreToken,
    snapshotEnvelopePortableWire: snapshotArtifact.snapshotEnvelopePortableWire,
    snapshotEnvelopeRestoreMode: "SameRuntimeExact",
  });
}

export function createReplayParityProofReport(
  proofSchemaVersion,
  expected,
  replayed,
) {
  const mismatchClasses = expected.stateDigest === replayed.stateDigest
    ? []
    : ["BranchStateDigestMismatch"];
  return freezeObject({
    proofSchemaVersion,
    expectedBranchId: expected.branchId,
    expectedBranchName: expected.branchName,
    expectedSnapshotId: expected.snapshotId,
    expectedStateDigest: expected.stateDigest,
    replayedBranchId: replayed.branchId,
    replayedBranchName: replayed.branchName,
    replayedSnapshotId: replayed.snapshotId,
    replayedStateDigest: replayed.stateDigest,
    parity: mismatchClasses.length === 0,
    mismatchClasses: freezeObject(mismatchClasses),
  });
}

export function createReplayArtifactProofReport(
  proofSchemaVersion,
  expected,
  replayed,
) {
  const frozenExpected = freezeObject({ ...expected });
  const frozenReplayed = freezeObject({ ...replayed });
  const mismatchClasses = [];
  compareOptionalDigest(
    frozenExpected.registryBundleDigest,
    frozenReplayed.registryBundleDigest,
    "MissingRegistryBundleDigest",
    "RegistryBundleDigestMismatch",
    mismatchClasses,
  );
  compareOptionalDigest(
    frozenExpected.loweredStrategyBundleDigest,
    frozenReplayed.loweredStrategyBundleDigest,
    "MissingLoweredStrategyBundleDigest",
    "LoweredStrategyBundleDigestMismatch",
    mismatchClasses,
  );
  compareOptionalDigest(
    frozenExpected.mergePlanDigest,
    frozenReplayed.mergePlanDigest,
    "MissingMergePlanDigest",
    "MergePlanDigestMismatch",
    mismatchClasses,
  );
  compareOptionalDigest(
    frozenExpected.mergeResultDigest,
    frozenReplayed.mergeResultDigest,
    "MissingMergeResultDigest",
    "MergeResultDigestMismatch",
    mismatchClasses,
  );
  compareOptionalDigest(
    frozenExpected.lineageDigest,
    frozenReplayed.lineageDigest,
    "MissingLineageDigest",
    "LineageDigestMismatch",
    mismatchClasses,
  );
  if (!frozenExpected.proofSchemaVersion.startsWith(proofSchemaVersion)
    || !frozenReplayed.proofSchemaVersion.startsWith(proofSchemaVersion)) {
    mismatchClasses.push("LegacyMergeArtifactUnsupported");
  }
  if (frozenExpected.proofSchemaVersion !== frozenReplayed.proofSchemaVersion) {
    mismatchClasses.push("ProofSchemaVersionMismatch");
  }
  if (frozenExpected.branchStateDigest !== frozenReplayed.branchStateDigest) {
    mismatchClasses.push("BranchStateDigestMismatch");
  }
  return freezeObject({
    proofSchemaVersion,
    expected: frozenExpected,
    replayed: frozenReplayed,
    parity: mismatchClasses.length === 0,
    mismatchClasses: freezeObject(mismatchClasses),
  });
}

function compareOptionalDigest(expected, replayed, missingClass, mismatchClass, output) {
  if (expected == null && replayed == null) {
    return;
  }
  if (expected == null || replayed == null) {
    output.push(missingClass);
    return;
  }
  if (expected !== replayed) {
    output.push(mismatchClass);
  }
}
