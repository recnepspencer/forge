import type { DiagnosticsTier } from "../gear-scene/core/types";
import type { BranchId } from "../gear-scene/core/types";
import type { WorkerCommand, WorkerEvent, WorkerSnapshot } from "../gear-scene/worker/protocol";

const DEBUG_CONSOLE = true;

export type WorkerTransportOptions = {
  onDebugStatus: (status: string) => void;
  onSnapshot: (snapshot: WorkerSnapshot, hasNewFrames: boolean) => void;
  onFrames: (
    snapshot: WorkerSnapshot,
    frames: Array<{ branchId: BranchId; bitmap: ImageBitmap }>,
    reviewFrames: Array<{ id: string; bitmap: ImageBitmap }>,
  ) => void;
  onError: (message: string) => void;
  onPendingInspectReady: (branchId: BranchId, nodeId: string) => void;
  getPendingInspectNode: () => string | null;
  consumePendingInspectNode: () => string | null;
  getSelectedDiagnosticsTier: () => DiagnosticsTier;
};

export class WorkerTransport {
  private worker: Worker | null = null;
  private initPosted = false;
  private readonly options: WorkerTransportOptions;

  constructor(options: WorkerTransportOptions) {
    this.options = options;
  }

  start() {
    const worker = new Worker(new URL("../gear-scene/worker/demo-worker.ts", import.meta.url), {
      type: "module",
    });
    this.worker = worker;

    worker.onerror = (event) => {
      console.error("[forge-signal-demo] worker error", event);
      this.options.onError(event.message || "Worker failed to load");
      this.options.onDebugStatus("worker:error");
    };

    worker.onmessageerror = () => {
      this.options.onError("Worker message deserialization failed");
      this.options.onDebugStatus("worker:message-error");
    };

    worker.onmessage = (event: MessageEvent<WorkerEvent>) => {
      const message = event.data;
      if (message.type === "debug") {
        if (DEBUG_CONSOLE) {
          console.log(
            "[forge-signal-demo]",
            message.phase,
            message.detail ? `- ${message.detail}` : "",
            message.elapsedMs != null ? `(${message.elapsedMs.toFixed(1)} ms)` : "",
          );
        }
        if (message.phase === "worker:handler-attached" && !this.initPosted) {
          this.initPosted = true;
          this.post({ type: "init" });
          const selectedTier = this.options.getSelectedDiagnosticsTier();
          if (selectedTier !== "webDevelopment") {
            this.post({ type: "setDiagnosticsTier", tier: selectedTier });
          }
        }
        this.options.onDebugStatus(`${message.phase}${message.detail ? ` - ${message.detail}` : ""}`);
        return;
      }

      if (message.type === "error") {
        this.options.onError(message.error);
        return;
      }

      this.options.onFrames(message.snapshot, message.frames, message.reviewFrames);
      this.options.onSnapshot(message.snapshot, message.frames.length > 0 || message.reviewFrames.length > 0);

      if (
        this.options.getPendingInspectNode()
        && message.snapshot.ready
        && message.snapshot.activeBranchId != null
      ) {
        const nodeId = this.options.consumePendingInspectNode();
        if (nodeId) {
          this.options.onPendingInspectReady(message.snapshot.activeBranchId, nodeId);
        }
      }
    };
  }

  post(command: WorkerCommand) {
    this.worker?.postMessage(command);
  }
}
