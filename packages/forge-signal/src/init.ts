import * as wasmModule from "../pkg/forge_signal_wasm.js";

import { SignalApp } from "./surface/app.ts";
import { SignalRuntime } from "./surface/runtime.ts";

let initPromise: Promise<typeof wasmModule> | undefined;

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
