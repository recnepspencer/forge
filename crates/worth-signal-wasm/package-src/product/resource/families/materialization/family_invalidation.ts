import { invalidateLine } from "../../lines/actions/line_invalidation_execution.js";

function invalidateFamilyLine(linesByCanonicalKey, canonicalKey) {
  const entry = linesByCanonicalKey.get(canonicalKey);
  if (!entry) {
    return false;
  }
  invalidateLine(
    entry.materialization,
    "manualFamilyInvalidate",
    "familyMember",
  );
  return true;
}

function invalidateAllFamilyLines(linesByCanonicalKey) {
  let invalidatedCount = 0;
  for (const entry of linesByCanonicalKey.values()) {
    invalidateLine(
      entry.materialization,
      "manualFamilyInvalidateAll",
      "familyAll",
    );
    invalidatedCount += 1;
  }
  return invalidatedCount;
}

export { invalidateAllFamilyLines, invalidateFamilyLine };
