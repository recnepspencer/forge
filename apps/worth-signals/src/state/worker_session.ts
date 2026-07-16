import type { BranchId, DiagnosticsTier } from "../gear-scene/core/types";
import type { WorkerCommand, WorkerSnapshot } from "../gear-scene/worker/protocol";
import {
  type DemoShellSignalKey,
  type DemoShellSignals,
} from "./shell_signal_schema";
import { ShellSignalStore } from "./shell_signal_store";
import { WorkerTransport } from "./worker_transport";

export class WorkerSession {
  private workerClient: WorkerTransport | null = null;
  private signals = new ShellSignalStore();
  private pendingInspectNode: string | null = null;
  private readonly onFrames: (
    snapshot: WorkerSnapshot,
    frames: Array<{ branchId: BranchId; bitmap: ImageBitmap }>,
    reviewFrames: Array<{ id: string; bitmap: ImageBitmap }>,
  ) => void;

  constructor(
    onFrames: (
      snapshot: WorkerSnapshot,
      frames: Array<{ branchId: BranchId; bitmap: ImageBitmap }>,
      reviewFrames: Array<{ id: string; bitmap: ImageBitmap }>,
    ) => void,
  ) {
    this.onFrames = onFrames;
  }

  getApp() {
    return this.signals.getApp();
  }

  getWorkerClient() {
    return this.workerClient;
  }

  subscribeSignal(key: DemoShellSignalKey, listener: () => void) {
    return this.signals.subscribe(key, listener);
  }

  readSignal<K extends DemoShellSignalKey>(key: K): DemoShellSignals[K] {
    return this.signals.read(key);
  }

  async start() {
    await this.signals.start();
    this.startWorker();
  }

  post(command: WorkerCommand) {
    this.workerClient?.post(command);
  }

  queueTraceJump(index: number) {
    const app = this.signals.getApp();
    if (!app) return;
    const tracedNode = app.read<string | null>("uiTracedNode");
    if (!tracedNode) return;
    this.pendingInspectNode = tracedNode;
    this.post({ type: "scrub", index });
  }

  private startWorker() {
    this.workerClient = new WorkerTransport({
      onDebugStatus: (status) => {
        this.signals.patchWorkerStatus({
          debugStatus: status,
        });
      },
      onSnapshot: (snapshot, hasNewFrames) => {
        this.signals.applySnapshot(snapshot, hasNewFrames);
      },
      onFrames: (snapshot, frames, reviewFrames) => {
        this.onFrames(snapshot, frames, reviewFrames);
      },
      onError: (message) => {
        this.signals.patchWorkerStatus({
          error: message,
          debugStatus: message === "Worker message deserialization failed" ? "worker:message-error" : "worker:error",
        });
      },
      getPendingInspectNode: () => this.pendingInspectNode,
      consumePendingInspectNode: () => {
        const nodeId = this.pendingInspectNode;
        this.pendingInspectNode = null;
        return nodeId;
      },
      onPendingInspectReady: (branchId, nodeId) => {
        this.post({
          type: "inspectNode",
          branchId,
          nodeId,
        });
      },
      getSelectedDiagnosticsTier: () => this.signals.getApp()?.read<DiagnosticsTier>("uiDiagnosticsTier") ?? "webDevelopment",
    });
    this.workerClient.start();
  }
}
