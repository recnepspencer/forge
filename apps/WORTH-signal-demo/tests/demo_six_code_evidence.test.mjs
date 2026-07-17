import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import { buildGearCodeEvidence } from "../src/ui/demos/gear_code_evidence.ts";

test("Demo 6 code evidence is extracted from its production value and merge path", async () => {
  const [compositionSection, gearScenario] = await Promise.all([
    readFile(new URL("../src/ui/CompositionSection.tsx", import.meta.url), "utf8"),
    readFile(new URL("../src/local-truth-gear/gear_scenario.ts", import.meta.url), "utf8"),
  ]);

  const evidence = buildGearCodeEvidence({ compositionSection, gearScenario });

  assert.match(evidence, /commitBranchAspect/u);
  assert.match(evidence, /\{ \[aspect\]: value \}/u);
  assert.match(evidence, /designOperations\(committedGear, patch\)/u);
  assert.match(evidence, /truth\.commit\(\{/u);
  assert.match(evidence, /truth\.previewMerge\(\{/u);
  assert.match(evidence, /truth\.resolveMerge\(\{/u);
  assert.doesNotMatch(evidence, /value:\s*0\.\d+|value:\s*\d{2,}/u);
  assert.doesNotMatch(evidence, /Object\.assign|\.\.\.target|\.\.\.source/u);
});

test("Demo 6 renders committed Local Truth views instead of React-owned gear drafts", async () => {
  const compositionSection = await readFile(
    new URL("../src/ui/CompositionSection.tsx", import.meta.url),
    "utf8",
  );

  assert.doesNotMatch(compositionSection, /GearDrafts|setDrafts|drafts\./u);
  assert.match(compositionSection, /values=\{view\.main\}/u);
  assert.match(compositionSection, /values=\{view\.design\}/u);
  assert.match(compositionSection, /disabled=\{!canEditBranches \|\| busy\}/u);
});
