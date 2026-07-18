import { readFile } from "node:fs/promises";
import path from "node:path";

import type { Plugin } from "vite";

import { buildGearCodeEvidence } from "../src/ui/demos/gear_code_evidence.ts";

const PUBLIC_MODULE_ID = "virtual:gear-code-evidence";
const RESOLVED_MODULE_ID = `\0${PUBLIC_MODULE_ID}`;

export function gearCodeEvidencePlugin(appRoot: string): Plugin {
  const compositionSectionPath = path.resolve(appRoot, "src/ui/CompositionSection.tsx");
  const gearScenarioPath = path.resolve(appRoot, "src/local-truth-gear/gear_scenario.ts");

  return {
    name: "worth-gear-code-evidence",
    enforce: "pre",
    resolveId(id) {
      return id === PUBLIC_MODULE_ID ? RESOLVED_MODULE_ID : undefined;
    },
    async load(id) {
      if (id !== RESOLVED_MODULE_ID) return undefined;

      this.addWatchFile(compositionSectionPath);
      this.addWatchFile(gearScenarioPath);
      const [compositionSection, gearScenario] = await Promise.all([
        readFile(compositionSectionPath, "utf8"),
        readFile(gearScenarioPath, "utf8"),
      ]);
      const evidence = buildGearCodeEvidence({ compositionSection, gearScenario });

      return `export const GEAR_ASPECT_CODE = ${JSON.stringify(evidence)};`;
    },
  };
}
