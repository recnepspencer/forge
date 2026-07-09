import { createMutableRawInputHandle } from "./mutable_raw_input_handle.mjs";

export function buildHostRawSignals(runtimeState, calls) {
  return {
    input(id, initial) {
      runtimeState.values.set(id, initial);
      calls.push(["input", id, initial]);
      return createMutableRawInputHandle(id, runtimeState);
    },
    computedSpec(id, spec) {
      calls.push(["computedSpec", id, spec]);
      return createMutableRawInputHandle(id, spec);
    },
    computedCallback(id, callback) {
      const result = callback();
      calls.push(["computedCallback", id, result]);
      return createMutableRawInputHandle(id, result.value);
    },
    outputSpec(id, spec) {
      calls.push(["outputSpec", id, spec]);
      return createMutableRawInputHandle(id, spec);
    },
    read(target) {
      return typeof target === "string"
        ? runtimeState.values.get(target)
        : target.get();
    },
    watch() {
      return { free() {} };
    },
    effect() {
      return { free() {} };
    },
    transaction(callback) {
      const operations = [];
      callback({
        set(target, value) {
          runtimeState.values.set(target.id, value);
          operations.push(["set", target.id, value]);
        },
        free() {},
      });
      calls.push(["transaction", operations]);
      return { touchedNodes: operations.length };
    },
    batch(callback) {
      return this.transaction(callback);
    },
    nuke() {
      return true;
    },
    diagnostics() {
      return {};
    },
    history() {
      return {};
    },
    specialist() {
      return {};
    },
    adapters() {
      return {};
    },
    compatibilityApp() {
      return {};
    },
    compatibilityRuntime() {
      return {};
    },
    free() {
      calls.push(["free"]);
    },
  };
}
