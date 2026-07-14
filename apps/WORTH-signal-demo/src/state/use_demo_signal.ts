import { useSyncExternalStore } from "react";

import type { SignalSlice } from "./demo_reads";

export function useDemoSignal<T>(slice: SignalSlice<T>): T {
  return useSyncExternalStore(slice.subscribe, slice.getSnapshot, slice.getSnapshot);
}
