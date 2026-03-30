import * as wasmModule from "@forge/signal/wasm";

import { SignalApp } from "./surface/app.js";
import { SignalRuntime } from "./surface/runtime.js";

let initPromise;

export async function initForgeSignal() {
  if (!initPromise) {
    initPromise = Promise.resolve(wasmModule);
  }

  return initPromise;
}

export async function createSignalApp() {
  const wasm = await initForgeSignal();
  return new SignalApp(new wasm.SignalApp());
}

export async function createSignalRuntime() {
  const wasm = await initForgeSignal();
  return new SignalRuntime(new wasm.SignalRuntime());
}

export async function currentForgeSignalModule() {
  return initForgeSignal();
}
