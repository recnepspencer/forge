import type { BranchId, DiagnosticsTier } from "../gear-scene/core/types";
import type { WorkerCommand } from "../gear-scene/worker/protocol";
import type { DemoShellSignalKey } from "./shell_signal_schema";
import { BranchFrames } from "./branch_frames";
import {
  closeWalkthrough,
  nextWalkthrough,
  openWalkthrough,
  prevWalkthrough,
  setDiagnosticsTier,
  setTracedNode,
  toggleControls,
} from "./ui_mutations";
import { WorkerSession } from "./worker_session";

class RuntimeAccess {
  private frames = new BranchFrames();
  private started = false;
  private runtime = new WorkerSession((snapshot, incomingFrames) => {
    this.frames.update(snapshot, incomingFrames);
  });

  subscribeSignal(key: DemoShellSignalKey, listener: () => void) {
    this.ensureStarted();
    return this.runtime.subscribeSignal(key, listener);
  }

  readSignal<K extends DemoShellSignalKey>(key: K) {
    this.ensureStarted();
    return this.runtime.readSignal(key);
  }

  getFrame(branchId: BranchId) {
    return this.frames.get(branchId);
  }

  command(command: WorkerCommand) {
    this.ensureStarted();
    this.runtime.post(command);
  }

  toggleControls() {
    this.ensureStarted();
    const app = this.runtime.getApp();
    if (!app) return;
    toggleControls(app);
  }

  setTracedNode(nodeId: string | null) {
    this.ensureStarted();
    const app = this.runtime.getApp();
    if (!app) return;
    setTracedNode(app, nodeId);
  }

  openWalkthrough() {
    this.ensureStarted();
    const app = this.runtime.getApp();
    if (!app) return;
    openWalkthrough(app);
  }

  setDiagnosticsTier(tier: DiagnosticsTier) {
    this.ensureStarted();
    const app = this.runtime.getApp();
    if (!app) return;
    setDiagnosticsTier(app, tier);
    this.runtime.getWorkerClient()?.post({ type: "setDiagnosticsTier", tier });
  }

  closeWalkthrough() {
    this.ensureStarted();
    const app = this.runtime.getApp();
    if (!app) return;
    closeWalkthrough(app);
  }

  nextWalkthrough(maxIndex: number) {
    this.ensureStarted();
    const app = this.runtime.getApp();
    if (!app) return;
    nextWalkthrough(app, maxIndex);
  }

  prevWalkthrough() {
    this.ensureStarted();
    const app = this.runtime.getApp();
    if (!app) return;
    prevWalkthrough(app);
  }

  jumpToTrace(index: number) {
    this.ensureStarted();
    this.runtime.queueTraceJump(index);
  }

  private ensureStarted() {
    if (this.started) {
      return;
    }
    this.started = true;
    void this.runtime.start();
  }
}

export const runtimeAccess = new RuntimeAccess();
