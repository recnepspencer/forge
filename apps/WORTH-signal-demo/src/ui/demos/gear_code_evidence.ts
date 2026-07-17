interface GearCodeSources {
  compositionSection: string;
  gearScenario: string;
}

interface SourceExcerpt {
  end: string;
  file: string;
  source: string;
  start: string;
}

export function buildGearCodeEvidence(sources: GearCodeSources) {
  return [
    excerpt({
      file: "CompositionSection.tsx",
      source: sources.compositionSection,
      start: "  const commitBranchAspect = (",
      end: "\n\n  const forkBranches =",
    }),
    excerpt({
      file: "gear_scenario.ts",
      source: sources.gearScenario,
      start: "      const branchId = role === \"main\" ? main.id : activeDesignBranchId;",
      end: "\n      const aspectNames =",
    }),
    excerpt({
      file: "gear_scenario.ts",
      source: sources.gearScenario,
      start: "  async function commitOperations(",
      end: "\n\n  async function buildScenarioView()",
    }),
    excerpt({
      file: "gear_scenario.ts",
      source: sources.gearScenario,
      start: "    async resolveMerge(",
      end: "\n    async terminate()",
    }),
    excerpt({
      file: "gear_scenario.ts",
      source: sources.gearScenario,
      start: "  async function previewDesignBranchMerge(",
      end: "\n\n  async function commitActiveReview(",
    }),
    excerpt({
      file: "gear_scenario.ts",
      source: sources.gearScenario,
      start: "  async function commitActiveReview(",
      end: "\n    activeDesignBranchId = null;",
    }),
  ].join("\n\n");
}

function excerpt({ end, file, source, start }: SourceExcerpt) {
  const startIndex = source.indexOf(start);
  const endIndex = source.indexOf(end, startIndex);
  if (startIndex < 0 || endIndex < 0) {
    throw new Error(`Demo 6 code evidence is missing its ${file} production excerpt.`);
  }
  return `// ${file} — production source\n${dedent(source.slice(startIndex, endIndex))}`;
}

function dedent(source: string) {
  const lines = source.split("\n");
  const indentation = Math.min(
    ...lines.filter((line) => line.trim()).map((line) => line.match(/^\s*/)?.[0].length ?? 0),
  );
  return lines.map((line) => line.slice(indentation)).join("\n");
}
