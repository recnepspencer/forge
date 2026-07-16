import type { DiagnosticsTier } from "../gear-scene/core/types";
import type { ReviewManualSelections } from "../gear-scene/worker/protocol";
import type { ShellStoreApp } from "./shell_signal_store";

export function toggleControls(app: ShellStoreApp) {
  app.batch([{ kind: "set", id: "uiControlsOpen", value: !app.read<boolean>("uiControlsOpen") }]);
}

export function setTracedNode(app: ShellStoreApp, nodeId: string | null) {
  app.batch([{ kind: "set", id: "uiTracedNode", value: nodeId }]);
}

export function openWalkthrough(app: ShellStoreApp) {
  app.batch([
    { kind: "set", id: "uiWalkthroughIndex", value: 0 },
    { kind: "set", id: "uiReviewPolicyLane", value: "current" },
    { kind: "set", id: "uiReviewManualSelections", value: {
      teeth: "source",
      outerRadius: "source",
      innerRadius: "source",
      thickness: "source",
      lightIntensity: "source",
      lightPosition: "source",
      rotation: "source",
      camera: "source",
    } },
    { kind: "set", id: "uiWalkthroughOpen", value: true },
  ]);
}

export function closeWalkthrough(app: ShellStoreApp) {
  app.batch([{ kind: "set", id: "uiWalkthroughOpen", value: false }]);
}

export function nextWalkthrough(app: ShellStoreApp, maxIndex: number) {
  const current = app.read<number>("uiWalkthroughIndex");
  app.batch([{ kind: "set", id: "uiWalkthroughIndex", value: Math.min(current + 1, maxIndex) }]);
}

export function prevWalkthrough(app: ShellStoreApp) {
  const current = app.read<number>("uiWalkthroughIndex");
  app.batch([{ kind: "set", id: "uiWalkthroughIndex", value: Math.max(current - 1, 0) }]);
}

export function setDiagnosticsTier(app: ShellStoreApp, tier: DiagnosticsTier) {
  app.batch([{ kind: "set", id: "uiDiagnosticsTier", value: tier }]);
}

export function setReviewPolicyLane(app: ShellStoreApp, lane: string) {
  app.batch([
    { kind: "set", id: "uiReviewPolicyLane", value: lane },
    { kind: "set", id: "uiReviewManualSelections", value: {
      teeth: "source",
      outerRadius: "source",
      innerRadius: "source",
      thickness: "source",
      lightIntensity: "source",
      lightPosition: "source",
      rotation: "source",
      camera: "source",
    } },
  ]);
}

export function setReviewManualSelections(app: ShellStoreApp, selections: ReviewManualSelections) {
  app.batch([{ kind: "set", id: "uiReviewManualSelections", value: selections }]);
}
