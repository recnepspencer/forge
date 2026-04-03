import { createSignalApp, tx, type SignalApp } from "@forge/signal";

import type { WorkerSnapshot } from "../gear-scene/worker/protocol";
import { applySnapshotToShellSignals } from "./shell_signal_projection";
import { registerShellSignals } from "./shell_signal_registration";
import {
  type DemoShellSignalKey,
  type DemoShellSignals,
} from "./shell_signal_schema";
import { SignalValues } from "./signal_values";

export class ShellSignalStore {
  private app: SignalApp | null = null;
  private signalValues = new SignalValues();

  getApp() {
    return this.app;
  }

  subscribe(key: DemoShellSignalKey, listener: () => void) {
    return this.signalValues.subscribe(key, listener);
  }

  read<K extends DemoShellSignalKey>(key: K): DemoShellSignals[K] {
    return this.signalValues.read(key);
  }

  async start() {
    this.app = await createSignalApp();
    this.signalValues.reset();
    registerShellSignals(this.app);
    this.signalValues.attach(this.app);
  }

  applySnapshot(snapshot: WorkerSnapshot, incrementFrameVersion: boolean) {
    if (!this.app) return;
    applySnapshotToShellSignals(this.app, snapshot);
    if (incrementFrameVersion) {
      this.app.batch([tx.set("uiFrameVersion", this.app.read<number>("uiFrameVersion") + 1)]);
    }
  }

  patchWorkerStatus(patch: Partial<WorkerSnapshot>) {
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
}
