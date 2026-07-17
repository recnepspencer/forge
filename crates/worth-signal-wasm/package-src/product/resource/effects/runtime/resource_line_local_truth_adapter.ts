import { createLocalTruthAuthority } from "../../../local_truth/authority/local_truth_authority.js";
import { declareLocalTruthSchema } from "../../../local_truth/schema/schema_declaration.js";
import { canonicalDigest, deepFreeze, immutableClone } from "../../../local_truth/support/canonical.js";

export function createResourceLineLocalTruthAdapter(lineId, initialValue, initialRevision) {
  const schema = declareLocalTruthSchema({
    id: `resource-line:${lineId}`,
    version: 1,
    aspects: [{
      id: "confirmedValue",
      field: "value",
      valueType: "any",
      equivalence: { kind: "exact" },
      costClass: "linearInValue",
    }],
  });
  const authority = createLocalTruthAuthority({
    authorityId: `resource-line:${lineId}`,
    schema,
    initialEntities: { line: { value: initialValue } },
  });
  let confirmedProjection = deepFreeze({
    value: immutableClone(initialValue),
    revision: initialRevision,
    commitId: null,
  });
  let nextRequestSequence = 0;

  return deepFreeze({
    readConfirmedValue() {
      return confirmedProjection.value;
    },
    readConfirmedRevision() {
      return confirmedProjection.revision;
    },
    async replaceConfirmedValue(value, revision, reason) {
      const branch = await authority.branch();
      if (branch.posture !== "success") {
        throw new Error("resource line local truth main branch is unavailable");
      }
      nextRequestSequence += 1;
      const outcome = await authority.commit({
        requestId: `resource-line:${lineId}:${nextRequestSequence}:${canonicalDigest({ revision, reason })}`,
        branchId: branch.value.id,
        expectedBasis: branch.value.basis,
        operations: [{ entityId: "line", aspectId: "confirmedValue", value }],
        metadata: { source: "resourceLine", revision, reason },
      });
      if (outcome.posture !== "success") {
        throw new Error(`resource line local truth commit failed: ${outcome.code ?? outcome.posture}`);
      }
      confirmedProjection = deepFreeze({
        value: immutableClone(value),
        revision,
        commitId: outcome.value.commit.id,
      });
      return deepFreeze({
        artifactFamily: "ResourceLineLocalTruthCommitReceipt",
        localTruthCommit: outcome.value.commit,
        confirmedProjection,
      });
    },
    inspect() {
      return authority.inspect();
    },
  });
}
