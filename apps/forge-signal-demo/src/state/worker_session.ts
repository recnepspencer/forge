import { createSignalApp, tx, type SignalApp } from "@forge/signal";

import type { BranchId, DiagnosticsTier } from "../gear-scene/core/types";
import type { WorkerCommand, WorkerSnapshot } from "../gear-scene/worker/protocol";
import { applySnapshotToShellSignals } from "./shell_signal_projection";
import { registerShellSignals } from "./shell_signal_registration";
import {
  type DemoShellSignalKey,
  type DemoShellSignals,
} from "./shell_signal_schema";
import { SignalCache } from "./signal_cache";
import { WorkerTransport } from "./worker_transport";

export class WorkerSession {
  private app: SignalApp | null = null;
  private workerClient: WorkerTransport | null = null;
  private signalCache = new SignalCache();
  private pendingInspectNode: string | null = null;
  private readonly onFrames: (
    snapshot: WorkerSnapshot,
    frames: Array<{ branchId: BranchId; bitmap: ImageBitmap }>,
  ) => void;

  constructor(
    onFrames: (snapshot: WorkerSnapshot, frames: Array<{ branchId: BranchId; bitmap: ImageBitmap }>) => void,
  ) {
    this.onFrames = onFrames;
  }

  getApp() {
    return this.app;
  }

  getWorkerClient() {
    return this.workerClient;
  }

  subscribeSignal(key: DemoShellSignalKey, listener: () => void) {
    return this.signalCache.subscribe(key, listener);
  }

  readSignal<K extends DemoShellSignalKey>(key: K): DemoShellSignals[K] {
    return this.signalCache.read(key);
  }

  async start() {
    this.app = await createSignalApp();
    this.signalCache.reset();
    registerShellSignals(this.app);
    this.signalCache.attach(this.app);
    this.startWorker();
  }

  post(command: WorkerCommand) {
    this.workerClient?.post(command);
  }

  queueTraceJump(index: number) {
    if (!this.app) return;
    const tracedNode = this.app.read<string | null>("uiTracedNode");
    if (!tracedNode) return;
    this.pendingInspectNode = tracedNode;
    this.post({ type: "scrub", index });
  }

  private startWorker() {
    this.workerClient = new WorkerTransport({
      onDebugStatus: (status) => {
        this.patchWorkerStatus({
          debugStatus: status,
        });
      },
      onSnapshot: (snapshot, hasNewFrames) => {
        this.setSnapshot(snapshot, hasNewFrames);
      },
      onFrames: (snapshot, frames) => {
        this.onFrames(snapshot, frames);
      },
      onError: (message) => {
        this.patchWorkerStatus({
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
      getSelectedDiagnosticsTier: () => this.app?.read<DiagnosticsTier>("uiDiagnosticsTier") ?? "webDevelopment",
    });
    this.workerClient.start();
  }

  private patchWorkerStatus(patch: Partial<WorkerSnapshot>) {
    if (!this.app) return;
    const ops = [];
    if ("error" in patch) {
      ops.push(tx.set("uiError", patch.error ?? null));
    }
    if ("debugStatus" in patch) {
      ops.push(tx.set("uiDebugStatus", patch.debugStatus ?? null));
    }
    if (ops.length > 0) {
      this.app.batch(ops);
    }
  }

  private setSnapshot(snapshot: WorkerSnapshot, incrementFrameVersion: boolean) {
    if (!this.app) return;
    applySnapshotToShellSignals(this.app, snapshot);
    if (incrementFrameVersion) {
      this.app.batch([tx.set("uiFrameVersion", this.app.read<number>("uiFrameVersion") + 1)]);
    }
  }
}
