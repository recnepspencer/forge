import {
  createSignals,
  localTruthSchema,
  type LocalTruthBasis,
  type LocalTruthCommit,
} from "./index.js";

interface Gear {
  teeth: number;
  material: string;
  rotation: number;
  label: string;
}

const schema = localTruthSchema<Gear>({
  id: "type-smoke.gear",
  aspects: [
    { id: "teeth", field: "teeth", valueType: "number", equivalence: { kind: "exact" }, costClass: "constant" },
    { id: "material", field: "material", valueType: "string", equivalence: { kind: "exact" }, costClass: "constant" },
    { id: "rotation", field: "rotation", valueType: "number", equivalence: { kind: "exact" }, costClass: "constant" },
    { id: "label", field: "label", valueType: "string", equivalence: { kind: "exact" }, costClass: "constant" },
  ],
});
const initial: Gear = { teeth: 18, material: "steel", rotation: 0, label: "Drive" };
const signals = await createSignals({ deployment: "mainThreadCompatibility" });
const gear = signals.input(initial, { producesAspects: [0, 1, 2, 3] });
const truth = signals.localTruth({
  authorityId: "type-smoke",
  schema,
  initialEntities: { gear: initial },
  bindings: [{
    entityId: "gear",
    input: gear,
    aspectMap: { teeth: 0, material: 1, rotation: 2, label: 3 },
  }],
});
const main = await truth.branch();
if (main.posture === "success") {
  const history = await truth.history(main.value.id);
  if (history.posture === "success" && history.value.toCommitId) {
    await truth.historicalSnapshot({
      branchId: main.value.id,
      commitId: history.value.toCommitId,
    });
  }
  const source = await truth.forkBranch({
    parentBranchId: main.value.id,
    expectedParentBasis: main.value.basis,
    name: "source",
  });
  if (source.posture === "success") {
    await truth.commit({
      requestId: "edit",
      branchId: source.value.id,
      expectedBasis: source.value.basis,
      operations: [{ entityId: "gear", aspectId: "teeth", value: 22 }],
    });
  }
}

// @ts-expect-error LocalTruthBasis is sealed and cannot be constructed structurally.
const forgedBasis: LocalTruthBasis = {
  artifactFamily: "LocalTruthBasis",
  authorityId: "forged",
  schemaIdentity: "forged",
  branchId: "branch:main",
  headCommitId: "forged",
  snapshotId: "forged",
  revision: 0,
  identityDigest: "forged",
};

// @ts-expect-error LocalTruthCommit is sealed and cannot be constructed structurally.
const forgedCommit: LocalTruthCommit = {
  artifactFamily: "LocalTruthCommit",
  id: "forged",
  integrityDigest: "forged",
  authorityId: "forged",
  authorityKind: "typescriptInMemoryLocalTruth",
  schemaIdentity: "forged",
  branchId: "branch:main",
  parentCommitId: null,
  beforeSnapshotId: null,
  afterSnapshotId: "forged",
  kind: "mutation",
  operations: [],
};

void forgedBasis;
void forgedCommit;
await truth.terminate();
await signals.terminate();
