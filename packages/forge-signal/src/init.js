import * as wasm from "../pkg/forge_signal_wasm.js";

import { SignalApp } from "./surface/app.js";
import { SignalRuntime } from "./surface/runtime.js";

let initPromise;

export async function initForgeSignal(input) {
  if (!initPromise) {
    initPromise = Promise.resolve(input).then(() => wasm);
  }
  await initPromise;
  return wasm;
}

export async function createSignalApp() {
  await initForgeSignal();
  return new SignalApp(new wasm.SignalApp());
}

export async function createSignalRuntime() {
  await initForgeSignal();
  return new SignalRuntime(new wasm.SignalRuntime());
}

export function currentForgeSignalModule() {
  return wasm;
}
