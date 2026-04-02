import type { WorkerSnapshot } from "../gear-scene/worker/protocol";

export function friendlyScenarioMode(mode: "manual-gear" | "adversarial-gear-merge"): string {
  return mode === "manual-gear" ? "Manual Gear" : "Adversarial Arena";
}

export function friendlyScenarioStatus(status: string): string {
  switch (status) {
    case "idle": return "Idle";
    case "scripted": return "Script Loaded";
    case "planned": return "Plan Ready";
    case "merged": return "Merge Executed";
    case "replayed": return "Replay Verified";
    default: return status;
  }
}

export function friendlyDiagnosticsTier(tier: string): string {
  switch (tier) {
    case "webDevelopment": return "Web Dev";
    case "development": return "Development";
    case "forensic": return "Forensic";
    case "kernel": return "Kernel";
    default: return tier;
  }
}

export function friendlyReplayStatus(proof: WorkerSnapshot["scenario"] extends infer T ? T extends { proof: infer P } ? P : never : never) {
  if (!proof) {
    return "Pending";
  }
  if (proof.replayParity == null) {
    return "Pending";
  }
  return proof.replayParity ? "Parity Pass" : "Parity Fail";
}

export function describeScenarioStatus(
  scenario: NonNullable<WorkerSnapshot["scenario"]>,
  diagnosticsTier: "webDevelopment" | "development" | "forensic" | "kernel",
): string {
  if (scenario.mode === "manual-gear") {
    return `Manual Gear mode is active. Use the sliders, branch, and merge controls directly. Diagnostics tier is ${friendlyDiagnosticsTier(diagnosticsTier)}.`;
  }
  switch (scenario.status) {
    case "idle":
      return "The adversarial arena is armed but no scripted branch pressure has been applied yet.";
    case "scripted":
      return "The hostile branch edits have been applied. You can inspect the prepared merge or execute it.";
    case "planned":
      return "The merge plan has been lowered. The runtime has already decided which policies apply before execution.";
    case "merged":
      return "The merge has executed. Use the conflict walkthrough to see why each conflict resolved the way it did.";
    case "replayed":
      return "The merged runtime has been rebuilt from timeline state and compared against the canonical merge artifacts.";
    default:
      return scenario.lastAction;
  }
}
