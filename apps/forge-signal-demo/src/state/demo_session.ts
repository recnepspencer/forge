import type { BranchId } from "../gear-scene/core/types";
import type { WorkerCommand } from "../gear-scene/worker/protocol";
import type { DemoShellSignalKey } from "./shell_signal_schema";
import { BranchFrames } from "./branch_frames";
import { WorkerSession } from "./worker_session";

class DemoSession {
  private frames = new BranchFrames();
  private started = false;
  private workerSession = new WorkerSession((snapshot, incomingFrames, incomingReviewFrames) => {
    this.frames.update(snapshot, incomingFrames, incomingReviewFrames);
  });

  subscribeSignal(key: DemoShellSignalKey, listener: () => void) {
    this.ensureStarted();
    return this.workerSession.subscribeSignal(key, listener);
  }

  readSignal<K extends DemoShellSignalKey>(key: K) {
    this.ensureStarted();
    return this.workerSession.readSignal(key);
  }

  getFrame(branchId: BranchId) {
    return this.frames.get(branchId);
  }

  getReviewFrame(frameId: string) {
    return this.frames.getReview(frameId);
  }

  command(command: WorkerCommand) {
    this.ensureStarted();
    this.workerSession.post(command);
  }

  getApp() {
    this.ensureStarted();
    return this.workerSession.getApp();
  }

  getWorkerClient() {
    this.ensureStarted();
    return this.workerSession.getWorkerClient();
  }

  jumpToTrace(index: number) {
    this.ensureStarted();
    this.workerSession.queueTraceJump(index);
  }

  private ensureStarted() {
    if (this.started) {
      return;
    }
    this.started = true;
    void this.workerSession.start();
  }
}

export const demoSession = new DemoSession();
