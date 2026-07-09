import {
  ComputedSignal as RawComputedSignal,
  DisposableHandle as RawDisposableHandle,
  default as initWasm,
  InputSignal as RawInputSignal,
  OutputSignal as RawOutputSignal,
  SignalAdapters,
  SignalApp,
  SignalDiagnostics,
  SignalHistory,
  SignalRuntime,
  SignalSpecialist,
  SignalWorkerRuntime,
  Signals as RawSignals,
  SignalsTransaction as RawSignalsTransaction,
  createSignals as createRawSignals,
  WORTHSignalCoreProfile,
  WORTHSignalMaxAspects,
  start,
} from "./worth_signal_wasm.js";

export default async function init(input) {
  await initWasm(input);
  return undefined;
}

export {
  RawComputedSignal as ComputedSignal,
  RawDisposableHandle as DisposableHandle,
  RawInputSignal as InputSignal,
  RawOutputSignal as OutputSignal,
  RawSignals as Signals,
  RawSignalsTransaction as SignalsTransaction,
  RawComputedSignal,
  RawDisposableHandle,
  RawInputSignal,
  RawOutputSignal,
  RawSignals,
  RawSignalsTransaction,
  SignalAdapters,
  SignalApp,
  SignalDiagnostics,
  SignalHistory,
  SignalRuntime,
  SignalSpecialist,
  SignalWorkerRuntime,
  createRawSignals,
  WORTHSignalCoreProfile,
  WORTHSignalMaxAspects,
  start,
};
