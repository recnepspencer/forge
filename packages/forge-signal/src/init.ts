import { SignalApp } from "./surface/app.ts";
import { SignalRuntime } from "./surface/runtime.ts";

type WasmModule = typeof import("@forge/signal/wasm");

let initPromise: Promise<WasmModule> | undefined;

export async function initForgeSignal() {
  if (!initPromise) {
    initPromise = import("@forge/signal/wasm");
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
