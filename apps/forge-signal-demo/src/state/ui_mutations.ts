import { tx, type SignalApp } from "@forge/signal";

import type { DiagnosticsTier } from "../gear-scene/core/types";

export function toggleControls(app: SignalApp) {
  app.batch([tx.set("uiControlsOpen", !app.read<boolean>("uiControlsOpen"))]);
}

export function setTracedNode(app: SignalApp, nodeId: string | null) {
  app.batch([tx.set("uiTracedNode", nodeId)]);
}

export function openWalkthrough(app: SignalApp) {
  app.batch([
    tx.set("uiWalkthroughIndex", 0),
    tx.set("uiReviewPolicyLane", "current"),
    tx.set("uiReviewManualChoice", "source"),
    tx.set("uiWalkthroughOpen", true),
  ]);
}

export function closeWalkthrough(app: SignalApp) {
  app.batch([tx.set("uiWalkthroughOpen", false)]);
}

export function nextWalkthrough(app: SignalApp, maxIndex: number) {
  const current = app.read<number>("uiWalkthroughIndex");
  app.batch([tx.set("uiWalkthroughIndex", Math.min(current + 1, maxIndex))]);
}

export function prevWalkthrough(app: SignalApp) {
  const current = app.read<number>("uiWalkthroughIndex");
  app.batch([tx.set("uiWalkthroughIndex", Math.max(current - 1, 0))]);
}

export function setDiagnosticsTier(app: SignalApp, tier: DiagnosticsTier) {
  app.batch([tx.set("uiDiagnosticsTier", tier)]);
}

export function setReviewPolicyLane(app: SignalApp, lane: string) {
  app.batch([
    tx.set("uiReviewPolicyLane", lane),
    tx.set("uiReviewManualChoice", "source"),
  ]);
}

export function setReviewManualChoice(app: SignalApp, choice: "source" | "target") {
  app.batch([tx.set("uiReviewManualChoice", choice)]);
}
