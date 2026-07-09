import type { WorkerSnapshot } from "../gear-scene/worker/protocol";
import { applySnapshotToShellSignals } from "./shell_signal_projection";
import {
  SHELL_SIGNAL_BINDINGS,
  SHELL_SIGNAL_KEYS,
  type DemoShellSignalKey,
  type DemoShellSignals,
} from "./shell_signal_schema";
import { SignalValues, type LocalSignalStore } from "./signal_values";

type ShellBatchOp = {
  kind: "set";
  id: string;
  value: unknown;
};

export type ShellStoreApp = LocalSignalStore & {
  batch(ops: ShellBatchOp[]): void;
};

class InMemoryShellStoreApp implements ShellStoreApp {
  private values = new Map<string, unknown>();
  private watchers = new Map<string, Set<(value: unknown) => void>>();

  constructor() {
    for (const key of SHELL_SIGNAL_KEYS) {
      const binding = SHELL_SIGNAL_BINDINGS[key];
      this.values.set(binding.id, binding.initial);
      this.watchers.set(binding.id, new Set());
    }
  }

  read<T>(id: string): T {
    return this.values.get(id) as T;
  }

  batch(ops: ShellBatchOp[]) {
    const changedIds: string[] = [];
    for (const op of ops) {
      if (op.kind !== "set") {
        continue;
      }
      const current = this.values.get(op.id);
      if (Object.is(current, op.value)) {
        continue;
      }
      this.values.set(op.id, op.value);
      changedIds.push(op.id);
    }

    for (const id of changedIds) {
      const next = this.values.get(id);
      for (const listener of this.watchers.get(id) ?? []) {
        listener(next);
      }
    }
  }

  watch<T>(id: string, listener: (value: T) => void, options?: { emitCurrent?: boolean }) {
    let bucket = this.watchers.get(id);
    if (!bucket) {
      bucket = new Set();
      this.watchers.set(id, bucket);
    }
    const typed = listener as (value: unknown) => void;
    bucket.add(typed);
    if (options?.emitCurrent ?? true) {
      typed(this.values.get(id));
    }
    return () => {
      bucket?.delete(typed);
    };
  }
}

export class ShellSignalStore {
  private app: ShellStoreApp | null = null;
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
    this.app = new InMemoryShellStoreApp();
    this.signalValues.reset();
    this.signalValues.attach(this.app);
  }

  applySnapshot(snapshot: WorkerSnapshot, incrementFrameVersion: boolean) {
    if (!this.app) return;
    applySnapshotToShellSignals(this.app, snapshot);
    if (incrementFrameVersion) {
      this.app.batch([
        { kind: "set", id: "uiFrameVersion", value: this.app.read<number>("uiFrameVersion") + 1 },
      ]);
    }
  }

  patchWorkerStatus(patch: Partial<WorkerSnapshot>) {
    if (!this.app) return;
    const ops: ShellBatchOp[] = [];
    if ("error" in patch) {
      ops.push({ kind: "set", id: "uiError", value: patch.error ?? null });
    }
    if ("debugStatus" in patch) {
      ops.push({ kind: "set", id: "uiDebugStatus", value: patch.debugStatus ?? null });
    }
    if (ops.length > 0) {
      this.app.batch(ops);
    }
  }
}
