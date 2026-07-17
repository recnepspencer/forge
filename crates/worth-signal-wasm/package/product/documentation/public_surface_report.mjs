import { readFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";
import {
  assignSurfacesToCoverageGroups,
  collectPublicContractInventory,
  defaultCrateDir,
  publicSurfaceName,
  summarizeCoveragePolicy,
} from "./public_surface_inventory.mjs";

const [policy, semanticLedger] = await Promise.all([
  readFile(path.join(defaultCrateDir, "docs", "metadata", "public-surface-policy.json"), "utf8").then(JSON.parse),
  readFile(path.join(defaultCrateDir, "docs", "metadata", "semantic-contracts.json"), "utf8").then(JSON.parse),
]);
const inventory = collectPublicContractInventory(defaultCrateDir);
const summary = summarizeCoveragePolicy(inventory, policy);
const requestedGroupId = process.argv[2] ?? null;

console.log(
  `Worth Signals public declaration census: ${inventory.surfaces.length} declarations across ${inventory.declarationFiles.length} files`,
);
console.log("group\tstatus\tguide\treference\tdeclarations\tdirect members\tfiles\tcontracts\tsurface digest\tfile digest");
for (const group of policy.groups) {
  const actual = summary.groups.find((candidate) => candidate.id === group.id);
  const contractCount = semanticLedger.contracts.filter((contract) => contract.coverageGroups.includes(group.id)).length;
  console.log([
    group.id,
    group.status,
    group.guideStatus,
    group.referenceStatus,
    actual.declarationCount,
    actual.directMemberCount,
    actual.declarationFileCount,
    contractCount,
    actual.signatureDigest.slice(0, 12),
    actual.declarationFileDigest.slice(0, 12),
  ].join("\t"));
}

if (summary.surfaceCoverage.unassigned.length > 0) {
  console.log("\nUNASSIGNED");
  for (const surface of summary.surfaceCoverage.unassigned) console.log(publicSurfaceName(surface));
}
if (summary.surfaceCoverage.ambiguous.length > 0) {
  console.log("\nAMBIGUOUS");
  for (const item of summary.surfaceCoverage.ambiguous) {
    console.log(`${publicSurfaceName(item.surface)} -> ${item.groups.join(", ")}`);
  }
}
if (summary.fileCoverage.unassigned.length > 0) {
  console.log("\nUNASSIGNED DECLARATION FILES");
  for (const file of summary.fileCoverage.unassigned) console.log(file.source);
}
if (summary.fileCoverage.ambiguous.length > 0) {
  console.log("\nAMBIGUOUS DECLARATION FILES");
  for (const item of summary.fileCoverage.ambiguous) {
    console.log(`${item.file.source} -> ${item.groups.join(", ")}`);
  }
}

if (requestedGroupId) {
  const { assignments } = assignSurfacesToCoverageGroups(inventory.surfaces, policy);
  if (!assignments.has(requestedGroupId)) throw new Error(`Unknown coverage group: ${requestedGroupId}`);
  const requestedGroup = policy.groups.find((group) => group.id === requestedGroupId);
  console.log(`\n${requestedGroupId.toUpperCase()} COVERAGE`);
  console.log(`guide: ${requestedGroup.guideStatus}`);
  console.log(`reference: ${requestedGroup.referenceStatus}`);
  console.log(`canonical docs: ${requestedGroup.canonicalDocs.join(", ")}`);
  console.log(`truth owners: ${requestedGroup.truthOwners.join(", ")}`);
  console.log(`evidence: ${requestedGroup.evidence.join(", ")}`);
  console.log(`\n${requestedGroupId.toUpperCase()} DECLARATIONS`);
  for (const surface of assignments.get(requestedGroupId)) {
    console.log(`${publicSurfaceName(surface)}\t${surface.kind}\t${surface.source}\t${surface.memberCount} members`);
    for (const member of surface.members) console.log(`  ${member}`);
  }
  console.log(`\n${requestedGroupId.toUpperCase()} SEMANTIC CONTRACTS`);
  for (const contract of semanticLedger.contracts.filter((item) => item.coverageGroups.includes(requestedGroupId))) {
    console.log(`${contract.id}: ${contract.guarantee}`);
    for (const limit of contract.nonGuarantees) console.log(`  does not promise: ${limit}`);
  }
}

if (
  summary.surfaceCoverage.unassigned.length > 0
  || summary.surfaceCoverage.ambiguous.length > 0
  || summary.fileCoverage.unassigned.length > 0
  || summary.fileCoverage.ambiguous.length > 0
) process.exitCode = 1;
