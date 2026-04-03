import type { SignalApp } from "@forge/signal";

import {
  SHELL_SIGNAL_BINDINGS,
  SHELL_SIGNAL_KEYS,
  type DemoShellSignalKey,
  type DemoShellSignals,
} from "./shell_signal_schema";

export class SignalValues {
  private values = new Map<DemoShellSignalKey, unknown>();
  private listeners = new Map<DemoShellSignalKey, Set<() => void>>();
  private watchUnsubs: Array<() => void> = [];

  constructor() {
    this.reset();
  }

  subscribe(key: DemoShellSignalKey, listener: () => void) {
    let bucket = this.listeners.get(key);
    if (!bucket) {
      bucket = new Set();
      this.listeners.set(key, bucket);
    }
    bucket.add(listener);
    return () => {
      bucket.delete(listener);
    };
  }

  read<K extends DemoShellSignalKey>(key: K): DemoShellSignals[K] {
    return this.values.get(key) as DemoShellSignals[K];
  }

  reset() {
    this.values = new Map();
    for (const key of SHELL_SIGNAL_KEYS) {
      this.values.set(key, SHELL_SIGNAL_BINDINGS[key].initial);
      if (!this.listeners.has(key)) {
        this.listeners.set(key, new Set());
      }
    }
  }

  attach(app: SignalApp) {
    for (const unsubscribe of this.watchUnsubs) {
      unsubscribe();
    }
    this.watchUnsubs = [];

    for (const key of SHELL_SIGNAL_KEYS) {
      const id = SHELL_SIGNAL_BINDINGS[key].id;
      const unsubscribe = app.watch(id, (value) => {
        this.values.set(key, value);
        for (const listener of this.listeners.get(key) ?? []) {
          listener();
        }
      });
      this.watchUnsubs.push(unsubscribe);
    }
  }
}
