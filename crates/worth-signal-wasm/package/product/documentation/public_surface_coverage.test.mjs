import assert from "node:assert/strict";
import { readFile, stat } from "node:fs/promises";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import {
  assignDeclarationFilesToCoverageGroups,
  assignSurfacesToCoverageGroups,
  collectPublicContractInventory,
  publicSurfaceName,
  summarizeCoveragePolicy,
} from "./public_surface_inventory.mjs";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const crateDir = path.resolve(testDir, "..", "..", "..");
const docsDir = path.join(crateDir, "docs");
const policyPath = path.join(docsDir, "metadata", "public-surface-policy.json");
const contractsPath = path.join(docsDir, "metadata", "semantic-contracts.json");
const allowedStatuses = new Set(["compatibility", "mixed", "stable"]);
const allowedGuideStatuses = new Set(["compatibility", "current", "rewrite-needed"]);
const allowedReferenceStatuses = new Set(["compatibility", "complete", "incomplete"]);

async function readJson(filePath) {
  return JSON.parse(await readFile(filePath, "utf8"));
}

async function assertPathExists(relativePath, context) {
  const pathStat = await stat(path.join(crateDir, relativePath));
  assert.equal(pathStat.isFile() || pathStat.isDirectory(), true, context);
}

async function assertDocExists(docPath, context) {
  const pathStat = await stat(path.join(docsDir, `${docPath}.md`));
  assert.equal(pathStat.isFile(), true, context);
}

function assertUnique(values, context) {
  assert.equal(new Set(values).size, values.length, context);
}

test("every declaration surface belongs to one frozen documentation group", async () => {
  const policy = await readJson(policyPath);
  const inventory = collectPublicContractInventory(crateDir);
  const summary = summarizeCoveragePolicy(inventory, policy);

  assert.equal(policy.schemaVersion, 1);
  assert.equal(policy.product, "worth-signals-wasm");
  assert.deepEqual(policy.entrypoints, inventory.entrypoints, "published entrypoints need explicit coverage");
  assert.deepEqual(
    summary.surfaceCoverage.unassigned.map(publicSurfaceName),
    [],
    "new public declarations need an explicit documentation owner",
  );
  assert.deepEqual(
    summary.surfaceCoverage.ambiguous,
    [],
    "public declarations cannot have competing documentation owners",
  );
  assert.deepEqual(
    summary.fileCoverage.unassigned.map((file) => file.source),
    [],
    "reachable declaration files need an explicit documentation owner",
  );
  assert.deepEqual(
    summary.fileCoverage.ambiguous,
    [],
    "reachable declaration files cannot have competing documentation owners",
  );
  assertUnique(policy.groups.map((group) => group.id), "coverage group ids must be unique");

  for (const group of policy.groups) {
    const actual = summary.groups.find((candidate) => candidate.id === group.id);
    assert.deepEqual(
      actual,
      { id: group.id, ...group.baseline },
      `${group.id} changed; review its docs and deliberately update the baseline`,
    );
    assert.equal(actual.declarationCount > 0, true, `${group.id} is an empty filing category`);
    assert.equal(allowedStatuses.has(group.status), true, `${group.id} has an unknown support status`);
    assert.equal(allowedGuideStatuses.has(group.guideStatus), true, `${group.id} has an unknown guide status`);
    assert.equal(
      allowedReferenceStatuses.has(group.referenceStatus),
      true,
      `${group.id} has an unknown reference status`,
    );
    assert.equal(group.canonicalDocs.length > 0, true, `${group.id} lacks a canonical document`);
    assert.equal(group.truthOwners.length > 0, true, `${group.id} lacks an implementation truth owner`);
    assert.equal(group.evidence.length > 0, true, `${group.id} lacks evidence`);
    for (const docPath of group.canonicalDocs) await assertDocExists(docPath, `${group.id} doc is missing`);
    for (const owner of group.truthOwners) await assertPathExists(owner, `${group.id} truth owner is missing`);
    for (const evidence of group.evidence) await assertPathExists(evidence, `${group.id} evidence is missing`);
  }
});

test("semantic promises that declarations cannot express remain explicit and evidenced", async () => {
  const [ledger, policy] = await Promise.all([readJson(contractsPath), readJson(policyPath)]);
  const coverageGroupIds = new Set(policy.groups.map((group) => group.id));
  assert.equal(ledger.schemaVersion, 1);
  assert.equal(ledger.product, "worth-signals-wasm");
  assertUnique(ledger.contracts.map((contract) => contract.id), "semantic contract ids must be unique");

  for (const contract of ledger.contracts) {
    assert.match(contract.id, /^[a-z][a-z0-9-]+\.[a-z][a-z0-9-]+$/u);
    assert.equal(contract.coverageGroups.length > 0, true, `${contract.id} lacks a coverage group`);
    for (const groupId of contract.coverageGroups) {
      assert.equal(coverageGroupIds.has(groupId), true, `${contract.id} names unknown group ${groupId}`);
    }
    assert.equal(allowedStatuses.has(contract.status), true, `${contract.id} has an unknown support status`);
    assert.equal(contract.guarantee.length > 30, true, `${contract.id} lacks a useful guarantee`);
    assert.equal(contract.nonGuarantees.length > 0, true, `${contract.id} hides its limits`);
    assert.equal(contract.requiredDocTerms.length > 0, true, `${contract.id} lacks documentation anchors`);
    assert.equal(contract.evidence.length > 0, true, `${contract.id} lacks evidence`);
    await assertDocExists(contract.canonicalDoc, `${contract.id} canonical doc is missing`);
    const canonicalDoc = (await readFile(path.join(docsDir, `${contract.canonicalDoc}.md`), "utf8")).toLowerCase();
    for (const term of contract.requiredDocTerms) {
      assert.equal(
        canonicalDoc.includes(term.toLowerCase()),
        true,
        `${contract.id} is absent from its canonical doc: ${term}`,
      );
    }
    await assertPathExists(contract.truthOwner, `${contract.id} truth owner is missing`);
    for (const evidence of contract.evidence) await assertPathExists(evidence, `${contract.id} evidence is missing`);
  }
  const coveredGroups = new Set(ledger.contracts.flatMap((contract) => contract.coverageGroups));
  assert.deepEqual(
    [...coveredGroups].sort(),
    [...coverageGroupIds].sort(),
    "every declaration family needs at least one explicit semantic contract",
  );
});

test("coverage enforcement rejects additions, signature drift, and unowned declarations", async () => {
  const policy = await readJson(policyPath);
  const inventory = collectPublicContractInventory(crateDir);
  const baseline = summarizeCoveragePolicy(inventory, policy);
  const resourceBaseline = baseline.groups.find((group) => group.id === "resources");

  const addedSurface = {
    ...inventory.surfaces.find((surface) => surface.source.startsWith("package/types/resource")),
    exportName: "UndocumentedResourceSurface",
    signature: "export interface UndocumentedResourceSurface { value(): string; }",
  };
  const withAddition = summarizeCoveragePolicy(
    { ...inventory, surfaces: [...inventory.surfaces, addedSurface] },
    policy,
  );
  assert.notDeepEqual(withAddition.groups.find((group) => group.id === "resources"), resourceBaseline);

  const changedSurfaces = inventory.surfaces.map((surface, index) =>
    index === 0 ? { ...surface, signature: `${surface.signature} changed(): void;` } : surface,
  );
  assert.notDeepEqual(
    summarizeCoveragePolicy({ ...inventory, surfaces: changedSurfaces }, policy).groups,
    baseline.groups,
  );

  const unknownSurface = { ...addedSurface, source: "package/types/unowned_surface.d.ts" };
  const surfaceAssignments = assignSurfacesToCoverageGroups([...inventory.surfaces, unknownSurface], policy);
  assert.deepEqual(surfaceAssignments.unassigned.map(publicSurfaceName), ["root:UndocumentedResourceSurface"]);

  const transitiveOnlyPath = "package/types/resource/resource_verification.d.ts";
  assert.equal(
    inventory.surfaces.some((surface) => surface.source === transitiveOnlyPath),
    false,
    "the transitive-file test must not reuse a top-level export source",
  );
  const changedFiles = inventory.declarationFiles.map((file) =>
    file.source === transitiveOnlyPath ? { ...file, signature: `${file.signature} changed` } : file,
  );
  const fileDrift = summarizeCoveragePolicy({ ...inventory, declarationFiles: changedFiles }, policy);
  const resourceFileDrift = fileDrift.groups.find((group) => group.id === "resources");
  assert.equal(resourceFileDrift.signatureDigest, resourceBaseline.signatureDigest);
  assert.notEqual(resourceFileDrift.declarationFileDigest, resourceBaseline.declarationFileDigest);
  const fileAssignments = assignDeclarationFilesToCoverageGroups(
    [...inventory.declarationFiles, { source: "package/types/unowned_surface.d.ts", signature: "unknown" }],
    policy,
  );
  assert.deepEqual(fileAssignments.unassigned.map((file) => file.source), ["package/types/unowned_surface.d.ts"]);
});
